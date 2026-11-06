# Design Notes

## Preserved Source Rationale

<a id="source-notes-10"></a>

### 🦀️.rs:10

🥡️ Only defining zero-allocation empty Vec and paged owners can transfer without an arbitrary Default callback.
```compile_fail,E0277
use semio_framework_value_clone_source::{RetainedCloneClose,RetainedCloneGrant};
use semio_framework_value_clone_source::retirement::{RetireOwned,RetirementCursor};
struct Foreign(String);
impl Default for Foreign{fn default()->Self{Self("unfunded".to_owned())}}
impl RetireOwned for Foreign{
 fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
let mut value=Foreign(String::new());
RetainedCloneClose::default().begin_empty_granted(&mut value,RetainedCloneGrant::default());
```
