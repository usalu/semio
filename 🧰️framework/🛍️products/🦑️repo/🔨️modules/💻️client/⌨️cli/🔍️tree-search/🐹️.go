// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Fuzzy in-memory and cached tree search.

// #endregion 🧲️Header

package client

import (
	"context"
	"errors"
	"fmt"
	search "github.com/usalu/semio/repo/search"
	"strings"
)

func searchMonorepoTreeWithCache(ctx context.Context, root *TreeNode, query string) (*TreeNode, error) {
	if query == "" {
		return root, nil
	}
	idx, err := ensureCacheIndexed(ctx, root, nil)
	if err != nil {
		return nil, err
	}

	searchable, err := countSearchableTreeNodes(ctx, root)
	if err != nil {
		_ = idx.Close()
		return nil, err
	}
	matchedIDs, err := queryCacheIndex(ctx, idx, query, searchable, nil)
	if err != nil {
		_ = idx.Close()
		return nil, err
	}
	if err := idx.Close(); err != nil {
		return nil, err
	}
	if len(matchedIDs) == 0 {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}, nil
	}
	pruned, err := pruneUnmatchedContext(ctx, root, sliceToSet(matchedIDs))
	if err != nil {
		return nil, err
	}
	if pruned == nil {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}, nil
	}
	return pruned, nil
}

// 📝️SearchMonorepoTree MUST match case-insensitively against node labels and descriptions.
// 🔤️SearchMonorepoTree performs a text search across the monorepo tree.
func SearchMonorepoTree(root *TreeNode, query string) *TreeNode {
	if query == "" {
		return root
	}
	return searchTreeInMemory(root, query)
}

func countSearchableTreeNodes(ctx context.Context, root *TreeNode) (int, error) {
	if root == nil {
		return 0, errors.New("search tree is required")
	}
	count := 0
	visited := 0
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		if err := ctx.Err(); err != nil {
			return 0, err
		}
		last := len(stack) - 1
		node := stack[last]
		stack = stack[:last]
		visited++
		if visited > search.MaxTraversalNodes {
			return 0, fmt.Errorf("%w: tree nodes > %d", search.ErrTooLarge, search.MaxTraversalNodes)
		}
		if node.Kind != TreeNodeCategory {
			count++
			if count > search.MaxDocuments {
				return 0, fmt.Errorf("%w: tree documents > %d", search.ErrTooLarge, search.MaxDocuments)
			}
		}
		stack = append(stack, node.Children...)
	}
	if count == 0 {
		return 1, nil
	}
	return count, nil
}

func sliceToSet(values []string) map[string]bool {
	result := make(map[string]bool, len(values))
	for _, value := range values {
		result[value] = true
	}
	return result
}

func serializeTreeNodeData(data map[string]interface{}) string {
	if len(data) == 0 {
		return ""
	}
	parts := make([]string, 0, len(data))
	for _, value := range data {
		switch typed := value.(type) {
		case string:
			parts = append(parts, typed)
		case []string:
			parts = append(parts, strings.Join(typed, " "))
		case []interface{}:
			var values []string
			for _, item := range typed {
				values = append(values, fmt.Sprint(item))
			}
			parts = append(parts, strings.Join(values, " "))
		default:
			parts = append(parts, fmt.Sprint(typed))
		}
	}
	return strings.Join(parts, " ")
}

func buildSearchDocumentText(node *TreeNode) string {
	parts := []string{
		node.Label,
		node.ID,
		node.SubKind,
		node.Description,
		node.URI,
		node.Status,
		node.Contributor,
		string(node.Kind),
		serializeTreeNodeData(node.Data),
	}
	for _, child := range node.Children {
		if child.Kind != TreeNodeCategory {
			parts = append(parts, child.Label, child.ID, child.Description, serializeTreeNodeData(child.Data))
		}
	}
	return strings.Join(parts, " ")
}

