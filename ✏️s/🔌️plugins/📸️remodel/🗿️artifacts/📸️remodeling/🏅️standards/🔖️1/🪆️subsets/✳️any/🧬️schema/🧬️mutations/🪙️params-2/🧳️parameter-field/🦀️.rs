//! 🧳️ Fixed parameter field projection, sparse patch construction and exact inverse wrapping.

/// 🚦️ Finite scalar decision before a changed-value owner may construct its patch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedParameterDisposition { Changed, Unchanged, NumericRefusal, TargetedRefusal, UntargetedRefusal }

/// 🧮️ A fixed-width parameter record declares its bounded invariant and equality decision.
pub trait RetainedParameterScalar: Sized {
    const WORK_DEMAND: u64;
    const NOOP_MESSAGE: &'static str;
    const REFUSAL_MESSAGE: Option<&'static str> = None;
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (RetainedParameterDisposition, u64);
    fn retained_parameter_numeric_source(_: semio_framework_value::retained_clone::RetainedCloneRef<'_, Self>) -> Option<crate::RetainedParameterTextSource<'_>> { None }
}

/// 🏗️ Declares only the funded changed-value field path; its caller must first perform the semantic decision.
#[macro_export]
macro_rules! retained_remodeling_parameter_field {
    ($leaf:ty, $params:ty, $field:ident, $index:expr) => {
        impl<M> protocol::RetainedSetField<semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff, M> for $leaf
        where
            M: protocol::Mutation<semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, Diff = semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff> + protocol::RetainedLeafWrapper<$leaf> + Send + 'static,
        {
            type Authored = $params;
            type Displaced = $params;
            fn source_projection_work() -> Result<u64, semio_framework_value::ValueError> { Ok(1) }
            fn original_projection_work() -> Result<u64, semio_framework_value::ValueError> { Ok(1) }
            fn authored_birth_work() -> Result<u64, semio_framework_value::ValueError> { protocol::field_birth_work::<$params>() }
            fn displaced_birth_work() -> Result<u64, semio_framework_value::ValueError> { protocol::field_birth_work::<$params>() }
            fn source_accessor(payload: &Self) -> semio_framework_value::RetainedWorkOutcome<&$params> { semio_framework_value::RetainedWorkOutcome { result: Ok(&payload.params), work: 1 } }
            fn original_accessor(original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> semio_framework_value::RetainedWorkOutcome<&$params> { semio_framework_value::RetainedWorkOutcome { result: Ok(&original.params.$field), work: 1 } }
            fn source_discriminator() -> usize { 0 }
            fn original_discriminator() -> usize { $index }
            fn patch_work_demand() -> Result<u64, semio_framework_value::ValueError> { Ok(19) }
            fn patch(value: $params) -> (semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff, u64) {
                use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::{RemodelingDiff, RemodelingParamsDiff};
                (RemodelingDiff { params: Some(RemodelingParamsDiff { $field: Some(value), ..Default::default() }), ..Default::default() }, 19)
            }
            fn inverse_work_demand() -> Result<u64, semio_framework_value::ValueError> { <M as protocol::RetainedLeafWrapper<$leaf>>::wrap_work_demand().checked_add(1).ok_or_else(|| semio_framework_value::ValueError::invariant("parameter inverse wrapper work overflow")) }
            fn inverse(value: $params) -> (M, u64) { let (row, work) = <M as protocol::RetainedLeafWrapper<$leaf>>::wrap(Self { params: value }); (row, work + 1) }
        }
        impl<M> protocol::RetainedMutationLeaf<semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, M> for $leaf
        where
            M: protocol::Mutation<semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, Diff = semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff> + protocol::RetainedLeafWrapper<$leaf> + Send + 'static,
        {
            type Preparation = $crate::RetainedParameterPreparation<Self, M>;
        }
    };
}

/// 🧪️ Checks each leaf's fixed-scalar decision against its shared neutral vectors.
#[macro_export]
macro_rules! retained_parameter_scalar_vectors {
    ($leaf:ty, $facet:literal, $fixture:expr, $make:expr) => {
        #[cfg(test)]
        mod retained_scalar_vectors {
            use super::*;
            #[test]
            fn neutral_parameter_decisions_and_work() {
                let cases: serde_json::Value = serde_json::from_str($fixture).unwrap();
                let original = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene();
                let mut count = 0;
                for case in cases.as_array().unwrap().iter().filter(|case| case["facet"] == $facet) {
                    let payload: $leaf = ($make)(&original, case["change"].as_object().unwrap());
                    let (actual, work) = <$leaf as $crate::RetainedParameterScalar>::retained_parameter_disposition(&payload, &original);
                    let expected = case["expected"].as_str().unwrap();
                    let matches = match expected { "unchanged" => actual == $crate::RetainedParameterDisposition::Unchanged, "admitted" => actual == $crate::RetainedParameterDisposition::Changed, "refused" => matches!(actual, $crate::RetainedParameterDisposition::NumericRefusal | $crate::RetainedParameterDisposition::TargetedRefusal | $crate::RetainedParameterDisposition::UntargetedRefusal), _ => false };
                    assert!(matches, "{case}: {actual:?}");
                    assert!(work > 0 && work <= <$leaf as $crate::RetainedParameterScalar>::WORK_DEMAND, "{case}: {work}");
                    count += 1;
                }
                assert_eq!(count, 3);
            }
        }
    };
}
