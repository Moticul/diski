use clap::Parser; 

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliOptions {
    #[arg(
        short, long, 
        help = "show SMART info for target device",
        id = "smart",
    )]
    smart: bool,
    
    #[arg(
        value_name = "DEVICE",
        help = "target device (e.g. /dev/sda)",
        required_if_eq = ("smart", true)
    )]
    
    device: Option<String>,

    
}
