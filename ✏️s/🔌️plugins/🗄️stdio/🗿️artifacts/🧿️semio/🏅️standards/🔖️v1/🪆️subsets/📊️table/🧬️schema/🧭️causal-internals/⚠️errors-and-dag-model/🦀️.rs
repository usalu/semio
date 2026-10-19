//! ⚠️ Errors and the DAG model: the shared causal error type with its conversions, the pair normaliser and the causal DAG with its graph queries. See [`super`].

use super::*;

// #region 🔖️Errors
/// ⚠️ Fallible-computation error type shared by every function in this crate.
#[derive(Debug)]
pub enum CausalError {
    VariableNotFound(String),
    NotADag(Vec<usize>),
    ColumnType(usize, &'static str),
    DimensionMismatch(String),
    NotIdentifiable(String),
    Singular(&'static str),
    InvalidQuery(String),
    InferenceTooLarge(usize, usize),
    Stats(semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::StatisticsError),
    Tabular(semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::TabularError),
    Probability(semio_s_artifact_stdio_semio_table_probability::schema::probability_internals::ProbabilityError),
}

impl std::fmt::Display for CausalError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VariableNotFound(name) => write!(formatter, "variable `{name}` not found"),
            Self::NotADag(indices) => write!(formatter, "edges contain a cycle through node indices {indices:?}"),
            Self::ColumnType(column, expected) => write!(formatter, "column {column} has wrong type: expected {expected}"),
            Self::DimensionMismatch(message) => write!(formatter, "dimension mismatch: {message}"),
            Self::NotIdentifiable(message) => write!(formatter, "effect not identifiable: {message}"),
            Self::Singular(context) => write!(formatter, "singular linear system in {context}"),
            Self::InvalidQuery(message) => write!(formatter, "invalid query: {message}"),
            Self::InferenceTooLarge(entries, limit) => write!(formatter, "inference too large: factor would have {entries} entries (limit {limit})"),
            Self::Stats(error) => std::fmt::Display::fmt(error, formatter),
            Self::Tabular(error) => std::fmt::Display::fmt(error, formatter),
            Self::Probability(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for CausalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Stats(error) => Some(error),
            Self::Tabular(error) => Some(error),
            Self::Probability(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::StatisticsError> for CausalError {
    fn from(error: semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::StatisticsError) -> Self {
        Self::Stats(error)
    }
}

impl From<semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::TabularError> for CausalError {
    fn from(error: semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::TabularError) -> Self {
        Self::Tabular(error)
    }
}

impl From<semio_s_artifact_stdio_semio_table_probability::schema::probability_internals::ProbabilityError> for CausalError {
    fn from(error: semio_s_artifact_stdio_semio_table_probability::schema::probability_internals::ProbabilityError) -> Self {
        Self::Probability(error)
    }
}

/// 🧮️ Normalizes an unordered pair so `a <= b`, the canonical key for undirected-edge sets.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn norm_pair(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
// #endregion 🔖️Errors

// #region 🔖️Dag
/// 🧭️ A directed acyclic causal graph over named variables.
#[derive(Clone, Debug, PartialEq)]
pub struct CausalDag {
    pub(super) names: Vec<String>,
    index: HashMap<String, usize>,
    pub(super) parents: Vec<Vec<usize>>,
    children: Vec<Vec<usize>>,
    topo: Vec<usize>,
}

impl CausalDag {
    /// 🧭️ Builds a DAG from index edges, validating acyclicity via
    /// `semio_framework_graph_algorithms::topo_sort`.
    pub fn new(names: Vec<String>, edges: &[(usize, usize)]) -> Result<Self, CausalError> {
        let n = names.len();
        for &(a, b) in edges {
            if a >= n {
                return Err(CausalError::VariableNotFound(format!("index {a}")));
            }
            if b >= n {
                return Err(CausalError::VariableNotFound(format!("index {b}")));
            }
        }
        let adj = semio_framework_graph_algorithms::adjacency(n, edges, true);
        let topo = semio_framework_graph_algorithms::topo_sort(&adj).map_err(|e| CausalError::NotADag(e.cycle))?;
        let mut parents = vec![Vec::new(); n];
        let mut children = vec![Vec::new(); n];
        for &(a, b) in edges {
            children[a].push(b);
            parents[b].push(a);
        }
        for p in parents.iter_mut() {
            p.sort_unstable();
            p.dedup();
        }
        for c in children.iter_mut() {
            c.sort_unstable();
            c.dedup();
        }
        let index = names.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect();
        Ok(Self { names, index, parents, children, topo })
    }

    /// 🧭️ Builds a DAG from named edges, resolving each name against `names`.
    pub fn from_named_edges(names: Vec<String>, edges: &[(&str, &str)]) -> Result<Self, CausalError> {
        let index: HashMap<&str, usize> = names.iter().enumerate().map(|(i, n)| (n.as_str(), i)).collect();
        let mut resolved = Vec::with_capacity(edges.len());
        for &(a, b) in edges {
            let ai = *index.get(a).ok_or_else(|| CausalError::VariableNotFound(a.to_string()))?;
            let bi = *index.get(b).ok_or_else(|| CausalError::VariableNotFound(b.to_string()))?;
            resolved.push((ai, bi));
        }
        Self::new(names, &resolved)
    }

    pub fn n(&self) -> usize {
        self.names.len()
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for (child, parents) in self.parents.iter().enumerate() {
            for &parent in parents {
                out.push((parent, child));
            }
        }
        out.sort_unstable();
        out
    }

    pub fn parents(&self, v: usize) -> &[usize] {
        &self.parents[v]
    }

    pub fn children(&self, v: usize) -> &[usize] {
        &self.children[v]
    }

    /// 🧭️ Strict ancestors of `v` (excludes `v` itself), ascending.
    pub fn ancestors(&self, v: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = self.parents[v].clone();
        while let Some(u) = stack.pop() {
            if visited.insert(u) {
                stack.extend(self.parents[u].iter().copied());
            }
        }
        let mut out: Vec<usize> = visited.into_iter().collect();
        out.sort_unstable();
        out
    }

    /// 🧭️ Strict descendants of `v` (excludes `v` itself), ascending.
    pub fn descendants(&self, v: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = self.children[v].clone();
        while let Some(u) = stack.pop() {
            if visited.insert(u) {
                stack.extend(self.children[u].iter().copied());
            }
        }
        let mut out: Vec<usize> = visited.into_iter().collect();
        out.sort_unstable();
        out
    }

    pub fn topological_order(&self) -> &[usize] {
        &self.topo
    }

    /// 🕸️ Undirected moral-graph edges (`a < b`): the skeleton plus edges marrying every pair of co-parents.
    pub fn moralize(&self) -> Vec<(usize, usize)> {
        let mut edges: BTreeSet<(usize, usize)> = BTreeSet::new();
        for (child, parents) in self.parents.iter().enumerate() {
            for &parent in parents {
                edges.insert(norm_pair(parent, child));
            }
            for i in 0..parents.len() {
                for j in (i + 1)..parents.len() {
                    edges.insert(norm_pair(parents[i], parents[j]));
                }
            }
        }
        edges.into_iter().collect()
    }
}
// #endregion 🔖️Dag
