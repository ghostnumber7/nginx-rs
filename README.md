# nginx-rs
A framework for writing Nginx modules in pure Rust.

This module is in early stages. It lacks documentation and the API is still quite unstable.
But it can be used to write simple request handlers for content or access control.

Heavily copying things from:
* [arvancloud/nginx-rs](https://github.com/arvancloud/nginx-rs)
* [nginxinc/ngx-rust](https://github.com/nginxinc/ngx-rust)
* [dcoles/nginx-rs](https://github.com/dcoles/nginx-rs)

## Building Modules

Building modules requires a checkout of the Nginx sources. If no path is provided it will download nginx and configure it for you using the version specified with env `NGINX_VERSION`.
[configured for building dynamic modules](https://www.nginx.com/blog/compiling-dynamic-modules-nginx-plus/):

```bash
export NGINX_PATH=/path/to/nginx
cd "${NGINX_PATH}"
auto/configure --with-compat
```

Once Nginx is configured, you can then build your module:

```bash
cd /path/to/module
NGINX_PATH=/path/to/nginx cargo build --release
```

Or you can build the module letting the library download and configure nginx:

```bash
cd /path/to/module
NGINX_VERSION=1.19.10 cargo build --release
```

The resulting `.so` in `target/release` can then be loaded using the
[`load_module` directive](https://nginx.org/en/docs/ngx_core_module.html#load_module).

## Examples

- [hello_world](/examples/hello_world) — Demonstrations access control and content handlers

## TODO
* clean up code
* improve docs
* not sure about the derives for the commands config so think about that
* remove lots of `unsafe` calls
* add more features (mail, event)
* add more examples
* implement for static library compiled with nginx
* make it work on mac / windows
* add tests
* find and fix all the bugs I have no idea are there
* add more bugs

## Licence

[nginxinc/ngx-rust](https://github.com/nginxinc/ngx-rust) is licensed under [Apache License](https://github.com/nginxinc/ngx-rust/blob/master/LICENSE) and [nginxinc/ngx-rust](https://github.com/nginxinc/ngx-rust) is licensed under [MIT license](https://github.com/nginxinc/ngx-rust/blob/master/LICENSE) so I have no idea which to put here. Might revise when this is more stable / usable.
