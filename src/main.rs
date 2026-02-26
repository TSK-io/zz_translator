use std::env;
use std::process::{Command, Stdio};
use urlencoding::encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 获取命令行输入的单词或句子
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: zz <要翻译的单词或句子>");
        std::process::exit(1);
    }
    let text = args[1..].join(" ");
    let encoded_text = encode(&text);

    // 2. 并发发起网络请求（翻译 API）
    let trans_url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=zh-CN&dt=t&q={}",
        encoded_text
    );

    let res = reqwest::get(&trans_url).await?.text().await?;

    // 3. 解析并打印翻译结果
    let v: serde_json::Value = serde_json::from_str(&res)?;
    if let Some(translated) = v[0][0][0].as_str() {
        // 使用 ANSI 转义码打印带颜色的结果（绿色加粗）
        println!("\n✨ 翻译结果: \x1b[1;32m{}\x1b[0m\n", translated);
    } else {
        println!("⚠️ 未能解析翻译结果");
    }

    // 4. 发音 (TTS)
    let tts_url = format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=en&client=tw-ob",
        encoded_text
    );

    println!("🔊 正在发音...");
    
    // 调用系统的 mpv 播放器在后台静默播放音频流
    // 使用 spawn() 替代 status()，让程序瞬间退出返回终端
    let _ = Command::new("mpv")
        .arg("--no-video")
        .arg("--msg-level=all=no")
        .arg(&tts_url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    Ok(())
}
