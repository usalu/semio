//! ♻️ Outcome retirement retains its actual value and diagnostic field authorities.
use super::{Diagnostic,IoOutcome,IoError,ValueError};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes};
impl<T:RetireOwned> RetireOwned for IoOutcome<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{value,diagnostics}=self;sequence(vec![deferred(value),deferred(diagnostics)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.value),deferred_birth_bytes_for(&self.diagnostics)])}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()&&<Vec<Diagnostic>>::controlled_retirement_supported()}
}
impl RetireOwned for IoError{
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{cause,diagnostics}=self;sequence(vec![deferred(cause),deferred(diagnostics)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.cause),deferred_birth_bytes_for(&self.diagnostics)])}
 fn controlled_retirement_supported()->bool{ValueError::controlled_retirement_supported()&&<Vec<Diagnostic>>::controlled_retirement_supported()}
}
