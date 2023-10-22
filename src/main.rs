use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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

    Ok(())
}
