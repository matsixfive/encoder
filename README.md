# File Encoder/Decoder

This was originally meant to be a file compressor but rarely makes files smaller, just encodes them.

## How it works

The number of adjacent bits is tallied and encoded as a list of numbers.
e.g. `11111 000 1111111 00 1` would be encoded as `5, 3, 7, 2, 1`, using a byte for each number. If there are more than 255 of the same bit in a row, the chunk of bits is stopped, the next chunk of bits is set to 0 and the counter continues e.g. 256 of the same bits in a row would be encoded as `255, 0, 1` meaining there are 255 ones, then 0 zeroes, then 1 one.

## How to use

Run either one of the executables and it will create a directory for you to put the files you want to be encoded in.

## Download

### From [release](https://github.com/matsixfive/encoder/releases)

Download the latest release from the [releases](https://github.com/matsixfive/encoder/releases/latest) panel.

### Build from source

Dowload the repo and use `cargo build --release` to build the project.

## Pros/Cons

- The only real situation where this does compress files is in files almost entirely made of large clusters (a byte or more) of the same bits e.g. a large black image, which is almost entirely useless.

- Can be used to pseudo encrypt files, although there is nothing stopping anyone from decoding them if they figure out how they were encoded.

- Ignores directories (for now).

# UPDATE:
- This process is called [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
- Could use variable sized chunks using something like [Elias gamma coding](https://en.wikipedia.org/wiki/Elias_gamma_coding) whick uses `2 * floor(log_2(x)) + 1` bits per count `x` instead of fixed chunks (`2 * floor(chunk_size / x) - 1`)
- Could be nice to have first bit in the result be the same as the first bit in the source instead of a constant `1`
- Is sometimes useful e.g. in a screenshot with a large area of black / white
- [Helpful stackoverflow answer](https://stackoverflow.com/a/7603969)
