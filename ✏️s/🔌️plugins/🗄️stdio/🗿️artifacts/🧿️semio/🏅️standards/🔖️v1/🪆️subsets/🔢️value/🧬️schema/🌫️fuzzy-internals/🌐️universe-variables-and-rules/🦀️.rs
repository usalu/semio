//! 🌐️ Universe, linguistic variables and rules: the discretised universe, linguistic variables, antecedent clauses, consequents, rules and rule bases. See [`super`].

use super::*;

// #region 🔖️Universe
/// 📏️ Discrete universe of discourse for numerical integration and defuzzification.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Universe {
    pub min: f64,
    pub max: f64,
    pub samples: Vec<f64>,
}

impl Universe {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(min: f64, max: f64, n: usize) -> FuzzyResult<Self> {
        if min >= max || n == 0 {
            return Err(FuzzyError::InvalidDomain("universe bounds".into()));
        }
        Ok(Self { min, max, samples: linspace(min, max, n) })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sample(&self, index: usize) -> f64 {
        self.samples[index]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}
// #endregion 🔖️Universe

// #region 🔖️LinguisticVariable
/// 🗣️ Linguistic variable: universe plus named fuzzy terms.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct LinguisticVariable {
    pub name: String,
    pub universe: Universe,
    pub terms: Vec<FuzzySet>,
}

impl LinguisticVariable {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(name: impl Into<String>, universe: Universe, terms: Vec<FuzzySet>) -> Self {
        Self { name: name.into(), universe, terms }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fuzzify(&self, x: f64) -> Vec<f64> {
        self.terms.iter().map(|t| t.grade(x)).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn term_index(&self, name: &str) -> Option<usize> {
        self.terms.iter().position(|t| t.name == name)
    }
}
// #endregion 🔖️LinguisticVariable

// #region 🔖️Rule
/// 📜️ Antecedent clause: input variable index and term index.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct AntecedentClause {
    pub input: usize,
    pub term: usize,
    pub hedge: Option<Hedge>,
}

/// 📜️ Rule consequent variants for Mamdani, Sugeno, and Tsukamoto engines.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Consequent {
    Mamdani { output: usize, term: usize },
    SugenoConstant { output: usize, value: f64 },
    SugenoLinear { output: usize, coeffs: Vec<f64> },
    Tsukamoto { output: usize, term: usize },
    SoftConstraint { output: usize, term: usize, preference: f64 },
}

/// 📜️ Weighted fuzzy rule with optional confidence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Rule {
    pub id: usize,
    pub antecedents: Vec<AntecedentClause>,
    pub consequent: Consequent,
    pub weight: f64,
    pub confidence: f64,
}

impl Rule {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn firing_strength(&self, inputs: &[LinguisticVariable], values: &[f64], tnorm: TNorm) -> f64 {
        let strengths: Vec<f64> = self
            .antecedents
            .iter()
            .map(|clause| {
                let mu = inputs[clause.input].terms[clause.term].grade(values[clause.input]);
                clause.hedge.map_or(mu, |h| h.apply(mu))
            })
            .collect();
        self.weight * self.confidence * tnorm.fold(strengths.into_iter())
    }
}

/// 📚️ Collection of fuzzy rules.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RuleBase {
    pub rules: Vec<Rule>,
}

impl RuleBase {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
// #endregion 🔖️Rule
