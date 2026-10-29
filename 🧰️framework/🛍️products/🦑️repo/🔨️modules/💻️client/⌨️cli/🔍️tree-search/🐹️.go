// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔍️search carries tree sorting, filtering and the in-memory fuzzy search with pruning.

// #endregion 🧲️Header

package client

import (
	context "context"
	json "encoding/json"
	errors "errors"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"

	search "github.com/usalu/semio/repo/search"
)

type TreeSearchOptions struct {
	Scope    string
	Limit    int
	Filter   *TreeFilter
	Progress func(search.Progress)
}

// 🔎️ Searches a streamed generation while retaining a bounded ranked page and its tree context.
func SearchMonorepoTreeWithOptions(ctx context.Context, root *TreeNode, query string, options TreeSearchOptions) (*TreeNode, error) {
	if query == "" {
		return root, nil
	}
	if options.Limit < 1 || options.Limit > search.MaxQueryResults {
		return nil, fmt.Errorf("%w: search result limit must be between 1 and %d", search.ErrTooLarge, search.MaxQueryResults)
	}
	idx, err := ensureCacheIndexed(ctx, root, options.Scope, options.Progress)
	if err != nil {
		return nil, err
	}
	return queryTreeSearchIndex(ctx, root, query, options, idx)
}

func queryTreeSearchIndex(ctx context.Context, root *TreeNode, query string, options TreeSearchOptions, idx search.Index) (resultTree *TreeNode, failure error) {
	defer func() { failure = errors.Join(failure, idx.Close()) }()
	req := newTreeSearchRequest(query, options.Limit)
	if options.Filter != nil {
		filter := options.Filter
		encoded, encodeErr := json.Marshal(filter)
		if encodeErr != nil {
			return nil, encodeErr
		}
		req.FilterKey = string(encoded)
		req.Accept = func(id string, attributes map[string]string) bool {
			kind := TreeNodeKind(attributes["kind"])
			year, _ := strconv.Atoi(attributes["year"])
			month, _ := strconv.Atoi(attributes["month"])
			day, _ := strconv.Atoi(attributes["day"])
			return filter.IsKindVisible(kind) && filter.MatchesSubKind(kind, attributes["subkind"]) && (year == 0 || filter.MatchesDate(year, month, day)) && (attributes["status"] == "" || filter.MatchesStatus(attributes["status"])) && (attributes["contributor"] == "" || filter.MatchesContributor(attributes["contributor"]))
		}
	}
	result, err := idx.SearchContext(ctx, req, func(current, total int) {
		reportSearchProgress(options.Progress, search.Progress{Current: current, Total: total, Step: "searching"})
	})
	if err != nil {
		return nil, err
	}
	ids := make(map[string]bool, len(result.Hits))
	for _, hit := range result.Hits {
		ids[hit.ID] = true
	}
	pruned, err := pruneUnmatchedContext(ctx, root, ids, options.Progress)
	if err != nil {
		return nil, err
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if pruned == nil {
		return &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}, nil
	}
	return pruned, nil
}

func searchMonorepoTreeWithCache(ctx context.Context, root *TreeNode, query string) (*TreeNode, error) {
	return SearchMonorepoTreeWithOptions(ctx, root, query, TreeSearchOptions{Limit: search.DefaultTreeHits})
}

func newTreeSearchRequest(query string, limit int) *search.SearchRequest {
	terms := strings.Fields(query)
	queries := make([]search.Query, 0, len(terms))
	for _, term := range terms {
		match := search.NewMatchQuery(term)
		match.SetFuzziness(2)
		queries = append(queries, match)
	}
	request := search.NewSearchRequest(search.NewConjunctionQuery(queries...))
	request.Size = limit
	return request
}

// 🌱️ Builds a fresh bounded projection for an arbitrary tree without sharing a repository cache key.
func SearchTreeContext(ctx context.Context, root *TreeNode, query string, options TreeSearchOptions) (result *TreeNode, failure error) {
	if query == "" {
		return root, nil
	}
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if options.Limit < 1 || options.Limit > search.MaxQueryResults {
		return nil, fmt.Errorf("%w: search result limit", search.ErrTooLarge)
	}
	parent, err := filepath.EvalSymlinks(os.TempDir())
	if err != nil {
		return nil, err
	}
	temporary, err := os.MkdirTemp(parent, "semio-search-")
	if err != nil {
		return nil, err
	}
	defer func() { failure = errors.Join(failure, os.RemoveAll(temporary)) }()
	path := filepath.Join(temporary, "index")
	writer, err := search.NewSharded(path, "transient-tree")
	if err != nil {
		return nil, err
	}
	defer func() { failure = errors.Join(failure, writer.Abort()) }()
	if _, err = streamTreeSearchIndex(ctx, root, writer, options.Progress); err != nil {
		return nil, err
	}
	if err = writer.CloseContext(ctx, options.Progress); err != nil {
		return nil, err
	}
	reader, err := search.OpenShardedContext(ctx, path, options.Progress)
	if err != nil {
		return nil, err
	}
	return queryTreeSearchIndex(ctx, root, query, options, reader)
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
		search.FieldText(node.Data),
	}
	for _, child := range node.Children {
		if child.Kind != TreeNodeCategory {
			parts = append(parts, child.Label, child.ID, child.Description, search.FieldText(child.Data))
		}
	}
	return strings.Join(parts, " ")
}

func pruneUnmatchedContext(ctx context.Context, root *TreeNode, matchedIDs map[string]bool, progress func(search.Progress)) (*TreeNode, error) {
	total := 0
	if err := walkSearchTree(ctx, root, func(*TreeNode, int) error {
		total++
		reportSearchProgress(progress, search.Progress{Current: total, Step: "pruning-prepare"})
		return ctx.Err()
	}); err != nil {
		return nil, err
	}
	nodes, bytes, visited := 0, 0, 0
	var prune func(*TreeNode, bool, int) (*TreeNode, error)
	prune = func(node *TreeNode, ancestorMatched bool, depth int) (*TreeNode, error) {
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		visited++
		reportSearchProgress(progress, search.Progress{Current: visited, Total: total, Step: "pruning"})
		if err := ctx.Err(); err != nil {
			return nil, err
		}
		if depth > search.MaxTraversalDepth {
			return nil, fmt.Errorf("%w: result tree depth", search.ErrTooLarge)
		}
		id := node.ID
		if id == "" {
			id = node.Label
		}
		self := matchedIDs[id]
		keep := ancestorMatched || self
		var children []*TreeNode
		for _, child := range node.Children {
			found, err := prune(child, keep, depth+1)
			if err != nil {
				return nil, err
			}
			if found != nil {
				children = append(children, found)
			}
		}
		if !keep && len(children) == 0 {
			return nil, nil
		}
		copy := *node
		copy.Children = nil
		encoded, err := json.Marshal(&copy)
		if err != nil {
			return nil, err
		}
		nodes++
		bytes += len(encoded) + 1
		if nodes > search.MaxResultTreeNodes || bytes > search.MaxResultTreeBytes {
			return nil, fmt.Errorf("%w: expanded search result exceeds node or byte budget; narrow the query", search.ErrTooLarge)
		}
		copy.Children = children
		return &copy, nil
	}
	return prune(root, false, 1)
}

// #endregion 🩻️Monorepo Tree
