// 🔬️ Tests of the workspace domain, split out of the pre-split godfile suite.

package workspace

import (
	bufio "bufio"
	context "context"
	io "io"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"
)

func TestRepoMetaDirUsesSemioRoot(t *testing.T) {
	root := t.TempDir()
	previous := GetRootDir()
	SetRootDir(root)
	t.Cleanup(func() { SetRootDir(previous) })

	got := GetRepoMetaDir()
	want := filepath.Join(root, ".🧬semio", "🦑️repo")
	if got != want {
		t.Fatalf("GetRepoMetaDir() = %q, want %q", got, want)
	}
	if strings.Contains(got, ".🦑️repo") {
		t.Fatalf("GetRepoMetaDir() retained legacy path: %q", got)
	}
}

func TestMatchesIgnorePatternDirectoryCoverage(t *testing.T) {
	cases := []struct {
		name    string
		path    string
		isDir   bool
		pattern string
		want    bool
	}{
		{"dir by recursive pattern", "node_modules", true, "**/node_modules/**", true},
		{"nested file by recursive pattern", "a/node_modules/pkg/index.js", false, "**/node_modules/**", true},
		{"dir mismatch", "src", true, "**/node_modules/**", false},
		{"exact dir pattern", "dist", true, "dist/**", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := matchesIgnorePattern(tc.path, tc.isDir, tc.pattern)
			if got != tc.want {
				t.Fatalf("matchesIgnorePattern(%q, %t, %q) = %t, want %t", tc.path, tc.isDir, tc.pattern, got, tc.want)
			}
		})
	}
}

