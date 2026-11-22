//! 🌳️ Selection among artifact dialects that share a document and surface role.

use crate::SubsetId;

/// 🌿️ The result of selecting the unique most general declared dialect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MostGeneralDialect<T> {
    One(T),
    Ambiguous,
    Empty,
}

impl<T> MostGeneralDialect<T> {
    /// 🎯️ Returns the selected candidate when it is unique.
    pub fn one(self) -> Option<T> {
        match self {
            Self::One(candidate) => Some(candidate),
            Self::Ambiguous | Self::Empty => None,
        }
    }
}

/// 🏞️ Selects the only uncovered declaration. A whole-standard dialect covers the
/// named subsets of its own artifact kind and standard.
pub fn most_general_dialect<T>(mut candidates: Vec<T>, view: impl Fn(&T) -> (&str, &str, &str)) -> MostGeneralDialect<T> {
    let any = SubsetId::ANY.0;
    let mut uncovered = candidates.iter().enumerate().filter_map(|(index, candidate)| {
        let (kind, standard, subset) = view(candidate);
        let covered = subset != any && candidates.iter().any(|other| view(other) == (kind, standard, any));
        (!covered).then_some(index)
    });
    match (uncovered.next(), uncovered.next()) {
        (Some(index), None) => MostGeneralDialect::One(candidates.swap_remove(index)),
        (Some(_), Some(_)) => MostGeneralDialect::Ambiguous,
        (None, _) => MostGeneralDialect::Empty,
    }
}
