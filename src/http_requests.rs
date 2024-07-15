pub struct HttpResponse {
    pub status_code: u16,

    pub content_type: String,
    pub body: String,
}

pub fn make_get_request(url: &str) -> Result<HttpResponse, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;

    let status_code = response.status().as_u16();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .split(';') // To evade things like text/html; charset=utf-8
        .next()
        .unwrap_or("")
        .to_string();
    let body = response.text()?;

    Ok(HttpResponse {
        status_code,
        content_type,
        body,
    })
}
