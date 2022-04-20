# prost struct msg id trait

```protobuf
message Fail{
   enum MsgId {  None=0;Id = 150002; }
   int64 number=1;
   string message=2;
}
```
### auto impl MsgId
```rust
impl MsgId for Fail {
    fn get_msg_id(&self) -> i32 {
        Self::get_msg_id()
    }
}
```