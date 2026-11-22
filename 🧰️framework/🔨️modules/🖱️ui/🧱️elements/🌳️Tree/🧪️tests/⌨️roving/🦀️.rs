use super::*;
use serde_json::Value;
use std::collections::BTreeSet;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/⌨️roving/🔣️.json")).expect("the neutral tree-roving vectors parse")
}

fn strings(value: &Value) -> Vec<String> {
    value.as_array().expect("an array").iter().map(|entry| entry.as_str().expect("a string").to_owned()).collect()
}

fn ordered(items: &[Value], level: usize, out: &mut Vec<TreeNavRow>) {
    for item in items {
        let id = item["id"].as_str().expect("id").to_owned();
        let children = item["items"].as_array();
        out.push(TreeNavRow { level, expanded: None, label: item["label"].as_str().expect("label").to_owned(), id });
        if let Some(children) = children {
            ordered(children, level + 1, out);
        }
    }
}

fn visible(items: &[Value], level: usize, expanded: &BTreeSet<String>, out: &mut Vec<TreeNavRow>) {
    for item in items {
        let id = item["id"].as_str().expect("id").to_owned();
        let children = item["items"].as_array();
        out.push(TreeNavRow { level, expanded: children.map(|_| expanded.contains(&id)), label: item["label"].as_str().expect("label").to_owned(), id: id.clone() });
        if let Some(children) = children.filter(|_| expanded.contains(&id)) {
            visible(children, level + 1, expanded, out);
        }
    }
}

fn initially_expanded(items: &[Value], out: &mut BTreeSet<String>) {
    for item in items {
        if item["expanded"].as_bool() == Some(true) {
            out.insert(item["id"].as_str().expect("id").to_owned());
        }
        if let Some(children) = item["items"].as_array() {
            initially_expanded(children, out);
        }
    }
}

fn modifiers(step: &Value) -> TreeNavModifiers {
    let held = step["modifiers"].as_array().map(|list| strings(&Value::Array(list.clone()))).unwrap_or_default();
    let has = |name: &str| held.iter().any(|entry| entry == name);
    TreeNavModifiers { shift: has("shift"), ctrl: has("ctrl"), meta: has("meta"), alt: has("alt") }
}

struct Host {
    expanded: BTreeSet<String>,
    focus: String,
    selection: TreeSelection,
    clicked: Vec<String>,
    double_clicked: Vec<String>,
    query: String,
    multiple: bool,
}

impl Host {
    fn rows(&self, tree: &Value) -> Vec<TreeNavRow> {
        let mut rows = Vec::new();
        visible(tree.as_array().expect("tree"), 1, &self.expanded, &mut rows);
        rows
    }

    fn ordered(&self, tree: &Value) -> Vec<TreeNavRow> {
        let mut rows = Vec::new();
        ordered(tree.as_array().expect("tree"), 1, &mut rows);
        rows
    }

    fn apply(&mut self, tree: &Value, intents: &[TreeIntent]) {
        let before = self.focus.clone();
        for intent in intents {
            let rows = self.rows(tree);
            match *intent {
                TreeIntent::Focus(index) => self.focus = rows[index].id.clone(),
                TreeIntent::Click { index, merge } => {
                    let id = rows[index].id.clone();
                    if rows[index].expanded.is_some() && !self.expanded.remove(&id) {
                        self.expanded.insert(id.clone());
                    }
                    self.selection = tree_selection(&self.ordered(tree), &self.selection, Some(&before), &id, merge, self.multiple);
                    self.clicked.push(id);
                }
                TreeIntent::Pick { index, merge } => self.selection = tree_selection(&self.ordered(tree), &self.selection, Some(&before), &rows[index].id, merge, self.multiple),
                TreeIntent::DoubleClick(index) => self.double_clicked.push(rows[index].id.clone()),
            }
        }
    }

    fn step(&mut self, tree: &Value, step: &Value) -> bool {
        let rows = self.rows(tree);
        let held = modifiers(step);
        if let Some(target) = step["click"].as_str() {
            let index = rows.iter().position(|row| row.id == target).expect("clicked row is visible");
            let count = step["count"].as_u64().unwrap_or(1) as u32;
            let intents = tree_pointer(held, index, count);
            self.apply(tree, &intents);
            return true;
        }
        let key = step["key"].as_str().expect("key");
        if step["reset"].as_bool().unwrap_or(false) {
            self.query.clear();
        }
        let current = rows.iter().position(|row| row.id == self.focus).expect("focused row is visible");
        if let Some(intents) = tree_key(key, held, &rows, current, self.multiple) {
            self.query.clear();
            self.apply(tree, &intents);
            return true;
        }
        let typed = key.chars().count() == 1 && !held.ctrl && !held.meta && !held.alt;
        if !typed {
            return false;
        }
        self.query.push_str(key);
        match tree_typeahead(&rows, current, &self.query) {
            Some(index) => {
                self.apply(tree, &[TreeIntent::Focus(index)]);
                true
            }
            None => false,
        }
    }
}

