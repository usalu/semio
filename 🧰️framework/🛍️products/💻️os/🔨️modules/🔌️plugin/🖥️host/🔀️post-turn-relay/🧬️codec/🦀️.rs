//! 🧬️ The embedding CLI supplies immutable operation data; every pure codec call receives a genuinely fresh Source.
use semio_plugin_host_wake::{GuestRelayOperationCaps,GuestRelayOperationRequest,GuestRelayWakeDriver};
use semio_framework_value::ValueError;
#[path="📥️receiving/🦀️.rs"]
mod receiving;
pub(crate) use receiving::GuestCodecReceivingLedger;
/// 🏛️ A configured issuer contains neither an existing caller identity nor a reusable private clock authority.
#[derive(Clone,Copy)]
pub struct GuestCodecOperationIssuer{caps:GuestRelayOperationCaps,request:GuestRelayOperationRequest,clock:fn()->Option<u64>}
impl GuestCodecOperationIssuer{
 /// 📏️ Validates the explicit configured guest limits against cleanup and transit before any call begins.
 pub fn new(caps:GuestRelayOperationCaps,request:GuestRelayOperationRequest,clock:fn()->Option<u64>)->Result<Self,ValueError>{caps.bind_request(request)?;Ok(Self{caps,request,clock})}
 /// 🪪️ Publishes only effective logical limits; these values cannot resume or renew any private Source.
 pub fn admitted_request(&self)->Result<GuestRelayOperationRequest,ValueError>{self.caps.admitted_request(self.request)}
 /// 🌱️ Returns the actual new owner by value so its embedding parent retains it before publishing a future or capsule.
 pub fn issue_source(&self,cancelled:Box<dyn Fn()->bool+Send+Sync>)->Result<GuestRelayWakeDriver,ValueError>{self.caps.issue(self.request,self.clock,cancelled)}
}
