//! 🧪️ Original Record laws retain payload grants while admitting exact owned allocation events.
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};

pub fn retain<T:RetireOwned>(value:T)->ControlledRetirement<T>{ControlledRetirement::new(value).unwrap_or_else(|_|panic!("Record test owner lacks controlled retirement authority"))}
pub fn advance<T:RetireOwned>(owner:&mut ControlledRetirement<T>,maximum_copy_bytes:usize)->Result<RetainedCloneStep,ValueError>{
    let copy=owner.next_copy_byte_demand();assert!(copy<=maximum_copy_bytes);let release=owner.next_release_byte_demand()?;let capacity=owner.next_capacity_byte_demand(if copy==0{release}else{maximum_copy_bytes})?;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:owner.next_depth_demand()?};let step=owner.step(grant)?;assert!(step.progress().fits(grant));Ok(step)
}
