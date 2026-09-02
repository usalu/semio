//! ⚙️ ⚙️ Remodeling play app commands command — `set-match-params`.

use crate::artifacts::remodeling::mutations::update_match_params;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{MatchParams, MatcherKind, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "match-params")]
pub struct SetMatchParams {
    pub matcher: String,
    pub ratio_test: f32,
    pub cross_check: bool,
    pub sequential_window: u32,
    pub max_pairs_per_frame: u32,
    pub loop_closure: bool,
}

pub async fn handle(payload: &SetMatchParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_match_params(MatchParams {
        matcher: if payload.matcher == "kd-tree" { MatcherKind::KdTree } else { MatcherKind::BruteForce },
        ratio_test: payload.ratio_test,
        cross_check: payload.cross_check,
        sequential_window: payload.sequential_window,
        max_pairs_per_frame: payload.max_pairs_per_frame,
        loop_closure: payload.loop_closure,
    })]))
}
