# File Encoder / Decoder

This was originally meant to be a file compressor but rarely makes files smaller, just encodes them.

## How it works

The number of adjacent bits is tallied and encoded as a list of numbers.
e.g. `11111 000 1111111 00 1` would be encoded as `5, 3, 7, 2, 1`, using a byte for each number. If there are more than 255 same bits in a row, the chunk of bits is stopped, the chunk of the next bits is set to 0 and the counter continues e.g. 256 same bits in a row would be stored as `255, 0, 1` meaining there are 255 ones, then 0 zeroes, then 1 one.

## How to use

Run either one of the executables and it will create a directory for you to put the files you want to be encoded in.

## Download

### Release

Download the latest release from the [releases](/releases) panel

### Build from source

Dowload the repo and use `cargo build --release` to build the project.
