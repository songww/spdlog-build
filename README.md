# spdlog-src
Build staticlib of spdlog to link with ios project.

xcode 集成 spdlog 很难受，通过 rust 曲线 link spdlog.

## Usage

```rust
use sdplog_src as _;   // this will static link spdlog.

```
