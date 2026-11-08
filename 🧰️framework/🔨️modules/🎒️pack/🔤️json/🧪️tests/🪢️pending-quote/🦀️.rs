//! 🪢️ A pending original allocation quote yields the turn and resumes on the next one; it is not an invariant refusal.
use semio_framework_pack_json::JsonBorrowedWriteCursor;
use semio_framework_value::{native_encoding::NativeEncodeControl,DslValue,RetainedCloneGrant,ValueError};
use semio_framework_value_control_contract::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationReply,NativeAllocationReplyComplete,OriginalSourceTurnIdentity};

struct Late{calls:usize}
impl NativeAllocationAuthorityPort for Late{
    type Context<'turn,'source>=() where 'source:'turn;
    fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
    fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
    fn receive<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{self.calls+=1;if self.calls==1{Ok(reply.pending())}else{Ok(reply.complete(Ok(())))}}
    fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.complete(Ok(())))}
}

#[test]
fn json_writer_yields_on_a_pending_allocation_quote_and_resumes_it(){
    let word="pending quote";let source=DslValue::String(word.into());
    let grant=RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX};
    let mut authority=NativeAllocationAuthority::new(Late{calls:0});
    let output=authority.with_send_loan(&mut (),|loan|{
        let mut observe=|_|true;let mut native=NativeEncodeControl::new_forwarded(1<<20,&mut observe,loan);
        let mut cursor=JsonBorrowedWriteCursor::new();let mut yielded=0usize;
        for _ in 0..1024{match cursor.step(&source,1,&mut native,grant){Ok(Some(text))=>return(Some(text),yielded),Ok(None)=>yielded+=1,Err(error)=>panic!("pending quote refused as {error:?} after {yielded} yielded turns")}}
        (None,yielded)
    }).unwrap();
    assert!(authority.into_port().is_ok());
    assert!(output.1>0,"the pending quote must yield at least one turn");
    assert_eq!(output.0.unwrap(),serde_json::to_string(word).unwrap());
}