// 🔶️levenshtein holds the data fields for a levenshtein record.
func levenshtein(a, b string) int {
	la, lb := len(a), len(b)
	if la == 0 {
		return lb
	}
	if lb == 0 {
		return la
	}
	prev := make([]int, lb+1)
	curr := make([]int, lb+1)
	for j := 0; j <= lb; j++ {
		prev[j] = j
	}
	for i := 1; i <= la; i++ {
		curr[0] = i
		for j := 1; j <= lb; j++ {
			cost := 0
			if a[i-1] != b[j-1] {
				cost = 1
			}
			ins := curr[j-1] + 1
			del := prev[j] + 1
			sub := prev[j-1] + cost
			m := ins
			if del < m {
				m = del
			}
			if sub < m {
				m = sub
			}
			curr[j] = m
		}
		prev, curr = curr, prev
	}
	return prev[lb]
}

// 🔹️fuzzyContains holds the data fields for a fuzzyContains record.
func fuzzyContains(text, term string) bool {
	if strings.Contains(text, term) {
		return true
	}
	maxDist := 1
	if len(term) > 5 {
		maxDist = 2
	}
	words := strings.Fields(text)
	for _, w := range words {
		if levenshtein(w, term) <= maxDist {
			return true
		}
		if len(w) >= len(term) {
			for i := 0; i <= len(w)-len(term)+maxDist && i <= len(w)-1; i++ {
				end := i + len(term) + maxDist
				if end > len(w) {
					end = len(w)
				}
				sub := w[i:end]
				if levenshtein(sub, term) <= maxDist {
					return true
				}
			}
		}
	}
	return false
}

// 🔎️searchTreeInMemory holds the data fields for a searchTreeInMemory record.
func searchTreeInMemory(root *TreeNode, query string) *TreeNode {
	terms := strings.Fields(strings.ToLower(query))
	if len(terms) == 0 {
		return root
	}
	matchedIDs := make(map[string]bool)
	var walk func(node *TreeNode)
	walk = func(node *TreeNode) {
		if node.Kind != TreeNodeCategory {
			docID := node.ID
			if docID == "" {
				docID = node.Label
			}
			text := strings.ToLower(buildSearchDocumentText(node))
			allMatch := true
			for _, term := range terms {
				if !fuzzyContains(text, term) {
					allMatch = false
					break
				}
			}
			if allMatch {
				matchedIDs[docID] = true
			}
		}
		for _, c := range node.Children {
			walk(c)
		}
	}
	walk(root)
	if len(matchedIDs) == 0 {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
	}
	pruned := pruneUnmatched(root, matchedIDs)
	if pruned == nil {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
	}
	return pruned
}

// 🎯️pruneUnmatched holds the data fields for a pruneUnmatched record.
func pruneUnmatched(node *TreeNode, matchedIDs map[string]bool) *TreeNode {
	result, _ := pruneUnmatchedContext(context.Background(), node, matchedIDs)
	return result
}

func pruneUnmatchedContext(ctx context.Context, node *TreeNode, matchedIDs map[string]bool) (*TreeNode, error) {
	return pruneUnmatchedInner(ctx, node, matchedIDs, false)
}

// 🔸️pruneUnmatchedInner holds the data fields for a pruneUnmatchedInner record.
func pruneUnmatchedInner(ctx context.Context, node *TreeNode, matchedIDs map[string]bool, ancestorMatched bool) (*TreeNode, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	docID := node.ID
	if docID == "" {
		docID = node.Label
	}
	selfMatched := matchedIDs[docID]
	keepDescendants := ancestorMatched || selfMatched
	var kept []*TreeNode
	for _, c := range node.Children {
		fc, err := pruneUnmatchedInner(ctx, c, matchedIDs, keepDescendants)
		if err != nil {
			return nil, err
		}
		if fc != nil {
			kept = append(kept, fc)
		}
	}
	if keepDescendants || len(kept) > 0 || node.Kind == TreeNodeCategory {
		if node.Kind == TreeNodeCategory && len(kept) == 0 && !selfMatched && !ancestorMatched {
			return nil, nil
		}
		copy := *node
		copy.Children = kept
		return &copy, nil
	}
	return nil, nil
}
