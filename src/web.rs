use lol_html::{element, HtmlRewriter, Settings};
use reqwest::blocking::{RequestBuilder, Response};

pub(crate) fn is_remote_directory(web_content: &str) -> bool {
    let mut result: bool = false;
    // Python3's http.server
    if web_content.contains("Directory listing") {
        result = true;
    }
    // Apache & nginx
    if web_content.contains("Index of") {
        result = true;
    }
    // ToDo: Find a better way for detecting directories
    result
}

pub(crate) fn retrieve_content_from_web_server(
    download_url: &str,
    client: &reqwest::blocking::Client,
) -> Option<bytes::Bytes> {
    let request: RequestBuilder = client.get(download_url);
    let response: Response = match request.send() {
        Ok(res) => res,
        Err(_) => return None,
    };
    let response_status_code = response.status();
    if response_status_code != 200 {
        return None;
    }
    response.bytes().ok()
}

pub(crate) fn parse_html_and_search_links(web_content_text: &str) -> Vec<String> {
    let mut links: Vec<String> = vec![];
    let mut parser: HtmlRewriter<fn(&[u8])> = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("a[href]", |el| {
                    let href = el.get_attribute("href").expect("No href attribute!");
                    links.push(href);
                    Ok(())
                })
            ],
            ..Settings::new()
        },
        |_: &[u8]| ()
    );
    parser.write(web_content_text.as_ref()).expect("Could not parse HTML!");
    parser.end().expect("Could not finalize HTML parser!");
    links
}
