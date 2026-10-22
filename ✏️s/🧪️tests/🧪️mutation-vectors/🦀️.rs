//! 🧫️ The committed mutation vectors of an artifact subset as one data-driven law table, shared by every artifact family.
//!
//! Source of truth is every bundle under a subset's `🧫️fixtures/🧬️mutations/<leaf>/<case>/`: `📸️snapshot/⬅️before`,
//! `📸️snapshot/➡️after`, `🦠️mutation`, `🎯️outcome` and `🔺️diff` (or its empty `🚫️.absent` marker); a `🧾️wire-witness`
//! case carries the mutation only. The bundles are read at test runtime, so no fixture byte enters a compilation unit, and
//! the subset's independent Python engine replays the same bytes. Every vector lands on its committed after-snapshot under
//! exactly its committed messages: an `applied` one moves the document by exactly the committed diff, its own inverse
//! restores the before-snapshot and the inverse diffs sum to the negative diff; a `no-op` one keeps the document under the
//! committed diff; a `rejected` one is refused at error or fatal level and keeps it. Every mutation crosses the JSON wire
//! and the binary op codec unchanged, every kind is covered by an applied vector or a wire witness, and every leaf
//! descriptor declares exactly the outcome classes production dispatch reaches from its vectors.

use semio_framework_os_kernel::{apply_diff_cold, ColdMutation, Mutation, OpBinary, SemanticMutation};
use semio_framework_value::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

//#region 📂️Bundles
fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("committed vector file {} is readable: {error}", path.display()))
}

fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).unwrap_or_else(|error| panic!("committed JSON: {error}\n{text}"))
}

fn directories(path: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(path).unwrap_or_else(|error| panic!("fixture directory {} is readable: {error}", path.display())).map(|entry| entry.expect("fixture directory entry").path()).filter(|path| path.is_dir()).collect();
    found.sort();
    found
}

