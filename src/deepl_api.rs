use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::errors::DeepLEmptyAuthKeyError;
use crate::utils::{build_proxy, fliter_long, fliter_short};
use crate::TranslateResult;
use crate::TIMEOUT;

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeepLResponse {
    pub translations: Vec<Translation>,
}

async fn tranlate(
    sl: &str, // source language
    tl: &str, // target language
    content: &str,
    proxy_str: &str,
    auth_key: &str,
    translate_url: &str,
) -> Result<Vec<TranslateResult>> {
    let translate_string = fliter_long(content);
    let translate_string = fliter_short(&translate_string);

    let proxy = build_proxy(proxy_str);
    let client = match proxy {
        Some(p) => reqwest::Client::builder()
            .proxy(p)
            .build()
            .expect("proxy client build failed"),
        _ => reqwest::Client::new(),
    };

    let auth_value = format!("DeepL-Auth-Key {}", auth_key);
    let body = format!(
        "text={}&source_lang={}&target_lang={}&split_sentences=1",
        translate_string, sl, tl
    );
    let res = client
        .post(translate_url)
        .header("Authorization", auth_value)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await?
        .json::<DeepLResponse>()
        .await?;

    let mut result_vec = Vec::new();
    let trans = res.translations;
    for t in trans {
        let item = TranslateResult {
            trans: t.text,
            orig: translate_string.to_string(),
            alter: Vec::new(),
        };
        result_vec.push(item);
    }
    Ok(result_vec)
}

pub async fn translate_free(
    sl: &str, // source language
    tl: &str, // target language
    content: &str,
    proxy_str: &str,
    auth_key: &str,
) -> Result<Vec<TranslateResult>> {
    if auth_key == "null" || auth_key.len() == 0 {
        return Err(DeepLEmptyAuthKeyError.into());
    }
    let translate_url = format!("https://api-free.deepl.com/v2/translate");
    tranlate(sl, tl, content, proxy_str, auth_key, &translate_url).await
}

pub async fn translate_pro(
    sl: &str, // source language
    tl: &str, // target language
    content: &str,
    proxy_str: &str,
    auth_key: &str,
) -> Result<Vec<TranslateResult>> {
    if auth_key == "null" || auth_key.len() == 0 {
        return Err(DeepLEmptyAuthKeyError.into());
    }
    let translate_url = format!("https://api.deepl.com/v2/translate");
    tranlate(sl, tl, content, proxy_str, auth_key, &translate_url).await
}
