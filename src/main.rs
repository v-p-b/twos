use clap::Parser;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::ops::bytes::{FromBytes, ToBytes};

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

fn print_all(unsigned: BigUint, signed: BigInt) {
    println!("[ULEN] {}", unsigned.bits());
    println!("[SLEN] {}", signed.bits());
    println!("[UHEX] {}", unsigned.to_str_radix(16));
    println!("[SHEX] {}", signed.to_str_radix(16));
    println!("[UBIN] {}", unsigned.to_str_radix(2));
    println!("[SBIN] {}", signed.to_str_radix(2));
    println!("[UDEC] {}", unsigned.to_str_radix(10));
    println!("[SDEC] {}", signed.to_str_radix(10));
}

fn main() {
    let args = Args::parse();

    let tmp_unsigned = if args.number[0..2] == *"0x" {
        BigUint::parse_bytes(args.number[2..].as_bytes(), 16).expect("Hexadecimal parse error!")
    } else if args.number[0..2] == *"0b" {
        BigUint::parse_bytes(args.number[2..].as_bytes(), 2).expect("Binary parse error!")
    } else {
        BigUint::parse_bytes(args.number.as_bytes(), 10).expect("Decimal parse error!")
    };

    let mut tmp_bytes = tmp_unsigned.to_bytes_le();

    if let Some(b) = args.bits {
        if b > tmp_unsigned.bits() {
            let new_bytes_len = (b + (b % 8)) / 8;
            tmp_bytes.resize(new_bytes_len.try_into().unwrap(), 0);
        } else {
            eprintln!("[WARN] Bit size argument ignored!");
        }
    }

    let signed = if !args.neg {
        BigInt::from_le_bytes(&tmp_bytes)
    } else {
        BigInt::from_bytes_le(Sign::Minus, &tmp_bytes)
    };

    let unsigned = BigUint::from_le_bytes(&signed.to_le_bytes());

    print_all(unsigned, signed);
}
