use std::{error::Error, collections::HashMap};
use std::{fs::{File, OpenOptions}, io::{self, BufRead, BufReader, Write}, path::Path, process::Command};
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Num;

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    a: Vec<String>,
    b: Vec<Vec<String>>,
    c: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProofFile {
    scheme: String,
    curve: String,
    proof: Proof,
    inputs: Vec<String>,
}

fn parse_proof_and_input() -> Result<(Proof, Vec<String>), Box<dyn Error>> {
    // Open the proof.json file
    let file = File::open("proof.json")?;
    let reader = BufReader::new(file);

    // Deserialize the JSON data
    let proof_file: ProofFile = serde_json::from_reader(reader)?;

    Ok((proof_file.proof, proof_file.inputs))
}

fn main() -> io::Result<()> {
    // Check if `whitelist.zok` exists, if not create it
    if !Path::new("whitelist.zok").exists() {
        let contents = r#"
import "hashes/sha256/512bitPacked" as sha256packed;

def main(private field a, private field b, public field c, public field d) -> bool{
    assert(a == c);
    assert(b == d);
    return true;
}
"#;
        File::create("whitelist.zok")?.write_all(contents.as_bytes())?;
    }

    // Compile the program
    let output = Command::new("zokrates")
        .arg("compile")
        .arg("-i")
        .arg("whitelist.zok")
        .output()?;
    assert!(output.status.success());

    // Perform the setup phase
    let output = Command::new("zokrates").arg("setup").output()?;
    assert!(output.status.success());

    // Export a solidity verifier
    let output = Command::new("zokrates").arg("export-verifier").output()?;
    assert!(output.status.success());

    // Open the addresses.txt file
    let file = File::open("addresses.txt")?;
    let reader = io::BufReader::new(file);

    // Create a hashmap to hold all the data
    let mut all_data: HashMap<String, serde_json::Value> = HashMap::new();

    // Loop through each line in addresses.txt
    for line in reader.lines() {
        let address = line?;
        // Remove the "0x" prefix and parse the hex string as a U512
        let decimal = BigUint::from_str_radix(&address[2..], 16).unwrap();
        let decimal_str = decimal.to_string();
        let mid = decimal_str.len() / 2;
        let (a, b) = decimal_str.split_at(mid);
        let (c, d) = (a.to_string(), b.to_string());

        // Compute witness
        let _output = Command::new("zokrates")
        .arg("compute-witness")
        .arg("-a")
        .args(&[&a.to_string(), &b.to_string(), &c.to_string(), &d.to_string()])
        .output()?;
        // assert!(output.status.success(), "compute witness failed");

          // Generate the proofs
        let output = Command::new("zokrates").arg("generate-proof").output()?;
        assert!(output.status.success());

        // Parse the proof and input from proof.json
        match parse_proof_and_input() {
            Ok((proof, input)) => {
                all_data.insert(
                    address.clone(),
                    serde_json::json!({
                        "proof": [
                            proof.a,
                            proof.b,
                            proof.c
                        ],
                        "inputs": input
                    }),
                );
            }
            Err(e) => eprintln!("Failed to parse proof and input: {}", e),
        }
    }

    // Open the address-proof.json file for writing
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("address-proof.json")?;
    let mut writer = io::BufWriter::new(file);

    // Serialize the hashmap to a formatted JSON string
    let json_string = serde_json::to_string_pretty(&all_data)?;

    // Write the JSON string to the file
    writer.write_all(json_string.as_bytes())?;

    Ok(())
}
