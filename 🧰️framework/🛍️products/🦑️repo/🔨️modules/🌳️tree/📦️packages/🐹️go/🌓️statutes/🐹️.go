// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📜️statutes carries the ported statute and territory tree builders.

// #endregion 🧲️Header

package tree

import (
	sort "sort"
	strings "strings"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	statutespkg "github.com/usalu/semio/repo/statutes"
)

// #region 📜️PortedStatuteTree

// 📜️DeclaredStatuteCatalog is the catalog of the law this repository actually declares, resolved
// through 📜️statutes — the production StatuteCatalog every consumer of the statute and territory
// trees shares.
type DeclaredStatuteCatalog struct{}

// ℹInfo MUST return the declared metadata of the statute, and report whether it is declared.
func (DeclaredStatuteCatalog) Info(statute model.Statute) (model.StatuteMeta, bool) {
	meta, declared := model.StatuteInfoTable[statute]
	return meta, declared
}

// 🆔️StatuteID MUST return the artifact id of the statute.
func (DeclaredStatuteCatalog) StatuteID(statute model.Statute) string {
	return "📜️" + strings.NewReplacer("/", "", ".", "").Replace(string(statute))
}

// 🔗️StatuteURI MUST return the repo uri of the statute.
func (DeclaredStatuteCatalog) StatuteURI(statute model.Statute) string {
	return "repo://statute/" + string(statute)
}

// 🏷️StatuteLabel MUST return the last path segment of the statute.
func (DeclaredStatuteCatalog) StatuteLabel(statute model.Statute) string {
	parts := strings.Split(string(statute), "/")
	return parts[len(parts)-1]
}

// 🧱️EntityKind MUST return the first path segment of the statute.
func (DeclaredStatuteCatalog) EntityKind(statute model.Statute) string {
	return strings.Split(string(statute), "/")[0]
}

// 🆔️TerritoryID MUST return the artifact id of the territory.
func (DeclaredStatuteCatalog) TerritoryID(territory model.Territory) string {
	return "🗺️" + strings.NewReplacer(" ", "", "/", "").Replace(territory.Name)
}

// 📜️DeclaredStatutes returns every declared statute, identifier ascending, so the projection never
// depends on the order a map happened to iterate in.
func DeclaredStatutes() []model.Statute {
	kinds := make([]model.Statute, 0, len(model.StatuteInfoTable))
	for kind := range model.StatuteInfoTable {
		kinds = append(kinds, kind)
	}
	sort.Slice(kinds, func(i, j int) bool { return kinds[i] < kinds[j] })
	return kinds
}

// 🗺️DeclaredTerritories returns every territory a declared policy carries at its top level, name
// ascending, each name claimed by the first policy that declares it.
func DeclaredTerritories() []model.Territory {
	seen := map[string]model.Territory{}
	names := []string{}
	for _, policy := range statutespkg.GetPolicies() {
		for _, territory := range policy.Groups {
			if _, claimed := seen[territory.Name]; claimed {
				continue
			}
			seen[territory.Name] = territory
			names = append(names, territory.Name)
		}
	}
	sort.Strings(names)
	territories := make([]model.Territory, 0, len(names))
	for _, name := range names {
		territories = append(territories, seen[name])
	}
	return territories
}

// 🏗️BuildStatuteTree MUST build one category per path segment with statutes as leaves.
func BuildStatuteTree(statutes []model.Statute, catalog StatuteCatalog) []*TreeNode {
	type entry struct {
		node     *TreeNode
		order    []string
		children map[string]*entry
	}
	newEntry := func(node *TreeNode) *entry {
		return &entry{node: node, children: map[string]*entry{}}
	}
	var insert func(current *entry, parts []string, full model.Statute, depth int)
	insert = func(current *entry, parts []string, full model.Statute, depth int) {
		if depth >= len(parts) {
			return
		}
		part := parts[depth]
		if _, ok := current.children[part]; !ok {
			current.order = append(current.order, part)
			var node *TreeNode
			if depth == len(parts)-1 {
				meta, found := catalog.Info(full)
				priority := model.BreachPriorityLow
				if found {
					priority = meta.Priority
				}
				node = NewTreeNode(TreeNodeStatute, string(full), PriorityIcon(priority)+part, catalog.StatuteURI(full))
				if found {
					node.Description = meta.Reason
					if meta.Autofixable {
						node.SubKind = "autofixable"
					}
					node.Data = dataOf("id", string(full), "priority", string(meta.Priority), "autofixable", meta.Autofixable, "reason", meta.Reason, "solution", meta.Solution)
				}
			} else {
				prefix := strings.Join(parts[:depth+1], "/")
				node = NewTreeNode(TreeNodeCategory, "breachCategory:"+prefix, part, "repo://statute/"+identity.Flat(prefix))
			}
			current.children[part] = newEntry(node)
		}
		insert(current.children[part], parts, full, depth+1)
	}
	var collect func(current *entry, ordered bool) []*TreeNode
	collect = func(current *entry, ordered bool) []*TreeNode {
		keys := current.order
		if !ordered {
			keys = make([]string, 0, len(current.children))
			for key := range current.children {
				keys = append(keys, key)
			}
			sort.Strings(keys)
		}
		seen := map[string]bool{}
		result := []*TreeNode{}
		for _, key := range keys {
			if seen[key] {
				continue
			}
			seen[key] = true
			child, ok := current.children[key]
			if !ok {
				continue
			}
			delete(current.children, key)
			node := child.node
			if grandchildren := collect(child, false); len(grandchildren) > 0 {
				node.Children = grandchildren
			}
			result = append(result, node)
		}
		return result
	}
	root := newEntry(nil)
	for _, statute := range statutes {
		insert(root, strings.Split(string(statute), "/"), statute, 0)
	}
	return collect(root, true)
}

