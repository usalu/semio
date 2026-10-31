// 🔬️ Tests of the workspace domain, split out of the pre-split godfile suite.

package workspace

import (
	"encoding/json"
	"context"
	"runtime"
	"time"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
	"unicode/utf16"
)

// 🧭️FixtureOrigin is the directory of the module under test. The external sandbox `TestMain` assigns it from `sandbox.Origin`, because this package cannot import the sandbox without a cycle.
var FixtureOrigin string

// 🧱️fenceRoot gives a test directory a `.git` marker so root discovery ends in it even when the directory lies inside a checkout.
func fenceRoot(t *testing.T, dir string) string {
	t.Helper()
	if err := os.MkdirAll(filepath.Join(dir, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	return dir
}

func TestRepoMetaDirUsesSemioRoot(t *testing.T) {
	root := CanonicalPath(fenceRoot(t, t.TempDir()))
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
	tmpDirA := fenceRoot(t, t.TempDir())
	tmpDirB := fenceRoot(t, t.TempDir())
	oldRoot := GetRootDir()
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
	root := t.TempDir()
	if output, err := exec.Command("git", "-C", root, "init", "-q").CombinedOutput(); err != nil {
		t.Fatalf("git init: %v\n%s", err, output)
	}
	if err := os.WriteFile(filepath.Join(root, ".gitignore"), []byte("*.exe\n.🧬semio/🦑️repo/⚡️cache/\nserver\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	previous := GetRootDir()
	SetRootDir(root)
	t.Cleanup(func() { SetRootDir(previous) })
	artifacts := []string{
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo",
		".🧬semio/🦑️repo/⚡️cache/🗃️bin/repo.exe",
		"modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server",
		"modules/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server.exe",
		"coda/example/.coda/validators/programming.exe",
	}
	ignored := GetGitIgnoredSet(root, append(artifacts, "src/kept.go"))
	for _, path := range artifacts {
		if !ignored[path] {
			t.Errorf("expected build artifact %q to be gitignored", path)
		}
	}
	if ignored["src/kept.go"] {
		t.Error("src/kept.go must not be gitignored")
	}
}

func TestSetRootDirCanonicalizesToRepoRoot(t *testing.T) {
	repoRoot := CanonicalPath(t.TempDir())
	nested := filepath.Join(repoRoot, "a", "b", "c")
	if err := os.MkdirAll(filepath.Join(repoRoot, ".git"), 0755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	if err := os.MkdirAll(nested, 0755); err != nil {
		t.Fatalf("mkdir nested: %v", err)
	}
	oldRoot := GetRootDir()
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

func TestDotnetFormatterProjects(t *testing.T) {
	var vectors struct {
		Cases []struct {
			Name     string
			Source   string
			Files    []string
			Projects []string
		}
	}
	data, err := os.ReadFile(filepath.Join(FixtureOrigin, "..", "..", "🧪️tests", "🧫️fixtures", "📡️dotnet-format-owners.json"))
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(data, &vectors); err != nil {
		t.Fatal(err)
	}
	for _, row := range vectors.Cases {
		t.Run(row.Name, func(t *testing.T) {
			root := t.TempDir()
			for _, file := range row.Files {
				path := filepath.Join(root, filepath.FromSlash(file))
				if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
					t.Fatal(err)
				}
				if err := os.WriteFile(path, []byte(""), 0644); err != nil {
					t.Fatal(err)
				}
			}
			got := DotnetFormatterProjects(root, row.Source)
			if !reflect.DeepEqual(got, row.Projects) {
				t.Fatalf("projects = %v, want %v", got, row.Projects)
			}
		})
	}
}

func TestIdeConfigurationPaths(t *testing.T) {
	data, err := os.ReadFile(filepath.Join(FixtureOrigin, "..", "..", "..", "🎛️dashboard", "🧪️tests", "🧫️fixtures", "🧭️authority", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		PathCases []struct {
			Path     string
			Accepted bool
		}
		InventoryPolicy struct {
			BoundaryDirectories []string
			ExcludedRoots       []string
		}
		InventoryCases []struct {
			Path      string
			Inspected bool
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.PathCases {
		for _, path := range []string{row.Path, strings.ReplaceAll(strings.ToUpper(row.Path), "/", "\\")} {
			if got := IsIdeConfigurationPath(path); got == row.Accepted {
				t.Errorf("%s: forbidden = %v", path, got)
			}
		}
	}
	if !reflect.DeepEqual(ideInventoryBoundaryDirectories, fixture.InventoryPolicy.BoundaryDirectories) || !reflect.DeepEqual(ideInventoryExcludedRoots, fixture.InventoryPolicy.ExcludedRoots) {
		t.Fatal("Go inventory differs from shared authority policy")
	}
	parent := os.Getenv("SEMIO_TEST_ARTIFACT_DIR")
	if parent == "" {
		t.Fatal("Caller-owned authority inventory artifacts required")
	}
	if err := os.MkdirAll(parent, 0755); err != nil {
		t.Fatal(err)
	}
	root, err := os.MkdirTemp(parent, "go-authority-")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(root)
	for _, row := range fixture.InventoryCases {
		absolute := filepath.Join(root, filepath.FromSlash(row.Path))
		if len(utf16.Encode([]rune(absolute))) > 256 {
			t.Fatal("Portable authority fixture path capacity exceeded")
		}
		if err := os.MkdirAll(filepath.Dir(absolute), 0755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(absolute, []byte(""), 0644); err != nil {
			t.Fatal(err)
		}
	}
	for attempt := 0; attempt < 2; attempt++ {
		paths, err := IdeConfigurationPaths(root, "")
		if err != nil {
			t.Fatal(err)
		}
		for _, row := range fixture.InventoryCases {
			found := false
			for _, path := range paths {
				if path == filepath.ToSlash(filepath.Dir(row.Path)) {
					found = true
				}
			}
			if found != row.Inspected {
				t.Errorf("%s: inspected = %v", row.Path, found)
			}
		}
	}
	if _, err := IdeConfigurationPaths(root, "missing"); err == nil {
		t.Fatal("Missing inventory root must fail closed")
	}
}

func TestRunToolRunsRegisteredToolsAndRefusesUnknownOnes(t *testing.T) {
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git is not installed")
	}
	result, err := RunTool("git", []string{"--version"}, ToolRunOptions{})
	if err != nil || result.Status != 0 || !strings.HasPrefix(result.Stdout, "git version") {
		t.Fatalf("RunTool(git) = %+v, %v", result, err)
	}
	failed, err := RunTool("git", []string{"no-such-subcommand-for-the-registry"}, ToolRunOptions{})
	if err != nil || failed.Status == 0 {
		t.Fatalf("a failing tool must answer its status, got %+v, %v", failed, err)
	}
	if _, err := RunTool("no-such-tool-for-the-registry", nil, ToolRunOptions{}); err == nil {
		t.Fatal("an unknown tool id must be refused")
	}
}

func TestToolCommandMarksRegisteredRunsAndDropsIdeInjections(t *testing.T) {
	t.Setenv("VSCODE_INSPECTOR_OPTIONS", "x")
	command := ToolCommand("git", "--version")
	if command.Env == nil {
		t.Skip("no tool registry above the working directory")
	}
	joined := strings.Join(command.Env, "\n")
	if !strings.Contains(joined, "SEMIO_TOOL_RUN=git") || strings.Contains(joined, "VSCODE_INSPECTOR_OPTIONS") {
		t.Fatalf("the registry environment was not applied: %s", joined)
	}
	if plain := ToolCommand("echo", "x"); plain.Env != nil {
		t.Fatal("an unregistered command must keep the process environment")
	}
}

func TestDeclaredInvocationProjectionMatchesSharedCorpus(t *testing.T) {
	directory := FixtureOrigin
	if directory == "" { t.Fatal("sandbox fixture origin required") }
	for {
		if _, err := os.Stat(filepath.Join(directory, "MODULE.bazel")); err == nil { break }
		parent := filepath.Dir(directory)
		if parent == directory { t.Fatal("repository fixture root required") }
		directory = parent
	}
	text, err := os.ReadFile(filepath.Join(directory, "🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔎️resolve/📣️invocations/🔣️.json"))
	if err != nil { t.Fatal(err) }
	var corpus struct {
		Projections []struct { Commands []string; Invocations []registeredInvocation; Requested string; Selected string; Prefix []string }
		Invalid []json.RawMessage
		InvalidLists []json.RawMessage
	}
	if err := json.Unmarshal(text, &corpus); err != nil { t.Fatal(err) }
	if len(corpus.Projections) != 8 { t.Fatal("complete shared invocation projections required") }
	for _, row := range corpus.Projections {
		tool := registeredTool{Commands: row.Commands, Invocations: row.Invocations}
		if !validRegisteredInvocations(toolRegistryDocument{Tools: []registeredTool{tool}}) { t.Fatal("valid invocation refused") }
		selected, prefix := registeredToolInvocation(tool, row.Requested)
		if selected != row.Selected || strings.Join(prefix, "\x00") != strings.Join(row.Prefix, "\x00") { t.Fatalf("invocation projection = %q %q", selected, prefix) }
		if len(prefix) > 0 { prefix[0] = "changed"; if tool.Invocations[0].Prefix[0] == "changed" { t.Fatal("invocation prefix was aliased") } }
	}
	for _, invalid := range corpus.Invalid {
		var invocation registeredInvocation
		if json.Unmarshal(invalid, &invocation) == nil && validRegisteredInvocations(toolRegistryDocument{Tools: []registeredTool{{Commands: []string{"bun", "bunx"}, Invocations: []registeredInvocation{invocation}}}}) { t.Fatal("invalid invocation admitted") }
	}
	for _, invalid := range corpus.InvalidLists {
		bytes, err := json.Marshal(map[string]any{"tools": []any{map[string]any{"id": "fixture", "commands": []string{"bun", "bunx"}, "invocations": invalid}}})
		if err != nil { t.Fatal(err) }
		var document toolRegistryDocument
		if json.Unmarshal(bytes, &document) == nil && validRegisteredInvocations(document) { t.Fatal("invalid invocation list admitted") }
	}
}

func TestMappedRegisteredCommandKeepsPrefixBeforeCallerArguments(t *testing.T) {
	arguments := []string{"--version"}
	command := ToolCommand("bunx", arguments...)
	if len(command.Args) != 3 || command.Args[1] != "x" || command.Args[2] != "--version" { t.Fatalf("mapped argv = %q", command.Args) }
	if arguments[0] != "--version" { t.Fatal("caller arguments were mutated") }
	if !strings.Contains(strings.Join(command.Env, "\n"), "SEMIO_TOOL_RUN=bun") { t.Fatal("registered invocation environment required") }
}

func TestRegisteredWindowsInvocationArguments(t *testing.T) {
	checkWindowsBatchEncodingMatchesSharedOracle(t)
	native, nativeArgs, nativeRefusal := registeredToolCommandArguments("C:/bin/bun.exe", []string{"x"}, []string{"a\"b", "%VALUE%", "two words"}, true, "cmd.exe")
	if nativeRefusal != nil || native != "C:/bin/bun.exe" || !reflect.DeepEqual(nativeArgs, []string{"x", "a\"b", "%VALUE%", "two words"}) { t.Fatal("ordinary native executable argv changed") }
	plain, plainArgs, plainRefusal := registeredToolCommandArguments("C:/bin/explicit.cmd", nil, []string{"%VALUE%"}, false, "cmd.exe")
	if plainRefusal != nil || plain != "C:/bin/explicit.cmd" || !reflect.DeepEqual(plainArgs, []string{"%VALUE%"}) { t.Fatal("explicit or guard-off routing changed") }
	arguments := []string{"package"}
	program, actual, refusal := registeredToolCommandArguments("C:/bin/bun.cmd", []string{"x"}, arguments, true, "C:/Windows/System32/cmd.exe")
	if refusal != nil || program != "C:/Windows/System32/cmd.exe" || len(actual) != 5 || !reflect.DeepEqual(actual[:4], []string{"/d", "/v:off", "/s", "/c"}) { t.Fatalf("batch invocation = %q %q", program, actual) }
	if arguments[0] != "package" { t.Fatal("caller arguments changed") }
	var document toolRegistryDocument
	if json.Unmarshal([]byte(`{"tools":[{"id":"bun","commands":["bun","bunx"],"invocations":null}]}`), &document) == nil { t.Fatal("explicit null invocation policy admitted") }
}

func checkWindowsBatchEncodingMatchesSharedOracle(t *testing.T) {
	directory := FixtureOrigin
	if directory == "" { t.Fatal("sandbox fixture origin required") }
	for {
		if _, err := os.Stat(filepath.Join(directory, "MODULE.bazel")); err == nil { break }
		parent := filepath.Dir(directory)
		if parent == directory { t.Fatal("repository root required") }
		directory = parent
	}
	text, err := os.ReadFile(filepath.Join(directory, "🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔎️resolve/📣️invocations/🪟️batch/🔣️.json"))
	if err != nil { t.Fatal(err) }
	var corpus struct { Cases []struct { ID string; Program string; Args []string; Accepted bool; Expected *string; Reason string } }
	if err := json.Unmarshal(text, &corpus); err != nil { t.Fatal(err) }
	for _, row := range corpus.Cases {
		line, refusal := registeredBatchFragment(row.Program, row.Args)
		if row.Accepted { if refusal != nil || row.Expected == nil || line != *row.Expected { t.Fatalf("%s: encoding %q refusal %v", row.ID, line, refusal) } } else {
			capability, ok := refusal.(*ToolCapabilityError)
			if !ok || capability.Reason != row.Reason { t.Fatalf("%s: typed refusal %v", row.ID, refusal) }
		}
	}
}

func TestWindowsInvocationRecorder(t *testing.T) {
	if os.Getenv("SEMIO_TEST_WINDOWS_RECORDER") != "1" { return }
	for index, value := range os.Args { if value == "--" { if err := json.NewEncoder(os.Stdout).Encode(os.Args[index+1:]); err != nil { os.Exit(2) }; os.Exit(0) } }
	os.Exit(3)
}

func TestWindowsRegisteredBatchLiteralArguments(t *testing.T) {
 if runtime.GOOS != "windows" { t.Skip("physical Windows process required") }
 root := filepath.Join(t.TempDir(), "space Ω^ folder")
 if err := os.MkdirAll(root, 0700); err != nil { t.Fatal(err) }
 executable, err := os.Executable()
 if err != nil { t.Fatal(err) }
 script := filepath.Join(root, "recorder.cmd")
 if err := os.WriteFile(script, []byte("@echo off\r\n\""+executable+"\" -test.run=TestWindowsInvocationRecorder -- %*\r\n"), 0600); err != nil { t.Fatal(err) }
 t.Setenv("SEMIO_TOOL_BUN", script)
 t.Setenv("SEMIO_TEST_WINDOWS_RECORDER", "1")
 ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
 defer cancel()
 arguments := []string{"two words", "Grüße Ω", "a&b|c(d)^e!f", "", "C:\\path\\"}
 command := ToolCommandContext(ctx, "bunx", arguments...)
 output, err := command.Output()
 if err != nil { t.Fatalf("physical batch: %v", err) }
 var observed []string
 if err := json.Unmarshal(output, &observed); err != nil { t.Fatal(err) }
 if !reflect.DeepEqual(observed, append([]string{"x"}, arguments...)) { t.Fatalf("literal batch argv = %q", observed) }
 for _, refused := range []string{"a\"b", "%PATH%", "line\nnext"} {
  if _, ok := ToolCommandContext(ctx, "bunx", refused).Err.(*ToolCapabilityError); !ok { t.Fatal("unsupported batch input must be a typed prelaunch refusal") }
 }
}
