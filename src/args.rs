use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'f', long = "filesystem")]
    pub filesystem: bool,
    #[arg(short = 's', long = "size")]
    pub size: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
