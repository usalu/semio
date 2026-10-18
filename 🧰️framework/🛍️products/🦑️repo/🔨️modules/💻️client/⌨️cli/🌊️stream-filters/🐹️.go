// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Stream options and filtering.

// #endregion 🧲️Header

package client

import (
	"fmt"
	search "github.com/usalu/semio/repo/search"
	"regexp"
	"strconv"
	"strings"
)

// 🟣️StreamOptions holds the data fields for a stream options record.
type StreamOptions struct {
	ShowIgnored    bool
	ShowGenerated  bool
	ExcludeKinds   []string
	IncludeKinds   []string
	Filter         string
	Query          string
	Regex          bool
	MatchCase      bool
	MatchWholeWord bool

	ExcludeBundleKinds     []BundleKind
	IncludeBundleKinds     []BundleKind
	ExcludeFolderKinds     []FolderKind
	IncludeFolderKinds     []FolderKind
	ExcludeDefinitionKinds []DefinitionKind
	IncludeDefinitionKinds []DefinitionKind

	ExcludeYears        []int
	IncludeYears        []int
	ExcludeMonths       []int
	IncludeMonths       []int
	ExcludeDays         []int
	IncludeDays         []int
	ExcludeContributors []string
	IncludeContributors []string
	ExcludePolicies     []string
	IncludePolicies     []string
	ExcludeBreachs      []string
	IncludeBreachs      []string
}

// 🟤️matchesFilter holds the data fields for a matchesFilter record.
func matchesFilter(name string, opts StreamOptions) bool {
	if opts.Filter == "" {
		return true
	}

	target := name
	pattern := opts.Filter

	if !opts.MatchCase && !opts.Regex {
		target = strings.ToLower(target)
		pattern = strings.ToLower(pattern)
	}

	if opts.Regex {
		re, err := regexp.Compile(opts.Filter)
		if err != nil {
			return false
		}
		return re.MatchString(name)
	}

	if opts.MatchWholeWord {
		words := strings.FieldsFunc(target, func(r rune) bool {
			return !((r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || (r >= '0' && r <= '9') || r == '_')
		})
		for _, w := range words {
			if w == pattern {
				return true
			}
		}
		return false
	}

	return strings.Contains(target, pattern)
}

// 🔍️matchesQuery holds the data fields for a matchesQuery record.
func matchesQuery(text string, opts StreamOptions) bool {
	if opts.Query == "" {
		return true
	}
	lowerText := strings.ToLower(text)
	lowerQuery := strings.ToLower(opts.Query)
	if strings.Contains(lowerText, lowerQuery) {
		return true
	}
	mapping := search.NewIndexMapping()
	index, err := search.NewMemOnly(mapping)
	if err != nil {
		return true
	}
	defer index.Close()
	doc := map[string]interface{}{"text": text}
	index.Index("item", doc)
	mq := search.NewMatchQuery(opts.Query)
	mq.SetFuzziness(2)
	searchRequest := search.NewSearchRequest(mq)
	searchRequest.Size = 1
	results, err := index.Search(searchRequest)
	if err != nil {
		return true
	}
	return results.Total > 0
}

// 🔖️queryableItem holds the data fields for a queryableItem record.
type queryableItem struct {
	text string
	idx  int
}

// ⚪️searchFilterItems returns the matching item positions.
func searchFilterItems(items []queryableItem, query string) map[int]bool {
	result := make(map[int]bool)
	if query == "" {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	mapping := search.NewIndexMapping()
	index, err := search.NewMemOnly(mapping)
	if err != nil {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	defer index.Close()
	for _, item := range items {
		doc := map[string]interface{}{"text": item.text}
		index.Index(fmt.Sprintf("%d", item.idx), doc)
	}
	mq := search.NewMatchQuery(query)
	mq.SetFuzziness(2)
	searchRequest := search.NewSearchRequest(mq)
	searchRequest.Size = len(items)
	results, err := index.Search(searchRequest)
	if err != nil {
		for _, item := range items {
			result[item.idx] = true
		}
		return result
	}
	for _, hit := range results.Hits {
		idx, err := strconv.Atoi(hit.ID)
		if err == nil {
			result[idx] = true
		}
	}
	return result
}

// ⚫️shouldIncludeKind holds the data fields for a shouldIncludeKind record.
func shouldIncludeKind(kind string, opts StreamOptions) bool {
	if len(opts.IncludeKinds) > 0 {
		found := false
		for _, k := range opts.IncludeKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩵️shouldIncludeBundleKind holds the data fields for a shouldIncludeBundleKind record.
func shouldIncludeBundleKind(kind BundleKind, opts StreamOptions) bool {
	if len(opts.IncludeBundleKinds) > 0 {
		found := false
		for _, k := range opts.IncludeBundleKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeBundleKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 📁️shouldIncludeFolderKind holds the data fields for a shouldIncludeFolderKind record.
func shouldIncludeFolderKind(kind FolderKind, opts StreamOptions) bool {
	if len(opts.IncludeFolderKinds) > 0 {
		found := false
		for _, k := range opts.IncludeFolderKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeFolderKinds {
		if k == kind {
			return false
		}
	}
	return true
}

// 🩶️shouldIncludeDefinitionKind holds the data fields for a shouldIncludeDefinitionKind record.
func shouldIncludeDefinitionKind(kind DefinitionKind, opts StreamOptions) bool {
	if len(opts.IncludeDefinitionKinds) > 0 {
		found := false
		for _, k := range opts.IncludeDefinitionKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeDefinitionKinds {
		if k == kind {
			return false
		}
	}
	return true
}
