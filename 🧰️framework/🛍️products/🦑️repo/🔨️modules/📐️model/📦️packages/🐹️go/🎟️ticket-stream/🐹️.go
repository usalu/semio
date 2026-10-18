// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎟️ticket_stream carries the ticket directory formatting and the stream filter matching.

// #endregion 🧲️Header

package model

import (
	regexp "regexp"
	strings "strings"

	search "github.com/usalu/semio/repo/search"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🎟️Ticket Stream

// 🎆️FormatYearDir returns the emoji-prefixed year directory segment.
func FormatYearDir(year int) string {
	return EmojiYear + workspace.PadNumber(year, 2)
}

// 🌙️FormatMonthDir returns the emoji-prefixed month directory segment.
func FormatMonthDir(month int) string {
	return EmojiMonth + workspace.PadNumber(month, 2)
}

// ☀️FormatDayDir returns the emoji-prefixed day directory segment.
func FormatDayDir(day int) string {
	return EmojiDay + workspace.PadNumber(day, 2)
}

// 🎫️FormatTicketRelPath returns YY/MM/DD/SLUG with emoji date segments.
func FormatTicketRelPath(year, month, day int, slug string) string {
	return FormatYearDir(year) + "/" + FormatMonthDir(month) + "/" + FormatDayDir(day) + "/" + slug
}

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
func MatchesFilter(name string, opts StreamOptions) bool {
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
func MatchesQuery(text string, opts StreamOptions) bool {
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

// #endregion 🎟️Ticket Stream
