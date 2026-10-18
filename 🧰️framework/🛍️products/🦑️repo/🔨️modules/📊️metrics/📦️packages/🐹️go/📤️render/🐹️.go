// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📤️render carries the markdown and text renderers of the report.

// #endregion 🧲️Header

package metrics

import (
	"fmt"
	"sort"
	"strings"
)

// #region 📤️Render

// 📤️MarkdownTable renders a GitHub-flavoured pipe table.
func MarkdownTable(title string, rows map[string]LocLangStats, fullTree, sincePrev bool) string {
	names := SortedRowKeysChurn(rows)
	if fullTree {
		names = SortedRowKeys(rows)
	}
	var out strings.Builder
	if title != "" {
		out.WriteString("### " + title + "\n\n")
	}
	switch {
	case fullTree && sincePrev:
		out.WriteString("| Category | loc | Δ% | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")
	case fullTree:
		out.WriteString("| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n")
	default:
		out.WriteString("| Category | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: |\n")
	}
	for _, name := range names {
		row := rows[name]
		switch {
		case fullTree && sincePrev:
			delta := "—"
			if row.SincePrevLocPercent != nil {
				delta = fmt.Sprintf("%+.2f%%", *row.SincePrevLocPercent)
			}
			out.WriteString(fmt.Sprintf("| %s | %d | %s | %.2f%% | %.2f%% | %d | %d | %d |\n", name, row.Loc, delta, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		case fullTree:
			out.WriteString(fmt.Sprintf("| %s | %d | %.2f%% | %.2f%% | %d | %d | %d |\n", name, row.Loc, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		default:
			out.WriteString(fmt.Sprintf("| %s | %.2f%% | %d | %d | %d |\n", name, row.WipPercent, row.Edited, row.Added, row.Removed))
		}
	}
	return out.String()
}

// 📤️RenderMarkdown renders the whole report as GitHub-flavoured markdown.
func RenderMarkdown(report *LocReport, withHistory, byContributor bool) string {
	var out strings.Builder
	out.WriteString("## LOC\n")
	out.WriteString(MarkdownTable("Snapshot", report.Snapshot, UseFullTreeTable(report.Snapshot), false))
	out.WriteString("\n")
	if byContributor && len(report.ByContributors) > 0 {
		out.WriteString("\n## By contributor\n")
		for _, alias := range sortedKeys(report.ByContributors) {
			table := report.ByContributors[alias]
			out.WriteString(fmt.Sprintf("\n### %s\n\n", ContributorEmojiID(alias)))
			out.WriteString(MarkdownTable("", table, UseFullTreeTable(table), false))
			out.WriteString("\n")
		}
	}
	if withHistory && len(report.History) > 0 {
		out.WriteString(fmt.Sprintf("\n## History ( %s )\n", DisplayHistoryBranch(report.Branch)))
		for _, entry := range report.History {
			if byContributor {
				out.WriteString(fmt.Sprintf("\n#### %s  %s  %s\n", HistoryCheckpointLabel(entry.SHA), entry.Date, ContributorEmojiID(entry.Author)))
				for _, alias := range sortedKeys(entry.ByContributors) {
					table := entry.ByContributors[alias]
					out.WriteString(fmt.Sprintf("\n- **%s**\n\n", ContributorEmojiID(alias)))
					out.WriteString(MarkdownTable("", table, UseFullTreeTable(table), true))
					out.WriteString("\n")
				}
			} else if entry.Languages != nil {
				out.WriteString(fmt.Sprintf("\n#### %s  %s\n\n", HistoryCheckpointLabel(entry.SHA), entry.Date))
				out.WriteString(MarkdownTable("", entry.Languages, UseFullTreeTable(entry.Languages), true))
				out.WriteString("\n")
			}
		}
	}
	return out.String()
}

// 🔤️sortedKeys is the sorted key list of a contributor-keyed table.
func sortedKeys(rows map[string]map[string]LocLangStats) []string {
	names := make([]string, 0, len(rows))
	for name := range rows {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// 📤️TextTable renders a fixed-width plain-text table.
func TextTable(title string, rows map[string]LocLangStats, fullTree, sincePrev bool) string {
	names := SortedRowKeysChurn(rows)
	if fullTree {
		names = SortedRowKeys(rows)
	}
	var out strings.Builder
	if title != "" && !strings.HasPrefix(title, "  ") {
		out.WriteString(title + "\n")
	}
	if len(names) == 0 {
		return out.String()
	}
	switch {
	case fullTree && sincePrev:
		out.WriteString(fmt.Sprintf("%-14s%8s%8s%7s%7s%8s%8s%8s\n", "Category", "loc", "Δ%", "%", "wip", "edited", "added", "removed"))
	case fullTree:
		out.WriteString(fmt.Sprintf("%-14s%8s%7s%7s%8s%8s%8s\n", "Category", "loc", "%", "wip", "edited", "added", "removed"))
	default:
		out.WriteString(fmt.Sprintf("%-14s%7s%8s%8s%8s\n", "Category", "wip", "edited", "added", "removed"))
	}
	for _, name := range names {
		row := rows[name]
		switch {
		case fullTree && sincePrev:
			delta := "    —"
			if row.SincePrevLocPercent != nil {
				delta = fmt.Sprintf("%+6.1f%%", *row.SincePrevLocPercent)
			}
			out.WriteString(fmt.Sprintf("%-14s%8d%8s%6.1f%%%6.1f%%%8d%8d%8d\n", name, row.Loc, delta, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		case fullTree:
			out.WriteString(fmt.Sprintf("%-14s%8d%6.1f%%%6.1f%%%8d%8d%8d\n", name, row.Loc, row.Percent, row.WipPercent, row.Edited, row.Added, row.Removed))
		default:
			out.WriteString(fmt.Sprintf("%-14s%6.1f%%%8d%8d%8d\n", name, row.WipPercent, row.Edited, row.Added, row.Removed))
		}
	}
	return out.String()
}

// 📤️RenderText renders the whole report as plain text, without terminal colour.
func RenderText(report *LocReport, withHistory, byContributor bool) string {
	var out strings.Builder
	out.WriteString(TextTable("Snapshot", report.Snapshot, UseFullTreeTable(report.Snapshot), false))
	if byContributor {
		for _, alias := range sortedKeys(report.ByContributors) {
			table := report.ByContributors[alias]
			out.WriteString("\nContributor: " + ContributorEmojiID(alias) + "\n")
			out.WriteString(TextTable("", table, UseFullTreeTable(table), false))
		}
	}
	if withHistory && len(report.History) > 0 {
		out.WriteString("\nHistory: " + DisplayHistoryBranch(report.Branch) + "\n")
		for _, entry := range report.History {
			if byContributor {
				out.WriteString(fmt.Sprintf("%s  %s %s\n", HistoryCheckpointLabel(entry.SHA), entry.Date, ContributorEmojiID(entry.Author)))
				for _, alias := range sortedKeys(entry.ByContributors) {
					table := entry.ByContributors[alias]
					out.WriteString("  " + ContributorEmojiID(alias) + "\n")
					out.WriteString(TextTable("  ", table, UseFullTreeTable(table), true))
				}
			} else if entry.Languages != nil {
				out.WriteString(fmt.Sprintf("%s  %s\n", HistoryCheckpointLabel(entry.SHA), entry.Date))
				out.WriteString(TextTable("", entry.Languages, UseFullTreeTable(entry.Languages), true))
			}
		}
	}
	return out.String()
}

// #endregion 📤️Render
