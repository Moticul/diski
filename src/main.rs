use comfy_table::{Table, presets::UTF8_FULL};
use std::process::Command;

mod args;
use args::get_args;
mod lsblk_parser;
use lsblk_parser::LsblkOutput;

fn main() {
    let args = get_args();

    let mut headers = vec!["Device"];
    if args.filesystem {
        headers.push("Filesystem");
    }
    if args.size {
        headers.push("Size");
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(headers);

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
        if item.disk_type != "disk" {
            continue;
        }
        let mut row = Vec::new();

        row.push(format!("/dev/{}", item.name));

        if args.filesystem {
            row.push(item.fstype.clone().unwrap_or_else(|| "Unknown".to_string()));
        }

        if args.size {
            row.push(item.size.clone());
        }

        table.add_row(row);
    }

    println!("{table}");
}
