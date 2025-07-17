use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'f', long = "filesystem", help = "display filesystem")]
    pub filesystem: bool,

    #[arg(short = 's', long = "size", help = "display size of disk")]
    pub size: bool,

    #[arg(short = 't', long = "type", help = "display the type of disk")]
    pub disk_type: bool,
}

impl Args {
    pub fn lsblk_args(&self) -> Vec<&'static str> {
        let fields = vec![(&self.filesystem, "FSTYPE"), (&self.size, "SIZE")];

        let mut lsblk_args = vec!["NAME", "TYPE"];
        for (flag, column) in fields {
            if *flag {
                lsblk_args.push(column);
            }
        }
        lsblk_args
    }
}

pub fn get_args() -> Args {
    Args::parse()
}
