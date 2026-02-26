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
    
    // 提取翻译结果，并准备后续的中文发音 URL
    let translated_text = if let Some(translated) = v[0][0][0].as_str() {
        println!("\n✨ 翻译结果: \x1b[1;32m{}\x1b[0m\n", translated);
        translated.to_string()
    } else {
        println!("⚠️ 未能解析翻译结果");
        // 如果解析失败，为了不让程序崩溃，我们给一个默认的空字符串
        "".to_string() 
    };

    // 4. 发音 (TTS)
    // 英文发音 URL
    let tts_url_en = format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=en&client=tw-ob",
        encoded_text
    );

    // 只有在成功获取到中文翻译时，才生成中文发音 URL
    let mut args_for_mpv = vec![
        "--no-video".to_string(),
        "--msg-level=all=no".to_string(),
        tts_url_en, // 先播放英文
    ];

    if !translated_text.is_empty() {
        // 对中文翻译结果进行 URL 编码
        let encoded_zh = encode(&translated_text);
        // 中文发音 URL (tl=zh-CN)
        let tts_url_zh = format!(
            "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=zh-CN&client=tw-ob",
            encoded_zh
        );
        args_for_mpv.push(tts_url_zh); // 后播放中文
    }

    println!("🔊 正在发音 (英 -> 中)...");
    
    // 调用系统的 mpv 播放器在后台静默顺序播放音频流
    let _ = Command::new("mpv")
        .args(&args_for_mpv)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    Ok(())
}
