pub(crate) fn is_remote_directory(web_content: &str) -> bool {
    let mut result = false;
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
    let request = client.get(download_url);
    let response = match request.send() {
        Ok(res) => res,
        Err(_) => return None,
    };
    let response_status_code = response.status();
    if response_status_code != 200 {
        return None;
    }
    response.bytes().ok()
}
