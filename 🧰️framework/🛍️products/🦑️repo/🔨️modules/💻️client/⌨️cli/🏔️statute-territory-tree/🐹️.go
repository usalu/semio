// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Policy entity-kind, statute, and territory tree builders.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"path/filepath"
	"sort"
	"strings"
)

// 📁️buildFolderRoots holds the data fields for a buildFolderRoots record.
func buildFolderRoots(folders []Folder) []*TreeNode {
	type folderEntry struct {
		folder *Folder
		node   *TreeNode
	}
	folderMap := make(map[string]*folderEntry)
	for i := range folders {
		f := &folders[i]
		folderMap[f.Path] = &folderEntry{
			folder: f,
			node: &TreeNode{
				Kind:    TreeNodeFolder,
				ID:      f.GetID(),
				Label:   f.Name,
				URI:     f.GetURI(),
				SubKind: string(f.Kind),
				Data:    map[string]interface{}{"path": f.Path, "name": f.Name, "kind": string(f.Kind)},
			},
		}
	}
	var roots []*TreeNode
	for path, fe := range folderMap {
		parentPath := filepath.Dir(path)
		if parent, ok := folderMap[parentPath]; ok {
			parent.node.Children = append(parent.node.Children, fe.node)
			continue
		}
		roots = append(roots, fe.node)
	}
	return roots
}

func attachFilesToFolders(root *TreeNode, files []File, fileNodes map[string]*TreeNode) {
	folderNodeByPath := make(map[string]*TreeNode)
	var walk func(node *TreeNode)
	walk = func(node *TreeNode) {
		if node.Kind == TreeNodeFolder {
			if path, ok := node.Data["path"].(string); ok && path != "" {
				folderNodeByPath[path] = node
			}
		}
		for _, child := range node.Children {
			walk(child)
		}
	}
	walk(root)
	for i := range files {
		f := &files[i]
		templateNode, ok := fileNodes[f.Path]
		if !ok {
			continue
		}
		fileNode := cloneTreeNode(templateNode)
		folderPath := filepath.Dir(f.Path)
		if folderNode, ok := folderNodeByPath[folderPath]; ok {
			folderNode.Children = append(folderNode.Children, fileNode)
			continue
		}
		root.Children = append(root.Children, fileNode)
	}
}

// 🌳️cloneTreeNode holds the data fields for a cloneTreeNode record.
func cloneTreeNode(node *TreeNode) *TreeNode {
	copyNode := *node
	if node.Data != nil {
		copyNode.Data = make(map[string]interface{}, len(node.Data))
		for key, value := range node.Data {
			copyNode.Data[key] = value
		}
	}
	if len(node.Children) > 0 {
		copyNode.Children = make([]*TreeNode, 0, len(node.Children))
		for _, child := range node.Children {
			copyNode.Children = append(copyNode.Children, cloneTreeNode(child))
		}
	} else {
		copyNode.Children = nil
	}
	return &copyNode
}

func buildPolicyEntityKindTree(groups []Territory) []*TreeNode {
	statutesByEntityKind := make(map[string]map[Statute]bool)
	var collect func(entries []Territory)
	collect = func(entries []Territory) {
		for _, entry := range entries {
			for _, statute := range entry.Kinds {
				entityKind := inferEntityKindFromStatute(statute)
				if statutesByEntityKind[entityKind] == nil {
					statutesByEntityKind[entityKind] = make(map[Statute]bool)
				}
				statutesByEntityKind[entityKind][statute] = true
			}
			if len(entry.Groups) > 0 {
				collect(entry.Groups)
			}
		}
	}
	collect(groups)
	entityKinds := make([]string, 0, len(statutesByEntityKind))
	for entityKind := range statutesByEntityKind {
		entityKinds = append(entityKinds, entityKind)
	}
	sort.Strings(entityKinds)
	result := make([]*TreeNode, 0, len(entityKinds))
	for _, entityKind := range entityKinds {
		entityNode := &TreeNode{
			Kind:    TreeNodeCategory,
			ID:      fmt.Sprintf("entitykind:%s", entityKind),
			Label:   entityKind,
			URI:     "repo://entitykind/" + entityKind,
			SubKind: "entitykind",
			Data: map[string]interface{}{
				"kind": entityKind,
			},
		}
		statutes := make([]Statute, 0, len(statutesByEntityKind[entityKind]))
		for statute := range statutesByEntityKind[entityKind] {
			statutes = append(statutes, statute)
		}
		sort.Slice(statutes, func(i, j int) bool {
			return statutes[i] < statutes[j]
		})
		for _, statute := range statutes {
			meta := statute.Info()
			priorityIcon := "🟢️"
			if meta.Priority == BreachPriorityHigh {
				priorityIcon = "🔴️"
			} else if meta.Priority == BreachPriorityMedium {
				priorityIcon = "🟡️"
			}
			statuteNode := &TreeNode{
				Kind:        TreeNodeStatute,
				ID:          meta.GetID(),
				Label:       priorityIcon + StatutePathToIdValue(string(statute)),
				URI:         meta.GetURI(),
				Description: meta.Reason,
				Data: map[string]interface{}{
					"id":          string(statute),
					"priority":    string(meta.Priority),
					"autofixable": meta.Autofixable,
					"reason":      meta.Reason,
					"solution":    meta.Solution,
				},
			}
			if meta.Autofixable {
				statuteNode.SubKind = "autofixable"
			}
			entityNode.Children = append(entityNode.Children, statuteNode)
		}
		result = append(result, entityNode)
	}
	return result
}

