// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Stream options and filtering.

// #endregion 🧲️Header

package client

import (
	"context"
	model "github.com/usalu/semio/repo/model"
	search "github.com/usalu/semio/repo/search"
	"regexp"
	"strings"
)

// 🟣️StreamOptions holds the data fields for a stream options record.
type StreamProgress struct {
	Current int
	Phase   string
}

// 📡️StreamOptions controls authored discovery and matching.
type StreamOptions struct {
	Layout         *model.ArtifactLayout `json:"-"`
	Progress       func(StreamProgress)  `json:"-"`
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
func matchesQuery(ctx context.Context, text string, opts StreamOptions) (bool, error) {
	if err := ctx.Err(); err != nil {
		return false, err
	}
	if opts.Query == "" {
		return true, nil
	}
	match := search.NewMatchQuery(opts.Query)
	match.SetFuzziness(2)
	return search.MatchContext(ctx, text, match)
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
