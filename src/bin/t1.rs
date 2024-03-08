use web_yaml::data_manage::*;
use web_yaml::pause;

#[allow(unused)]
pub fn main() {
    config::初始化配置文件();
    dbg!(config::将配置文件读取到结构体中());
    pause()
}