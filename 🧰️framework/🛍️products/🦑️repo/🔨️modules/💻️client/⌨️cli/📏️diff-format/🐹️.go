// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Diff and path formatting helpers.

// #endregion 🧲️Header

package client

import (
	"fmt"
	"sort"
	"strings"
)

// ❤️ResolveBundleForPath MUST return the resolved value or an error if unresolvable.
// 📩️ResolveBundleForPath resolves and returns the bundle for path.
func ResolveBundleForPath(filePath string, bundles []Bundle) string {
	var bestMatch string
	var maxLen int
	for _, b := range bundles {
		if b.Name == "" {
			continue
		}
		if strings.HasPrefix(filePath, b.Root+"/") || filePath == b.Root {
			if len(b.Root) > maxLen {
				maxLen = len(b.Root)
				bestMatch = normalizeBundleLabel(b.Name)
			}
		}
	}
	return bestMatch
}

// 🤍️formatLineMetrics holds the data fields for a formatLineMetrics record.
func formatLineMetrics(metrics *LineMetrics) string {
	if metrics == nil {
		return ""
	}
	var parts []string
	if metrics.Removed > 0 {
		parts = append(parts, fmt.Sprintf("-%d", metrics.Removed))
	}
	if metrics.Added > 0 {
		parts = append(parts, fmt.Sprintf("+%d", metrics.Added))
	}
	if len(parts) == 0 {
		return ""
	}
	return " " + strings.Join(parts, " ")
}

// 🖤️formatPathWithBundle holds the data fields for a formatPathWithBundle record.
func formatPathWithBundle(path string, bundles []Bundle) string {
	bundleName := ResolveBundleForPath(path, bundles)
	if bundleName == "" {
		return path
	}
	bundleLabel := normalizeBundleLabel(bundleName)
	root := ""
	for _, bundle := range bundles {
		if normalizeBundleLabel(bundle.Name) == bundleLabel || bundle.Name == bundleName {
			root = bundle.Root
			break
		}
	}
	if root != "" {
		relative := strings.TrimPrefix(path, root+"/")
		if relative != path {
			if relative == "" {
				return bundleLabel
			}
			return bundleLabel + "/" + relative
		}
	}
	return bundleLabel + "/" + path
}

// 🤎️formatSemanticPath holds the data fields for a formatSemanticPath record.
func formatSemanticPath(path string, bundles []Bundle) string {
	filePath := extractFilePrefix(path)
	remainder := path[len(filePath):]
	return formatPathWithBundle(filePath, bundles) + remainder
}

// 💗️formatDeletedPath holds the data fields for a formatDeletedPath record.
func formatDeletedPath(path string, bundles []Bundle) string {
	filePath := extractFilePrefix(path)
	remainder := path[len(filePath):]
	base := formatPathWithBundle(filePath, bundles)
	if remainder == "" {
		return "<del>" + base + "</del>"
	}
	return base + "<del>" + remainder + "</del>"
}

// 💖️commonPrefixLength holds the data fields for a commonPrefixLength record.
func commonPrefixLength(a, b string) int {
	limit := len(a)
	if len(b) < limit {
		limit = len(b)
	}
	idx := 0
	for idx < limit {
		if a[idx] != b[idx] {
			break
		}
		idx++
	}
	return idx
}

// 💝️commonSuffixLength holds the data fields for a commonSuffixLength record.
func commonSuffixLength(a, b string, prefix int) int {
	max := len(a) - prefix
	if len(b)-prefix < max {
		max = len(b) - prefix
	}
	idx := 0
	for idx < max {
		if a[len(a)-1-idx] != b[len(b)-1-idx] {
			break
		}
		idx++
	}
	return idx
}

// 💘️formatRenameDelta holds the data fields for a formatRenameDelta record.
func formatRenameDelta(from, to string) string {
	if from == to {
		return from
	}
	prefix := commonPrefixLength(from, to)
	suffix := commonSuffixLength(from, to, prefix)
	fromMiddle := from[prefix : len(from)-suffix]
	toMiddle := to[prefix : len(to)-suffix]
	return from[:prefix] + "<del>" + fromMiddle + "</del>" + toMiddle + from[len(from)-suffix:]
}

func formatRenamePath(from, to string, bundles []Bundle) string {
	fromFormatted := formatSemanticPath(from, bundles)
	toFormatted := formatSemanticPath(to, bundles)
	return formatRenameDelta(fromFormatted, toFormatted)
}

// ➡️appendDiffLines holds the data fields for a appendDiffLines record.
func appendDiffLines(lines *[]string, diffSet TicketDiffSet, iconAdded, iconChanged, iconRemoved, iconRenamed string, bundles []Bundle, formatter func(string) string, renameFormatter func(string, string) string) {
	if len(diffSet.Added) > 0 {
		sort.Slice(diffSet.Added, func(i, j int) bool { return diffSet.Added[i].Path < diffSet.Added[j].Path })
		for _, entry := range diffSet.Added {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconAdded, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Modified) > 0 {
		sort.Slice(diffSet.Modified, func(i, j int) bool { return diffSet.Modified[i].Path < diffSet.Modified[j].Path })
		for _, entry := range diffSet.Modified {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconChanged, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Renamed) > 0 {
		sort.Slice(diffSet.Renamed, func(i, j int) bool { return diffSet.Renamed[i].To < diffSet.Renamed[j].To })
		for _, entry := range diffSet.Renamed {
			path := renameFormatter(entry.From, entry.To)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRenamed, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Deleted) > 0 {
		sort.Slice(diffSet.Deleted, func(i, j int) bool { return diffSet.Deleted[i].Path < diffSet.Deleted[j].Path })
		for _, entry := range diffSet.Deleted {
			path := formatDeletedPath(entry.Path, bundles)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRemoved, path, formatLineMetrics(entry.Lines)))
		}
	}
}

// 💕️generateMetricsComment holds the data fields for a generateMetricsComment record.
func generateMetricsComment(diffs *TicketDiffs, bundles []Bundle) string {
	if diffs == nil {
		return ""
	}
	var lines []string
	appendDiffLines(&lines, diffs.Bundles, "📦️", "📦️", "📦️", "📦️", bundles, func(path string) string {
		return path
	}, func(from, to string) string {
		return formatRenameDelta(from, to)
	})
	appendDiffLines(&lines, diffs.Folders, "📂️", "📁️", "📁️", "📁️", bundles, func(path string) string {
		return formatPathWithBundle(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Files, "📄️", "📝️", "📄️", "📄️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Sections, "📑️", "🔖️", "🔖️", "🔖️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Definitions, "🏷️", "🏷️", "🏷️", "🏷️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	if len(lines) == 0 {
		return ""
	}
	return strings.Join(lines, "\n")
}
