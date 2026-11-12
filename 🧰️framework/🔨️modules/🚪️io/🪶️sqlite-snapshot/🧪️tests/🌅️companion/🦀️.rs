//! 🌅️ Companion tests of io-sqlite-snapshot: snapshot unit, limits, refusal, retirement, receiving and artifact reader tests.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

use semio_framework_value_testing_allocator as _;

pub use semio_framework_io_sqlite_snapshot::*;
use semio_framework_io_control as original_control;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError as OriginalValueError};
use semio_framework_value::retained_clone::{OriginalSourceTurnIdentity,OriginalSourceTurnIssuer};
use semio_framework_value_native_owned::native_decoding::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationRefusal,NativeAllocationReply,NativeAllocationReplyComplete,NativeSendAllocationLoan,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionLoan,OriginalConstructionRecipient,OriginalWorkAuthority,OriginalWorkLoan,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient};

#[path="../../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
pub(crate) mod effect;

/// 🔌️ Adapts one recording closure to an actual original allocation authority port.
struct EventPort<F>(F);
impl<F:FnMut(NativeAllocationEvent)->Result<(),NativeAllocationRefusal>> NativeAllocationAuthorityPort for EventPort<F>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),OriginalValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,OriginalValueError>where 'source:'turn{Ok(reply.complete((self.0)(event)))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,OriginalValueError>where 'source:'turn{Ok(reply.pending())}
}

/// 🛂️ Lends one genuine Send allocation loan of an original port that answers every event through the recording closure.
pub(crate) fn with_allocation_port<R>(port:impl FnMut(NativeAllocationEvent)->Result<(),NativeAllocationRefusal>+Send,operation:impl FnOnce(&mut dyn NativeSendAllocationLoan)->R)->R{
 let mut authority=NativeAllocationAuthority::new(EventPort(port));
 let result=authority.with_send_loan(&mut (),operation).expect("fixture allocation port validates");
 assert!(authority.into_port().is_ok(),"fixture allocation port settles every transaction");
 result
}

/// 🪙️ A fixture Work recipient of one Source turn that admits every quoted action.
struct WorkScope{turn:OriginalSourceTurnIdentity}
impl OriginalWorkRecipient for WorkScope{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),OriginalValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn settle_partial<'a,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'a,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}
 fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'a>,_:u64,_:u64)->OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}
}

/// 🏗️ A fixture construction recipient of one Source turn that grants the original owner grant and accepts every receipt.
struct ConstructionScope{grant:RetainedCloneGrant,submitted:RetainedCloneProgress,turn:OriginalSourceTurnIdentity}
impl OriginalConstructionRecipient for ConstructionScope{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),OriginalValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(self.grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let progress=receipt.progress();let result=self.submitted.checked_add(progress).map(|sum|self.submitted=sum);receipt.complete(result)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}

/// 🧰️ Lends the genuine Work and construction loans an original caller gives its snapshot owner for the duration of one operation.
pub(crate) fn with_original_authorities<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut OriginalWorkLoan<'_>,&mut OriginalConstructionLoan<'_>)->Result<R,OriginalValueError>)->R{
 let issuer=OriginalSourceTurnIssuer::new();let source=issuer.issue().expect("fixture Source dispatch");let turn=source.identity();
 let mut construction=OriginalConstructionAuthority::new(ConstructionScope{grant,submitted:Default::default(),turn});let mut work=OriginalWorkAuthority::new(WorkScope{turn});
 construction.with_loan(&mut (),|construction|work.with_loan(&mut (),|work|operation(work,construction))).expect("fixture construction authority").expect("fixture work authority").expect("fixture original authorities")
}

#[path = "."]
pub mod component {
    pub use semio_framework_io_sqlite_snapshot::testkit::component::*;

    #[path = "../🔬️unit/🦀️.rs"]
    mod tests;

    #[path = "../../⚠️refusal/🧪️tests/🦀️.rs"]
    mod typed_refusal_tests;

    #[path = "../../♻️retirement/🧪️tests/🦀️.rs"]
    mod original_field_retirement_tests;

    #[path = "../../⚠️refusal/💰️limits/🧪️tests/🦀️.rs"]
    mod original_fixed_limit_tests;

    #[path = "../../🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs"]
    mod original_physical_receiving_tests;

    #[path = "."]
    pub mod artifact {
        pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::*;
        use semio_framework_io_sqlite_snapshot::testkit::component::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind};
        pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::testkit::row_writer;

        #[path = "../../🧩️artifact/🧪️tests/🫳️reconstruction/🦀️.rs"]
        mod reconstruction_copy_tests;

        #[path = "../../🧩️artifact/🧪️tests/🧮️allocation/🦀️.rs"]
        mod shared_provider_allocation_tests;

        #[path = "../../🧩️artifact/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;

        #[path = "../../🔢️ieee754/🧪️tests/🔬️unit/🦀️.rs"]
        mod ieee_tests;

        #[path = "../../🧩️artifact/🫴️receiving/🧪️tests"]
        pub mod receiving {
            pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::receiving::*;
            use semio_framework_io_sqlite_snapshot::testkit::component::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl};
            use semio_framework_value::{ValueError, ValueRefusalKind, NativeEncodeControl, RetainedCloneGrant, RetainedCloneProgress};
            use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

            #[path = "🦀️.rs"]
            mod tests;
        }
    }

    #[path = "../../🧵️composition/🧪️tests"]
    pub mod composition {
        pub use semio_framework_io_sqlite_snapshot::testkit::component::composition::*;

        #[path = "🦀️.rs"]
        mod tests;
    }
}
