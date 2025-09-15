use crate::models::Request;

pub fn send_request(req: &Request) -> Result<String, String> {
    let response = match req.method.as_str() {
        "GET" => ureq::get(&req.url).call(),
        "POST" => ureq::post(&req.url).call(),
        "PUT" => ureq::put(&req.url).call(),
        "DELETE" => ureq::delete(&req.url).call(),
        _ => ureq::get(&req.url).call(),
    };

    match response {
        Ok(res) => {
            let body = res
                .into_string()
                .unwrap_or_else(|_| "Failed to read body".to_string());
            Ok(body)
        }
        Err(err) => Err(format!("Error: {}", err)),
    }
}
