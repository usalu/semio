//! 📊️ Schema for one Jack results window's ephemeral query output.

use crate::ast::QueryResult;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "trinity.jackresultswindowtransient")]
#[dsl(layout = "lines")]
pub struct JackResultsWindowTransient {
    pub query_execution_id: Option<String>,
    pub result: Option<QueryResult>,
    pub query_error: Option<String>,
}

/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of one Jack results window's ephemeral query output: names only the slots a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct JackResultsWindowTransientDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub query_execution_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub result: Option<Option<QueryResult>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub query_error: Option<Option<String>>,
}

impl protocol::MutationDiff<JackResultsWindowTransient> for JackResultsWindowTransientDiff {
    fn apply(&self, base: &JackResultsWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<JackResultsWindowTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.query_execution_id {
            next.query_execution_id.clone_from(value);
        }
        if let Some(value) = &self.result {
            next.result.clone_from(value);
        }
        if let Some(value) = &self.query_error {
            next.query_error.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.query_execution_id.is_some() {
            self.query_execution_id = other.query_execution_id;
        }
        if other.result.is_some() {
            self.result = other.result;
        }
        if other.query_error.is_some() {
            self.query_error = other.query_error;
        }
    }
}

impl protocol::DiffAlgebra<JackResultsWindowTransient> for JackResultsWindowTransientDiff {
    fn inverse(&self, base: &JackResultsWindowTransient) -> Self {
        Self {
            query_execution_id: self.query_execution_id.as_ref().map(|_| base.query_execution_id.clone()),
            result: self.result.as_ref().map(|_| base.result.clone()),
            query_error: self.query_error.as_ref().map(|_| base.query_error.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.query_execution_id.is_none() && self.result.is_none() && self.query_error.is_none()
    }
}
