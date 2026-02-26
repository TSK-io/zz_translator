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
    // 这里调用的是 Google 翻译的公开免费接口
    let trans_url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=zh-CN&dt=t&q={}",
        encoded_text
    );

    // Rust 的 reqwest 会被 daed 完美透明代理，瞬间返回！
    let res = reqwest::get(&trans_url).await?.text().await?;

    // 3. 解析并打印翻译结果
    // Google 返回的 JSON 是个嵌套数组，比如：[[["你好","hello",null,null,10]],null,"en"]
    let v: serde_json::Value = serde_json::from_str(&res)?;
    if let Some(translated) = v[0][0][0].as_str() {
        // 使用 ANSI 转义码打印带颜色的结果（绿色加粗）
        println!("\n✨ 翻译结果: \x1b[1;32m{}\x1b[0m\n", translated);
    } else {
        println!("⚠️ 未能解析翻译结果");
    }

    // 4. 发音 (TTS)
    // 直接把文本喂给 Google 的 TTS 接口
    let tts_url = format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl=en&client=tw-ob",
        encoded_text
    );

    println!("🔊 正在发音...");
    
    // 调用系统的 mpv 播放器在后台静默播放音频流
    let _ = Command::new("mpv")
        .arg("--no-video")         // 不显示视频窗口
        .arg("--msg-level=all=no") // 屏蔽 mpv 烦人的日志输出
        .arg(&tts_url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    Ok(())
}