func TestGlobByExtensionSkipsIgnoredDirectoryRoot(t *testing.T) {
	tempRoot := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tempRoot, "src"), 0755); err != nil {
		t.Fatalf("failed to create src dir: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tempRoot, "node_modules", "pkg"), 0755); err != nil {
		t.Fatalf("failed to create node_modules dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempRoot, "src", "kept.go"), []byte("package src\n"), 0644); err != nil {
		t.Fatalf("failed to create kept.go: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempRoot, "node_modules", "pkg", "ignored.go"), []byte("package pkg\n"), 0644); err != nil {
		t.Fatalf("failed to create ignored.go: %v", err)
	}

	files, err := GlobByExtension(tempRoot, "**/*", []string{"go"}, []string{"**/node_modules/**"}, false)
	if err != nil {
		t.Fatalf("globByExtension failed: %v", err)
	}
	if len(files) != 1 || files[0] != "src/kept.go" {
		t.Fatalf("expected only src/kept.go, got %v", files)
	}
}

func TestFlat(t *testing.T) {
	cases := []struct {
		input    string
		expected string
	}{
		{"repo", "repo"},
		{"Design.tsx", "designtsx"},
		{".devcontainer", "devcontainer"},
		{"devcontainer.json", "devcontainerjson"},
		{"RUNNING-SKETCHPAD", "runningsketchpad"},
		{"R26-02-1", "r26021"},
		{"compose.ts", "composets"},
		{"State Managment", "statemanagment"},
		{"createSketchpadStore", "createsketchpadstore"},
	}
	for _, tc := range cases {
		t.Run(tc.input, func(t *testing.T) {
			got := Flat(tc.input)
			if got != tc.expected {
				t.Errorf("Flat(%q): expected %q, got %q", tc.input, tc.expected, got)
			}
		})
	}
}

func TestPathToUriPath(t *testing.T) {
	tests := []struct {
		path string
		want string
	}{
		{"compose/js/src", "compose/js/src"},
		{"repo/client/main.go", "repo/client/main.go"},
		{"test.txt", "test.txt"},
		{"a b/c d", "a%20b/c%20d"},
	}
	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			if got := PathToUriPath(tt.path); got != tt.want {
				t.Errorf("PathToUriPath(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestPathFromUriPath(t *testing.T) {
	tests := []struct {
		uriPath string
		want    string
	}{
		{"compose/js/src", "compose/js/src"},
		{"repo/client/main.go", "repo/client/main.go"},
		{"a%20b/c%20d", "a b/c d"},
	}
	for _, tt := range tests {
		t.Run(tt.uriPath, func(t *testing.T) {
			if got := PathFromUriPath(tt.uriPath); got != tt.want {
				t.Errorf("PathFromUriPath(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestTitleizeSlug(t *testing.T) {
	tests := []struct {
		name string
		slug string
		want string
	}{
		{"single word", "code", "Code"},
		{"two words", "inline-comment", "Inline Comment"},
		{"three words", "missing-region-marker", "Missing Region Marker"},
		{"already titleized", "Code", "Code"},
		{"uppercase input", "CODE", "Code"},
		{"empty", "", ""},
		{"single char", "a", "A"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := TitleizeSlug(tt.slug)
			if got != tt.want {
				t.Errorf("TitleizeSlug(%q) = %q, want %q", tt.slug, got, tt.want)
			}
		})
	}
}

func TestStatutePathToIdValue(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"single segment", "code", "Code"},
		{"two segments", "code/inline-comment", "Code#Inline Comment"},
		{"three segments", "code/file/missing-header-region", "Code#File#Missing Header Region"},
		{"four segments", "code/header/region/nested", "Code#Header#Region#Nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatutePathToIdValue(tt.path)
			if got != tt.want {
				t.Errorf("StatutePathToIdValue(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestStatuteIdValueToPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"single segment", "Code", "code"},
		{"two segments", "Code#Inline Comment", "code/inline-comment"},
		{"three segments", "Code#File#Missing Header Region", "code/file/missing-header-region"},
		{"four segments", "Code#Header#Region#Nested", "code/header/region/nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdValueToPath(tt.value)
			if got != tt.want {
				t.Errorf("StatuteIdValueToPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestStatutePathIdValueRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		path string
	}{
		{"single segment", "code"},
		{"two segments", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			idValue := StatutePathToIdValue(tt.path)
			gotPath := StatuteIdValueToPath(idValue)
			if gotPath != tt.path {
				t.Errorf("round trip failed: path %q -> idValue %q -> path %q", tt.path, idValue, gotPath)
			}
		})
	}
}

func TestSetRootDirResetsGitignoreCache(t *testing.T) {
	tmpDirA := t.TempDir()
	tmpDirB := t.TempDir()
	oldRoot := RootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(tmpDirA)
	_ = IsGitIgnored("any.txt")
	os.WriteFile(filepath.Join(tmpDirB, ".gitignore"), []byte("node_modules/\n"), 0644)
	SetRootDir(tmpDirB)
	ignored := IsGitIgnored(filepath.Join(tmpDirB, "node_modules", "x.ts"))
	if !ignored {
		t.Error("expected SetRootDir to refresh gitignore cache for new root")
	}
}

func TestBuildBinaryArtifactsGitIgnored(t *testing.T) {
	artifacts := []string{
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo",
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo.exe",
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp",
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp.exe",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server",
		"🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server.exe",
		"coda/example/compose-blnbo-roomprogram/.coda/validators/programming.exe",
	}
	ignored := GetGitIgnoredSet(artifacts)
	for _, path := range artifacts {
		if !ignored[path] {
			t.Errorf("expected build artifact %q to be gitignored", path)
		}
	}
}

func TestSetRootDirCanonicalizesToRepoRoot(t *testing.T) {
	repoRoot := t.TempDir()
	nested := filepath.Join(repoRoot, "a", "b", "c")
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(nested, 0755); err != nil {
		t.Fatalf("mkdir nested: %v", err)
	}
	oldRoot := RootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(nested)
	if got := GetRootDir(); got != repoRoot {
		t.Fatalf("expected repo root %q, got %q", repoRoot, got)
	}
	if got := GetRepoMetaDir(); got != filepath.Join(repoRoot, ".🧬semio", "🦑️repo") {
		t.Fatalf("expected repo meta dir at monorepo root, got %q", got)
	}
}

func TestSplitCommandSegments(t *testing.T) {
	cases := []struct {
		cmd      string
		expected []string
	}{
		{"git checkout main", []string{"git checkout main"}},
		{"cd /tmp && git checkout main", []string{"cd /tmp", "git checkout main"}},
		{"echo done; git stash", []string{"echo done", "git stash"}},
		{"ls | grep foo", []string{"ls", "grep foo"}},
		{"a || b", []string{"a", "b"}},
		{"  ", []string{}},
		{"", []string{}},
	}
	for _, tc := range cases {
		t.Run(tc.cmd, func(t *testing.T) {
			got := SplitCommandSegments(tc.cmd)
			if len(got) != len(tc.expected) {
				t.Fatalf("expected %v, got %v", tc.expected, got)
			}
			for i := range tc.expected {
				if got[i] != tc.expected[i] {
					t.Errorf("segment %d: expected %q, got %q", i, tc.expected[i], got[i])
				}
			}
		})
	}
}

func TestWriteWarningfUsesStderr(t *testing.T) {
	read, write, err := os.Pipe()
	if err != nil {
		t.Fatal(err)
	}
	previous := os.Stderr
	os.Stderr = write
	defer func() { os.Stderr = previous }()

	WriteWarningf("Failed to update: %s", "boom")
	if err := write.Close(); err != nil {
		t.Fatal(err)
	}
	os.Stderr = previous
	output, err := io.ReadAll(read)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := string(output), "Warning: Failed to update: boom\n"; got != want {
		t.Fatalf("warning output = %q, want %q", got, want)
	}
}

func TestMcpStdioInitializeHandshake(t *testing.T) {
	repoRoot := FindRepoRoot(".")
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
	if !strings.Contains(line, `"result"`) || !strings.Contains(line, `jsonrpc`) || !strings.Contains(line, `"name":"repo-cursor"`) {
		t.Fatalf("unexpected initialize response: %s", line)
	}
}
