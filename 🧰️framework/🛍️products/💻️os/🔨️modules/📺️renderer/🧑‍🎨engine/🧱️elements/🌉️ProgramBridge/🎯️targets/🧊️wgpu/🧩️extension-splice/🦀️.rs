//! 🧩️ Extension splice: React's `ExtensionView` mounts the host of each `Component::Extension` node in place; here the host is the extension program, and its own `renderDocument` records are joined to the parent's records BEFORE the one `UiDocumentLease::try_publish`, so the subtree travels the ordinary lease protocol and its ceilings. The splice itself ([`extension_hosts`], [`splice_extension_records`], [`splice_extension_documents`]) is pure over records and shared by the browser and the native backend. See [`super`].

use super::*;
use std::future::Future;
use ui_contract::{Component, UiNodeId, UiNodeRecord};

/// 🪜️ How deep extensions nest inside extensions; a deeper (or self-referential) extension stays a childless node.
pub const EXTENSION_SPLICE_DEPTH: usize = 4;

//#region 🔎️PureSplice
/// 🔎️ The childless `Extension` records at or after `from`: their index, extension id and props as JSON.
pub fn extension_hosts(nodes: &[UiNodeRecord], from: usize) -> Vec<(usize, String, String)> {
    nodes
        .iter()
        .enumerate()
        .skip(from)
        .filter_map(|(index, node)| match &node.component {
            Component::Extension(props) if node.children.is_empty() => Some((index, props.extension.as_str().to_string(), serde_json::to_string(&props.props).unwrap_or_else(|_| "null".to_string()))),
            _ => None,
        })
        .collect()
}

/// 🔌️ Shifts every node id a record carries (its own, its children and the node references of tree props) by `offset`, so a second program's document can join another's without an id collision.
pub fn shift_record_ids(record: &mut UiNodeRecord, offset: u64) {
    let shift = |id: UiNodeId| UiNodeId(id.0 + offset);
    record.id = shift(record.id);
    let children: Vec<UiNodeId> = record.children.iter().copied().collect();
    record.children = Default::default();
    for child in children {
        let _ = record.children.try_push(shift(child));
    }
    match &mut record.component {
        Component::TreeSection(props) => props.header_toolbar = props.header_toolbar.map(shift),
        Component::TreeItem(props) => {
            props.inline_toolbar = props.inline_toolbar.map(shift);
            props.detail = props.detail.map(shift);
        }
        _ => {}
    }
}

/// 🧩️ Joins `child` (an extension's records, rooted at `root`) to `nodes` as the one child of the extension record at `index`. Refused — leaving `nodes` untouched — when the child is empty, `index` is no childless extension, the result would exceed `UI_DOCUMENT_NODES`, or the child's root is not among its records.
pub fn splice_extension_records(nodes: &mut Vec<UiNodeRecord>, index: usize, mut child: Vec<UiNodeRecord>, root: u64) -> bool {
    let hosted = nodes.get(index).is_some_and(|node| matches!(node.component, Component::Extension(_)) && node.children.is_empty());
    if !hosted || child.is_empty() || nodes.len() + child.len() > ui_contract::UI_DOCUMENT_NODES || !child.iter().any(|record| record.id.0 == root) {
        return false;
    }
    let offset = nodes.iter().map(|node| node.id.0).max().unwrap_or(0) + 1;
    if nodes[index].children.try_push(UiNodeId(root + offset)).is_err() {
        return false;
    }
    child.iter_mut().for_each(|record| shift_record_ids(record, offset));
    nodes.extend(child);
    true
}

/// 🧩️ Splices every extension of `nodes` whose host `fetch` answers, then those of the spliced records in turn, to [`EXTENSION_SPLICE_DEPTH`] levels. `fetch(extension_id, props_json)` answers the extension's records and root id, or `None` for an extension that has no host or whose render failed.
pub async fn splice_extension_documents<F, Fut>(nodes: &mut Vec<UiNodeRecord>, mut fetch: F)
where
    F: FnMut(String, String) -> Fut,
    Fut: Future<Output = Option<(Vec<UiNodeRecord>, u64)>>,
{
    let mut from = 0;
    for _ in 0..EXTENSION_SPLICE_DEPTH {
        let hosts = extension_hosts(nodes, from);
        if hosts.is_empty() {
            return;
        }
        let appended = nodes.len();
        for (index, extension, props) in hosts {
            if let Some((child, root)) = fetch(extension, props).await {
                splice_extension_records(nodes, index, child, root);
            }
        }
        from = appended;
    }
}
//#endregion 🔎️PureSplice

