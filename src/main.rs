extern crate colored;
extern crate rpassword;

use colored::*;
use std::{io::stdin, process};

use anyhow::anyhow;
use chacha20poly1305::{
    aead::{stream, Aead, NewAead},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
use rpassword::read_password;
use std::{
    fs::{self, File},
    io::{Read, Write},
};
use web3_rust_wrapper::KeyPair;
use web3_rust_wrapper::Web3Manager;
use clap::Parser;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    encrypt: String,
    decrypt: String,
    filepath: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    /*
    let args = Args::parse();

    for _ in 0..args.count {
        println!("encrypt {}!", args.encrypt);
        println!("decrypt {}!", args.decrypt);
        println!("filepath {}!", args.filepath);
    }

    //process::exit(0);
    */

    // generate new keypair 🔒
    let keypair: KeyPair = generate_keypair();
    println!("{}", "Keypair generated successfully ✅".green());
    print_keypair(keypair);

    let password = read_passwords();
    println!("{}", password.green());
}

fn generate_keypair() -> KeyPair {
    let (secret_key, pub_key) = Web3Manager::generate_keypair();
    let keypair: KeyPair = KeyPair {
        secret_key: secret_key.display_secret().to_string(),
        public_key: pub_key.to_string(),
    };
    keypair
}
//let passwords_match: bool = compare_passwords(password1.to_string(), password2.to_string());
fn read_passwords() -> String {
    let password1: String = "".to_string();

    loop {
        println!("{}", "Introduce your password: ".yellow());
        let password1 = read_password().unwrap();

        println!("{}", "Introduce your password again: ".yellow());
        let password2 = read_password().unwrap();

        let is_password1_valid: bool = true;

        if !is_password1_valid {
            println!("{}", "Invalid password, please try again".red());
        }

        let is_password2_valid: bool = true;

        if !is_password2_valid {
            println!("{}", "Invalid password, please try again".red());
        }

        if is_password1_valid && is_password2_valid && password1 == password2 {
            if password1 == password2 {
               
            } else {
                println!("{}", "Invalid password, please try again".red());
            }
        }
    }
    password1
}

fn compare_passwords(password1: String, password2: String) -> bool {
    password1 == password2
}

fn encrypt_keypair(keypair: KeyPair) {}

fn print_keypair(keypair: KeyPair) {
    println!("{}: {}", "Public key".cyan(), keypair.public_key.yellow());
    println!(
        "{}: {}{}{}...",
        "Private key".cyan(),
        keypair
            .secret_key
            .chars()
            .nth(0)
            .unwrap()
            .to_string()
            .yellow(),
        keypair
            .secret_key
            .chars()
            .nth(1)
            .unwrap()
            .to_string()
            .yellow(),
        keypair
            .secret_key
            .chars()
            .nth(2)
            .unwrap()
            .to_string()
            .yellow()
    );
}

fn read_string_asking(txt: &str) -> String {
    let mut readed_string = String::new();
    println!("{}", txt);
    stdin().read_line(&mut readed_string).unwrap();
    let res = match readed_string.trim_end() {
        "" => "".to_owned(),
        readed_value => format!("{}", readed_value),
    };
    res
}

fn encrypt_large_file(
    source_file_path: &str,
    dist_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), anyhow::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(dist_file_path)?;

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
            break;
        }
    }

    Ok(())
}

fn decrypt_large_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), anyhow::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut dist_file = File::create(dist)?;

    loop {
        let read_count = encrypted_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dist_file.write(&plaintext)?;
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dist_file.write(&plaintext)?;
            break;
        }
    }

    Ok(())
}
