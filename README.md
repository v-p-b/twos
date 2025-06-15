# twos

Signed/unsigned ([two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)) command-line integer converter based on `num_bigint`.


## Usage

```
Usage: twos [OPTIONS] <NUMBER>

Arguments:
  <NUMBER>  Number - "0x"/"0b" prefixes are recognized for hexadecimal/binary input

Options:
  -b, --bits <BITS>  Bit length (default: auto detect)
  -n, --neg          Treat value as negative magnitude (negative numbers don't play well with argument parsers...)
  -h, --help         Print help
  -V, --version      Print version
```


### Examples

Get signed and unsigned values in different number systems:

```
% twos 0xdeadbeef
[ULEN] 32
[SLEN] 30
[UHEX] deadbeef
[SHEX] -21524111
[UBIN] 11011110101011011011111011101111
[SBIN] -100001010100100100000100010001
[UDEC] 3735928559
[SDEC] -559038737
```

Force wider integer width:

```
% twos -b 64 0xdeadbeef
[ULEN] 32
[SLEN] 32
[UHEX] deadbeef
[SHEX] deadbeef
[UBIN] 11011110101011011011111011101111
[SBIN] 11011110101011011011111011101111
[UDEC] 3735928559
[SDEC] 3735928559
```

Two's complement representation of negative values:

```
% twos -n 0xdeadbeef 
[ULEN] 40
[SLEN] 32
[UHEX] ff21524111
[SHEX] -deadbeef
[UBIN] 1111111100100001010100100100000100010001
[SBIN] -11011110101011011011111011101111
[UDEC] 1095775699217
[SDEC] -3735928559
```

