use std::{
    fs,
    io::{Error, Write},
    path::{Path, PathBuf},
};

fn bits_to_bytes(bits: Vec<bool>) -> Vec<u8> {
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

    bits_to_bytes(decoded)
}

fn main() -> Result<(), Error> {
    let input_dir_name = "./decode_input";
    let output_dir_name = "./decode_output";

    // check if input dir exists / is not empty
    if !Path::new(input_dir_name).is_dir()
        || PathBuf::from(input_dir_name).read_dir()?.next().is_none()
    {
        if !Path::new(input_dir_name).is_dir() {
            fs::create_dir(input_dir_name)?;
        }

        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Input directory ({}) is empty", input_dir_name),
        ));
    }

    if !Path::new(&output_dir_name).is_dir() {
        fs::create_dir(&output_dir_name)?;
    }

    for path in fs::read_dir(&input_dir_name).unwrap() {
        let input_path = path.unwrap().path();
        if input_path.extension().unwrap() != "enc" {
            continue;
        }

        let bytes = fs::read(&input_path)?;

        let decoded = decode(&bytes);

        let mut output_path = PathBuf::from(&output_dir_name);
        output_path.push(input_path.file_stem().unwrap());

        println!("{:?} => {:?}", input_path.file_stem().unwrap(), output_path);

        let mut file = fs::File::create(output_path)?;
        file.write_all(&decoded)?;
    }

    Ok(())
}
