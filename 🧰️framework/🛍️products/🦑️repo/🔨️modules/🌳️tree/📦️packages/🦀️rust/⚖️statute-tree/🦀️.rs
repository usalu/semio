//! ⚖️ Statute tree: the statute and territory trees and the declared statute catalog.
//!
//! Holds the `📜️StatuteTree` region of the crate root; the root re-exports its items.

use super::*;

//#region 📜️StatuteTree

/// 📜️ The catalog of the law this repository actually declares, resolved through `📜️statutes` —
/// the production [`StatuteCatalog`] every consumer of the statute and territory trees shares.
pub struct DeclaredStatuteCatalog;

impl StatuteCatalog for DeclaredStatuteCatalog {
    fn info(&self, statute: &Statute) -> Option<StatuteMeta> {
        semio_framework_repo_statutes::statutes().iter().find(|meta| meta.kind == *statute).cloned()
    }

    fn statute_id(&self, statute: &Statute) -> String {
        format!("📜️{}", statute.0.replace(['/', '.'], ""))
    }

    fn statute_uri(&self, statute: &Statute) -> String {
        format!("repo://statute/{}", statute.0)
    }

    fn statute_label(&self, statute: &Statute) -> String {
        statute.0.rsplit('/').next().unwrap_or(&statute.0).to_string()
    }

    fn entity_kind(&self, statute: &Statute) -> String {
        statute.0.split('/').next().unwrap_or_default().to_string()
    }

    fn territory_id(&self, territory: &Territory) -> String {
        format!("🗺️{}", territory.name.replace([' ', '/'], ""))
    }
}

/// 📜️ Every declared statute, identifier ascending, so the projection never depends on the order a
/// map happened to iterate in.
pub fn declared_statutes() -> Vec<Statute> {
    let mut kinds: Vec<Statute> = semio_framework_repo_statutes::statutes().iter().map(|meta| meta.kind.clone()).collect();
    kinds.sort_by(|left, right| left.0.cmp(&right.0));
    kinds
}

/// 🗺️ Every territory a declared policy carries at its top level, name ascending, each name claimed
/// by the first policy that declares it.
pub fn declared_territories() -> Vec<Territory> {
    let mut seen: BTreeMap<String, Territory> = BTreeMap::new();
    for policy in semio_framework_repo_statutes::policies() {
        for territory in policy.groups.iter().flatten() {
            seen.entry(territory.name.clone()).or_insert_with(|| territory.clone());
        }
    }
    seen.into_values().collect()
}

/// 📜️ Builds the statute tree: one category per path segment, the leaf carrying its metadata.
pub fn build_statute_tree(statutes: &[Statute], catalog: &dyn StatuteCatalog) -> Vec<TreeNode> {
    #[derive(Default)]
    struct Entry {
        node: Option<TreeNode>,
        order: Vec<String>,
        children: BTreeMap<String, Entry>,
    }

    fn insert(entry: &mut Entry, parts: &[String], full: &Statute, catalog: &dyn StatuteCatalog, depth: usize) {
        let Some(part) = parts.get(depth) else { return };
        if !entry.children.contains_key(part) {
            entry.order.push(part.clone());
            let is_leaf = depth == parts.len() - 1;
            let node = if is_leaf {
                let meta = catalog.info(full);
                let priority = meta.as_ref().map_or(BreachPriority::Low, |meta| meta.priority);
                let mut node = tree_node(
                    TreeNodeKind::Statute,
                    &full.0,
                    &format!("{}{part}", priority_icon(priority)),
                    &catalog.statute_uri(full),
                );
                if let Some(meta) = meta {
                    node.description = meta.reason.clone();
                    if meta.autofixable {
                        node.sub_kind = "autofixable".to_string();
                    }
                    node.data = Some(data_of(vec![
                        ("id", text(&full.0)),
                        ("priority", text(meta.priority.as_str())),
                        ("autofixable", Value::Bool(meta.autofixable)),
                        ("reason", text(&meta.reason)),
                        ("solution", text(&meta.solution)),
                    ]));
                }
                node
            } else {
                let prefix = parts[..=depth].join("/");
                tree_node(
                    TreeNodeKind::Category,
                    &format!("breachCategory:{prefix}"),
                    part,
                    &format!("repo://statute/{}", semio_framework_repo_identity::flat(&prefix)),
                )
            };
            entry.children.insert(part.clone(), Entry { node: Some(node), order: Vec::new(), children: BTreeMap::new() });
        }
        let child = entry.children.get_mut(part).expect("child was just inserted");
        insert(child, parts, full, catalog, depth + 1);
    }

    fn collect(entry: &mut Entry, ordered: bool) -> Vec<TreeNode> {
        let keys: Vec<String> = if ordered { entry.order.clone() } else { entry.children.keys().cloned().collect() };
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut result = Vec::new();
        for key in keys {
            if !seen.insert(key.clone()) {
                continue;
            }
            let Some(mut child) = entry.children.remove(&key) else { continue };
            let mut node = child.node.take().expect("every entry carries a node");
            let grandchildren = collect(&mut child, false);
            if !grandchildren.is_empty() {
                node.children = Some(grandchildren);
            }
            result.push(node);
        }
        result
    }

    let mut root = Entry::default();
    for statute in statutes {
        let parts: Vec<String> = statute.0.split('/').map(str::to_string).collect();
        insert(&mut root, &parts, statute, catalog, 0);
    }
    collect(&mut root, true)
}

