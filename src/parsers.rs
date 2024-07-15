use scraper::{Html, Selector};

#[derive(Debug)]
pub struct DocumentMetaData {
    pub title: String,
}

impl DocumentMetaData {
    pub fn default() -> DocumentMetaData {
        DocumentMetaData {
            title: String::new(),
        }
    }
}

// Add the /link -> http://baseurl/link + link pages ..
fn filter_links(links: &mut Vec<String>) {
    links.retain(|link| !link.starts_with('#'));
}

// @TODO handle errors and return only a DocumentMetaData object even if u get an err
pub fn parse_html(html: &str) -> (DocumentMetaData, Vec<String>) {
    let document = Html::parse_document(html);

    // Parse title if an err occures or no title found return an empty string
    let title: String = match Selector::parse("title") {
        Ok(title_selector) => document
            .select(&title_selector)
            .next()
            .map(|element| element.text().collect::<String>())
            .unwrap_or_default(),
        Err(e) => {
            println!("Error with selector: {:?}", e);
            String::new()
        }
    };

    let links: Vec<String> = match Selector::parse("a") {
        Ok(links_selector) => {
            let mut collected_links = document
                .select(&links_selector)
                .filter_map(|element| element.attr("href").map(String::from))
                .collect();

            // Apply filtering only if we successfully collected links
            filter_links(&mut collected_links);
            collected_links
        }
        Err(e) => {
            println!("Error parsing selector: {:?}", e);
            Vec::new()
        }
    };

    (DocumentMetaData { title }, links)
}
