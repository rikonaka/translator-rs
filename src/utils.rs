use anyhow::Result;
use reqwest::Proxy;
use std::process::Command;

use crate::errors::UnsupportApiError;

fn get_clipboard_text_linux() -> Result<String> {
    let output = match Command::new("xsel").arg("-b").output() {
        Ok(o) => o,
        Err(_) => panic!("please install xsel"),
    };
    let output = String::from_utf8_lossy(&output.stdout).to_string();
    // println!("clipboard text: {}", &output);
    if output.trim().len() > 0 {
        return Ok(output.trim().to_string());
    } else {
        return Ok("".to_string());
    }
}

fn get_clipboard_text_windows() -> Result<String> {
    let output = match Command::new("powershell")
        .args(["-Command", "Get-Clipboard"])
        .output()
    {
        Ok(o) => o,
        Err(_) => panic!("run command Get-Clipboard failed"),
    };
    let output = String::from_utf8_lossy(&output.stdout).to_string();
    // println!("select text: {}", &output);
    if output.trim().len() > 0 {
        return Ok(output.trim().to_string());
    } else {
        return Ok("".to_string());
    }
}

pub fn get_clipboard_text() -> Result<String> {
    if cfg!(target_os = "linux") {
        get_clipboard_text_linux()
    } else if cfg!(target_os = "windows") {
        get_clipboard_text_windows()
    } else {
        Ok(String::new())
    }
}

fn get_select_text_linux() -> Result<String> {
    let output = match Command::new("xsel").output() {
        Ok(o) => o,
        Err(_) => panic!("please install xsel"),
    };
    let output = String::from_utf8_lossy(&output.stdout).to_string();
    // println!("select text: {}", &output);
    if output.trim().len() > 0 {
        return Ok(output.trim().to_string());
    } else {
        return Ok("".to_string());
    }
}

pub fn get_select_text() -> Result<String> {
    if cfg!(target_os = "linux") {
        get_select_text_linux()
    } else {
        Ok(String::new())
    }
}

pub struct Text {
    pub content: String,
}

impl Text {
    fn new(content: &str) -> Text {
        let content = content.to_string();
        Text { content }
    }
    fn filter(content: &str) -> String {
        let x = content.trim();
        let x = match x.strip_prefix(".") {
            Some(x) => x,
            _ => x,
        };
        let x = match x.strip_prefix(",") {
            Some(x) => x,
            _ => x,
        };
        x.replace("-\n", "")
            .replace("%", "%25")
            .replace("&", "%26")
            .replace("#", "%23")
            .replace("\n", " ")
            .trim()
            .to_string()
    }
    pub fn get_text(use_clipboard: bool) -> Text {
        match use_clipboard {
            true => {
                let t = match get_clipboard_text() {
                    Ok(t) => t,
                    Err(e) => {
                        println!("get clipboard text failed: {}", e);
                        "".to_string()
                    }
                };
                let ft = Text::filter(&t);
                return Text::new(&ft);
            }
            false => {
                let t = match get_select_text() {
                    Ok(t) => {
                        // only get text from select
                        t
                    }
                    Err(e) => {
                        println!("get select text failed: {}", e);
                        "".to_string()
                    }
                };
                let ft = Text::filter(&t);
                return Text::new(&ft);
            }
        }
    }
}

pub fn standardized_lang<'a>(
    sl: &'a str,  // source language
    tl: &'a str,  // target language
    api: &'a str, // api privoder
) -> Result<(&'a str, &'a str)> {
    let convert = match api {
        "google" => {
            let converter = |x: &str| -> &str {
                let result = match x {
                    "English" => "en",
                    "Chinese (Simplified)" => "zh-CN",
                    "Chinese (Traditional)" => "zh-TW",
                    "Japanese" => "ja",
                    "Korean" => "ko",
                    "French" => "fr",
                    "Russian" => "ru",
                    "German" => "de",
                    "Spanish" => "es",
                    "Italian" => "it",
                    _ => "en",
                };
                result
            };
            converter
        }
        "deepl" | "deeplpro" => {
            let convert_language = |x: &str| -> &str {
                let result = match x {
                    "English" => "EN",
                    "Chinese" => "ZH",
                    "Japanese" => "JA",
                    "French" => "FR",
                    "German" => "DE",
                    "Korean" => "KO",
                    "Russian" => "RU",
                    "Spanish" => "ES",
                    "Italian" => "IT",
                    "English (American)" => "EN-US",
                    "English (British)" => "EN-GB",
                    "Chinese (Simplified)" => "ZH",
                    _ => "EN-US",
                };
                result
            };
            convert_language
        }
        _ => return Err(UnsupportApiError.into()),
    };
    let sl_ret = convert(sl);
    let tl_ret = convert(tl);
    Ok((sl_ret, tl_ret))
}

pub fn fliter_long(input: &str) -> String {
    input.replace("al.", "al")
}

pub fn fliter_short(input: &str) -> String {
    input
        .replace(".", "")
        .replace(",", "")
        .replace("?", "")
        .replace("!", "")
        .replace(":", "")
        .replace("\"", "")
        .replace("(", "")
        .replace(")", "")
        .replace("<", "")
        .replace(">", "")
        // UTF-8 char
        .replace("“", "")
        .replace("”", "")
        .replace("。", "")
        .replace("，", "")
        .replace("：", "")
        .replace("（", "")
        .replace("）", "")
        .replace("《", "")
        .replace("》", "")
}

pub fn build_proxy(proxy_str: &str) -> Option<Proxy> {
    let proxy = match proxy_str {
        "null" => None,
        _ => Some(reqwest::Proxy::https(proxy_str).expect("set proxy failed")),
    };
    proxy
}
