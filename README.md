[![Documentation](https://docs.rs/prost-msg-build/badge.svg)](https://docs.rs/prost-msg-build/)
[![Crate](https://img.shields.io/crates/v/prost-msg-build.svg)](https://crates.io/crates/prost-msg-build)

# prost-msg-build

build prost struct with msg id so as to realize type judgment
* [examples](https://github.com/luyikk/prost-test/)

```protobuf
message Fail{
   enum MsgId {  None=0;Id = 150002; }
   int64 number=1;
   string message=2;
}
```
### auto impl MsgId and const fn
```rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fail {
    #[prost(int64, tag = "1")]
    pub number: i64,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}

#[allow(dead_code)]
pub const FAIL_ID: i32 = Fail::get_msg_id();

impl Fail {
    #[allow(dead_code)]
    pub const fn get_msg_id() -> i32 {
        150002
    }
}

impl ::prost_msg_id::MsgId for Fail {
    fn get_msg_id(&self) -> i32 {
        Self::get_msg_id()
    }
}

///get all msg type id
#[allow(dead_code)]
pub const fn msg_ids()->&'static [i32]{
    &[
        150002, //.RunTT.Fail
    ]
}
```



## about `prost-build`

# `prost-build`

`prost-build` makes it easy to generate Rust code from `.proto` files as part of
a Cargo build. See the crate [documentation](https://docs.rs/prost-build/) for examples
of how to integrate `prost-build` into a Cargo project.

## `protoc`

`prost-build` uses `protoc` to parse the proto files. There are two ways to make `protoc`
available for `prost-build`:

* Include `protoc` in your `PATH`. This can be done by following the [`protoc` install instructions].
* Pass the `PROTOC=<my/path/to/protoc>` environment variable with the path to
  `protoc`.

[`protoc` install instructions]: https://github.com/protocolbuffers/protobuf#protocol-compiler-installation

## License

`prost-build` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](../LICENSE) for details.

Copyright 2017 Dan Burkert

