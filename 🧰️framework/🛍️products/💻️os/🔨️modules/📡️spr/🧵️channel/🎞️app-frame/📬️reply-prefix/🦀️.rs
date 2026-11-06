//! 📨️ Bounded reply-prefix classification borrows bytes before the complete frame is received.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct AppFrameReplyPrefix{pub in_reply_to:Option<u64>,pub done:bool,pub transaction_terminal:bool}
impl AppFrameReplyPrefix{pub fn answers(self,sequence:u64)->bool{self.in_reply_to==Some(sequence)||self.in_reply_to.is_none()&&self.transaction_terminal}}
/// 🔍️ Reads at most twelve bytes; callers must validate the complete retained frame before delivery.
pub fn app_frame_reply_prefix(bytes:&[u8])->Option<AppFrameReplyPrefix>{
 let tag=*bytes.first()?;let mut at=1;let mut reply=AppFrameReplyPrefix{in_reply_to:None,done:tag==0,transaction_terminal:false};
 match tag{0|1|3|4|6|7|8|10|11|12|14|15|24|26|27|28|29|30|31|32=>{},9|19|20|21=>{match bytes.get(at)?{0=>return Some(reply),1=>at+=1,_=>return None}},16..=18=>{reply.transaction_terminal=true;return Some(reply)},2|5|13|22|23|25=>return Some(reply),_=>return None}
 let mut value=0u64;for index in 0..10{let byte=*bytes.get(at+index)?;if index==9&&byte>1{return None}value|=u64::from(byte&127)<<(index*7);if byte<128{reply.in_reply_to=Some(value);return Some(reply)}}None
}
