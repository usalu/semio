//#region 🧪️NativeSourceInventory
use super::*;

#[test]
fn return_source_native_layout_census_before_backing_admission() {
    use std::mem::{align_of, size_of};
    let inventory = serde_json::json!({
        "pointerBytes": size_of::<usize>(),
        "turnResultBytes": size_of::<TurnResult>(),
        "turnResultAlignment": align_of::<TurnResult>(),
        "effectBytes": size_of::<Effect>(),
        "effectAlignment": align_of::<Effect>(),
        "presenceBytes": size_of::<PresenceUpdate>(),
        "presenceAlignment": align_of::<PresenceUpdate>(),
        "uiTurnPatchBytes": size_of::<UiTurnPatches>(),
        "fixedReturnPageBytes": size_of::<semio_framework_actor::byte_page::ActorBytePage>(),
        "fixedReturnResultBytes": size_of::<semio_framework_actor::return_page::ActorReturnResult>(),
        "borrowedMessageCursorBytes": size_of::<return_message::ReturnMessageCursor<'_>>(),
        "effectPageDescriptorBytes": size_of::<Vec<Effect>>(),
        "presencePageDescriptorBytes": size_of::<Vec<PresenceUpdate>>(),
        "sourceBackingAdmitted": false,
        "nativeOwnerMounted": false,
    });
    eprintln!("[DEBUG] return-source native layout census {inventory}");
}
//#endregion 🧪️NativeSourceInventory
