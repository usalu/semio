//! ❓️ What-if query: the what-if request record and its builder methods. See [`super`].

// #region 🔖️Query
/// 🔮️ One what-if request: `do(...)` interventions and, for counterfactual queries, `given(...)` factual evidence.
#[derive(Clone, Debug, Default)]
pub struct WhatIf {
    pub interventions: Vec<(usize, f64)>,
    pub evidence: Vec<(usize, f64)>,
}

impl WhatIf {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self::default()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn do_(mut self, var: usize, value: f64) -> Self {
        self.interventions.push((var, value));
        self
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn given(mut self, var: usize, value: f64) -> Self {
        self.evidence.push((var, value));
        self
    }
}
// #endregion 🔖️Query
