// Importing necessary libraries and modules
use std::{collections::HashMap, error::Error};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, ErrorKind, Write},
    path::Path,
    process::{Command, Output},
};
// Extern crate declarations for using external libraries
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Num;
// Struct definitions for Proof and ProofFile
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
// Entry point of the program
fn main() -> io::Result<()> {
    // Generating a zokrates program file if it doesn't exist
    generate_zok_file()?;
    // Running zokrates commands for compilation, setup, and verifier export
    run_command("zokrates", &["compile", "-i", "whitelist.zok"])?;
    run_command("zokrates", &["setup"])?;
    run_command("zokrates", &["export-verifier"])?;

    // Open the addresses.txt file
    let file = File::open("addresses.txt")?;
    let reader = io::BufReader::new(file);

    // Create a hashmap to hold all the data
    let mut all_data: HashMap<String, serde_json::Value> = HashMap::new();

    // Loop through each line in addresses.txt
    for line in reader.lines() {
        let address = line?;
        process_addresses(address, &mut all_data)?;
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

/*
* Function to generate a zokrates program file if it doesn't already exist.
* The program, even tha is simple it contains a public part, a private part and assertion.
* The a and b should be the decimal representation of an ethereum address split in two.
* The c and d are again the same values so that we can have them as inputs in solidity and handle them there also.
* The assertion is just a simple assertion so that the verifier can revert on fault proofs.
*/
fn generate_zok_file() -> io::Result<()> {
    if !Path::new("whitelist.zok").exists() {
        let contents = r#"
def main(private field a, private field b, public field c, public field d) -> bool{
    assert(a == c);
    assert(b == d);
    return true;
}
"#;
        File::create("whitelist.zok")?.write_all(contents.as_bytes())?;
    }
    Ok(())
}

// Function to run a shell command with given arguments
fn run_command(command: &str, args: &[&str]) -> io::Result<Output> {
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        let args_str = args.join(" ");
        let error_message = format!("Command '{}' with arguments '{}' failed", command, args_str);
        return Err(io::Error::new(ErrorKind::Other, error_message));
    }
    Ok(output)
}

// Function to parse proof and input from proof.json file that ZoKrates produces
fn parse_proof_json_file() -> Result<(Proof, Vec<String>), Box<dyn Error>> {
    // Open the proof.json file
    let file = File::open("proof.json")?;
    let reader = BufReader::new(file);

    // Deserialize the JSON data
    let proof_file: ProofFile = serde_json::from_reader(reader)?;

    Ok((proof_file.proof, proof_file.inputs))
}

// Function to remove leading zeros from a string
fn remove_leading_zeros(s: &str) -> &str {
    s.trim_start_matches('0')
}

/// Function to process each address, generate a proof using zokrates, and populate the hashmap with proofs and inputs.
///
/// # Arguments
///
/// * `address` - A String representing an Ethereum address.
/// * `all_data` - A mutable reference to a HashMap for storing the proofs and inputs.
///
/// # Returns
///
/// * A Result indicating successful execution or an error.
fn process_addresses(
    address: String,
    all_data: &mut HashMap<String, serde_json::Value>,
) -> Result<(), io::Error> {
    // Convert the hexadecimal address to a decimal BigUint
    let decimal = BigUint::from_str_radix(&address[2..], 16).unwrap();
    let decimal_str = decimal.to_string();

    // Find the midpoint of the decimal string
    let mid = decimal_str.len() / 2;

    // Split the decimal string into two halves and duplicate them
    let (a, b) = decimal_str.split_at(mid);
    let (c, d) = (a.to_string(), b.to_string());

    // Run zokrates command to compute a witness with the split values as arguments
    // Note: the leading zeros are remove cause they are not accepted as compute-witness args
    run_command(
        "zokrates",
        &[
            "compute-witness",
            "-a",
            &remove_leading_zeros(&a.to_string()),
            &remove_leading_zeros(&b.to_string()),
            &remove_leading_zeros(&c.to_string()),
            &remove_leading_zeros(&d.to_string()),
        ],
    )?;

    // Run zokrates command to generate a proof
    run_command("zokrates", &["generate-proof"])?;

    // Parse the generated proof and inputs
    Ok(match parse_proof_json_file() {
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_zok_file() {
        let result = generate_zok_file();
        assert!(result.is_ok());
        assert!(Path::new("whitelist.zok").exists());
    }

    #[test]
    fn test_parse_proof_json_file() {
        // Create a sample proof.json file
        let sample_proof = r#"{
            "scheme": "G16",
            "curve": "Bn128",
            "proof": {
                "a": ["0x1", "0x2"],
                "b": [["0x3", "0x4"], ["0x5", "0x6"]],
                "c": ["0x7", "0x8"]
            },
            "inputs": ["0x9", "0xA"]
        }"#;
        fs::write("proof.json", sample_proof).expect("Unable to write file");

        let result = parse_proof_json_file();
        assert!(result.is_ok());

        let (proof, inputs) = result.unwrap();
        assert_eq!(proof.a, vec!["0x1", "0x2"]);
        assert_eq!(proof.b, vec![vec!["0x3", "0x4"], vec!["0x5", "0x6"]]);
        assert_eq!(proof.c, vec!["0x7", "0x8"]);
        assert_eq!(inputs, vec!["0x9", "0xA"]);
    }

    #[test]
    fn test_remove_leading_zeros() {
        let input = "000123456";
        let result = remove_leading_zeros(input);
        assert_eq!(result, "123456");
    }

    #[test]
    fn test_run_command() {
        let result = run_command("echo", &["Hello, world!"]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
    }
}