#[test]
fn the_shared_vectors_drive_every_key_and_click_to_the_same_focus_selection_and_fold() {
    let law = fixture();
    let tree = &law["tree"];
    for case in law["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let mut expanded = BTreeSet::new();
        initially_expanded(tree.as_array().expect("tree"), &mut expanded);
        let mut host = Host { expanded, focus: case["focus"].as_str().expect("focus").to_owned(), selection: TreeSelection::default(), clicked: Vec::new(), double_clicked: Vec::new(), query: String::new(), multiple: case["multiple"].as_bool().expect("multiple") };
        for (index, step) in case["steps"].as_array().expect("steps").iter().enumerate() {
            let handled = host.step(tree, step);
            let expect = &step["expect"];
            let at = format!("{name} / step {index}");
            assert_eq!(handled, expect["handled"].as_bool().unwrap_or(true), "{at}: handled");
            if let Some(focus) = expect["focus"].as_str() {
                assert_eq!(host.focus, focus, "{at}: focus");
            }
            if expect.get("selected").is_some() {
                assert_eq!(host.selection.ids, strings(&expect["selected"]), "{at}: selected");
            }
            if expect.get("expanded").is_some() {
                assert_eq!(host.expanded.iter().cloned().collect::<Vec<_>>(), strings(&expect["expanded"]), "{at}: expanded");
            }
            if expect.get("clicked").is_some() {
                assert_eq!(host.clicked, strings(&expect["clicked"]), "{at}: clicked");
            }
            if expect.get("doubleClicked").is_some() {
                assert_eq!(host.double_clicked, strings(&expect["doubleClicked"]), "{at}: doubleClicked");
            }
        }
    }
}

#[test]
fn the_roving_tab_stop_is_the_remembered_row_else_the_selected_row_else_the_first() {
    let law = fixture();
    let mut expanded = BTreeSet::new();
    initially_expanded(law["tree"].as_array().expect("tree"), &mut expanded);
    let mut rows = Vec::new();
    visible(law["tree"].as_array().expect("tree"), 1, &expanded, &mut rows);
    for vector in law["roving"].as_array().expect("roving") {
        let selected = strings(&vector["selected"]);
        let stop = tree_roving_stop(&rows, vector["remembered"].as_str(), &selected).map(|index| rows[index].id.as_str());
        assert_eq!(stop, vector["stop"].as_str(), "{vector}");
    }
    assert_eq!(tree_roving_stop(&[], None, &[]), None, "an empty tree has no tab stop");
}

#[test]
fn react_parity_the_plain_keys_answer_the_intents_the_react_pattern_answers() {
    let rows = [
        TreeNavRow { id: "a".into(), level: 1, expanded: Some(true), label: "A".into() },
        TreeNavRow { id: "b".into(), level: 2, expanded: None, label: "B".into() },
        TreeNavRow { id: "c".into(), level: 1, expanded: Some(false), label: "C".into() },
    ];
    let plain = TreeNavModifiers::default();
    assert_eq!(tree_key("ArrowDown", plain, &rows, 0, false), Some(vec![TreeIntent::Focus(1)]));
    assert_eq!(tree_key("ArrowUp", plain, &rows, 0, false), None);
    assert_eq!(tree_key("Home", plain, &rows, 2, false), Some(vec![TreeIntent::Focus(0)]));
    assert_eq!(tree_key("End", plain, &rows, 0, false), Some(vec![TreeIntent::Focus(2)]));
    assert_eq!(tree_key("ArrowRight", plain, &rows, 2, false), Some(vec![TreeIntent::Click { index: 2, merge: TreeMerge::Replace }]));
    assert_eq!(tree_key("ArrowRight", plain, &rows, 0, false), Some(vec![TreeIntent::Focus(1)]));
    assert_eq!(tree_key("ArrowLeft", plain, &rows, 0, false), Some(vec![TreeIntent::Click { index: 0, merge: TreeMerge::Replace }]));
    assert_eq!(tree_key("ArrowLeft", plain, &rows, 1, false), Some(vec![TreeIntent::Focus(0)]));
    assert_eq!(tree_key("Enter", plain, &rows, 1, false), Some(vec![TreeIntent::Click { index: 1, merge: TreeMerge::Replace }]));
    assert_eq!(tree_key("x", plain, &rows, 1, false), None);
    assert_eq!(tree_key("ArrowDown", plain, &[], 0, false), None, "no rows");
}
