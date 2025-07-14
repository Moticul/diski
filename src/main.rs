use comfy_table::{Table, presets::UTF8_FULL};
use std::process::Command;

mod args;
use args::get_args;
mod lsblk_parser;
use lsblk_parser::LsblkOutput;
mod columns;
use columns::get_columns;

fn main() {
    let args = get_args();
    let all_columns = get_columns();
    let active_columns: Vec<_> = all_columns
        .into_iter()
        .filter(|col| (col.enabled)(&args))
        .collect();

    let mut table = Table::new();
    let mut headers = vec!["Device"];

    headers.extend(active_columns.iter().map(|col| col.header));
    table.load_preset(UTF8_FULL);
    table.set_header(headers);

    let columns = args.lsblk_args().join(",");
    let output = Command::new("lsblk")
        .arg("-J")
        .arg("-o")
        .arg(columns)
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
        let mut row = vec![format!("/dev/{}", item.name)];
        row.extend(active_columns.iter().map(|col| (col.extract)(&item)));
        table.add_row(row);
    }

    println!("{table}");
}
