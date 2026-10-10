//! 🎟️ A retained turn intersects independent whole-owner authority before cumulative arithmetic.
use crate::{ValueError,ValueRefusalKind};
pub(crate) fn maximum(owned:usize,capacity:usize,whole:Option<usize>)->Result<usize,ValueError>{
 let refusal=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native turn capacity exceeds address space");
 let permitted=match whole{Some(maximum)=>maximum.min(isize::MAX as usize).checked_sub(owned).map(|remaining|remaining.min(capacity)).ok_or_else(refusal)?,None=>capacity};
 owned.checked_add(permitted).filter(|next|*next<=isize::MAX as usize).ok_or_else(refusal)
}
