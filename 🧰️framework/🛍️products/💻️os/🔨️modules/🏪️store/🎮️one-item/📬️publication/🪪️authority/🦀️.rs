//! 🪪️ Publication metadata retains its original strings through typed controlled retirement.
use super::ArtifactStoreOneItemLiveAuthority;
use semio_framework_value::{ValueError,RetainedAllocationEffect,retirement::{RetireOwned,RetirementCursor,RetirementStep,erased_cursor_turn,controlled::ControlledRetirement}};

impl semio_os_spr::command::ArtifactCanonicalEditAuthority for ArtifactStoreOneItemLiveAuthority {
    fn sequence_number(&self)->i32{self.next_sequence_number}
    fn clock(&self)->semio_os_spr::HybridLogicalTimestamp{self.next_clock}
    fn actor(&self)->&(dyn semio_framework_value::paged::Utf8Text+Sync){&self.actor}
    fn line(&self)->Option<&(dyn semio_framework_value::paged::Utf8Text+Sync)>{self.line.as_ref().map(|value|value as &(dyn semio_framework_value::paged::Utf8Text+Sync))}
    fn group(&self)->Option<&(dyn semio_framework_value::paged::Utf8Text+Sync)>{self.group_id.as_ref().map(|value|value as &(dyn semio_framework_value::paged::Utf8Text+Sync))}
}

type AuthorityFields=(semio_framework_value::SharedUtf8,Option<String>,Option<String>,Option<String>);
pub struct PublicationAuthorityFieldsRetirement(ControlledRetirement<AuthorityFields>);
impl RetireOwned for ArtifactStoreOneItemLiveAuthority {
    type RetirementCursor=PublicationAuthorityFieldsRetirement;
    fn retirement_seed(self)->PublicationAuthorityFieldsRetirement{
        let Self{actor,line,group_id,stamped_edit_id,..}=self;
        PublicationAuthorityFieldsRetirement(ControlledRetirement::new((actor,line,group_id,stamped_edit_id)).unwrap_or_else(|_|unreachable!("publication fields support controlled retirement")))
    }
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PublicationAuthorityFieldsRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}
impl PublicationAuthorityFieldsRetirement {
    fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{erased_cursor_turn(self.0.step(body,effect))}
}
impl RetirementCursor for PublicationAuthorityFieldsRetirement {
    semio_framework_value::native_retirement_reply!();
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.0.next_demand(body) }
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.0.next_copy_byte_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_computational_work(body)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.0.next_depth_demand()}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
