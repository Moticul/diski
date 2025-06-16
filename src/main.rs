use comfy_table::{Table, presets::UTF8_FULL};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};
use std::process::Command;

#[derive(Deserialize)]
struct BlockDevice {
    name: String,
    #[serde(rename = "type")]
    disk_type: String,
    size: String,
    fstype: Option<String>,
}

#[derive(Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<BlockDevice>,
}

fn main() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Device", "Filesystem", "Size"]);

    let output = Command::new("lsblk")
        .arg("-J")
        .arg("-o")
        .arg("NAME,TYPE,SIZE,FSTYPE")
        .output()
        .expect("Failed to execute command");

    println!("{:?}", output);

    println!("{table}");
}
