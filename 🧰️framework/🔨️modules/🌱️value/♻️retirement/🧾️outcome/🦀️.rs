//! 🧾️ Retained outcome retirement preserves the actual Result owner; work is closed receipt metadata.
use super::{RetireOwned,RetirementSupport};
use crate::RetainedWorkOutcome;
impl<T:RetireOwned,E:RetireOwned> RetireOwned for RetainedWorkOutcome<T,E>{
 type RetirementCursor=<Result<T,E>as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.result.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{self.result.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.result.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{super::support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<Result<T,E>>()}
}
