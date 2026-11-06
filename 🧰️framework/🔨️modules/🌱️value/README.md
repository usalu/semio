# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.

Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
payload decodes to: the authority validates it, the optimistic replica applies it, and the
pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
and stay product-side.

🚱️ In-place shared retirement drains fields of records without a custom destructor.
```
#[derive(semio_framework_value::RetireOwned)]
#[value(crate="semio_framework_value")]
struct Drained(u8);
assert!(semio_framework_value::retirement::RetireOwned::shared_original_birth_bytes(&Drained(7)).is_some());
```
```compile_fail,E0283
#[derive(semio_framework_value::RetireOwned)]
#[value(crate="semio_framework_value")]
struct CustomDestructor(u8);
impl Drop for CustomDestructor { fn drop(&mut self) {} }
```
🎯️ Every clone cursor must author a correlated close selector.
```
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep}};
struct Cursor;
impl RetainedCloneCursor<u8> for Cursor {
    fn advance(&mut self,_:RetainedCloneRef<'_,u8>,_:usize,_:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
    fn take(&mut self,_:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<(u8,semio_framework_value::RetainedCloneProgress)>>{semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0}}
    fn begin_close(&mut self)->bool{true}
    fn terminal_is_empty(&self)->bool{true}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
}
assert_eq!(RetainedCloneCursor::<u8>::close_demands(&Cursor,4096).unwrap(),RetirementDemand::default());
```
```compile_fail,E0046
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep}};
struct Cursor;
impl RetainedCloneCursor<u8> for Cursor {
    fn advance(&mut self,_:RetainedCloneRef<'_,u8>,_:usize,_:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
    fn take(&mut self,_:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<(u8,semio_framework_value::RetainedCloneProgress)>>{semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0}}
    fn begin_close(&mut self)->bool{true}
    fn terminal_is_empty(&self)->bool{true}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
}
```
🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9
