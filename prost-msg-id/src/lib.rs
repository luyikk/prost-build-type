/*
message Fail{
   enum MsgId {  None=0;Id = 150002; }
   int64 number=1;
   string message=2;
}
 */




/// prost type msg id
pub trait MsgId {
    /// get prost struct msg id
    fn get_msg_id(&self) -> i32;
}

