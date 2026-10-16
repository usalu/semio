//! 🚧️ Token-sampling constraints and stop conditions: stop matching, regex automata, trie, JSON mode, EBNF and JSON-schema constraints.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_framework_math_sampling_vocabulary::{FinishReason, SamplingError, SamplingLimits, TokenBitset, TokenId};
use semio_framework_math_sampling_config::ConstraintSpec;
use semio_framework_math_sampling_workspace::{ConstraintMark, StepView, StopMark};
#[cfg(test)] use semio_framework_math_sampling_vocabulary::{JsonValue, TokenTextAdapters, parse_json};
#[cfg(test)] use semio_framework_math_sampling_config::StopTextMode;

#[path = "🚦️stop-conditions/🦀️.rs"]
mod stop_conditions;
pub use stop_conditions::*;
#[path = "🤖️regex-automata/🦀️.rs"]
mod regex_automata;
pub use regex_automata::*;
#[path = "🧱️mask-constraints/🦀️.rs"]
mod mask_constraints;
pub use mask_constraints::*;

/// 🔌️ A structural/lexical constraint on the next token (regex, grammar, JSON mode, ...).
#[dyn_enum]
pub trait Constraint {
    async fn name(&self) -> &'static str;
    /// 🧱️ ANDs the set of currently-valid tokens into `mask` (starts all-ones for the first
    /// constraint in a composition).
    async fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError>;
    async fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError>;
    async fn is_satisfied(&self) -> bool;
    async fn is_finished(&self) -> bool;
    async fn is_dead(&self) -> bool;
    async fn save(&mut self) -> ConstraintMark;
    async fn rollback_to(&mut self, mark: ConstraintMark);
    async fn reset(&mut self);
    async fn fork(&self) -> Constraints;
}

/// 🛑️ Result of feeding one token's bytes to a [`StopCondition`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopPoll {
    Continue,
    Hold { ambiguous_bytes: usize },
    Finished { reason: FinishReason, matched_bytes: usize },
}

#[dyn_enum]
pub trait StopCondition {
    async fn name(&self) -> &'static str;
    async fn on_token(&mut self, view: &StepView<'_>, token: TokenId) -> StopPoll;
    async fn save(&mut self) -> StopMark;
    async fn rollback_to(&mut self, mark: StopMark);
    async fn reset(&mut self);
    async fn fork(&self) -> StopConditions;
}

// #region 🔖️Stops
dyn_enum_close! {
    pub enum StopConditions: StopCondition {
        Token(TokenStopCondition),
        Text(TextStopCondition),
    }
}
// #endregion 🔖️Stops

// #region 🔖️Constraints

dyn_enum_close! {
    pub enum Constraints: Constraint {
        Regex(RegexConstraint),
        Trie(TrieConstraint),
        MustInclude(MustIncludeConstraint),
        JsonMode(JsonModeConstraint),
        Ebnf(EbnfConstraint),
        JsonSchema(JsonSchemaConstraint),
    }
}

/// 🧱️ Builds a [`Constraint`] from a [`ConstraintSpec`]; exhaustive over every variant.
pub async fn build_constraint(spec: &ConstraintSpec, limits: &SamplingLimits) -> Result<Constraints, SamplingError> {
    match spec {
        ConstraintSpec::Regex(pattern) => Ok((RegexConstraint::new(pattern, limits).await?).into()),
        ConstraintSpec::Trie(phrases) => Ok((TrieConstraint::new(phrases).await).into()),
        ConstraintSpec::MustInclude(alternatives) => Ok((MustIncludeConstraint::new(alternatives.clone()).await).into()),
        ConstraintSpec::JsonMode => Ok((JsonModeConstraint::new().await).into()),
        ConstraintSpec::Ebnf(text) => Ok((EbnfConstraint::new(text, limits).await?).into()),
        ConstraintSpec::JsonSchema(schema) => Ok((JsonSchemaConstraint::new(schema.clone()).await).into()),
    }
}
// #endregion 🔖️Constraints

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
