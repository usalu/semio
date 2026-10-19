//! 🧬️ Empty app-presence schema; canonical window views and interaction own live state.
use ::semio_framework_schema::ArtifactSchema;
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
#[artifact_schema(id = "s.puzzle.puzzle2d.presence")]
pub struct Puzzle2dPresence {}