fn decode<T: FromValue>(path: &Path) -> T {
    semio_framework_pack_json::from_json_str(&read(path), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{} decodes: {error}", path.display()))
}
//#endregion 📂️Bundles

//#region ⚖️Laws
/// 🏷️ Production dispatch's `<Level>:<code>` spelling of a committed outcome's messages.
fn committed_messages(outcome: &serde_json::Value) -> Vec<String> {
    let spelled = |message: &serde_json::Value| {
        let level = message["level"].as_str().unwrap_or_default();
        format!("{}{}:{}", level.get(..1).unwrap_or_default().to_uppercase(), level.get(1..).unwrap_or_default(), message["code"].as_str().unwrap_or_default())
    };
    outcome["messages"].as_array().map(|messages| messages.iter().map(spelled).collect()).unwrap_or_default()
}

fn refused(messages: &[String]) -> bool {
    messages.iter().any(|message| message.starts_with("Error:") || message.starts_with("Fatal:"))
}

/// 🌉️ One mutation through the central applier: the next snapshot and the raised `<Level>:<code>` messages.
fn apply<S, M>(base: &S, mutation: &M) -> Result<(S, Vec<String>), String>
where
    M: Mutation<S>,
{
    let raised = mutation.diff(base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// 🎯️ The mutation addressing a target no document holds, or `None` when its payload addresses nothing.
fn stray<S, M: Mutation<S>>(mutation: &M) -> Option<M> {
    let mut payload = serde_json::Value::from(mutation.payload_value());
    let mut addressed = false;
    for (name, value) in payload.as_object_mut()? {
        if (name == "index" || name.ends_with("Index")) && value.is_u64() {
            *value = serde_json::Value::from(u64::from(u32::MAX));
            addressed = true;
        } else if (name == "id" || name.ends_with("Id")) && value.is_string() {
            *value = serde_json::Value::from("∅");
            addressed = true;
        }
    }
    addressed.then(|| mutation.with_payload_value(semio_framework_value::DslValue::from(&payload)).ok()).flatten()
}

/// 🧾️ The committed mutation JSON is the canonical wire of the decoded operation.
fn wire_breaches<M: FromValue + ToValue>(bundle: &Path) -> Vec<String> {
    let text = read(&bundle.join("🦠️mutation/🔣️.json"));
    let committed = json(&text);
    match M::from_value(committed.clone().into()) {
        Ok(operation) if serde_json::Value::from(operation.to_value()) == committed => Vec::new(),
        Ok(_) => vec![format!("{}: the committed mutation is not the canonical Rust wire", bundle.display())],
        Err(error) => vec![format!("{}: the committed mutation does not decode: {error}", bundle.display())],
    }
}

/// 💾️ The committed mutation crosses the binary op codec unchanged.
fn binary_breaches<M: OpBinary + PartialEq + Debug>(bundle: &Path, mutation: &M) -> Vec<String> {
    let bytes = match mutation.encode_op() {
        Ok(bytes) => bytes,
        Err(error) => return vec![format!("{}: the binary op encoding failed: {error}", bundle.display())],
    };
    match M::decode_op(&bytes) {
        Ok(decoded) if &decoded == mutation => Vec::new(),
        Ok(decoded) => vec![format!("{}: the binary op decodes to {decoded:?}", bundle.display())],
        Err(error) => vec![format!("{}: the binary op does not decode: {error}", bundle.display())],
    }
}

/// ⚖️ One bundle's breaches and the outcome classes it shows production dispatch reaching.
fn vector_breaches<S, M>(bundle: &Path, mutation: &M, reached: &mut BTreeSet<String>) -> Vec<String>
where
    S: Clone + PartialEq + Debug + FromValue,
    M: Mutation<S>,
    M::Diff: ToValue,
{
    let name = bundle.display();
    let step = |document: &S, operation: &M| apply(document, operation).unwrap_or_else(|error| panic!("{name}: {error}"));
    let before: S = decode(&bundle.join("📸️snapshot/⬅️before/🔣️.json"));
    let after: S = decode(&bundle.join("📸️snapshot/➡️after/🔣️.json"));
    let outcome = json(&read(&bundle.join("🎯️outcome/🔣️.json")));
    let status = outcome["status"].as_str().unwrap_or_default().to_string();
    let (applied, messages) = step(&before, mutation);
    let mut breaches = Vec::new();
    if applied != after {
        breaches.push(format!("{name}: the applied document is not the committed after-snapshot"));
    }
    if messages != committed_messages(&outcome) {
        breaches.push(format!("{name}: production dispatch raises {messages:?}, the committed outcome {:?}", committed_messages(&outcome)));
    }
    if (status == "applied") == (applied == before) || refused(&messages) != (status == "rejected") {
        breaches.push(format!("{name}: a {status:?} vector must move the document exactly when applied and be refused exactly when rejected"));
    }
    let diff = bundle.join("🔺️diff");
    if diff.join("🔣️.json").is_file() {
        let raised = mutation.diff(&before);
        if json(&semio_framework_pack_json::to_json_string(raised.diff())) != json(&read(&diff.join("🔣️.json"))) {
            breaches.push(format!("{name}: the produced diff is not the committed diff"));
        }
    } else if status != "rejected" {
        breaches.push(format!("{name}: a {status:?} vector commits no diff"));
    }
    if status == "applied" {
        let steps = mutation.inverse(&before).unwrap_or_else(|error| panic!("{name}: valid retained mutation inverse fixture: {error}"));
        let restored = steps.iter().rev().fold(applied, |document, operation| step(&document, operation).0);
        if steps.is_empty() || restored != before {
            breaches.push(format!("{name}: the mutation's own inverse ({} step(s)) does not restore the before-snapshot", steps.len()));
        }
    }
    let again = step(&after, mutation).1;
    if again.iter().any(|message| message == "Warning:mutation.no-op") {
        reached.insert("no-op".to_string());
    }
    if refused(&again) || stray::<S, M>(mutation).is_some_and(|target| refused(&step(&before, &target).1)) {
        reached.insert("rejected".to_string());
    }
    reached.insert(status);
    breaches
}
//#endregion ⚖️Laws

//#region 🧫️Table
/// 🧫️ Replays every committed bundle below `root` (a subset's `🧫️fixtures/🧬️mutations`) and asserts the law table;
/// `prepare` installs whatever process state the artifact's documents decode against (catalogs, registries).
pub async fn assert_committed_vectors<S, M>(root: &Path, prepare: fn())
where
    S: Clone + PartialEq + Debug + FromValue,
    M: Mutation<S> + SemanticMutation<S> + OpBinary + PartialEq + Debug + FromValue + ToValue,
    M::Diff: ToValue,
{
    prepare();
    assert!(root.is_dir(), "the fixture mirror {} must exist", root.display());
    let (mut covered, mut breaches, mut classes) = (BTreeSet::new(), Vec::new(), BTreeMap::new());
    let mut applied_vectors = Vec::new();
    for bundle in directories(root).iter().flat_map(|leaf| directories(leaf)) {
        breaches.extend(wire_breaches::<M>(&bundle));
        let mutation: M = decode(&bundle.join("🦠️mutation/🔣️.json"));
        let descriptor = mutation.descriptor();
        breaches.extend(binary_breaches(&bundle, &mutation));
        if bundle.ends_with("🧾️wire-witness") {
            covered.insert(descriptor.semantic_kind);
            continue;
        }
        let declared: BTreeSet<String> = descriptor.outcome_classes.iter().map(|class| class.as_str().to_string()).collect();
        let (canonical, reached, _) = classes.entry(descriptor.semantic_kind).or_insert_with(|| (false, BTreeSet::new(), declared));
        let found = vector_breaches::<S, M>(&bundle, &mutation, reached);
        let applied = json(&read(&bundle.join("🎯️outcome/🔣️.json")))["status"] == "applied";
        if found.is_empty() && applied {
            covered.insert(descriptor.semantic_kind);
            applied_vectors.push(bundle.clone());
        }
        *canonical |= applied;
        breaches.extend(found);
    }
    for (kind, (canonical, reached, declared)) in &classes {
        if (*canonical && reached != declared) || !reached.is_subset(declared) {
            breaches.push(format!("{kind}: the descriptor declares {declared:?}, production dispatch reaches {reached:?} from the committed vectors"));
        }
    }
    assert!(breaches.is_empty(), "{}", breaches.join("\n"));
    let missing: Vec<&str> = <M as SemanticMutation<S>>::kinds().iter().map(|descriptor| descriptor.kind).filter(|kind| !covered.contains(kind)).collect();
    assert!(missing.is_empty(), "kinds without a committed applied vector or wire witness: {missing:?}");
    for bundle in applied_vectors {
        let mutation: M = decode(&bundle.join("🦠️mutation/🔣️.json"));
        let before: S = decode(&bundle.join("📸️snapshot/⬅️before/🔣️.json"));
        semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
    }
}
//#endregion 🧫️Table
