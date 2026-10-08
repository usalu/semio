//! 🧪️ `remove-content-type` satisfies the inverse sum law for a removed default and a removed override at their positions.

use super::super::*;

#[semio_framework_async_macros::async_test]
async fn remove_content_type_restores_the_entry_at_its_position() {
    let base = fixture();
    let laws = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    laws(&DocxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: "zzdemo".into() }), &base).await;
    let grown = protocol::apply_diff(opc_layer::with_package(&base, |opc| opc_layer::content_type_write_diff(opc, false, "zzmiddle", "application/x-semio-demo", Some(1))).unwrap().as_ref().unwrap(), &base).expect("the middle default is added");
    laws(&DocxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: "zzmiddle".into() }), &grown).await;
    let with_override = protocol::apply_diff(opc_layer::with_package(&base, |opc| opc_layer::content_type_write_diff(opc, true, "/ppt/zz-demo.bin", "application/x-semio-demo", Some(0))).unwrap().as_ref().unwrap(), &base).expect("the override is added");
    laws(&DocxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: true, name: "/ppt/zz-demo.bin".into() }), &with_override).await;
}
