# http_server

A sample http-server to demonstrate running wasi in Kubernetes

The boilerplate web server is in [`src/main.rs`](./src/main.rs). It's configured to invoke the `index`
function in [`src/handler.rs`](./src/handler.rs) in response to both
GET and POST requests. You should put your desired behavior inside
that `index` function.

In case you need to configure some resources for your function, you can do that in the [`configure` function](./src/config.rs).

The app will expose three endpoints:

  * `/` Triggers the `index` function, for either GET or POST methods
  * `/health/readiness` The endpoint for a readiness health check
  * `/health/liveness` The endpoint for a liveness health check

## Development

To get started you will need the following

1. [install WASMEdge](https://wasmedge.org/book/en/quick_start/install.html)

2. Install `cargo wasi` with `cargo install cargo-wasi`

3. Set `CARGO_TARGET_WASM32_WASI_RUNNER=wasmedge` in your `.profile`.


Once the setup is complete you should be able to run the following commands successfully
```shell script
cargo wasi build # build your code in debug mode for the wasi target.

cargo wasi build --release # build the optimized version of your *.wasm.

cargo wasi run # execute a binary.

cargo wasi test # run your tests in wasm32-wasi.

cargo wasi bench # run your benchmarks in wasm32-wasi.
```

Once running, the function is available at <http://localhost:8080> and
the health checks are at <http://localhost:8080/health/readiness> and
<http://localhost:8080/health/liveness>. To POST data to the function,
a utility such as `curl` may be used:

```console
curl -d '{"hello": "world"}' \
  -H'content-type: application/json' \
  http://localhost:8080
```


## Run in Podman

Make sure you have a WASMEdge compatible crun.

```
$ crun --version

crun version 1.7.2
commit: 0356bf4aff9a133d655dc13b1d9ac9424706cac4
rundir: /run/user/1000/crun
spec: 1.0.0
+SYSTEMD +SELINUX +APPARMOR +CAP +SECCOMP +EBPF +CRIU +WASM:wasmedge +YAJL
```
See [local crun build instructions](#crun-build) for more information.


### build
```
cargo wasi build --release
buildah build --annotation "module.wasm.image/variant=compat" -t quay.io/uirlis/http_server
```

### run

```
podman run -p 8080:8080 quay.io/uirlis/http_server
```


## deploy

```
cd charts/http_server
helm install http_server . --create-namespace --namespace wasiservice
```


## crun build

See https://github.com/containers/crun#build for dependency lists etc.

```
$ git clone --depth 1 --recursive https://github.com/containers/crun.git
$ cd /crun
$ ./autogen.sh
$ ./configure --with-wasmedge
$ make
$ ./crun --version

crun version 1.7.2
commit: 0356bf4aff9a133d655dc13b1d9ac9424706cac4
rundir: /run/user/1000/crun
spec: 1.0.0
+SYSTEMD +SELINUX +APPARMOR +CAP +SECCOMP +EBPF +CRIU +WASM:wasmedge +YAJL

# cp $(which crun) crun-bak
# cp crun $(which crun)
```