//! 🧮️ Net of one snapshot edit as value domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `value` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_keyed, net_ordered, NetStep};
use crate::standards::v1::subsets::value::schema::mutations::{insert_list_item, remove_list_item, remove_map_entry, remove_node, set_map_entry, set_node, set_value, SemioValueMutation, SemioValuePath, SemioValuePathSegment};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioValueSnapshot, next: &SemioValueSnapshot) -> Vec<SemioValueMutation> {
    let mut out = Vec::new();
    net_value(&mut Vec::new(), &base.root, &next.root, &mut out);
    let nodes = net_keyed(&base.nodes, &next.nodes, |node| node.id.clone());
    out.extend(nodes.removed.iter().map(|node| SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: node.id.clone() })));
    out.extend(nodes.modified.iter().map(|(_, node)| SemioValueMutation::SetNode(set_node::SetNode { id: node.id.clone(), value: node.value.clone(), at: None })));
    out.extend(nodes.added.iter().map(|node| SemioValueMutation::SetNode(set_node::SetNode { id: node.id.clone(), value: node.value.clone(), at: None })));
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_value(path: &mut SemioValuePath, before: &SemioValue, after: &SemioValue, out: &mut Vec<SemioValueMutation>) {
    if before == after {
        return;
    }
    match (before, after) {
        (SemioValue::Map { entries: before_entries }, SemioValue::Map { entries: after_entries }) => {
            let survivors = before_entries.iter().filter(|entry| after_entries.iter().any(|other| other.key == entry.key)).map(|entry| &entry.key);
            let kept_order = survivors.eq(after_entries.iter().map(|entry| &entry.key).take(before_entries.iter().filter(|entry| after_entries.iter().any(|other| other.key == entry.key)).count()));
            if !kept_order {
                out.push(SemioValueMutation::SetValue(set_value::SetValue { path: path.clone(), value: after.clone() }));
                return;
            }
            for entry in before_entries.iter().filter(|entry| !after_entries.iter().any(|other| other.key == entry.key)) {
                out.push(SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: path.clone(), key: entry.key.clone() }));
            }
            for entry in before_entries {
                if let Some(other) = after_entries.iter().find(|other| other.key == entry.key) {
                    path.push(SemioValuePathSegment::Key { key: entry.key.clone() });
                    net_value(path, &entry.value, &other.value, out);
                    path.pop();
                }
            }
            for entry in after_entries.iter().filter(|entry| !before_entries.iter().any(|other| other.key == entry.key)) {
                out.push(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: path.clone(), key: entry.key.clone(), value: entry.value.clone(), at: None }));
            }
        }
        (SemioValue::List { items: before_items }, SemioValue::List { items: after_items }) => {
            for step in net_ordered(before_items, after_items) {
                match step {
                    NetStep::Modify { index, item } => {
                        path.push(SemioValuePathSegment::Index { index });
                        net_value(path, &before_items[index], item, out);
                        path.pop();
                    }
                    NetStep::Remove { index } => out.push(SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: path.clone(), index })),
                    NetStep::Insert { index, item } => out.push(SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: path.clone(), index, value: item.clone() })),
                }
            }
        }
        _ => out.push(SemioValueMutation::SetValue(set_value::SetValue { path: path.clone(), value: after.clone() })),
    }
}
