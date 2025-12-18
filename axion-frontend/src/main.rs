use clap::Parser;

#[derive(Parser, Debug)]
enum Args {
    Send { rec_pub_key: String, amount: u32 },
    CheckBalance { pub_key: String },
}

fn main() {
    let args = Args::parse();
}
