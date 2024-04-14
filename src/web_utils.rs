use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, HeaderValue, HOST, USER_AGENT};

pub async fn send_orzmic_request(body: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:8888")?)
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_static("orzmic.big-true.top"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Orzmic/0 CFNetwork/1240.0.4 Darwin/20.5.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("zh_cn"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    headers.insert("X-Unity-Version", HeaderValue::from_static("2020.3.33f1c2"));
    headers.insert("Proxy-Connection", HeaderValue::from_static("Keep-Alive"));

    let response = client
        .post("http://orzmic.big-true.top/update/")
        .headers(headers)
        .body(body)
        .send()
        .await?;

    let response_body = response.text().await?;

    Ok(response_body)
}