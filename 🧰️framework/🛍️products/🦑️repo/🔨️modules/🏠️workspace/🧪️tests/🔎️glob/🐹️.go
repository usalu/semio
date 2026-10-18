// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unit tests for the owned glob and ignore primitives, the layout constants, root discovery and
// the `📋️config.toml` reader.

// #endregion 🧲️Header

package workspace

import (
	"os"
	"path/filepath"
	"testing"
)

// #region 🃏️Glob

func TestMatchHandlesRecursiveAndSingleSegmentWildcards(t *testing.T) {
	cases := []struct {
		pattern string
		name    string
		want    bool
	}{
		{"**/*.go", "root.go", true},
		{"**/*.go", "a/one.go", true},
		{"**/*.go", "a/b/two.go", true},
		{"*.go", "a/one.go", false},
		{"a/*/two.go", "a/b/two.go", true},
		{"a/*/two.go", "a/b/c/two.go", false},
		{"a/**", "a/b/c", true},
		{"?.go", "a.go", true},
		{"?.go", "ab.go", false},
		{"[abc].go", "b.go", true},
		{"[a-c].go", "c.go", true},
		{"[!a].go", "b.go", true},
		{"[!a].go", "a.go", false},
	}
	for _, item := range cases {
		got, err := Match(item.pattern, item.name)
		if err != nil {
			t.Fatalf("Match(%q, %q) returned %v", item.pattern, item.name, err)
		}
		if got != item.want {
			t.Errorf("Match(%q, %q) = %v, want %v", item.pattern, item.name, got, item.want)
		}
	}
}

func TestMatchRejectsMalformedPatterns(t *testing.T) {
	if _, err := Match("[abc", "a"); err == nil {
		t.Error("an unclosed character class must be an error")
	}
}

