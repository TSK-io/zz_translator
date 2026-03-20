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

    // 2. 请求翻译 API
    let trans_url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=zh-CN&dt=t&q={}",
        encoded_text
    );

    let client = reqwest::Client::builder()
        .user_agent("zz/0.1.2")
        .build()?;

    let res = client.get(&trans_url).send().await?.text().await?;

    // 3. 解析并打印翻译结果
    let v: serde_json::Value = serde_json::from_str(&res)?;

    let translated_text = if let Some(translated) = v[0][0][0].as_str() {
        println!("\n✨ 翻译结果: \x1b[1;32m{}\x1b[0m\n", translated);
        translated.to_string()
    } else {
        println!("⚠️ 未能解析翻译结果");
        String::new()
    };

    // 4. 发音 (TTS)
    let tts_url_en = format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=en&client=tw-ob",
        encoded_text
    );

    let mut args_for_mpv = vec![
        "--no-video".to_string(),
        "--msg-level=all=no".to_string(),
        tts_url_en,
    ];

    if !translated_text.is_empty() {
        let encoded_zh = encode(&translated_text);
        let tts_url_zh = format!(
            "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=zh-CN&client=tw-ob",
            encoded_zh
        );
        args_for_mpv.push(tts_url_zh);
    }

    println!("🔊 正在发音 (英 -> 中)...");

    if let Err(err) = Command::new("mpv")
        .args(&args_for_mpv)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        eprintln!("⚠️ 启动 mpv 失败: {}", err);
    }

    Ok(())
}
