use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

// 设置pub的作用，可以从外面访问这里的定义的变量和数据
#[derive(Debug, Deserialize)]
pub struct WarpDate {
    private_key: String,
    public_key: String,
    reserved: Option<Vec<u8>>,
    v4: String,
    v6: String,
}

// 设置pub的作用，可以从外面访问这里的定义的变量和数据
#[derive(Debug, Deserialize)]
pub struct WarpDates {
    warp_parameters: Vec<WarpDate>, // warp_parameters 是yaml文件中第一行的字段
}

// 目的，可以在外部访问私有变量和数据
impl WarpDates {
    // 创建一个公共方法来获取私有字段 warp_parameters 的引用
    pub fn get_warp_parameters(&self) -> &Vec<WarpDate> {
        &self.warp_parameters // warp_parameters 是yaml文件中第一行的字段
    }
}
// 目的，可以在外部访问私有变量和数据
impl WarpDate {
    pub fn get_private_key(&self) -> String {
        Arc::new(self).private_key.clone()
    }
    pub fn get_public_key(&self) -> String {
        Arc::new(self).public_key.clone()
    }
    pub fn get_reserved(&self) -> Option<Vec<u8>> {
        Arc::new(self).reserved.clone()
    }
    pub fn get_v4(&self) -> String {
        Arc::new(self).v4.clone()
    }
    pub fn get_v6(&self) -> String {
        Arc::new(self).v6.clone()
    }
}

pub fn read_yaml_data(yaml_file: &str) -> Result<WarpDates, Box<dyn std::error::Error>> {
    let file = File::open(yaml_file)?;
    let reader = BufReader::new(file);
    let items: WarpDates = serde_yaml::from_reader(reader)?;
    Ok(items)
}
