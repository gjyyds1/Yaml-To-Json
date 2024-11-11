use clap::Parser;
use indexmap::IndexMap;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// 输入文件 (JSON或YAML)
    input: String,
    /// 输出文件
    #[arg(short, long)]
    output: String,
}

// 定义支持有序键值的类型
type OrderedMap = IndexMap<String, JsonValue>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_path = Path::new(&args.input);
    let output_path = Path::new(&args.output);

    // 读取输入文件内容
    let input_content = fs::read_to_string(input_path)?;

    // 判断文件格式并进行转换
    if input_path.extension().and_then(|ext| ext.to_str()) == Some("json") {
        // JSON 转 YAML
        let json_data: OrderedMap = serde_json::from_str(&input_content)?;
        let yaml_data = serde_yaml::to_string(&json_data)?;
        fs::write(output_path, yaml_data)?;
    } else if input_path.extension().and_then(|ext| ext.to_str()) == Some("yaml") || input_path.extension().and_then(|ext| ext.to_str()) == Some("yml") {
        // YAML 转 JSON
        let yaml_data: OrderedMap = serde_yaml::from_str(&input_content)?;
        let json_data = serde_json::to_string_pretty(&yaml_data)?;
        fs::write(output_path, json_data)?;
    } else {
        eprintln!("不支持的文件格式。请使用.json或.yml/.yaml文件。");
        std::process::exit(1);
    }

    println!("转换成功！");
    Ok(())
}
