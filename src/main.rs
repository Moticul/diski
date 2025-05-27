use comfy_table::Table;
use sysinfo::Disks;

fn main() {
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{:?}", disk.name());
        println!("{:?}", disk.mount_point());
    }

    let mut table = Table::new();
    table
        .set_header(vec!["Disk Name", "Disk Kind", "Filsystem"])
        .add_row(vec!["", "This is another text", "This is the third text"])
        .add_row(vec![
            "This is another text",
            "Now\nadd some \nmulti line stuff",
            "This is awesome",
        ]);

    println!("{table}");
}
