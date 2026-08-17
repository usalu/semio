//! 🏙️ 🏙️ S Home launcher app command — `import-space`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use crate::artifacts::home::mutations::change_catalog_generation;
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};

use semio_framework_os::import_os_space_from_dsl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "import-space")]
pub struct ImportSpace {
    pub dsl: Option<String>,
}

pub fn handle(payload: &ImportSpace, doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    match &payload.dsl {
        Some(dsl) => {
            if import_os_space_from_dsl(dsl, crate::catalog_port()).is_ok() {
                Ok(Emit::mutations(vec![change_catalog_generation(generation + 1)]))
            } else {
                Ok(Emit::default())
            }
        }
        None => Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".os".into(), read_as: None, import_action: "importSpace".into(), multiple: false })),
    }
}
