//! **CYS431 PEX2**
//!
//! *Author:* Ethan Chapman
//!
//! *Documentation:* No help received

use std::fs;
use itertools::Itertools;
use md5::{Digest, Md5};

fn main() {
    let sample = fs::read("samplefile.txt").unwrap();
    let mut contract = fs::read("contract.txt").unwrap();

    let mut hasher = Md5::new();
    hasher.update(sample.clone());
    let target_md5 = u128::from_be_bytes(hasher.clone().finalize().into());
    let target_tinyhash = target_md5 >> 108;
    let alphabet = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let mut length = 1;
    let mut collisions = vec![];
    'outer: loop {
        for c in alphabet.iter().cloned().combinations(length) {
            let mut hasher = hasher.clone();
            hasher.update(c.clone());
            let new_md5 = u128::from_be_bytes(hasher.finalize().into());
            let new_tinyhash = new_md5 >> 108;

            if new_tinyhash == target_tinyhash {
                let filename = format!("collisions/sample_{}.txt", collisions.len() + 1);
                println!("Found collision {:?}, saving as {}", c, filename);
                collisions.push(c.clone());

                let mut file_data = sample.clone();
                file_data.extend(c);
                fs::write(filename, file_data).unwrap();

                if collisions.len() == 5 {
                    break 'outer;
                }
            }
        }
        length += 1;
    }

    let mut hasher = Md5::new();
    hasher.update(contract.clone());
    let target_md5 = u128::from_be_bytes(hasher.clone().finalize().into());
    let target_tinyhash = target_md5 >> 108;
    contract.pop().unwrap();
    contract.pop().unwrap();
    contract.pop().unwrap();
    contract.pop().unwrap();
    contract.pop().unwrap();
    contract.pop().unwrap();
    let mut hasher2 = Md5::new();
    hasher2.update(contract.clone());
    for n in 0..<100000 {
        let mut h = hasher2.clone();
        h.update(n.to_string().as_bytes());
        let new_md5 = u128::from_be_bytes(h.finalize().into());
        let new_tinyhash = new_md5 >> 108;

        if new_tinyhash == target_tinyhash {
            println!("Found collision using ${}", n);

            let filename = format!("collisions/contract_{}.txt", n);

            let mut file_data = contract.clone();
            file_data.extend(n.to_string().as_bytes());
            fs::write(filename, file_data).unwrap();

            break;
        }
    }
}
