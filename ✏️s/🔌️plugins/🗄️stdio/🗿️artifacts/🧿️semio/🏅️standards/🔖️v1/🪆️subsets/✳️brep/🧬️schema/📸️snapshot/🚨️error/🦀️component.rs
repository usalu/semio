//! 🚨️ Flat, hand-rolled error enums for every kernel subsystem (no `thiserror` — matches the
//! `math::wfc` convention). Each variant carries just enough context to explain *why* an
//! operation refused to produce a result; the kernel's hard invariant is "never wrong, fail loud"
//! rather than silently returning a plausible-looking but invalid shape.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🚨️error` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4.

// #region 🔖️Errors

/// 🚨️ Top-level error returned by every `Brep` mutating/query method.
#[derive(Clone, Debug, PartialEq)]
pub enum KernelError {
    /// 🚨️ A caller-supplied parameter is out of range or otherwise malformed.
    InvalidInput(String),
    /// 🚨️ A referenced entity id does not exist (or belongs to another `Body`).
    MissingEntity(String),
    /// 🚨️ The operation is well-formed but could not be completed.
    Operation(String),
    /// 🚨️ An intersection sub-problem could not be resolved to certified geometry.
    Intersect(IntersectError),
    /// 🚨️ A Boolean combination could not be completed.
    Boolean(BooleanError),
    /// 🚨️ STEP import/export failed.
    Step(StepError),
}

impl std::fmt::Display for KernelError {
    async fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            KernelError::MissingEntity(id) => write!(f, "missing entity: {id}"),
            KernelError::Operation(msg) => write!(f, "operation failed: {msg}"),
            KernelError::Intersect(e) => write!(f, "intersection failed: {e}"),
            KernelError::Boolean(e) => write!(f, "boolean failed: {e}"),
            KernelError::Step(e) => write!(f, "step failed: {e}"),
        }
    }
}

impl std::error::Error for KernelError {}

impl From<IntersectError> for KernelError {
    async fn from(e: IntersectError) -> Self {
        KernelError::Intersect(e)
    }
}

impl From<BooleanError> for KernelError {
    async fn from(e: BooleanError) -> Self {
        KernelError::Boolean(e)
    }
}

impl From<StepError> for KernelError {
    async fn from(e: StepError) -> Self {
        KernelError::Step(e)
    }
}

/// 🚨️ Curve/curve, curve/surface and surface/surface intersection failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum IntersectError {
    /// 🚨️ The two operands are tangent within tolerance; the caller must use a dedicated
    /// tangency-aware path rather than the generic intersector.
    Tangent,
    /// 🚨️ The general (marching) path failed to converge or close a loop within its iteration budget.
    Unresolved(String),
    /// 🚨️ The operands are geometrically degenerate (zero-length curve, singular surface point, …).
    Degenerate(String),
}

impl std::fmt::Display for IntersectError {
    async fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntersectError::Tangent => write!(f, "tangent configuration"),
            IntersectError::Unresolved(msg) => write!(f, "unresolved: {msg}"),
            IntersectError::Degenerate(msg) => write!(f, "degenerate: {msg}"),
        }
    }
}

/// 🚨️ Boolean pipeline failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum BooleanError {
    /// 🚨️ Face imprinting could not resolve a consistent UV arrangement.
    ImprintFailed(String),
    /// 🚨️ A cell produced by the arrangement could not be classified with certainty.
    ClassificationAmbiguous(String),
    /// 🚨️ The stitched result failed shape validation.
    InvalidResult(String),
    /// 🚨️ An intersection sub-step failed.
    Intersect(IntersectError),
}

impl std::fmt::Display for BooleanError {
    async fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanError::ImprintFailed(msg) => write!(f, "imprint failed: {msg}"),
            BooleanError::ClassificationAmbiguous(msg) => write!(f, "ambiguous classification: {msg}"),
            BooleanError::InvalidResult(msg) => write!(f, "invalid result: {msg}"),
            BooleanError::Intersect(e) => write!(f, "{e}"),
        }
    }
}

impl From<IntersectError> for BooleanError {
    async fn from(e: IntersectError) -> Self {
        BooleanError::Intersect(e)
    }
}

/// 🚨️ Hand-rolled ISO 10303-21 (STEP) reader/writer failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum StepError {
    /// 🚨️ The Part-21 lexer/parser rejected the input text.
    Syntax(String),
    /// 🚨️ An instance reference (`#123`) does not resolve to any parsed entity.
    UnresolvedReference(u64),
    /// 🚨️ An entity type is recognized but not translatable by this subset reader.
    Unsupported(String),
}

impl std::fmt::Display for StepError {
    async fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepError::Syntax(msg) => write!(f, "syntax error: {msg}"),
            StepError::UnresolvedReference(id) => write!(f, "unresolved reference #{id}"),
            StepError::Unsupported(name) => write!(f, "unsupported entity: {name}"),
        }
    }
}

// #endregion 🔖️Errors

// #region 🔖️Issues

/// 🚨️ A single finding from `validate::validate_body`, scoped to one entity.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationIssue {
    /// 🚨️ Human-readable entity label the issue is scoped to (e.g. `"edge-3"`).
    pub entity: String,
    /// 🚨️ Machine-readable, stable diagnostic code (e.g. `"same-parameter-violated"`).
    pub code: &'static str,
    /// 🚨️ One-line description of the failure, including the measured residual where relevant.
    pub message: String,
}

impl std::fmt::Display for ValidationIssue {
    async fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.code, self.entity, self.message)
    }
}

// #endregion 🔖️Issues

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn kernel_error_displays_readable_message() {
        let e = KernelError::InvalidInput("radius must be positive".to_string());
        assert_eq!(e.to_string(), "invalid input: radius must be positive");
    }

    #[test]
    async fn intersect_error_converts_into_kernel_error() {
        let e: KernelError = IntersectError::Tangent.into();
        assert!(matches!(e, KernelError::Intersect(IntersectError::Tangent)));
    }

    #[test]
    async fn boolean_error_wraps_intersect_error() {
        let e: BooleanError = IntersectError::Degenerate("zero length".to_string()).into();
        assert!(matches!(e, BooleanError::Intersect(IntersectError::Degenerate(_))));
    }

    #[test]
    async fn validation_issue_displays_code_entity_message() {
        let issue = ValidationIssue { entity: "edge-3".to_string(), code: "same-parameter-violated", message: "residual 1e-3 exceeds tol 1e-6".to_string() };
        assert_eq!(issue.to_string(), "[same-parameter-violated] edge-3: residual 1e-3 exceeds tol 1e-6");
    }
}
// #endregion 🔖️Tests
