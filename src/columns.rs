use crate::args::Args;
use crate::lsblk_parser::BlockDevice;

pub fn get_columns() -> Vec<Column> {
    vec![
        Column {
            header: "Filesystem",
            enabled: |args: &Args| args.filesystem,
            extract: |dev: &BlockDevice| {
                dev.fstype.clone().unwrap_or_else(|| "Unknown".to_string())
            },
        },
        Column {
            header: "Size",
            enabled: |args: &Args| args.size,
            extract: |dev: &BlockDevice| dev.size.clone().unwrap_or("Unknown".to_string()),
        },
    ]
}

pub struct Column {
    pub header: &'static str,
    pub enabled: fn(&crate::args::Args) -> bool,
    pub extract: fn(&crate::lsblk_parser::BlockDevice) -> String,
}
