//! 📇 Immutable canonical owner indices schedule every retained declaration without request-time catalog scans.
pub struct OwnerIndex {
    pub owner: &'static str,
    pub rules: &'static [usize],
    pub derived: &'static [usize],
    pub inverse: &'static [usize],
    pub bounds: &'static [usize],
    pub unique: &'static [usize],
}
include!("program-index-data.rs");
pub fn owner(name: &str) -> Option<&'static OwnerIndex> {
    OWNERS.binary_search_by(|row| row.owner.cmp(name)).ok().map(|index| &OWNERS[index])
}
pub fn entity(name: &str) -> Option<&'static OwnerIndex> {
    ENTITIES.binary_search_by(|row| row.owner.cmp(name)).ok().map(|index| &ENTITIES[index])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_static_positions_and_inherited_owners_match_the_complete_program() {
        let program = crate::validation::ifc4x3_express_program();
        assert_eq!(PROGRAM_SHA256, crate::validation::EXPRESS_PROGRAM_SHA256);
        for (kind, owners) in [
            (0, program.rules.iter().map(|rule| rule.owner).collect::<Vec<_>>()),
            (1, program.derived.iter().map(|rule| rule.owner).collect()),
            (2, program.inverse.iter().map(|rule| rule.owner).collect()),
            (3, program.bounds.iter().map(|rule| rule.owner).collect()),
            (4, program.unique.iter().map(|rule| rule.owner).collect()),
        ] {
            let select = |row: &'static OwnerIndex| match kind {
                0 => row.rules,
                1 => row.derived,
                2 => row.inverse,
                3 => row.bounds,
                _ => row.unique,
            };
            for (position, expected) in owners.iter().enumerate() {
                assert!(select(owner(expected).unwrap()).contains(&position));
            }
            assert_eq!(OWNERS.iter().map(|row| select(row).len()).sum::<usize>(), owners.len());
            for declaration in crate::validation::metadata::ENTITIES {
                let actual = select(entity(declaration.key).unwrap());
                let expected: Vec<_> = owners.iter().enumerate().filter_map(|(index, owner)| crate::validation::metadata::is_subtype(declaration.key, owner).then_some(index)).collect();
                assert_eq!(actual, expected);
            }
        }
    }
}
