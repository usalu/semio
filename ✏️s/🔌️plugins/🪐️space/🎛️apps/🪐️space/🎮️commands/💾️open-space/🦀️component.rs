//! 💾️ 💾️ S Studio app command — `open-space`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{create_backbone_document, WorkflowMutation, WorkflowSnapshot, S_SPACE_SCHEMA};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, HostEffect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "open-space")]
pub struct OpenSpace {
    pub space_id: String,
}

pub fn handle(payload: &OpenSpace, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let space_id = &payload.space_id;
    // 🚧️ `parse_demo_space_document()` yields a `workflow::WorkflowSnapshot`, not a
    // `space::SpaceSnapshot`-backed catalog entry — the "demo" id fallback below synthesizes a
    // minimal ephemeral space manifest with the same id/name instead, so `"demo"` still resolves to
    // *something* openable.
    let document = crate::apps::home::resolve_studio_document(space_id).or_else(|| {
        if space_id == "demo" {
            let name = {
                let demo = crate::parse_demo_space_document();
                if demo.name.trim().is_empty() {
                    "Demo Studio".into()
                } else {
                    demo.name
                }
            };
            let projection = semio_framework_os::empty_space_snapshot(&name, semio_framework_os::SpaceKind::Atelier, semio_framework_os::SpaceVisibility::Private);
            Some(create_backbone_document(S_SPACE_SCHEMA, "demo", &name, projection))
        } else {
            None
        }
    });
    let Some(document) = document else {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.not-found"), format!("studio `{space_id}` not found")));
    };
    let mut config_mutations = vec![
        SpaceConfigMutation::SetSpaceId { space_id: Some(space_id.clone()) },
        SpaceConfigMutation::SetFocusedNode { node_id: None },
        SpaceConfigMutation::SetClipboard { node_ids: Vec::new() },
    ];
    // 🕸️ `document` is a `space::SpaceSnapshot`-backed manifest — it carries no workflow graph of
    // its own anymore; the graph lives on a separate `s.workflow` artifact document within one of
    // the space's collections. Resolve, in order: (1) a real workflow artifact already registered
    // in one of `document`'s collections, (2) the bundled demo fixture's real content for the demo
    // space, (3) a freshly-minted, valid, empty `WorkflowSnapshot` for any other space that has none
    // yet — never the space manifest's own bytes.
    let is_demo_space = space_id == "demo" || document.name == crate::DEMO_STUDIO_NAME;
    let workflow_snapshot =
        crate::apps::home::resolve_workflow_artifact_document(space_id, &document).or_else(|| is_demo_space.then(crate::parse_demo_space_document)).unwrap_or_else(|| crate::apps::home::empty_workflow_artifact_document(space_id, &document.name));
    let active_node_id = workflow_snapshot.vcs.initial_snapshot.graph.nodes.first().map(|node| node.id.clone());
    config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: active_node_id });
    match crate::apps::home::workflow_artifact_envelope_pack(&workflow_snapshot) {
        Some(files) => {
            eprintln!("[DEBUG] openSpace id={} workflow_id={} nodes={} collections={}", space_id, workflow_snapshot.id, workflow_snapshot.vcs.initial_snapshot.graph.nodes.len(), document.vcs.initial_snapshot.collections.len());
            Ok(Emit { config_mutations, effects: vec![HostEffect::LoadDocument { pack: files.pack, spr: files.spr }], ..Default::default() })
        }
        None => {
            eprintln!("[DEBUG] openSpace workflow pack export failed id={space_id}");
            Ok(Emit::config(config_mutations))
        }
    }
}
