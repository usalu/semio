//! 📇 Canonical owner indices schedule every retained declaration without request-time catalog scans: derived once from the canonical program and the released hierarchy, so they cannot go stale.
use super::ast::ExpressProgram;
use super::metadata;
use semio_s_artifact_stdio_ifc_json_pack::Leaker;
use std::collections::BTreeMap;
use std::sync::OnceLock;

pub struct OwnerIndex {
    pub owner: &'static str,
    pub rules: &'static [usize],
    pub derived: &'static [usize],
    pub inverse: &'static [usize],
    pub bounds: &'static [usize],
    pub unique: &'static [usize],
}

struct Index {
    owners: &'static [OwnerIndex],
    entities: &'static [OwnerIndex],
}
static INDEX: OnceLock<Index> = OnceLock::new();

type Lanes = [Vec<usize>; 5];

fn exact(program: &ExpressProgram) -> BTreeMap<&'static str, Lanes> {
    let mut rows: BTreeMap<&'static str, Lanes> = BTreeMap::new();
    let lanes: [Vec<&'static str>; 5] = [
        program.rules.iter().map(|row| row.owner).collect(),
        program.derived.iter().map(|row| row.owner).collect(),
        program.inverse.iter().map(|row| row.owner).collect(),
        program.bounds.iter().map(|row| row.owner).collect(),
        program.unique.iter().map(|row| row.owner).collect(),
    ];
    for (lane, owners) in lanes.iter().enumerate() {
        for (position, owner) in owners.iter().enumerate() {
            rows.entry(*owner).or_default()[lane].push(position);
        }
    }
    rows
}

fn row(owner: &'static str, lanes: Lanes) -> OwnerIndex {
    let [rules, derived, inverse, bounds, unique] = lanes;
    OwnerIndex { owner, rules: Leaker::slice(rules), derived: Leaker::slice(derived), inverse: Leaker::slice(inverse), bounds: Leaker::slice(bounds), unique: Leaker::slice(unique) }
}

fn build() -> Index {
    let rows = exact(super::ifc4x3_express_program());
    let entities = metadata::entities()
        .iter()
        .map(|entity| {
            let mut lanes = Lanes::default();
            let mut current = Some(entity);
            for _ in 0..32 {
                let Some(declaration) = current else { break };
                if let Some(found) = rows.get(declaration.key) {
                    for (merged, own) in lanes.iter_mut().zip(found) {
                        merged.extend_from_slice(own);
                    }
                }
                current = declaration.supertype.and_then(metadata::entity);
            }
            for lane in &mut lanes {
                lane.sort_unstable();
            }
            row(entity.key, lanes)
        })
        .collect();
    Index { owners: Leaker::slice(rows.into_iter().map(|(owner, lanes)| row(owner, lanes)).collect()), entities: Leaker::slice(entities) }
}

fn index() -> &'static Index {
    INDEX.get_or_init(build)
}

/// 📇 Declarations owned exactly by one owner name, in canonical program order.
pub fn owner(name: &str) -> Option<&'static OwnerIndex> {
    let rows = index().owners;
    rows.binary_search_by(|row| row.owner.cmp(name)).ok().map(|position| &rows[position])
}

/// 📇 Declarations an entity inherits from itself and every supertype, in canonical program order.
pub fn entity(name: &str) -> Option<&'static OwnerIndex> {
    let rows = index().entities;
    rows.binary_search_by(|row| row.owner.cmp(name)).ok().map(|position| &rows[position])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_static_positions_and_inherited_owners_match_the_complete_program() {
        let program = super::super::ifc4x3_express_program();
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
            assert_eq!(index().owners.iter().map(|row| select(row).len()).sum::<usize>(), owners.len());
            for declaration in metadata::entities() {
                let actual = select(entity(declaration.key).unwrap());
                let expected: Vec<_> = owners.iter().enumerate().filter_map(|(index, owner)| metadata::is_subtype(declaration.key, owner).then_some(index)).collect();
                assert_eq!(actual, expected);
            }
        }
    }
}