// 🔷️BuildTerritoryTree MUST build one category per territory with its statutes and children.
func BuildTerritoryTree(territories []model.Territory, catalog StatuteCatalog) []*TreeNode {
	territoryEmoji := identity.EmojiText(identity.Entity("territory"))
	result := []*TreeNode{}
	for _, territory := range territories {
		node := NewTreeNode(TreeNodeCategory, territoryEmoji+territory.Name, territory.Name, "repo://territory/"+catalog.TerritoryID(territory))
		node.Description = territory.Description
		node.SubKind = "territory"
		node.Data = dataOf("name", territory.Name, "description", territory.Description, "scopes", list(territory.Scopes))
		for _, statute := range territory.Kinds {
			PushChild(node, StatuteLeafNode(statute, catalog))
		}
		for _, child := range BuildTerritoryTree(territory.Groups, catalog) {
			PushChild(node, child)
		}
		result = append(result, node)
	}
	return result
}

// 📜️StatuteLeafNode MUST build the leaf node of one statute, labelled with its priority icon.
func StatuteLeafNode(statute model.Statute, catalog StatuteCatalog) *TreeNode {
	meta, found := catalog.Info(statute)
	priority := model.BreachPriorityLow
	if found {
		priority = meta.Priority
	}
	node := NewTreeNode(TreeNodeStatute, catalog.StatuteID(statute), PriorityIcon(priority)+catalog.StatuteLabel(statute), catalog.StatuteURI(statute))
	if found {
		node.Description = meta.Reason
		if meta.Autofixable {
			node.SubKind = "autofixable"
		}
		node.Data = dataOf("id", string(statute), "priority", string(meta.Priority), "autofixable", meta.Autofixable, "reason", meta.Reason, "solution", meta.Solution)
	}
	return node
}

// 🧱️BuildPolicyEntityKindTree MUST group the statutes of a policy's territories by entity kind.
func BuildPolicyEntityKindTree(territories []model.Territory, catalog StatuteCatalog) []*TreeNode {
	grouped := map[string]map[string]bool{}
	var collect func(entries []model.Territory)
	collect = func(entries []model.Territory) {
		for _, entry := range entries {
			for _, statute := range entry.Kinds {
				kind := catalog.EntityKind(statute)
				if grouped[kind] == nil {
					grouped[kind] = map[string]bool{}
				}
				grouped[kind][string(statute)] = true
			}
			collect(entry.Groups)
		}
	}
	collect(territories)
	kinds := make([]string, 0, len(grouped))
	for kind := range grouped {
		kinds = append(kinds, kind)
	}
	sort.Strings(kinds)
	result := []*TreeNode{}
	for _, kind := range kinds {
		node := NewTreeNode(TreeNodeCategory, "entitykind:"+kind, kind, "repo://entitykind/"+kind)
		node.SubKind = "entitykind"
		node.Data = dataOf("kind", kind)
		statutes := make([]string, 0, len(grouped[kind]))
		for statute := range grouped[kind] {
			statutes = append(statutes, statute)
		}
		sort.Strings(statutes)
		for _, statute := range statutes {
			PushChild(node, StatuteLeafNode(model.Statute(statute), catalog))
		}
		result = append(result, node)
	}
	return result
}

// #endregion 📜️PortedStatuteTree
