use web_yaml::data_manage::{file_manage, web_server};
use web_yaml::view;

fn main() {
    loop {
        let v = file_manage::按后缀搜索文件并按修改日期倒叙排序(".yaml");
        if v.is_none() {
            println!("无文件，请将文件放入当前文件夹或其子文件夹下再重试");
            let _ = std::process::Command::new("cmd.exe")
                .arg("/c")
                .arg("pause")
                .status();
            continue;
        }
        let v = v.unwrap();
        view::展示文件及修改时间(&v);
        let f = view::选择文件(&v);

        println!("正在启动web server 端口: 7878");
        println!("启动成功！");
        println!("http://127.0.0.1:7878");
        web_server::请求处理::监听端口等待并处理任务(f.path().into());
        break;
    }
    let _ = std::process::Command::new("cmd.exe")
        .arg("/c")
        .arg("pause")
        .status();
}
