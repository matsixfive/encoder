use std::{
    fs,
    io::{Error, Write},
};

/* fn print_bytes(bytes: &Vec<u8>) -> () {
    for byte in bytes {
        for i in (0..8).rev() {
            print! {"{}" , if byte & (1 << (7 - i)) != 0 { "1" } else { "0" }}
        }
        print!(" ")
    }
    print!("\n")
} */

fn encode(bytes: &Vec<u8>) -> Vec<u8> {
    let mut encoded: Vec<u8> = vec![];
    let mut num_of_same = 0u8;
    let mut previous = false;

    //TODO: allow for smaller 'chunk' sizes eg max is 4 bits (better for more random files). First byte is size of each chunk
    let max_size = 255;

    for byte in bytes {
        // println!("{:#010b}", byte /*  & (1 << 6)  */);
        for i in (0..8).rev() {
            let bit = byte & (1 << i) != 0;

            // stop overflow
            if num_of_same >= max_size {
                encoded.push(num_of_same);
                num_of_same = 0;
                previous = !bit;
            }

            if bit == previous {
                num_of_same += 1;
                continue;
            } else {
                encoded.push(num_of_same);
                num_of_same = 0;
            }

            num_of_same += 1;
            previous = bit;
        }
    }
    encoded.push(num_of_same);

    encoded
}

fn bit_to_byte(bits: Vec<bool>) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    let mut byte = 0u8;
    let mut count = 0u8;

    for bit in bits {
        if bit {
            byte |= 1 << (7 - count);
        }

        count += 1;

        if count == 8 {
            bytes.push(byte);
            byte = 0;
            count = 0;
        }
    }

    bytes
}

fn decode(bytes: &Vec<u8>) -> Vec<u8> {
    let mut decoded: Vec<bool> = vec![];
    let mut current = false;

    for byte in bytes {
        for _ in 0..*byte {
            decoded.push(current)
        }
        current = !current;
    }

    bit_to_byte(decoded)
}

fn main() -> Result<(), Error> {
    let filename = "input.txt";

    //TODO: include metadata in encoded file to be transferred when decoded

    {
        let bytes = fs::read(filename)?;
        // println!("{:?}", bytes);
        // print_bytes(&bytes);
        println!("{:?}", bytes.last());

        let encoded = encode(&bytes);
        // println!("{:?}", encoded);
        // print_bytes(&encoded);

        let mut file = fs::File::create(format!("{}.bin", filename))?;
        file.write_all(&encoded)?;
    }

    {
        let bytes = fs::read(format!("{}.bin", filename))?;

        let decoded = decode(&bytes);
        // println!("{:?}", decoded);
        // print_bytes(&decoded);

        let mut file = fs::File::create(format!("output_{}", filename))?;
        file.write_all(&decoded)?;
        println!("{:?}", decoded.last());
    }

    Ok(())
}
