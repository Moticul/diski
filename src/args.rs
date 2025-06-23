use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'f', long = "filesystem")]
    pub filesystem: bool,

    #[arg(short = 's', long = "size")]
    pub size: bool,

    #[arg(short = 't', long = "type")]
    pub disk_type: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
