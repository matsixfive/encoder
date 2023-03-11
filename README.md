# File Encoder / Decoder

This was originally meant to be a file compressor but rarely makes files smaller.

## How it works

The number of adjacent bits is tallied and encoded as a list of numbers.
e.g. `111110001111111001` would be encoded as `53721`, using a byte for each number. If there are more than 255 same bits in a row, the chunk of bits is stopped, the chunk of the next bits is set to 0 and the counter continues e.g. 256 same bits in a row would be stored as `11111111 (255) 00000000 (0) 00000001 (1)` meaining there are 255 ones, then 0 zeroes, then 1 one.

## How to use

### Download

Download the latest release from the releases panel

### Build from source

Dowload the repo and use `cargo build` to build the project.