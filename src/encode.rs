use std::{
    fs,
    io::{Error, Result, Write},
    path::{Path, PathBuf},
};

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

// https://users.rust-lang.org/t/append-an-additional-extension/23586/12
fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
}

fn main() -> Result<()> {
    let input_dir_name = "./encode_input";
    let output_dir_name = "./encode_output";

    {
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
    }

    let paths = fs::read_dir(&input_dir_name).unwrap();

    for path in paths {
        let input_path = path.unwrap().path();

        let bytes = fs::read(&input_path)?;

        let encoded = encode(&bytes);

        let mut output_path = PathBuf::from(output_dir_name);
        output_path.push(input_path.file_name().unwrap());
        add_extension(&mut output_path, "enc");

        let mut file = fs::File::create(&output_path)?;
        file.write_all(&encoded)?;

        let mut perms = fs::metadata(input_path)?.permissions();
        perms.set_readonly(true);
        fs::set_permissions(output_path, perms)?;
    }

    Ok(())
}
