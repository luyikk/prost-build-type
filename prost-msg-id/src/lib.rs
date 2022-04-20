/*
message Fail{
   enum MsgId {  None=0;Id = 150002; }
   int64 number=1;
   string message=2;
}
 */




use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

/// prost type msg id trait
pub trait MsgId : Debug {
    /// get struct msg id
    fn get_msg_id(&self) -> i32;
}

impl <T:MsgId> MsgId for Box<T>{
    fn get_msg_id(&self) -> i32 {
        (**self).get_msg_id()
    }
}

impl <T:MsgId> MsgId for Rc<T>{
    fn get_msg_id(&self) -> i32 {
        (**self).get_msg_id()
    }
}

impl <T:MsgId> MsgId for Arc<T>{
    fn get_msg_id(&self) -> i32 {
        (**self).get_msg_id()
    }
}