//#region 📚️Registry
thread_local! {
    static EXTENSION_PROGRAMS: std::cell::RefCell<Vec<ExtensionProgram>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[derive(Clone)]
struct ExtensionProgram {
    id: String,
    entry: ProgramBridgeEntry,
    instance: Option<u32>,
    /// 🧩️ Native only: the plugin this extension extends. Present, the kernel activates the extension beside the live instance that first names it (no parent re-creation) and the extension then renders under ITS OWN instance.
    parent_plugin: Option<String>,
    /// 🔐️ The capability names its package manifest requests (honoured as the web registry honours `manifest.caps`: granted only as the intersection with the host's own grants).
    capabilities: Vec<String>,
    enabled: bool,
}

thread_local! {
    #[cfg(not(target_arch = "wasm32"))]
    static RETIRING: std::cell::RefCell<Vec<semio_renderer_wgpu_kernel_protocol::KernelCloseHandle>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 🔌️ Registers the installed extension program `id` as the host of its `Component::Extension` nodes. `instance` is the program instance its documents render under (`None`: the instance of the document that names it). Replaces an earlier registration.
pub fn register_extension_program(id: &str, entry: ProgramBridgeEntry, instance: Option<u32>) {
    register_extension_program_extending(id, entry, instance, None, Vec::new());
}

/// 🔌️ Like [`register_extension_program`] for a native extension that is activated LIVE: `parent_plugin` is the DECLARED host (the plugin it extends) and `capabilities` its manifest's requests: at the first render of an instance OF that plugin the kernel activates it beside that instance (an instance of any other plugin is refused, so the next render by a real host retries) with its requests scoped to the host's grants.
pub fn register_extension_program_extending(id: &str, entry: ProgramBridgeEntry, instance: Option<u32>, parent_plugin: Option<String>, capabilities: Vec<String>) {
    unregister_extension_program(id);
    EXTENSION_PROGRAMS.with_borrow_mut(|programs| programs.push(ExtensionProgram { id: id.to_string(), entry, instance, parent_plugin, capabilities, enabled: true }));
}

/// 🗑️ Drops the program registered for `id`; its nodes are childless again at the next render. A live-activated native extension's kernel instance is retired (the ordinary app close, polled to completion by later registry calls).
pub fn unregister_extension_program(id: &str) {
    let dropped = EXTENSION_PROGRAMS.with_borrow_mut(|programs| programs.iter().position(|program| program.id == id).map(|index| programs.remove(index)));
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(ExtensionProgram { entry, instance: Some(instance), parent_plugin: Some(_), .. }) = dropped {
            if let Ok(handle) = entry.begin_retire_extension_instance(instance) {
                RETIRING.with_borrow_mut(|retiring| retiring.push(handle));
            }
        }
        RETIRING.with_borrow_mut(|retiring| retiring.retain(|handle| matches!(handle.poll(), semio_renderer_wgpu_kernel_protocol::KernelCloseStatus::Pending | semio_renderer_wgpu_kernel_protocol::KernelCloseStatus::AdmissionBlocked)));
    }
    #[cfg(target_arch = "wasm32")]
    let _ = dropped;
}

/// ⏸️ Enables or disables the registered program `id` without dropping it (a disabled extension renders no subtree).
pub fn set_extension_program_enabled(id: &str, enabled: bool) {
    EXTENSION_PROGRAMS.with_borrow_mut(|programs| programs.iter_mut().filter(|program| program.id == id).for_each(|program| program.enabled = enabled));
}

/// 🔎️ What an invoke needs of the enabled program `id`: its entry, the host it extends, its live actor instance and its capability requests.
#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ExtensionInvokeTarget {
    pub entry: ProgramBridgeEntry,
    pub host: Option<String>,
    pub instance: Option<u32>,
    pub capabilities: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn extension_invoke_target(id: &str) -> Option<ExtensionInvokeTarget> {
    extension_program(id).map(|program| ExtensionInvokeTarget { entry: program.entry, host: program.parent_plugin, instance: program.instance, capabilities: program.capabilities })
}

/// 📌️ Keeps the live instance an invoke activated, so the next render and invoke find it (and its retirement stays owned).
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn remember_extension_instance(id: &str, instance: Option<u32>) {
    set_extension_program_instance(id, instance);
}

fn set_extension_program_instance(id: &str, instance: Option<u32>) {
    EXTENSION_PROGRAMS.with_borrow_mut(|programs| programs.iter_mut().filter(|program| program.id == id).for_each(|program| program.instance = instance));
}

fn extension_program(id: &str) -> Option<ExtensionProgram> {
    EXTENSION_PROGRAMS.with_borrow(|programs| programs.iter().find(|program| program.id == id && program.enabled).cloned())
}

fn any_extension_program() -> bool {
    EXTENSION_PROGRAMS.with_borrow(|programs| programs.iter().any(|program| program.enabled))
}

/// 🎛️ The records of the extension `extension`'s own render for `props_json` (as `extension_input_json`), or `None` without an enabled program or after a failed render.
async fn render_extension(extension: String, props_json: String, instance_id: u32, view_state: &ViewModel) -> Option<(Vec<UiNodeRecord>, u64, Vec<Effect>)> {
    let program = extension_program(&extension)?;
    let mut view = view_state.clone();
    view.extension_input_json = Some(props_json);
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(parent_plugin) = program.parent_plugin.as_deref() {
        if let Some(instance) = program.instance {
            if let Some(rendered) = program.entry.render_extension_records(instance, &extension, &view).await {
                return Some(rendered);
            }
        }
        let instance = program.entry.activate_extension_instance(instance_id, parent_plugin, &program.capabilities).await.ok()?;
        set_extension_program_instance(&extension, Some(instance));
        return program.entry.render_extension_records(instance, &extension, &view).await;
    }
    program.entry.render_extension_records(program.instance.unwrap_or(instance_id), &extension, &view).await
}

/// 🌐️ Splices the extension subtrees of a browser program's parsed records (before publication), merging their render effects into `effects`.
#[cfg(target_arch = "wasm32")]
pub(super) async fn splice_browser_extensions(nodes: &mut Vec<UiNodeRecord>, instance_id: u32, view_state: &ViewModel, effects: &mut Vec<Effect>) {
    if !any_extension_program() {
        return;
    }
    let collected = std::cell::RefCell::new(Vec::new());
    splice_extension_documents(nodes, |extension, props| {
        let collected = &collected;
        async move {
            let (records, root, mut more) = render_extension(extension, props, instance_id, view_state).await?;
            collected.borrow_mut().append(&mut more);
            Some((records, root))
        }
    })
    .await;
    effects.append(&mut collected.into_inner());
}

/// 🎠️ Splices the extension subtrees of a native program's published lease. Generation-safe by construction: the successor is published through `UiDocumentLease::try_publish` BEFORE the predecessor is released, and it carries the predecessor's identity (generation, revision, root, layout epoch) because the predecessor never left this function, so the engine's monotonic ingress check (`document.generation() >= header.generation` is stale) sees exactly one document of that generation and the kernel's next render stays strictly newer. If the successor cannot be published (arena permit, ceiling), the predecessor is returned untouched: a refused splice costs the extension's subtree, never the render. A lease that names no hosted extension is returned as is.
#[cfg(not(target_arch = "wasm32"))]
pub(super) async fn splice_native_extensions(lease: UiDocumentLease, instance_id: u32, view_state: &ViewModel, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, ProgramFault> {
    if !any_extension_program() {
        return Ok(lease);
    }
    let Some((header, mut nodes)) = lease_records(&lease) else { return Ok(lease) };
    if !extension_hosts(&nodes, 0).iter().any(|(_, extension, _)| extension_program(extension).is_some()) {
        return Ok(lease);
    }
    let collected = std::sync::Mutex::new(Vec::new());
    splice_extension_documents(&mut nodes, |extension, props| {
        let collected = &collected;
        async move {
            let (records, root, mut more) = render_extension(extension, props, instance_id, view_state).await?;
            if let Ok(mut sink) = collected.lock() { sink.append(&mut more) }
            Some((records, root))
        }
    })
    .await;
    if let Some(sink) = refresh_effects {
        sink.append(&mut collected.into_inner().unwrap_or_default());
    }
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation: header.generation, revision: header.revision, root: Some(header.root), layout_epoch: header.layout_epoch };
    match UiDocumentLease::try_publish(header.surface, identity, nodes) {
        Ok(successor) => {
            drop(lease);
            Ok(successor)
        }
        Err(_) => Ok(lease),
    }
}

/// 📖️ A lease's header and records, read back through the arena's exact read; `None` while it stays contended or a record cannot be credited.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn lease_records(lease: &UiDocumentLease) -> Option<(ui_contract::UiDocumentLeaseHeader, Vec<UiNodeRecord>)> {
    let header = lease.header().ok()?;
    let read = (0..8).map(|_| lease.try_read()).find(|read| !matches!(read, Err(ui_contract::UiDocumentLeaseError::Contended))).and_then(Result::ok)?;
    let nodes = (0..read.len()).map(|ordinal| read.node_at(ordinal)?.credited_clone()).collect::<Option<Vec<_>>>()?;
    Some((header, nodes))
}
//#endregion 📚️Registry

#[cfg(test)]
#[path = "../../../🧪️tests/🧩️extension-splice/🦀️.rs"]
mod extension_splice_tests;
