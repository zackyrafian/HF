use actix_web::{HttpRequest, HttpResponse, web};
use reqwest::{Client, Method};
use reqwest::header::{HeaderMap, HeaderValue};
use actix_web::http::header;

pub async fn proxy_user_service(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let path_tail = req.match_info().query("tail");
    let url = format!("http://localhost:8081/{}", path_tail);

    let method_str = req.method().as_str();
    let method = Method::from_bytes(method_str.as_bytes()).unwrap_or(Method::GET);

    let client = Client::new();
    let mut builder = client.request(method, &url).body(body.to_vec());

    if let Some(auth) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(val) = HeaderValue::from_bytes(auth.as_bytes()) {
            builder = builder.header("Authorization", val);
        }
    }

    if let Some(cookie) = req.headers().get(header::COOKIE) {
        if let Ok(val) = HeaderValue::from_bytes(cookie.as_bytes()) {
            builder = builder.header("cookie", val);
        }
    }

    if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) { 
        if let Ok(val) = HeaderValue::from_bytes(content_type.as_bytes()){ 
            builder = builder.header("Content-Type", val);
        }
    }

    match builder.send().await {
        Ok(response) => {
            let status_code = actix_web::http::StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

            let headers = response.headers().clone(); 
            let body = response.bytes().await.unwrap_or_default();

            let mut client_resp = HttpResponse::build(status_code);

            if let Some(set_cookie) = headers.get("set-cookie") {
                client_resp.insert_header(("set-cookie", set_cookie.to_str().unwrap_or("")));
            }

            client_resp.body(body)
        }

        Err(err) => {
            println!(" Proxy error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to connect to backend")
        }
    }
}
