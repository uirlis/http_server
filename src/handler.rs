use httpcodec::{HttpVersion, Method, ReasonPhrase, Request, Response, StatusCode};
use log::info;

pub fn index(req: Request<String>) -> bytecodec::Result<Response<String>> {
    info!("{:#?}", req);
    if req.method() == Method::new("GET").unwrap() {
        Ok(Response::new(
            HttpVersion::V1_0,
            StatusCode::new(200)?,
            ReasonPhrase::new("")?,
            "Get Request!\n".to_string(),
        ))
    } else {
        Ok(Response::new(
            HttpVersion::V1_0,
            StatusCode::new(200)?,
            ReasonPhrase::new("")?,
            format!("echo: {}", req.body()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpcodec::RequestTarget;
    #[test]
    fn get_test() {
        let req = Request::<String>::new(
            Method::new("GET").unwrap(),
            RequestTarget::new("/").unwrap(),
            HttpVersion::V1_0,
            String::new(),
        );
        let res = index(req);
        assert_eq!(res.unwrap().body(), "Get Request!\n");
    }
    #[test]
    fn post_test() {
        let req = Request::<String>::new(
            Method::new("POST").unwrap(),
            RequestTarget::new("/").unwrap(),
            HttpVersion::V1_0,
            "echo".to_string(),
        );
        let res = index(req);
        assert_eq!(res.unwrap().body(), "echo: echo");
    }
}
