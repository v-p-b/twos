use clap::Parser;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::ops::bytes::FromBytes;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number - "0x"/"0b" prefixes are recognized for hexadecimal/binary input
    number: String,

    /// Bit length (default: auto detect)
    #[arg(long, short)]
    bits: Option<u64>,

    /// Treat value as negative magnitude (negative numbers don't play well with argument
    /// parsers...)
    #[arg(long, short)]
    neg: bool,
}

pub fn get_ulen(unsigned: &BigUint) -> u64 {
    return unsigned.bits();
}

pub fn get_slen(signed: &BigInt) -> u64 {
    return signed.bits();
}

pub fn get_uhex(unsigned: &BigUint) -> String {
    unsigned.to_str_radix(16)
}

pub fn get_shex(signed: &BigInt) -> String {
    signed.to_str_radix(16)
}

pub fn get_ubin(unsigned: &BigUint) -> String {
    unsigned.to_str_radix(2)
}

pub fn get_sbin(signed: &BigInt) -> String {
    signed.to_str_radix(2)
}

pub fn get_udec(unsigned: &BigUint) -> String {
    unsigned.to_str_radix(10)
}

pub fn get_sdec(signed: &BigInt) -> String {
    signed.to_str_radix(10)
}

fn print_all(unsigned: BigUint, signed: BigInt) {
    println!("[ULEN] {}", get_ulen(&unsigned));
    println!("[SLEN] {}", get_slen(&signed));
    println!("[UHEX] {}", get_uhex(&unsigned));
    println!("[SHEX] {}", get_shex(&signed));
    println!("[UBIN] {}", get_ubin(&unsigned));
    println!("[SBIN] {}", get_sbin(&signed));
    println!("[UDEC] {}", get_udec(&unsigned));
    println!("[SDEC] {}", get_sdec(&signed));
}

pub fn parse_input(number: String, bits: Option<u64>, neg: bool) -> (BigUint, BigInt) {
    let tmp_unsigned = if number.len() > 2 && number[0..2] == *"0x" {
        BigUint::parse_bytes(number[2..].as_bytes(), 16).expect("Hexadecimal parse error!")
    } else if number.len() > 2 && number[0..2] == *"0b" {
        BigUint::parse_bytes(number[2..].as_bytes(), 2).expect("Binary parse error!")
    } else {
        BigUint::parse_bytes(number.as_bytes(), 10).expect("Decimal parse error!")
    };

    let mut tmp_bytes = tmp_unsigned.to_bytes_le();

    if let Some(b) = bits {
        if b > tmp_unsigned.bits() {
            eprintln!("[WARN] Resizing");
            let new_bytes_len = (b + (b % 8)) / 8;
            tmp_bytes.resize(new_bytes_len.try_into().unwrap(), 0);
        } else {
            eprintln!("[WARN] Bit size argument ignored!");
        }
    }
    eprintln!("New array size: {}", tmp_bytes.len());

    let mut unsigned = BigUint::from_le_bytes(&tmp_bytes);

    let signed = if !neg {
        BigInt::from_le_bytes(&tmp_bytes)
    } else {
        let s = BigInt::from_bytes_le(Sign::Minus, &tmp_bytes);
        unsigned = BigUint::from_bytes_le(&s.to_signed_bytes_le());
        s
    };

    return (unsigned, signed);
}

fn main() {
    let args = Args::parse();
    let (unsigned, signed) = parse_input(args.number, args.bits, args.neg);

    print_all(unsigned, signed);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn all_ones() {
        let (unsigned, signed) = parse_input(String::from_str("0xFFFFFFFF").unwrap(), None, false);
        assert_eq!(get_ulen(&unsigned), 32);
        assert_eq!(get_slen(&signed), 1);
    }

    #[test]
    fn bits() {
        let (_, signed30) = parse_input(String::from_str("0xDEADBEEF").unwrap(), None, false);
        assert_eq!(get_slen(&signed30), 30);

        let (_, signed32) = parse_input(String::from_str("0xDEADBEEF").unwrap(), Some(64), false);
        assert_eq!(get_slen(&signed32), 32);

        let (_, signed1) = parse_input(String::from_str("0xFFFFFFFF").unwrap(), None, false);
        assert_eq!(get_slen(&signed1), 1);
    }

    #[test]
    fn neg() {
        let (unsigned, signed) = parse_input(String::from_str("0xDEADBEEF").unwrap(), None, true);
        assert_eq!(
            String::from_str("ff21524111").unwrap(),
            get_uhex(&unsigned).to_lowercase()
        );
        assert_eq!(
            String::from_str("-deadbeef").unwrap(),
            get_shex(&signed).to_lowercase()
        );
    }
}
