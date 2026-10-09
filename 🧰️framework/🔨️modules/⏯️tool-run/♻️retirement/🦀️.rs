//! ⏯️ Native scalar leaves accompany the original writer's typed ownership tree.
use super::*;
semio_framework_value::artifact_retire_leaf!(ToolRunIdentity);
semio_framework_value::artifact_retire_leaf!(ToolRunState);
semio_framework_value::artifact_retire_leaf!(ToolRunStepKind);
semio_framework_value::artifact_retire_leaf!(ToolRunStepArg);
semio_framework_value::artifact_retire_leaf!(ToolRunCounter);
semio_framework_value::artifact_retire_leaf!(ToolRunTraceOp);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
