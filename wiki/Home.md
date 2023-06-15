## Why NAPI-RS (Rust) instead of Neon (Rust) or Node-API (C++)?

This is the most ergonomic way that I could find. I tried Neon, but it's a bit
too verbose for the sort of wrapping that I want to do. The C++ situation is
even worse with more weird boilerplate.

```rs
#[macro_use]
extern crate napi_derive;
use napi::*;

#[napi]
fn hello_world() -> String {
  "world".to_string()
}
```

☝ That's _way_ better than the alternatives!

```cpp
#include <napi.h>

using namespace Napi;

Napi::String Method(const Napi::CallbackInfo& info) {
  Napi::Env env = info.Env();
  return Napi::String::New(env, "world");
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports.Set(Napi::String::New(env, "HelloWorld"),
              Napi::Function::New(env, Method));
  return exports;
}

NODE_API_MODULE(addon, Init)
```

## Devcontainer virtual desktop

This project uses a devcontainer to provide a well-defined development
environment. Inside of our devcontainer, we provide a VNC server that can be
used to connect to a virtual desktop! We are using [desktop-lite]. If you're
using GitHub Codespaces, just click the <kbd>Ports</kbd> tab in the bottom
<kbd>Terminal</kbd> pane and click the <kbd>🌐</kbd> icon next to the `6080`
port. This will open a new browser tab with your virtual desktop! The password
is `vscode`.

## CI/CD

There's a bit of a departure from the usual test/publish divide here. We re-use
a `build-napi-rs.yml` workflow to compile the Rust NAPI-RS addon across all
platforms and targets. We run this both in `test-rust.yml` and
`publish-npm.yml`, so it's worth it to externalize it into a separate workflow
file.

<!-- prettier-ignore-start -->
[desktop-lite]: https://github.com/devcontainers/features/tree/main/src/desktop-lite
<!-- prettier-ignore-end -->
