FROM scratch 

COPY  target/wasm32-wasi/release/http_server.wasm /

CMD ["./http_server.wasm"]