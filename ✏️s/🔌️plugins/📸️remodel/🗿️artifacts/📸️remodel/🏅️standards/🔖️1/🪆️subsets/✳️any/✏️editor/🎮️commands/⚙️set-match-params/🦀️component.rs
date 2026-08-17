//! ⚙️ ⚙️ Remodel play app commands command — `set-match-params`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::update_match_params;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MatchParams, MatcherKind, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "match-params")]
pub struct SetMatchParams {
    pub matcher: String,
    pub ratio_test: f32,
    pub cross_check: bool,
    pub sequential_window: u32,
    pub max_pairs_per_frame: u32,
    pub loop_closure: bool,
}

pub fn handle(payload: &SetMatchParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_match_params(MatchParams {
        matcher: if payload.matcher == "kd-tree" { MatcherKind::KdTree } else { MatcherKind::BruteForce },
        ratio_test: payload.ratio_test,
        cross_check: payload.cross_check,
        sequential_window: payload.sequential_window,
        max_pairs_per_frame: payload.max_pairs_per_frame,
        loop_closure: payload.loop_closure,
    })]))
}
