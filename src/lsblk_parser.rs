use serde::Deserialize;

#[derive(Deserialize)]
pub struct BlockDevice {
    pub name: String,
    #[serde(rename = "type")]
    pub disk_type: Option<String>,
    pub size: Option<String>,
    pub fstype: Option<String>,
}

#[derive(Deserialize)]
pub struct LsblkOutput {
    pub blockdevices: Vec<BlockDevice>,
}
