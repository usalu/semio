// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Lines-of-code markdown and text rendering and command.

// #endregion 🧲️Header

package client

import (
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"io"
	"sort"
	"strings"
)

// 📤️ renderLocMarkdown outputs GitHub-flavoured tables for `loc`.
func renderLocMarkdown(w io.Writer, r *LocReport, withHistory, byContrib bool) {
	fmt.Fprintln(w, "## LOC")
	fmt.Fprintln(w, locMarkdownTable("Snapshot", r.Snapshot, locUseFullTreeTable(r.Snapshot), false))
	if byContrib && len(r.ByContributors) > 0 {
		keys := make([]string, 0, len(r.ByContributors))
		for a := range r.ByContributors {
			keys = append(keys, a)
		}
		sort.Strings(keys)
		fmt.Fprintln(w, "")
		fmt.Fprintln(w, "## By contributor")
		for _, a := range keys {
			s := r.ByContributors[a]
			fmt.Fprintln(w, "")
			fmt.Fprintf(w, "### %s\n\n", locContributorEmojiID(a))
			fmt.Fprintln(w, locMarkdownTable("", s, locUseFullTreeTable(s), false))
		}
	}
	if withHistory && len(r.History) > 0 {
		fmt.Fprintln(w, "")
		fmt.Fprintln(w, "## History (", locDisplayHistoryBranch(r.Branch), ")")
		if byContrib {
			for _, e := range r.History {
				fmt.Fprintln(w, "")
				fmt.Fprintf(w, "#### %s  %s  %s\n", locHistoryCheckpointLabel(e.SHA), e.Date, locContributorEmojiID(e.Author))
				acs := make([]string, 0, len(e.ByContributors))
				for a := range e.ByContributors {
					acs = append(acs, a)
				}
				sort.Strings(acs)
				for _, a := range acs {
					if e.ByContributors == nil {
						break
					}
					s := e.ByContributors[a]
					showSub := len(acs) > 1 || !strings.EqualFold(strings.TrimSpace(a), strings.TrimSpace(e.Author))
					if showSub {
						fmt.Fprintln(w, "")
						fmt.Fprintf(w, "- **%s**\n\n", locContributorEmojiID(a))
					} else {
						fmt.Fprintln(w, "")
					}
					fmt.Fprintln(w, locMarkdownTable("", s, locUseFullTreeTable(s), true))
				}
			}
		} else {
			for _, e := range r.History {
				fmt.Fprintln(w, "")
				fmt.Fprintf(w, "#### %s  %s\n\n", locHistoryCheckpointLabel(e.SHA), e.Date)
				if e.Languages != nil {
					fmt.Fprintln(w, locMarkdownTable("", e.Languages, locUseFullTreeTable(e.Languages), true))
				}
			}
		}
	}
}

// 📤️ locMarkdownTable renders a GH-flavoured pipe table. sincePrevCol adds Δ% vs prior history row (fullTree only).
func locMarkdownTable(title string, rows map[string]LocLangStats, fullTree, sincePrevCol bool) string {
	var langs []string
	if fullTree {
		langs = locSortedRowKeys(rows)
	} else {
		langs = locSortedRowKeysChurn(rows)
	}
	var b strings.Builder
	if title != "" {
		b.WriteString("### " + title + "\n\n")
	}
	if fullTree {
		if sincePrevCol {
			b.WriteString("| Category | loc | Δ% | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n")
			for _, l := range langs {
				st := rows[l]
				d := "—"
				if st.SincePrevLocPercent != nil {
					d = fmt.Sprintf("%+.2f%%", *st.SincePrevLocPercent)
				}
				b.WriteString(fmt.Sprintf("| %s | %d | %s | %.2f%% | %.2f%% | %d | %d | %d |\n", l, st.Loc, d, st.Percent, st.WipPercent, st.Edited, st.Added, st.Removed))
			}
		} else {
			b.WriteString("| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n")
			for _, l := range langs {
				st := rows[l]
				b.WriteString(fmt.Sprintf("| %s | %d | %.2f%% | %.2f%% | %d | %d | %d |\n", l, st.Loc, st.Percent, st.WipPercent, st.Edited, st.Added, st.Removed))
			}
		}
	} else {
		b.WriteString("| Category | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: |\n")
		for _, l := range langs {
			st := rows[l]
			b.WriteString(fmt.Sprintf("| %s | %.2f%% | %d | %d | %d |\n", l, st.WipPercent, st.Edited, st.Added, st.Removed))
		}
	}
	return b.String()
}

