use rand::Rng as _;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let salt = match env::var("SALT") {
        Ok(val) => {
            let val = val.trim_start_matches("0x");
            u128::from_str_radix(val, 16).expect("SALT must be a valid hex u128")
        }
        Err(_) => {
            let mut rng = rand::rng();
            let mut buf = [0u8; 16];
            rng.fill_bytes(&mut buf);
            u128::from_ne_bytes(buf)
        }
    };
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("salt.rs");
    fs::write(
        &dest_path,
        &format!("pub const COMPILE_SALT: u128 = 0x{:032x};\n", salt),
    )
    .unwrap();
}
