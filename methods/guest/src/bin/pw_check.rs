// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
// #![no_std]  // std support is experimental, but you can remove this to try it

use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let pw: String = env::read();

    if !pw.chars().any(|c| c.is_ascii_punctuation()) {
        panic!();
    }

    let digest = sha::digest_u8_slice(pw.as_bytes());

    env::commit(digest);
}
