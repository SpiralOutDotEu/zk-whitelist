use std::io::{self,  Write};
use std::path::Path;
use std::fs::File;

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
    Ok(())
}
