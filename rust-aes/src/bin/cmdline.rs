use aeslib::aes::{decrypt, encrypt, AesSize, Key};
use clap::{arg, Command};
use std::io::Read;
use std::path::PathBuf;

fn cli() -> Command<'static> {
    Command::new("aes-cli")
        .about("A utility to encrypt and decrypt AES data")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts data to AES")
                .arg(arg!(-i --in <INFILE> "File to encrypt"))
                .arg_required_else_help(true)
                .arg(arg!(-p --password <KEY> "Key to use for encryption"))
                .arg_required_else_help(true)
                .arg(arg!(-o --out <OUTFILE> "File to write encrypted data to").required(false))
                .arg(arg!(-s --size <SIZE> "Size of AES key to use").required(false)),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts data from AES")
                .arg(arg!(-i --in <INFILE> "File to decrypt"))
                .arg_required_else_help(true)
                .arg(arg!(-p --password <KEY> "Key to use for decryption"))
                .arg_required_else_help(true)
                .arg(arg!(-o --out <OUTFILE> "File to write decrypted data to").required(false))
                .arg(arg!(-s --size <SIZE> "Size of AES key to use").required(false)),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("in").expect("required");
            let password = sub_matches.get_one::<String>("password").expect("required");
            let size = match sub_matches.get_one::<String>("size") {
                Some(size) => AesSize::parse(size).expect("invalid size"),
                None => AesSize::S128,
            };
            let file_buf = PathBuf::from(file_path);
            let out_file_path = match sub_matches.get_one::<String>("out") {
                Some(out_file_path) => out_file_path.to_owned(),
                None => {
                    let mut out_file_path = file_buf.clone();
                    out_file_path.set_extension("aes");
                    out_file_path.to_str().unwrap().to_string()
                }
            };
            println!("Encrypting {}", file_path);
            let mut file = std::fs::File::open(file_buf).expect("file not found");
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents)
                .expect("failed to read file");
            let cipher = Key::from_password(password, size);
            let encrypted_data = encrypt(&cipher, &file_contents).expect("failed to encrypt");
            std::fs::write(&out_file_path, encrypted_data).expect("failed to write file");
            println!("Encrypted data written to {:#?}", out_file_path);
        }
        Some(("decrypt", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("in").expect("required");
            let password = sub_matches.get_one::<String>("password").expect("required");
            let size = match sub_matches.get_one::<String>("size") {
                Some(size) => AesSize::parse(size).expect("invalid size"),
                None => AesSize::S128,
            };
            let file_buf = PathBuf::from(file_path);
            let out_file_path = match sub_matches.get_one::<String>("out") {
                Some(out_file_path) => out_file_path.to_owned(),
                None => {
                    let mut out_file_path = file_buf.clone();
                    out_file_path.set_extension("txt");
                    out_file_path.to_str().unwrap().to_string()
                }
            };
            println!("Decrypting {}", file_path);
            let mut file = std::fs::File::open(file_buf).expect("file not found");
            let mut file_contents = Vec::new();
            file.read_to_end(&mut file_contents)
                .expect("failed to read file");
            let cipher = Key::from_password(password, size);
            let decrypted_data = decrypt(&cipher, &file_contents).expect("Invalid Password");
            std::fs::write(&out_file_path, decrypted_data).expect("failed to write file");
            println!("Decrypted data written to {:#?}", out_file_path);
        }
        _ => unreachable!(),
    }
}
