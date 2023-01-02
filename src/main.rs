use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

mod handler;

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => match req.request_target().as_str() {
            "/health/readiness" | "/health/liveness" => return200(),
            _ => handler::index(req),
        },
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write_all(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn return200() -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        String::new(),
    ))
}
