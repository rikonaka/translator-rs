use colored::Colorize;
use reqwest;
use serde_json;
#[cfg(target_os = "linux")]
use std::process::Command;
use std::{thread, time::Duration};
#[cfg(target_os = "windows")]
use cli_clipboard;

#[tokio::main]
async fn google_translate(
    translate_string: String,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let max_loop = 100;
    let translate_url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        "en", "zh-CN", translate_string
    );
    let request_result = reqwest::get(translate_url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    // println!("{:#?}", request_result);
    // [[["翻译","translate",null,null,10]],null,"en",null,null,null,null,[]]
    let mut i = 0;
    let mut result_vec: Vec<Vec<String>> = Vec::new();
    loop {
        let mut tmp_vec: Vec<String> = Vec::new();
        let match_string_0 = format!("{}", request_result[0][i][0]);
        let match_string_1 = format!("{}", request_result[0][i][1]);
        match match_string_0.as_str() {
            "null" => break,
            _ => {
                tmp_vec.push(match_string_0.replace("\"", ""));
                tmp_vec.push(match_string_1.replace("\"", ""));
            }
        }
        result_vec.push(tmp_vec);
        i += 1;
        if i > max_loop {
            break;
        }
    }
    Ok(result_vec)
}

fn translate(input_string: String, index: usize) {
    let translate_title = format!("Translate[{}]", index);
    println!(">>> {}", translate_title.bold().red());
    let result_vec = google_translate(input_string).unwrap();
    // println!("{:?}", result_vec);
    for v in result_vec {
        println!("[{}] {}", "O".bright_blue().bold(), v[1]);
        println!("[{}] {}", "T".green().bold(), v[0]);
    }
}

#[cfg(target_os = "windows")]
fn get_clipboard_text_windows() -> Option<String> {
    let output_string = cli_clipboard::get_contents().unwrap();
    Some(output_string)
}

#[cfg(target_os = "linux")]
fn get_select_text_linux() -> Option<String> {
    // return "" at least
    let output = Command::new("xsel")
        .output()
        .expect("Please install xsel first!");
    let output = String::from_utf8_lossy(&output.stdout);
    let output_string = output.to_string();
    let output_replace = output_string
        .replace("-\n", "")
        .replace("%", "")
        .replace("\n", " ");
    Some(output_replace)
}

fn main() {
    let mut index: usize = 1;
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        loop {
            thread::sleep(Duration::from_secs(1));
            let selected_text = get_select_text_linux().unwrap();
            if selected_text.trim().len() > 0 {
                // println!("{}", &selected_text);
                // let test_string = String::from("translate");
                translate(selected_text, index);
                index += 1;
            }
        }
    }
    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let mut last_clipboard_text = String::from("");
        loop {
            thread::sleep(Duration::from_secs(1));
            let clipboard_text = get_clipboard_text_windows().unwrap();
            if clipboard_text != last_clipboard_text {
                last_clipboard_text = clipboard_text.clone();
                if clipboard_text.trim().len() > 0 {
                    translate(clipboard_text, index);
                    index += 1;
                }
            }
        }
    } 
    panic!("Not support running at the other system!");
}
