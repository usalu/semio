// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// LOC command, MCP client kinds, and MCP stdio handshake.

// #endregion 🧲️Header

package client

import (
	"bufio"
	"context"
	command "github.com/usalu/semio/repo/client/internal/command"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func TestLocCommand(t *testing.T) {
	langs := locMakeLangSet([]string{"TypeScript", "Go", "C#", "Python", "Rust"})
	num := locMakeNumstatLangSet([]string{"TypeScript", "Go", "C#", "Python", "Rust"})
	t.Run("classify language", func(t *testing.T) {
		if got := locClassifyLanguage("a/b/foo.ts", num); got != "TypeScript" {
			t.Fatalf("ts: got %q", got)
		}
		if got := locClassifyLanguage("x.mtsx", num); got != "TypeScript" {
			t.Fatalf("mtsx: got %q", got)
		}
		if got := locClassifyLanguage("pkg/m.go", num); got != "Go" {
			t.Fatalf("go: got %q", got)
		}
		if got := locClassifyLanguage("noext", num); got != "" {
			t.Fatalf("noext: got %q", got)
		}
		if got := locClassifyLocBucket("x.md"); got != locAggMarkup {
			t.Fatalf("md bucket: got %q", got)
		}
		if got := locClassifyLocBucket("d.json"); got != locAggData {
			t.Fatalf("json bucket: got %q", got)
		}
		if locClassifyLanguage("x.md", langs) != "" {
			t.Fatalf("markdown should not match code-only weight map")
		}
	})
	t.Run("hidden path segments skipped", func(t *testing.T) {
		if !locPathHasHiddenSegment("pkg/.cache/x.go") {
			t.Fatal("expected hidden segment")
		}
		if locPathHasHiddenSegment("pkg/src/x.go") {
			t.Fatal("unexpected hidden")
		}
	})
	t.Run("json key counting", func(t *testing.T) {
		if n := locJSONKeyCount([]byte(`{"a":1,"b":{"c":2}}`)); n != 3 {
			t.Fatalf("keys want 3 got %d", n)
		}
		if n := locJSONKeyCount([]byte(`{}`)); n != 0 {
			t.Fatalf("empty object keys want 0 got %d", n)
		}
		if n := locCountBucketLoc("x.json", []byte(`{"x":1}`)); n != 1 {
			t.Fatalf("single-line json loc want 1 got %d", n)
		}
	})
	t.Run("parse numstat", func(t *testing.T) {
		oldR := GetRootDir()
		td := t.TempDir()
		_ = os.WriteFile(filepath.Join(td, ".gitignore"), []byte("# test\n"), 0o644)
		SetRootDir(td)
		defer SetRootDir(oldR)
		raw := "COMMIT\t" + strings.Repeat("a", 8) + "\tAlice\ta@x\t1600000000\n3\t1\tlib/main.go\n-\t-\tbin/legacy\n2\t0\tc.cs\n"
		out, err := locParseNumstatLog(raw, num)
		if err != nil {
			t.Fatal(err)
		}
		if len(out) != 1 {
			t.Fatalf("commits %d", len(out))
		}
		p := out[0].Delta["Go"]
		if p.Added != 3 || p.Removed != 1 {
			t.Fatalf("go delta %+v", p)
		}
		p2 := out[0].Delta["C#"]
		if p2.Added != 2 {
			t.Fatalf("cs %+v", p2)
		}
		// 0,0 and binary - skipped above
	})
	t.Run("merge cumulative and cloc", func(t *testing.T) {
		c := map[string]LocCumulative{"Go": {Added: 10, Removed: 2}}
		scanned := map[string]int{"Go": 100, "TypeScript": 0, "C#": 0, "Python": 0, "Rust": 0, "Markup": 5, "Data": 3}
		got := locMergeCumulativeClocEx(c, scanned, []string{"Go", "TypeScript", "C#", "Python", "Rust"})
		if got["Go"].Loc != 100 || got["Go"].Edited != 12 || got["TypeScript"].Loc != 0 {
			t.Fatalf("%+v", got["Go"])
		}
		if got[locAggCode].Loc != 100 {
			t.Fatalf("code aggregate %+v", got[locAggCode])
		}
		if got[locAggTotal].Loc != 108 {
			t.Fatalf("total loc want 108 got %d", got[locAggTotal].Loc)
		}
		if got["Go"].Percent <= 0 || got[locAggTotal].Percent != 100 {
			t.Fatalf("percents go=%v total=%v", got["Go"].Percent, got[locAggTotal].Percent)
		}
		if got["Go"].WipPercent != 100 || got[locAggTotal].WipPercent != 100 {
			t.Fatalf("wip percents go=%v total=%v", got["Go"].WipPercent, got[locAggTotal].WipPercent)
		}
	})
	t.Run("wip percent uses branch denominator for contributor rows", func(t *testing.T) {
		langs := []string{"Go", "TypeScript", "C#", "Python", "Rust"}
		by := map[string]map[string]locCumulativePair{
			"alice": {"Go": {Added: 5, Removed: 1}},
			"bob":   {"Go": {Added: 15, Removed: 3}},
		}
		branchDenom := locSumEditedPairs(map[string]locCumulativePair{
			"Go": {Added: 20, Removed: 4},
		}, locMakeNumstatLangSet(langs))
		if branchDenom != 24 {
			t.Fatalf("branch denom %d", branchDenom)
		}
		out := locByContributorsToSnapshot(by, langs, branchDenom)
		if out["alice"]["Go"].WipPercent != 25 {
			t.Fatalf("alice wip %% want 25 got %v", out["alice"]["Go"].WipPercent)
		}
		if out["bob"]["Go"].WipPercent != 75 {
			t.Fatalf("bob wip %% want 75 got %v", out["bob"]["Go"].WipPercent)
		}
	})
	t.Run("render markdown", func(t *testing.T) {
		scan := map[string]int{"Go": 1, "TypeScript": 0, "C#": 0, "Python": 0, "Rust": 0, "Markup": 0, "Data": 0}
		cum := map[string]locCumulativePair{"Go": {Added: 1, Removed: 1}}
		r := &LocReport{Snapshot: locComposeLocReportSnapshot(cum, scan, []string{"Go", "TypeScript", "C#", "Python", "Rust"}, 0)}
		var b strings.Builder
		renderLocMarkdown(&b, r, false, false)
		s := b.String()
		if !strings.Contains(s, "Go") || !strings.Contains(s, "| 1 |") || !strings.Contains(s, "%") {
			t.Fatalf("markdown: %q", s)
		}
	})
	t.Run("delta-only table omits loc and tree percent columns", func(t *testing.T) {
		rows := map[string]LocLangStats{
			"TypeScript": {Loc: 0, Edited: 5, Added: 1, Removed: 1, Percent: 0, WipPercent: 10},
			"Go":         {Loc: 0, Edited: 20, Added: 2, Removed: 2, Percent: 0, WipPercent: 40},
			locAggTotal:  {Loc: 0, Edited: 25, Added: 3, Removed: 3, Percent: 0, WipPercent: 50},
		}
		if locUseFullTreeTable(rows) {
			t.Fatal("expected delta-only rows (total loc 0)")
		}
		md := locMarkdownTable("", rows, false, false)
		if strings.Contains(md, "| loc |") || strings.Contains(md, "| Category | % |") {
			t.Fatalf("unexpected full-tree columns: %q", md)
		}
		if !strings.Contains(md, "| Category | wip% | edited | added | removed |") {
			t.Fatalf("want churn header: %q", md)
		}
		// churn sort: Go before TypeScript (edited 20 > 5), Total last
		goIdx := strings.Index(md, "| Go |")
		tsIdx := strings.Index(md, "| TypeScript |")
		totIdx := strings.Index(md, "| Total |")
		if goIdx <= 0 || tsIdx <= 0 || totIdx <= 0 || !(goIdx < tsIdx && tsIdx < totIdx) {
			t.Fatalf("row order: %q", md)
		}
	})
	t.Run("locSortedRowKeysChurn total last", func(t *testing.T) {
		rows := map[string]LocLangStats{
			"Rust":      {Edited: 1},
			"Go":        {Edited: 99},
			locAggTotal: {Edited: 100},
		}
		ks := locSortedRowKeysChurn(rows)
		if len(ks) != 3 || ks[0] != "Go" || ks[1] != "Rust" || ks[2] != locAggTotal {
			t.Fatalf("got %v", ks)
		}
	})
	t.Run("history since-prev loc percent", func(t *testing.T) {
		prev := map[string]LocLangStats{"Go": {Loc: 100}, locAggTotal: {Loc: 1000}}
		cur := map[string]LocLangStats{"Go": {Loc: 110}, locAggTotal: {Loc: 1100}}
		h := []LocHistoryEntry{{SHA: "aaa", Languages: prev}, {SHA: "bbb", Languages: cur}}
		locApplyHistoryLocSincePrev(h)
		if h[0].Languages["Go"].SincePrevLocPercent != nil {
			t.Fatalf("first row want nil delta")
		}
		if h[1].Languages["Go"].SincePrevLocPercent == nil {
			t.Fatal("second row want delta")
		}
		if g := *h[1].Languages["Go"].SincePrevLocPercent; g < 9.99 || g > 10.01 {
			t.Fatalf("go Δ%% want ~10 got %v", g)
		}
	})
	t.Run("locHistoryEntryStatsMap prefers languages", func(t *testing.T) {
		rows := map[string]LocLangStats{"Go": {Loc: 1}}
		e := LocHistoryEntry{Languages: rows, ByContributors: map[string]map[string]LocLangStats{"x": {}}}
		got := locHistoryEntryStatsMap(&e)
		if got == nil || got["Go"].Loc != 1 {
			t.Fatalf("expected Languages map, got %#v", got)
		}
	})
	t.Run("text no ansi for pipe", func(t *testing.T) {
		scan := map[string]int{"Go": 1, "TypeScript": 0, "C#": 0, "Python": 0, "Rust": 0, "Markup": 0, "Data": 0}
		r := &LocReport{Snapshot: locComposeLocReportSnapshot(map[string]locCumulativePair{}, scan, []string{"Go", "TypeScript", "C#", "Python", "Rust"}, 0)}
		var b strings.Builder
		renderLocText(&b, r, false, false, false)
		if strings.ContainsRune(b.String(), '\x1b') {
			t.Fatal("unexpected ansi")
		}
	})
	t.Run("root has loc", func(t *testing.T) {
		root, _ := NewRootWithConfig(testEngineFactory)
		var found *command.Command
		for _, c := range root.Commands() {
			if c.Name() == "loc" {
				found = c
				break
			}
		}
		if found == nil {
			t.Fatal("no loc")
		}
		if f := found.Flags().Lookup("by-contributor"); f == nil {
			t.Fatal("missing by-contributor flag")
		}
	})
}

func TestParseMcpClientKind(t *testing.T) {
	cases := []struct {
		raw  string
		want McpClientKind
	}{
		{"", McpClientGeneric},
		{"generic", McpClientGeneric},
		{"client", McpClientGeneric},
		{"cursor", McpClientCursor},
		{"kiro", McpClientKiro},
		{"copilot", McpClientCopilot},
		{"claude", McpClientClaude},
		{"codex", McpClientCodex},
		{"CURSOR", McpClientCursor},
		{"  codex  ", McpClientCodex},
	}
	for _, tc := range cases {
		got, err := ParseMcpClientKind(tc.raw)
		if err != nil {
			t.Fatalf("ParseMcpClientKind(%q) error: %v", tc.raw, err)
		}
		if got != tc.want {
			t.Fatalf("ParseMcpClientKind(%q) = %q, want %q", tc.raw, got, tc.want)
		}
	}
	if _, err := ParseMcpClientKind("unknown"); err == nil {
		t.Fatal("expected error for unknown kind")
	}
}

func TestMcpCommandKinds(t *testing.T) {
	cases := []struct {
		kind McpClientKind
		want string
	}{
		{McpClientGeneric, "repo"},
		{McpClientCursor, "repo"},
		{McpClientKiro, "repo"},
		{McpClientCopilot, "repo"},
		{McpClientClaude, "repo"},
		{McpClientCodex, "repo"},
	}
	for _, tc := range cases {
		parsed, err := ParseMcpClientKind(string(tc.kind))
		if err != nil {
			t.Fatalf("ParseMcpClientKind(%q): %v", tc.kind, err)
		}
		if parsed != tc.kind {
			t.Fatalf("ParseMcpClientKind(%q) = %q, want %q", tc.kind, parsed, tc.kind)
		}
		if got := McpServerName(tc.kind); got != tc.want {
			t.Fatalf("McpServerName(%q) = %q, want %q", tc.kind, got, tc.want)
		}
	}
}

func TestMicroCommitCommandExists(t *testing.T) {
	root, _ := NewRootWithConfig(testEngineFactory)
	var found *command.Command
	for _, c := range root.Commands() {
		if c.Name() == "micro-commit" {
			found = c
			break
		}
	}
	if found == nil {
		t.Fatal("root command tree missing micro-commit")
	}
	if found.Use != "micro-commit [subcommand] [args...]" {
		t.Fatalf("unexpected micro-commit use: %q", found.Use)
	}
}

func TestMcpStdioInitializeHandshake(t *testing.T) {
	repoRoot := findRepoRoot(".")
	if repoRoot == "" {
		t.Skip("repo root not found")
	}
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "bun", "./📜️script.ts", "dev", "mcp", "stdio", "cursor")
	cmd.Dir = repoRoot
	stdin, err := cmd.StdinPipe()
	if err != nil {
		t.Fatal(err)
	}
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	cmd.Stderr = os.Stderr
	if err := cmd.Start(); err != nil {
		t.Fatal(err)
	}
	time.Sleep(100 * time.Millisecond)
	defer func() {
		_ = stdin.Close()
		_ = cmd.Process.Kill()
		_ = cmd.Wait()
	}()
	initReq := `{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}` + "\n"
	if _, err := io.WriteString(stdin, initReq); err != nil {
		t.Fatal(err)
	}
	reader := bufio.NewReader(stdout)
	line, err := reader.ReadString('\n')
	if err != nil {
		t.Fatalf("read initialize response: %v", err)
	}
	if !strings.Contains(line, `"result"`) || !strings.Contains(line, `jsonrpc`) || !strings.Contains(line, `"name":"repo"`) {
		t.Fatalf("unexpected initialize response: %s", line)
	}
}
