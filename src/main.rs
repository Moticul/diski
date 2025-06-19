use comfy_table::{Table, presets::UTF8_FULL};
use std::process::Command;
mod lsblk_parser;
use lsblk_parser::LsblkOutput;

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

    let stdout = String::from_utf8_lossy(&output.stdout);

    let json = serde_json::from_str::<LsblkOutput>(&stdout);

    let devices = match json {
        Ok(parsed) => parsed.blockdevices,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return;
        }
    };

    for item in devices {
        if item.disk_type == "disk" {
            table.add_row(vec![
                format!("/dev/{}", item.name),
                item.fstype.unwrap_or_else(|| "Unknown".to_string()),
                item.size,
            ]);
        }
    }

    println!("{table}");
}
