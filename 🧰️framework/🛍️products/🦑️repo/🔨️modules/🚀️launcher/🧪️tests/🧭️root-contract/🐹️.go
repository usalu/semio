// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧭️root-contract proves workspace root resolution against the neutral corpus that the TypeScript, shell and PowerShell resolvers share.

// #endregion 🧲️Header

package launcher

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 🪟️A Windows final path reads as the shell spelling libuv's `realpath` gives it (the service hashes that text): the Win32 file namespace prefix goes, a UNC volume keeps its two leading separators, any other text stays.
func TestFinalPathSpelling(t *testing.T) {
	for final, want := range map[string]string{
		`\\?\C:\Users\ada\semio`:     `C:\Users\ada\semio`,
		`\\?\c:\semio`:               `c:\semio`,
		`\\?\UNC\server\share\semio`: `\\server\share\semio`,
		`\\server\share\semio`:       `\\server\share\semio`,
		`C:\semio`:                   `C:\semio`,
		`\\.\C:\semio`:               `\\.\C:\semio`,
	} {
		if got := FinalPathSpelling(final); got != want {
			t.Errorf("%s: %s, want %s", final, got, want)
		}
	}
}

// 🧭️Every corpus case resolves to the expected root, or to none.
func TestWorkspaceRootCorpus(t *testing.T) {
	var corpus struct {
		Cases []struct {
			ID       string
			Files    map[string]string
			Start    string
			Expected *string
		}
	}
	readFixture(t, "🧰️framework/🔨️modules/🏃️process/🧭️routing/🏠️workspace/🧪️tests/🧫️fixtures/🔣️.json", &corpus)
	for index, row := range corpus.Cases {
		base := t.TempDir()
		if row.Expected == nil {
			base = filepath.Join(append([]string{base}, strings.Split(strings.Repeat("a/", MaximumRootDepth), "/")[:MaximumRootDepth]...)...)
		}
		for name, content := range row.Files {
			path := filepath.Join(base, filepath.FromSlash(name))
			if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
				t.Fatal(err)
			}
		}
		start := filepath.Join(base, filepath.FromSlash(row.Start))
		if err := os.MkdirAll(start, 0o755); err != nil {
			t.Fatal(err)
		}
		got, err := ResolveRoot(start)
		if row.Expected == nil {
			if err == nil {
				t.Fatalf("%d %s: expected no root, got %s", index, row.ID, got)
			}
			continue
		}
		want, _ := filepath.EvalSymlinks(filepath.Join(base, *row.Expected))
		resolved, _ := filepath.EvalSymlinks(got)
		if err != nil || resolved != want {
			t.Fatalf("%d %s: got %q (%v) want %q", index, row.ID, got, err, want)
		}
	}
}

// 🧭️The environment chooses the start before the working directory.
func TestWorkspaceRootStart(t *testing.T) {
	cwd := func() (string, error) { return "/from/cwd", nil }
	for name, row := range map[string]struct {
		env  map[string]string
		want string
	}{"cwd": {nil, "/from/cwd"}, "semio": {map[string]string{"SEMIO_WORKSPACE_ROOT": " /x "}, "/x"}, "repo": {map[string]string{"REPO_ROOT": "/y"}, "/y"}, "semio wins": {map[string]string{"SEMIO_WORKSPACE_ROOT": "/x", "REPO_ROOT": "/y"}, "/x"}} {
		got, err := StartDirectory(environment(row.env), cwd)
		if err != nil || filepath.ToSlash(got) != row.want && !strings.HasSuffix(filepath.ToSlash(got), row.want) {
			t.Fatalf("%s: %q %v", name, got, err)
		}
	}
}

// #endregion 🧭️Root
