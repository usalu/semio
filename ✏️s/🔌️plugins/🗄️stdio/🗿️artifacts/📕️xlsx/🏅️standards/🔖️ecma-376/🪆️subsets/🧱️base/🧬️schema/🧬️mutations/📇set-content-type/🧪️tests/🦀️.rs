//! 🧪️ `set-content-type` satisfies the inverse sum law for a new default and override at the first, a middle and the last position.

use super::super::*;

#[semio_framework_async_macros::async_test]
async fn set_content_type_inverts_to_the_exact_previous_state() {
    let base = fixture();
    let laws = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let (defaults, overrides) = opc_layer::with_package(&base, |opc| (opc.content_types.defaults.len(), opc.content_types.overrides.len())).unwrap();
    let set = |is_override: bool, name: &str, index: Option<usize>| XlsxMutation::SetContentType(set_content_type::SetContentType { is_override, name: name.into(), content_type: "application/x-semio-demo".into(), index });
    laws(&set(false, "zzfirst", Some(0)), &base).await;
    laws(&set(false, "zzmiddle", Some(defaults / 2)), &base).await;
    laws(&set(false, "zzlast", None), &base).await;
    laws(&set(true, "/ppt/zz-first.bin", Some(0)), &base).await;
    laws(&set(true, "/ppt/zz-middle.bin", Some(overrides / 2)), &base).await;
    laws(&set(true, "/ppt/zz-last.bin", None), &base).await;
}
