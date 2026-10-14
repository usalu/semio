//! ⚙️ ⚙️ Remodeling play app commands command — `set-match-params`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_match_params::update_match_params;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MatchParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MatcherKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "match-params")]
pub struct SetMatchParams {
    pub matcher: String,
    pub ratio_test: f32,
    pub cross_check: bool,
    pub sequential_window: u32,
    pub max_pairs_per_frame: u32,
    pub loop_closure: bool,
}

pub fn handle(payload: &SetMatchParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_match_params(MatchParams {
        matcher: if payload.matcher == "kd-tree" { MatcherKind::KdTree } else { MatcherKind::BruteForce },
        ratio_test: payload.ratio_test,
        cross_check: payload.cross_check,
        sequential_window: payload.sequential_window,
        max_pairs_per_frame: payload.max_pairs_per_frame,
        loop_closure: payload.loop_closure,
    })]))
}