// 🔤️ renderLocText prints a colored fixed-width table for TTY; plain for pipes.
func renderLocText(w io.Writer, r *LocReport, withHistory, byContrib, isTTY bool) {
	locTextTable(w, "Snapshot", r.Snapshot, isTTY, locUseFullTreeTable(r.Snapshot), false)
	if byContrib && len(r.ByContributors) > 0 {
		ks := make([]string, 0, len(r.ByContributors))
		for a := range r.ByContributors {
			ks = append(ks, a)
		}
		sort.Strings(ks)
		for _, a := range ks {
			s := r.ByContributors[a]
			fmt.Fprintln(w, "")
			fmt.Fprintln(w, colorize("Contributor: "+locContributorEmojiID(a), ColorBlue, isTTY))
			locTextTable(w, "", s, isTTY, locUseFullTreeTable(s), false)
		}
	}
	if withHistory && len(r.History) > 0 {
		fmt.Fprintln(w, "")
		fmt.Fprintln(w, colorize("History: "+locDisplayHistoryBranch(r.Branch), ColorBold, isTTY))
		if byContrib {
			for _, e := range r.History {
				fmt.Fprintln(w, colorize(locHistoryCheckpointLabel(e.SHA)+"  "+e.Date, ColorDim, isTTY), locContributorEmojiID(e.Author))
				if e.ByContributors == nil {
					continue
				}
				acs := make([]string, 0, len(e.ByContributors))
				for a := range e.ByContributors {
					acs = append(acs, a)
				}
				sort.Strings(acs)
				for _, a := range acs {
					s := e.ByContributors[a]
					showSub := len(acs) > 1 || !strings.EqualFold(strings.TrimSpace(a), strings.TrimSpace(e.Author))
					if showSub {
						fmt.Fprint(w, "  ")
						fmt.Fprintln(w, colorize(locContributorEmojiID(a), ColorBlue, isTTY))
						locTextTable(w, "  ", s, isTTY, locUseFullTreeTable(s), true)
					} else {
						locTextTable(w, "", s, isTTY, locUseFullTreeTable(s), true)
					}
				}
			}
		} else {
			for _, e := range r.History {
				fmt.Fprintln(w, colorize(locHistoryCheckpointLabel(e.SHA)+"  "+e.Date, ColorDim, isTTY))
				locTextTable(w, "", e.Languages, isTTY, locUseFullTreeTable(e.Languages), true)
			}
		}
	}
}

// 🔤️ locTextTable prints a simple aligned table; title may be a prefix. sincePrevCol adds Δ% after loc (fullTree only).
func locTextTable(w io.Writer, title string, rows map[string]LocLangStats, isTTY, fullTree, sincePrevCol bool) {
	var keys []string
	if fullTree {
		keys = locSortedRowKeys(rows)
	} else {
		keys = locSortedRowKeysChurn(rows)
	}
	if title != "" && !strings.HasPrefix(title, "  ") {
		fmt.Fprintln(w, colorize(title, ColorBold, isTTY))
	}
	if len(keys) == 0 {
		return
	}
	if fullTree {
		if sincePrevCol {
			fmt.Fprintf(w, "%-14s%8s%8s%7s%7s%8s%8s%8s\n", "Category", "loc", "Δ%", "%", "wip", "edited", "added", "removed")
			for _, l := range keys {
				s := rows[l]
				d := "    —"
				if s.SincePrevLocPercent != nil {
					d = fmt.Sprintf("%+6.1f%%", *s.SincePrevLocPercent)
				}
				fmt.Fprintf(w, "%-14s%8d%8s%6.1f%%%6.1f%%%8d%8d%8d\n", l, s.Loc, d, s.Percent, s.WipPercent, s.Edited, s.Added, s.Removed)
			}
		} else {
			fmt.Fprintf(w, "%-14s%8s%7s%7s%8s%8s%8s\n", "Category", "loc", "%", "wip", "edited", "added", "removed")
			for _, l := range keys {
				s := rows[l]
				fmt.Fprintf(w, "%-14s%8d%6.1f%%%6.1f%%%8d%8d%8d\n", l, s.Loc, s.Percent, s.WipPercent, s.Edited, s.Added, s.Removed)
			}
		}
	} else {
		fmt.Fprintf(w, "%-14s%7s%8s%8s%8s\n", "Category", "wip", "edited", "added", "removed")
		for _, l := range keys {
			s := rows[l]
			fmt.Fprintf(w, "%-14s%6.1f%%%8d%8d%8d\n", l, s.WipPercent, s.Edited, s.Added, s.Removed)
		}
	}
}

// 🔢️ locCommand wires the `loc` command.
func locCommand(factory EngineFactory, config *Config) *command.Command {
	_ = factory
	cmd := &command.Command{
		Use:   "loc",
		Short: "Tracked-file LOC (code, markup, data) plus git deltas; internal scan (no cloc)",
		Long:  "Counts tracked files via git (not cloc). The runnable CLI is built from the repo client's MCP Go implementation into the platform-specific client binary. Wip% is each row's edited churn vs the whole first-parent walk on the logged ref (default ⛳️wip), including all contributors. With --history, each row scans the tree at that commit for physical LOC; Δ% is change vs the previous printed history step. Branch and checkpoint lines use the same emoji ids as the repo tree (⛳️wip, 🔀️abc1234, 🧑️‍💻️alias).",
		Args:  command.NoArgs,
		RunE: func(c *command.Command, args []string) error {
			langs, _ := c.Flags().GetStringSlice("languages")
			if len(langs) == 0 {
				langs = []string{"TypeScript", "Go", "C#", "Python", "Rust"}
			}
			h, _ := c.Flags().GetBool("history")
			by, _ := c.Flags().GetBool("by-contributors")
			br, _ := c.Flags().GetString("branch")
			bc, _ := c.Flags().GetString("by-contributor")
			if h {
				if strings.TrimSpace(br) == "" {
					br = locDefaultBranch
				}
			} else {
				br = ""
			}
			return runLocCommand(c, config, langs, h, by, br, bc)
		},
	}
	cmd.Flags().Bool("history", false, "Per-commit time series; walks --branch (default: "+locDefaultBranch+") without checkout")
	cmd.Flags().Bool("by-contributors", false, "Break down cumulative line deltas by first author (FindAndUpdateContributor alias)")
	cmd.Flags().String("by-contributor", "", "Restrict git deltas and history rows to this contributor alias (case-insensitive)")
	cmd.Flags().String("branch", locDefaultBranch, "With --history: git ref to log (default dev branch). Ignored when --history is false")
	cmd.Flags().StringSlice("languages", []string{"TypeScript", "Go", "C#", "Python", "Rust"}, "Limits which programming-language buckets are counted; markup/data always included when matched")
	return cmd
}
