//! 🏛️ Authored host configuration declarations select original source projections before any producer runs.
/// 🧾️ Domain-owned declarations bind one text projection and its actual configuration mutation constructor.
pub struct OriginalHostConfigurationDeclaration<M: 'static>{pub maximum_input_bytes:usize,pub maximum_output_bytes:usize,pub projection:OriginalHostConfigurationProjection,pub mutation:fn(String)->M}
/// 🧭️ A raw configuration field or an addressed contribution roster uses the same original producer custody.
#[derive(Clone,Copy)]
pub enum OriginalHostConfigurationProjection{RawText,Contributions(OriginalContributionProjection)}
/// 🧩️ Domain-neutral contribution schema and addressed installation limits retain host order and original payloads.
#[derive(Clone,Copy)]
pub struct OriginalContributionProjection{pub maximum_depth:usize,pub maximum_items:usize,pub maximum_string_bytes:usize,pub topic:&'static str,pub app_id:&'static str,pub payload_fields:&'static[&'static str],pub deduplicate_modules:bool,pub excluded_module_ids:&'static[&'static str],pub maximum_modules:usize}
