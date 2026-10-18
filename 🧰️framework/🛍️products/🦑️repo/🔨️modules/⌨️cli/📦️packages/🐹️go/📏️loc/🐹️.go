// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📏️loc carries the loc command of the cli.

// #endregion 🧲️Header

package cli

import (
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	sort "sort"
	strings "strings"

	contributorspkg "github.com/usalu/semio/repo/contributors"
	metricspkg "github.com/usalu/semio/repo/metrics"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔢️LOC Command

// 🙈️locIgnorer is the repository gitignore seen through the metrics domain's ignore port.
type locIgnorer struct{}

// 🙈️MatchesPath answers the metrics domain's only ignore question.
func (locIgnorer) MatchesPath(path string) bool { return workspace.IsIgnoredByGitignore(path) }

// 👤️locContributorAlias is the metrics domain's alias port, backed by the contributor registry.
func locContributorAlias(name, email string) string {
	author := strings.TrimSpace(name)
	mail := strings.TrimSpace(email)
	if mail != "" {
		author = fmt.Sprintf("%s <%s>", author, mail)
	}
	if author == "" {
		return "unknown"
	}
	return contributorspkg.FindAndUpdateContributor(author)
}

// 🔢️runLocCommand builds the report through 📊️metrics' ports and renders it in the requested format.
func runLocCommand(cmd *Command, config *Config, languages []string, history, byContrib bool, historyBranch, contribFilter string) error {
	repoRoot := strings.TrimSpace(config.Repo)
	if repoRoot == "" {
		if wd, err := os.Getwd(); err == nil {
			repoRoot = workspace.FindRepoRoot(wd)
		} else {
			repoRoot = "."
		}
	} else {
		repoRoot = workspace.FindRepoRoot(repoRoot)
	}
	workspace.SetRootDir(repoRoot)
	options := metricspkg.LocOptions{Languages: languages, History: history, ByContributors: byContrib, Branch: historyBranch, Contributor: contribFilter}
	report, err := metricspkg.BuildLocReport(metricspkg.SystemGit{Repo: repoRoot}, options, locIgnorer{}, locContributorAlias)
	if err != nil {
		return err
	}
	out := cmd.OutOrStdout()
	if config.IsJSON() {
		encoder := json.NewEncoder(out)
		encoder.SetIndent("", "  ")
		return encoder.Encode(map[string]*metricspkg.LocReport{"loc": report})
	}
	if config.IsMarkdown() {
		_, err := io.WriteString(out, metricspkg.RenderMarkdown(report, history, byContrib))
		return err
	}
	renderLocText(out, report, history, byContrib, locIsTTY(out))
	return nil
}

// 🖥️locIsTTY reports whether the sink is an interactive terminal that may carry colour.
func locIsTTY(out io.Writer) bool {
	if os.Getenv("NO_COLOR") != "" {
		return false
	}
	file, ok := out.(*os.File)
	if !ok {
		return false
	}
	info, err := file.Stat()
	if err != nil {
		return false
	}
	return (info.Mode() & os.ModeCharDevice) != 0
}

// 📤️renderLocText is the coloured terminal presentation of a report; every table comes from 📊️metrics.
func renderLocText(w io.Writer, report *metricspkg.LocReport, withHistory, byContrib, isTTY bool) {
	fmt.Fprintln(w, model.Colorize("Snapshot", model.ColorBold, isTTY))
	fmt.Fprint(w, metricspkg.TextTable("", report.Snapshot, metricspkg.UseFullTreeTable(report.Snapshot), false))
	if byContrib && len(report.ByContributors) > 0 {
		for _, alias := range locSortedAliases(report.ByContributors) {
			rows := report.ByContributors[alias]
			fmt.Fprintln(w, "")
			fmt.Fprintln(w, model.Colorize("Contributor: "+metricspkg.ContributorEmojiID(alias), model.ColorBlue, isTTY))
			fmt.Fprint(w, metricspkg.TextTable("", rows, metricspkg.UseFullTreeTable(rows), false))
		}
	}
	if !withHistory || len(report.History) == 0 {
		return
	}
	fmt.Fprintln(w, "")
	fmt.Fprintln(w, model.Colorize("History: "+metricspkg.DisplayHistoryBranch(report.Branch), model.ColorBold, isTTY))
	for _, entry := range report.History {
		label := model.Colorize(metricspkg.HistoryCheckpointLabel(entry.SHA)+"  "+entry.Date, model.ColorDim, isTTY)
		if !byContrib {
			fmt.Fprintln(w, label)
			fmt.Fprint(w, metricspkg.TextTable("", entry.Languages, metricspkg.UseFullTreeTable(entry.Languages), true))
			continue
		}
		fmt.Fprintln(w, label, metricspkg.ContributorEmojiID(entry.Author))
		aliases := locSortedAliases(entry.ByContributors)
		for _, alias := range aliases {
			rows := entry.ByContributors[alias]
			if len(aliases) > 1 || !strings.EqualFold(strings.TrimSpace(alias), strings.TrimSpace(entry.Author)) {
				fmt.Fprint(w, "  ")
				fmt.Fprintln(w, model.Colorize(metricspkg.ContributorEmojiID(alias), model.ColorBlue, isTTY))
			}
			fmt.Fprint(w, metricspkg.TextTable("", rows, metricspkg.UseFullTreeTable(rows), true))
		}
	}
}

// 🔤️locSortedAliases is the sorted contributor key list of a contributor-keyed table.
func locSortedAliases(rows map[string]map[string]metricspkg.LocLangStats) []string {
	aliases := make([]string, 0, len(rows))
	for alias := range rows {
		aliases = append(aliases, alias)
	}
	sort.Strings(aliases)
	return aliases
}

// 🔢️ locCommand wires the `loc` command.
func locCommand(factory EngineFactory, config *Config) *Command {
	_ = factory
	cmd := &Command{
		Use:   "loc",
		Short: "Tracked-file LOC (code, markup, data) plus git deltas; internal scan (no cloc)",
		Long:  "Counts tracked files via git (not cloc). The runnable CLI is built from the repo client's MCP Go implementation into the platform-specific client binary. Wip% is each row's edited churn vs the whole first-parent walk on the logged ref (default ⛳️wip), including all contributors. With --history, each row scans the tree at that commit for physical LOC; Δ% is change vs the previous printed history step. Branch and checkpoint lines use the same emoji ids as the repo tree (⛳️wip, 🔀️abc1234, 🧑️‍💻️alias).",
		Args:  NoArgs,
		RunE: func(c *Command, args []string) error {
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
					br = metricspkg.DefaultBranch
				}
			} else {
				br = ""
			}
			return runLocCommand(c, config, langs, h, by, br, bc)
		},
	}
	cmd.Flags().Bool("history", false, "Per-commit time series; walks --branch (default: "+metricspkg.DefaultBranch+") without checkout")
	cmd.Flags().Bool("by-contributors", false, "Break down cumulative line deltas by first author (FindAndUpdateContributor alias)")
	cmd.Flags().String("by-contributor", "", "Restrict git deltas and history rows to this contributor alias (case-insensitive)")
	cmd.Flags().String("branch", metricspkg.DefaultBranch, "With --history: git ref to log (default dev branch). Ignored when --history is false")
	cmd.Flags().StringSlice("languages", []string{"TypeScript", "Go", "C#", "Python", "Rust"}, "Limits which programming-language buckets are counted; markup/data always included when matched")
	return cmd
}

// #endregion 🔢️LOC Command
