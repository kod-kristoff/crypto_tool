use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}

/// RC4 file en/decryption
#[derive(Parser, Debug)]
struct Args {
    /// Name of file to en/decrypt
    #[clap(short, long, required=true, value_name = "FILE_NAME")]
    file: String,

    /// En/decryption key (hexadecimal bytes)
    #[clap(
        short,
        long,
        required = true,
        value_name = "HEX_BYTES",
        min_values = 5,
        max_values = 256,
    )]
    key: Vec<String>,
}