func spansEntireFile(lines map[int]struct{}, content string, lineCount int) bool {
	if len(lines) == 0 {
		return true
	}
	effectiveLineCount := lineCount
	if strings.HasSuffix(content, "\n") && effectiveLineCount > 1 {
		effectiveLineCount--
	}
	if len(lines) < effectiveLineCount {
		return false
	}
	for line := 1; line <= effectiveLineCount; line++ {
		if _, ok := lines[line]; !ok {
			return false
		}
	}
	return true
}

// 🔷️inferEntityKindFromStatute holds the data fields for a inferEntityKindFromStatute record.
func inferEntityKindFromStatute(statute Statute) string {
	path := string(statute)
	switch {
	case strings.Contains(path, "/definition/"):
		return "definition"
	case strings.Contains(path, "/section/"):
		return "section"
	case strings.Contains(path, "/file/"):
		return "file"
	case strings.Contains(path, "/folder/"):
		return "folder"
	case strings.Contains(path, "/bundle/"):
		return "bundle"
	case strings.Contains(path, "/technology/"):
		return "technology"
	default:
		return "repo"
	}
}

// 🏗️buildStatuteTree holds the data fields for a buildStatuteTree record.
func buildStatuteTree(kinds []Statute) []*TreeNode {
	type treeEntry struct {
		node     *TreeNode
		children map[string]*treeEntry
	}
	rootEntries := make(map[string]*treeEntry)
	orderedRootKeys := []string{}

	for _, k := range kinds {
		parts := strings.Split(string(k), "/")
		current := rootEntries
		currentOrdered := &orderedRootKeys
		for i, part := range parts {
			if _, ok := current[part]; !ok {
				isLeaf := i == len(parts)-1
				meta := k.Info()
				n := &TreeNode{
					Kind:  TreeNodeStatute,
					ID:    string(k),
					Label: part,
					URI:   meta.GetURI(),
				}
				if isLeaf {
					n.Description = meta.Reason
					priorityIcon := "🟢️"
					if meta.Priority == BreachPriorityHigh {
						priorityIcon = "🔴️"
					} else if meta.Priority == BreachPriorityMedium {
						priorityIcon = "🟡️"
					}
					n.Label = priorityIcon + part
					if meta.Autofixable {
						n.SubKind = "autofixable"
					}
					n.Data = map[string]interface{}{
						"id":          string(k),
						"priority":    string(meta.Priority),
						"autofixable": meta.Autofixable,
						"reason":      meta.Reason,
						"solution":    meta.Solution,
					}
				} else {
					prefix := strings.Join(parts[:i+1], "/")
					n.ID = "breachCategory:" + prefix
					n.URI = "repo://statute/" + Flat(prefix)
					n.Kind = TreeNodeCategory
				}
				current[part] = &treeEntry{node: n, children: make(map[string]*treeEntry)}
				*currentOrdered = append(*currentOrdered, part)
			}
			entry := current[part]
			current = entry.children
			childKeys := []string{}
			for ck := range entry.children {
				childKeys = append(childKeys, ck)
			}
			currentOrdered = &childKeys
		}
	}

	var buildNodes func(entries map[string]*treeEntry, keys []string) []*TreeNode
	buildNodes = func(entries map[string]*treeEntry, keys []string) []*TreeNode {
		seen := map[string]bool{}
		var result []*TreeNode
		for _, key := range keys {
			if seen[key] {
				continue
			}
			seen[key] = true
			entry := entries[key]
			childKeys := []string{}
			for ck := range entry.children {
				childKeys = append(childKeys, ck)
			}
			sort.Strings(childKeys)
			entry.node.Children = buildNodes(entry.children, childKeys)
			result = append(result, entry.node)
		}
		return result
	}

	return buildNodes(rootEntries, orderedRootKeys)
}

// 🔷️buildTerritoryTree holds the data fields for a buildTerritoryTree record.
func buildTerritoryTree(groups []Territory) []*TreeNode {
	var result []*TreeNode
	for _, g := range groups {
		groupNode := &TreeNode{
			Kind:        TreeNodeCategory,
			ID:          fmt.Sprintf("%s%s", emojiText(EmojiTerritory), g.Name),
			Label:       g.Name,
			URI:         "repo://territory/" + g.GetID(),
			Description: g.Description,
			SubKind:     "territory",
			Data: map[string]interface{}{
				"name":        g.Name,
				"description": g.Description,
				"scopes":      g.Scopes,
			},
		}
		for _, k := range g.Kinds {
			meta := k.Info()
			priorityIcon := "🟢️"
			if meta.Priority == BreachPriorityHigh {
				priorityIcon = "🔴️"
			} else if meta.Priority == BreachPriorityMedium {
				priorityIcon = "🟡️"
			}
			label := priorityIcon + StatutePathToIdValue(string(k))
			vkNode := &TreeNode{
				Kind:        TreeNodeStatute,
				ID:          meta.GetID(),
				Label:       label,
				URI:         meta.GetURI(),
				Description: meta.Reason,
				Data: map[string]interface{}{
					"id":          string(k),
					"priority":    string(meta.Priority),
					"autofixable": meta.Autofixable,
					"reason":      meta.Reason,
					"solution":    meta.Solution,
				},
			}
			if meta.Autofixable {
				vkNode.SubKind = "autofixable"
			}
			groupNode.Children = append(groupNode.Children, vkNode)
		}
		groupNode.Children = append(groupNode.Children, buildTerritoryTree(g.Groups)...)
		result = append(result, groupNode)
	}
	return result
}