func TestFilepathGlobIsSortedAndScopedToTheStaticPrefix(t *testing.T) {
	root := t.TempDir()
	for _, relative := range []string{"a/one.go", "a/b/two.go", "root.go", "a/skip.txt"} {
		full := filepath.Join(root, filepath.FromSlash(relative))
		if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(full, []byte("package a\n"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	matches, err := FilepathGlob(filepath.Join(root, "**", "*.go"))
	if err != nil {
		t.Fatal(err)
	}
	if len(matches) != 3 {
		t.Fatalf("got %d matches, want 3: %v", len(matches), matches)
	}
	for index := 1; index < len(matches); index++ {
		if matches[index-1] > matches[index] {
			t.Fatalf("matches are not sorted: %v", matches)
		}
	}
}

// #endregion 🃏️Glob

// #region 🙈️Ignore

func TestIgnorePrecedenceIsLastRuleWins(t *testing.T) {
	ignore := CompileIgnoreLines("# comment", "", "build/", "!build/keep.txt", "*.log")
	cases := map[string]bool{
		"build/out.o":       true,
		"build/keep.txt":    false,
		"deep/nested/a.log": true,
		"src/main.go":       false,
	}
	for path, want := range cases {
		if got := ignore.MatchesPath(path); got != want {
			t.Errorf("MatchesPath(%q) = %v, want %v", path, got, want)
		}
	}
}

func TestIgnoreDropsALeadingSlashAndThenLiftsToAnySegment(t *testing.T) {
	ignore := CompileIgnoreLines("/target")
	if !ignore.MatchesPath("target") {
		t.Error("the rule must match at the root")
	}
	if !ignore.MatchesPath("nested/target") {
		t.Error("a single-segment rule is lifted to **/target, so it also matches below the root")
	}
	if ignore.MatchesPath("target-other") {
		t.Error("the rule must match whole segments only")
	}
}

func TestCompileIgnoreFileReadsFromDisk(t *testing.T) {
	root := t.TempDir()
	path := filepath.Join(root, ".gitignore")
	if err := os.WriteFile(path, []byte("node_modules/\n!node_modules/.keep\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	ignore, err := CompileIgnoreFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !ignore.MatchesPath("node_modules/left-pad/index.js") {
		t.Error("the directory rule must match")
	}
	if ignore.MatchesPath("node_modules/.keep") {
		t.Error("the negation must win")
	}
}

// #endregion 🙈️Ignore

// #region 🧭️Layout

func TestLayoutPathsComposeFromTheRoot(t *testing.T) {
	root := filepath.Join("x", "y")
	if got, want := RepoMetaDirForRoot(root), filepath.Join(root, ".🧬semio", "🦑️repo"); got != want {
		t.Errorf("RepoMetaDirForRoot = %q, want %q", got, want)
	}
	if got, want := TicketsDirForRoot(root), filepath.Join(root, ".🧬semio", "🦑️repo", "🎫️tickets"); got != want {
		t.Errorf("TicketsDirForRoot = %q, want %q", got, want)
	}
	if got, want := GoalsDirForRoot(root), filepath.Join(root, ".🧬semio", "🦑️repo", "🎯️goals"); got != want {
		t.Errorf("GoalsDirForRoot = %q, want %q", got, want)
	}
	if got, want := DevsDirForRoot(root), filepath.Join(root, ".🧬semio", "🦑️repo", "🧑️‍💻️devs"); got != want {
		t.Errorf("DevsDirForRoot = %q, want %q", got, want)
	}
	if got, want := FilesIndexForRoot(root), filepath.Join(root, ".🧬semio", "🦑️repo", "📁️files.json"); got != want {
		t.Errorf("FilesIndexForRoot = %q, want %q", got, want)
	}
}

// #endregion 🧭️Layout

// #region 🧭️Discovery

func TestFindRepoRootPrefersTheNearestGitCheckout(t *testing.T) {
	root := t.TempDir()
	nested := filepath.Join(root, "a", "b", "c")
	if err := os.MkdirAll(nested, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(filepath.Join(root, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	got, err := filepath.EvalSymlinks(FindRepoRoot(nested))
	if err != nil {
		t.Fatal(err)
	}
	want, err := filepath.EvalSymlinks(root)
	if err != nil {
		t.Fatal(err)
	}
	if got != want {
		t.Errorf("FindRepoRoot = %q, want %q", got, want)
	}
}

func TestFindRepoRootFallsBackToAGoModule(t *testing.T) {
	root := t.TempDir()
	nested := filepath.Join(root, "a", "b")
	if err := os.MkdirAll(nested, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "a", "go.mod"), []byte("module x\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if filepath.Base(FindRepoRoot(nested)) != "a" {
		t.Errorf("FindRepoRoot = %q, want the directory holding go.mod", FindRepoRoot(nested))
	}
}

// #endregion 🧭️Discovery

// #region ⚙️RepoConfig

func TestParseRepoConfigAppliesTheLoggingSection(t *testing.T) {
	config := ParseRepoConfig("# a comment\n[logging]\nsession = \"on\"\noperations = no\nplan = 1\ndetail = 'full'\n[other]\nsession = true\n")
	if !config.Logging.Session {
		t.Error("session must be enabled")
	}
	if config.Logging.Operations {
		t.Error("operations must be disabled")
	}
	if !config.Logging.Plan {
		t.Error("plan must stay enabled")
	}
	if config.Logging.Detail != "full" {
		t.Errorf("detail = %q, want full", config.Logging.Detail)
	}
	if !config.Logging.IncludeNative() || !config.Logging.IncludeResponse() {
		t.Error("full detail includes both payloads")
	}
}

func TestLoadRepoConfigFallsBackToTheDefaults(t *testing.T) {
	if LoadRepoConfig(t.TempDir()) != DefaultRepoConfig() {
		t.Error("a missing config file must yield the defaults")
	}
	if LoadRepoConfig("") != DefaultRepoConfig() {
		t.Error("an empty root must yield the defaults")
	}
}

func TestLoadRepoConfigReadsTheMetaDirectory(t *testing.T) {
	root := t.TempDir()
	metaDir := RepoMetaDirForRoot(root)
	if err := os.MkdirAll(metaDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(metaDir, ConfigFileName), []byte("[logging]\ndetail = minimal\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	config := LoadRepoConfig(root)
	if config.Logging.IncludeResponse() {
		t.Error("minimal detail must exclude the response payload")
	}
}

// #endregion ⚙️RepoConfig