/// 🔷️ Builds the territory tree: one category per territory, its statutes and nested territories.
pub fn build_territory_tree(territories: &[Territory], catalog: &dyn StatuteCatalog) -> Vec<TreeNode> {
    let territory_emoji = semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::entity("territory"));
    let mut result = Vec::new();
    for territory in territories {
        let mut node = tree_node(
            TreeNodeKind::Category,
            &format!("{territory_emoji}{}", territory.name),
            &territory.name,
            &format!("repo://territory/{}", catalog.territory_id(territory)),
        );
        node.description = territory.description.clone();
        node.sub_kind = "territory".to_string();
        node.data = Some(data_of(vec![
            ("name", text(&territory.name)),
            ("description", text(&territory.description)),
            ("scopes", list(&territory.scopes)),
        ]));
        for statute in &territory.kinds {
            push_child(&mut node, statute_leaf_node(statute, catalog));
        }
        for child in build_territory_tree(&territory.groups, catalog) {
            push_child(&mut node, child);
        }
        result.push(node);
    }
    result
}

/// 📜️ Builds the leaf node of one statute, labelled with its priority icon.
pub fn statute_leaf_node(statute: &Statute, catalog: &dyn StatuteCatalog) -> TreeNode {
    let meta = catalog.info(statute);
    let priority = meta.as_ref().map_or(BreachPriority::Low, |meta| meta.priority);
    let mut node = tree_node(
        TreeNodeKind::Statute,
        &catalog.statute_id(statute),
        &format!("{}{}", priority_icon(priority), catalog.statute_label(statute)),
        &catalog.statute_uri(statute),
    );
    if let Some(meta) = meta {
        node.description = meta.reason.clone();
        if meta.autofixable {
            node.sub_kind = "autofixable".to_string();
        }
        node.data = Some(data_of(vec![
            ("id", text(&statute.0)),
            ("priority", text(meta.priority.as_str())),
            ("autofixable", Value::Bool(meta.autofixable)),
            ("reason", text(&meta.reason)),
            ("solution", text(&meta.solution)),
        ]));
    }
    node
}

/// 🧱️ Groups the statutes of a policy's territories by the entity kind they apply to.
pub fn build_policy_entity_kind_tree(territories: &[Territory], catalog: &dyn StatuteCatalog) -> Vec<TreeNode> {
    fn collect(entries: &[Territory], catalog: &dyn StatuteCatalog, grouped: &mut BTreeMap<String, BTreeSet<String>>) {
        for entry in entries {
            for statute in &entry.kinds {
                grouped.entry(catalog.entity_kind(statute)).or_default().insert(statute.0.clone());
            }
            collect(&entry.groups, catalog, grouped);
        }
    }
    let mut grouped: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    collect(territories, catalog, &mut grouped);
    let mut result = Vec::new();
    for (entity_kind, statutes) in grouped {
        let mut node = tree_node(
            TreeNodeKind::Category,
            &format!("entitykind:{entity_kind}"),
            &entity_kind,
            &format!("repo://entitykind/{entity_kind}"),
        );
        node.sub_kind = "entitykind".to_string();
        node.data = Some(data_of(vec![("kind", text(&entity_kind))]));
        for statute in statutes {
            push_child(&mut node, statute_leaf_node(&Statute(statute), catalog));
        }
        result.push(node);
    }
    result
}

//#endregion 📜️StatuteTree
