mod http_requests;
mod parsers;

use anyhow::{anyhow, Ok, Result};

// struct Document {
//     url: String,
//     parent_url: String,
//     child_urls: Vec<String>,
//     response: http_requests::HttpResponse,
// }
const SUPPORTED_CONTENT_TYPES: [&str; 1] = ["text/html"];

fn crawl_page(url: &str) -> Result<(), anyhow::Error> {
    let response = http_requests::make_get_request(url)?;

    // Debugging
    println!("STATUS CODE: {}", response.status_code);
    println!("CONTENT-TYPE: {}", response.content_type.trim());
    // println!("BODY: {}", response.body.trim());

    // Checks if we have any http errors (I.e status codes +399)
    if response.status_code > 399 {
        return Err(anyhow!("HTTP error: status code {}", response.status_code));
    }

    // Checks if the response content-type is supported
    if !SUPPORTED_CONTENT_TYPES.contains(&response.content_type.as_str()) {
        return Err(anyhow!(
            "Unsupported content type: {}",
            response.content_type
        ));
    }

    // Extract parse the document based on the content type, or return "" for all values if there was a problem
    let (metadata, child_urls) = match response.content_type.as_str() {
        "text/html" => parsers::parse_html(response.body.as_str()),
        _ => (parsers::DocumentMetaData::default(), Vec::new()),
    };

    println!("Page MetaData: {:?}", metadata);
    println!("Page Children: {:?}", child_urls);

    Ok(())
}

fn main() {
    let url: &str = "https://en.wikipedia.org/wiki/Main_Page";
    crawl_page(url).unwrap();
}
