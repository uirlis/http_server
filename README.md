# http_server

A sample http-server to demonstrate running wasi in Kubernetes

## build
```
cargo build --release --target wasm32-wasi
buildah build --annotation "module.wasm.image/variant=compat" -t quay.io/uirlis/http_server
```

## run

```
podman run -p 1234:1234 quay.io/uirlis/http_server
```

## deploy

```
cd charts/http_server
helm install http_server . --create-namespace --namespace wasiservice
```