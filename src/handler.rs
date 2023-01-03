use hyper::{Body, Method, Request, Response, StatusCode};
pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("Hi There!"))),
        (&Method::POST, "/") => Ok(Response::new(req.into_body())),

        (&Method::GET, "/health/readiness") => Ok(Response::new(Body::from(""))),
        (&Method::GET, "/health/liveness") => Ok(Response::new(Body::from(""))),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_test() -> Result<(), Box<dyn std::error::Error>> {
        let req = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri("/")
            .header("user-agent", "rust-test")
            .body(hyper::Body::from(""))?;

        let resp = handle_request(req).await?;
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body, "Hi There!");
        Ok(())
    }
    #[tokio::test]
    async fn post_test() -> Result<(), Box<dyn std::error::Error>> {
        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri("/")
            .header("user-agent", "rust-test")
            .body(hyper::Body::from("echo"))?;

        let resp = handle_request(req).await?;
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body, "echo");
        Ok(())
    }
}
