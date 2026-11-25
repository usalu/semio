//! 🛂️ The committed semantic descriptor pair of `stdio-pdf` is exactly the pair its assembly describes through the runtime path of the component's `describe` export, so a window kind, fault notice or action added to an app cannot ship with a stale descriptor: the browser shell lays out windows and localizes faults from the descriptor, never from the guest.
//! [descriptor emission](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs)

use semio_framework::{CanonicalDescriptorValue, PackageDescriptor};
use semio_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};

/// 🔁️ The command that regenerates the committed pair from the built component.
const REGENERATE: &str = "repo run workspace:components -- describe stdio-pdf --profile prototype";

/// 🔢️ Differing paths a refusal names.
const DIFF_LIMIT: usize = 40;

/// 🛂️ Describes the assembled bundle exactly as the component's `describe` export does.
fn described() -> PackageDescriptor {
    let runtime = PluginRuntime::<semio_hub_stdio_pdf::StdioPdfApps>::new(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).expect("explicit component mounted owner policy");
    install_plugin_bundle_result(&runtime, semio_hub_stdio_pdf::plugin());
    let bytes = semio_framework_async::poll::resolve_ready(semio_plugin::describe::describe_plugin(&runtime));
    semio_framework_value::FromValue::from_value(semio_os_kernel::pack_rt::decode_wire_value(&bytes).expect("descriptor bytes")).expect("strict descriptor")
}

/// 🧭️ The JSON paths where `committed` and `described` differ, at most [`DIFF_LIMIT`].
fn differences(committed: &serde_json::Value, described: &serde_json::Value, path: String, rows: &mut Vec<String>) {
    if rows.len() >= DIFF_LIMIT || committed == described {
        return;
    }
    match (committed, described) {
        (serde_json::Value::Object(left), serde_json::Value::Object(right)) => {
            for key in left.keys().chain(right.keys().filter(|key| !left.contains_key(*key))) {
                differences(left.get(key).unwrap_or(&serde_json::Value::Null), right.get(key).unwrap_or(&serde_json::Value::Null), format!("{path}.{key}"), rows);
            }
        }
        (serde_json::Value::Array(left), serde_json::Value::Array(right)) => {
            for index in 0..left.len().max(right.len()) {
                differences(left.get(index).unwrap_or(&serde_json::Value::Null), right.get(index).unwrap_or(&serde_json::Value::Null), format!("{path}[{index}]"), rows);
            }
        }
        _ => rows.push(format!("{path}: committed {} -> described {}", truncated(committed), truncated(described))),
    }
}

/// ✂️ One side of a differing path, cut to a readable length.
fn truncated(value: &serde_json::Value) -> String {
    value.to_string().chars().take(160).collect()
}

/// 📏️ The catalog's fixed bound on each descriptor form (`CATALOG_DESCRIPTOR_MAX_BYTES`); a larger pair is refused before it is ever committed.
const CATALOG_DESCRIPTOR_MAX_BYTES: usize = 4 * 1024 * 1024;

/// 📤️ Writes the described pair beside the test target, where the settle step of the descriptor emitter (`settleComponentDescriptorV1`) can take it, and answers its directory.
fn emitted(json: &str, pack: &[u8]) -> std::path::PathBuf {
    let directory = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("stdio-pdf-described");
    std::fs::create_dir_all(&directory).expect("described descriptor directory");
    std::fs::write(directory.join("🔣️.json"), json).expect("described descriptor JSON");
    std::fs::write(directory.join("🛂️.descriptor.semio"), pack).expect("described descriptor pack");
    directory
}

#[test]
fn the_committed_descriptor_pair_is_the_pair_the_assembly_describes() {
    let value = semio_framework_value::ToValue::to_value(&described());
    let json = format!("{}\n", semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value)));
    let pack = semio_os_kernel::pack_rt::encode_wire_value(CanonicalDescriptorValue::new(value).value());
    let directory = emitted(&json, &pack);
    assert!(json.len() <= CATALOG_DESCRIPTOR_MAX_BYTES && pack.len() <= CATALOG_DESCRIPTOR_MAX_BYTES, "the described stdio-pdf descriptor exceeds the catalog bound ({} JSON bytes, {} pack bytes, limit {CATALOG_DESCRIPTOR_MAX_BYTES}); written to {}", json.len(), pack.len(), directory.display());
    let committed = include_str!("../../🧩️component/🔣️.json");
    if json != committed {
        let mut rows = Vec::new();
        differences(&serde_json::from_str(committed).expect("committed descriptor JSON"), &serde_json::from_str(&json).expect("described descriptor JSON"), "$".into(), &mut rows);
        panic!("the committed stdio-pdf descriptor is stale; regenerate it with `{REGENERATE}` (the described pair is at {}):\n  {}", directory.display(), if rows.is_empty() { "$: byte encoding differs".into() } else { rows.join("\n  ") });
    }
    assert!(pack.as_slice() == include_bytes!("../../🧩️component/🛂️.descriptor.semio").as_slice(), "the committed stdio-pdf descriptor pack is stale; regenerate it with `{REGENERATE}` (the described pair is at {})", directory.display());
}
