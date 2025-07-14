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

impl Args {
    pub fn lsblk_args(&self) -> Vec<&'static str> {
        let fields = vec![
            (&self.filesystem, "FSTYPE"),
            (&self.size, "SIZE"),
            (&self.disk_type, "TYPE"),
        ];

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
