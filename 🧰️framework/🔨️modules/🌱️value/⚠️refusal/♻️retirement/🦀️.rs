//! ♻️ Native refusal prose retires through its original borrowed or owned text authority.
use super::ValueError;
use crate::retirement::{RetireOwned,RetirementCursor};
use std::borrow::Cow;
impl RetireOwned for ValueError {
 fn retirement(self)->Box<dyn RetirementCursor>{match self.message{Cow::Borrowed(_)=>().retirement(),Cow::Owned(text)=>text.retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match &self.message{Cow::Borrowed(_)=>().retirement_birth_bytes(),Cow::Owned(text)=>text.retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{true}
}
