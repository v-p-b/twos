use num_bigint::{BigUint, BigInt, Sign};
use num_traits::ops::bytes::{FromBytes, ToBytes};
use std::ops::Neg;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args{
    /// Number
    number: String,

    /// Bit length (default: auto detect)
    #[arg(long, short)]
    bits: Option<u64>,

    /// Treat value as negative magnitude (negative numbers don't play well with argument
    /// parsers...)
    #[arg(long, short)]
    neg: bool,
}


fn print_all(unsigned: BigUint, signed: BigInt){
    println!("[BITS] {}", signed.bits());
    println!("[UHEX] {}", unsigned.to_str_radix(16));
    println!("[IHEX] {}", signed.to_str_radix(16));
    println!("[UBIN] {}", unsigned.to_str_radix(2));
    println!("[IBIN] {}", signed.to_str_radix(2));
    println!("[UDEC] {}", unsigned.to_str_radix(10));
    println!("[IDEC] {}", signed.to_str_radix(10));


}

fn main() {
    let args = Args::parse();

    let tmp_unsigned = if args.number[0..2] == *"0x"{
        BigUint::parse_bytes(args.number[2..].as_bytes(), 16).expect("Hexadecimal parse error!")
    }else if args.number[0..2] == *"0b"{
        BigUint::parse_bytes(args.number[2..].as_bytes(), 2).expect("Binary parse error!")
    }else{
        BigUint::parse_bytes(args.number.as_bytes(), 10).expect("Decimal parse error!")
    };
    
    let mut tmp_bytes = tmp_unsigned.to_bytes_le();

    /*
    let mut sign = if tmp_bytes[tmp_bytes.len() - 1] > 0x7f{
        Sign::Minus
    }else{
        Sign::Plus
    };*/

    if let Some(b) = args.bits {
        if b > tmp_unsigned.bits()  {
            println!("{}", tmp_bytes.capacity());
            let new_bytes_len = (b + (b%8))/8;
            tmp_bytes.resize(new_bytes_len.try_into().unwrap(),0);
            println!("{}", tmp_bytes.capacity());
        }
    }

    let signed = if !args.neg {
        BigInt::from_le_bytes(&tmp_bytes)
    }else{
        BigInt::from_bytes_le(Sign::Minus,&tmp_bytes)
    };

    let unsigned = BigUint::from_le_bytes(&signed.to_le_bytes());

    print_all(unsigned, signed);
}

