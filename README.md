[![Documentation](https://docs.rs/prost-build/badge.svg)](https://docs.rs/prost-msg-build/)
[![Crate](https://img.shields.io/crates/v/prost-build.svg)](https://crates.io/crates/prost-msg-build)

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

`prost-build` makes it easy to generate Rust code from `.proto` files as part of
a Cargo build. See the crate [documentation](https://docs.rs/prost-build/) for examples
of how to integrate `prost-build` into a Cargo project.

## `protoc`

`prost-build` uses `protoc` to parse the proto files. There are a few ways to make `protoc`
available for `prost-build`. 

The first option is to include `protoc` in your `PATH` this
can be done by following the [`protoc` install instructions]. In addition, its possible to
pass the `PROTOC=<my/path/to/protoc>` environment variable.

[`protoc` install instructions]: https://github.com/protocolbuffers/protobuf#protocol-compiler-installation

The second option is to provide the `vendored` feature flag to `prost-build`. This will
force `prost-build` to compile `protoc` from the bundled source. This will require that
you have the correct dependencies installed include a C++ toolchain, cmake, etc. For
more info on what the required dependencies are check [here].

[here]: https://github.com/protocolbuffers/protobuf/blob/master/src/README.md

If you would like to always ignore vendoring `protoc` you can additionally pass
`PROTOC_NO_VENDOR` and this will always check the `PATH`/`PROTOC` environment
variables and never compile `protoc` from source.

## License

`prost-build` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](../LICENSE) for details.

Copyright 2017 Dan Burkert
