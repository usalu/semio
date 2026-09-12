package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/usalu/semio/repo/client/internal/command"
)

type goDispatchSourceVector struct {
	Directory string `json:"directory"`
	Module    string `json:"module"`
	Package   string `json:"package"`
	Content   string `json:"content"`
}

type goDispatchTestVector struct {
	Directory string `json:"directory"`
	Package   string `json:"package"`
	Content   string `json:"content"`
	File      string `json:"file"`
}

type goDispatchCancellationVector struct {
	TestName string `json:"testName"`
	Ready    string `json:"ready"`
	Marker   string `json:"marker"`
	Content  string `json:"content"`
}

type goDispatchVector struct {
	Contract           string                       `json:"contract"`
	Module             string                       `json:"module"`
	Dependency         goDispatchSourceVector       `json:"dependency"`
	Source             goDispatchSourceVector       `json:"source"`
	Tests              goDispatchTestVector         `json:"tests"`
	Cancellation       goDispatchCancellationVector `json:"cancellation"`
	ExpectedBundle     []string                     `json:"expectedBundle"`
	ExpectedDefinition []string                     `json:"expectedDefinition"`
}

func readGoDispatchVector(t *testing.T) goDispatchVector {
	t.Helper()
	path := filepath.Join(rootDir, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client", "⌨️cli", "🧫️fixtures", "🚦️test-dispatch", "🔣️.json")
	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	var vector goDispatchVector
	if err := json.Unmarshal(content, &vector); err != nil {
		t.Fatal(err)
	}
	if vector.Contract != "canonical-go-test-dispatch-v1" {
		t.Fatalf("contract = %q", vector.Contract)
	}
	return vector
}

func materializeGoDispatchVector(t *testing.T, vector goDispatchVector) (string, string) {
	t.Helper()
	return materializeGoDispatchVectorAt(t, vector, t.TempDir())
}

func materializeGoDispatchVectorAt(t *testing.T, vector goDispatchVector, root string) (string, string) {
	t.Helper()
	dependencyRoot := filepath.Join(root, vector.Dependency.Directory)
	dependencyDomain := filepath.Join(dependencyRoot, "⚡️effects")
	sourceDomain := filepath.Join(root, vector.Source.Directory)
	testDomain := filepath.Join(root, vector.Tests.Directory)
	for _, directory := range []string{dependencyDomain, sourceDomain, testDomain} {
		if err := os.MkdirAll(directory, 0o755); err != nil {
			t.Fatal(err)
		}
	}
	writes := map[string]string{
		filepath.Join(root, "go.mod"):                fmt.Sprintf("module %s\n\ngo 1.25\n\nrequire %s v0.0.0\n\nreplace %s => ./%s\n", vector.Module, vector.Dependency.Module, vector.Dependency.Module, vector.Dependency.Directory),
		filepath.Join(dependencyRoot, "go.mod"):      fmt.Sprintf("module %s\n\ngo 1.25\n", vector.Dependency.Module),
		filepath.Join(dependencyDomain, "🐹️.go"):     fmt.Sprintf("package %s\n%s\n", vector.Dependency.Package, vector.Dependency.Content),
		filepath.Join(sourceDomain, "🐹️.go"):         fmt.Sprintf("package %s\nimport offset %q\n%s\n", vector.Source.Package, vector.Source.Module, vector.Source.Content),
		filepath.Join(testDomain, vector.Tests.File): fmt.Sprintf("package %s\nimport (\"os\"; \"path/filepath\"; \"strconv\"; \"testing\"; \"time\")\n%s\n%s\n", vector.Tests.Package, vector.Tests.Content, vector.Cancellation.Content),
	}
	for path, content := range writes {
		if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root, filepath.Join(testDomain, vector.Tests.File)
}

func runGoDispatchCase(t *testing.T, markers string, run func(*command.Command) error) (string, string) {
	t.Helper()
	if err := os.RemoveAll(markers); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(markers, 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv("SEMIO_GO_DISPATCH_MARKERS", markers)
	t.Setenv("GOWORK", "off")
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	var stdout, stderr bytes.Buffer
	cmd := &command.Command{}
	cmd.SetContext(ctx)
	cmd.SetOut(&stdout)
	cmd.SetErr(&stderr)
	if err := run(cmd); err != nil {
		t.Fatalf("dispatch failed: %v\nstdout:\n%s\nstderr:\n%s", err, stdout.String(), stderr.String())
	}
	return stdout.String(), stderr.String()
}

func goDispatchMarkers(t *testing.T, root string) []string {
	t.Helper()
	entries, err := os.ReadDir(root)
	if err != nil {
		t.Fatal(err)
	}
	names := make([]string, 0, len(entries))
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	sort.Strings(names)
	return names
}

func TestCanonicalGoTestDispatcher(t *testing.T) {
	vector := readGoDispatchVector(t)
	moduleRoot, testFile := materializeGoDispatchVector(t, vector)
	t.Setenv("GOWORK", "off")
	oracle := exec.Command("go", "test", "./...")
	oracle.Dir = moduleRoot
	oracleOutput, oracleErr := oracle.CombinedOutput()
	if oracleErr == nil || !strings.Contains(string(oracleOutput), "malformed import path") {
		t.Fatalf("recursive native oracle must reject semantic package discovery: err=%v\n%s", oracleErr, oracleOutput)
	}
	markers := filepath.Join(t.TempDir(), "markers")
	stdout, stderr := runGoDispatchCase(t, markers, func(cmd *command.Command) error {
		return runTestScope(testScope{Kind: testScopeBundle, BundleRoot: moduleRoot, Language: "go"}, cmd)
	})
	wantBundle := append([]string(nil), vector.ExpectedBundle...)
	sort.Strings(wantBundle)
	if got := goDispatchMarkers(t, markers); strings.Join(got, ",") != strings.Join(wantBundle, ",") {
		t.Fatalf("bundle markers = %v, want %v\nstdout:\n%s\nstderr:\n%s", got, wantBundle, stdout, stderr)
	}
	if !strings.Contains(stdout, "Running: bun ") || !strings.Contains(stdout, "ok  \texample.com/semio/dispatch-fixture") {
		t.Fatalf("bundle output was not streamed through the dispatcher:\n%s", stdout)
	}
	stdout, stderr = runGoDispatchCase(t, markers, func(cmd *command.Command) error {
		return runTestScope(testScope{Kind: testScopeDefinition, FilePath: testFile, BundleRoot: moduleRoot, TestName: "testselecteddefinition", Language: "go"}, cmd)
	})
	wantDefinition := append([]string(nil), vector.ExpectedDefinition...)
	sort.Strings(wantDefinition)
	if got := goDispatchMarkers(t, markers); strings.Join(got, ",") != strings.Join(wantDefinition, ",") {
		t.Fatalf("definition markers = %v, want %v\nstdout:\n%s\nstderr:\n%s", got, wantDefinition, stdout, stderr)
	}
	if !strings.Contains(stdout, "=== RUN   TestSelectedDefinition") || strings.Contains(stdout, "=== RUN   TestBundlePeer") {
		t.Fatalf("definition output did not preserve the exact Go filter:\n%s", stdout)
	}
}

type goDispatchProcessRow struct {
	pid  int
	ppid int
	pgid int
}

func goDispatchProcessTable(t *testing.T) []goDispatchProcessRow {
	t.Helper()
	output, err := exec.Command("ps", "-axo", "pid=,ppid=,pgid=").Output()
	if err != nil {
		t.Fatal(err)
	}
	rows := make([]goDispatchProcessRow, 0)
	for _, line := range strings.Split(string(output), "\n") {
		fields := strings.Fields(line)
		if len(fields) != 3 {
			continue
		}
		pid, pidErr := strconv.Atoi(fields[0])
		ppid, ppidErr := strconv.Atoi(fields[1])
		pgid, pgidErr := strconv.Atoi(fields[2])
		if pidErr == nil && ppidErr == nil && pgidErr == nil {
			rows = append(rows, goDispatchProcessRow{pid: pid, ppid: ppid, pgid: pgid})
		}
	}
	return rows
}

func goDispatchOwnedProcesses(rows []goDispatchProcessRow, root int) []goDispatchProcessRow {
	owned := map[int]bool{root: true}
	result := make([]goDispatchProcessRow, 0)
	for changed := true; changed; {
		changed = false
		for _, row := range rows {
			if owned[row.pid] || !owned[row.ppid] {
				continue
			}
			owned[row.pid] = true
			result = append(result, row)
			changed = true
		}
	}
	return result
}

func cleanupGoDispatchProcesses(rows []goDispatchProcessRow) {
	owned := make(map[int]bool, len(rows))
	for _, row := range rows {
		owned[row.pid] = true
	}
	groups := make(map[int]bool)
	for _, row := range rows {
		if owned[row.pgid] {
			groups[row.pgid] = true
		}
	}
	for group := range groups {
		_ = exec.Command("kill", "-KILL", "-"+strconv.Itoa(group)).Run()
	}
	for _, row := range rows {
		_ = exec.Command("kill", "-KILL", strconv.Itoa(row.pid)).Run()
	}
}

func waitForGoDispatchFile(t *testing.T, path string, timeout time.Duration) {
	t.Helper()
	deadline := time.Now().Add(timeout)
	for time.Now().Before(deadline) {
		if _, err := os.Stat(path); err == nil {
			return
		}
		time.Sleep(20 * time.Millisecond)
	}
	t.Fatalf("timed out waiting for %s", path)
}

func assertGoDispatchProcessesGone(t *testing.T, observed []goDispatchProcessRow) {
	t.Helper()
	deadline := time.Now().Add(3 * time.Second)
	for time.Now().Before(deadline) {
		current := goDispatchProcessTable(t)
		live := make(map[int]bool, len(current))
		for _, row := range current {
			live[row.pid] = true
		}
		remaining := false
		for _, row := range observed {
			remaining = remaining || live[row.pid]
		}
		if !remaining {
			return
		}
		time.Sleep(20 * time.Millisecond)
	}
	cleanupGoDispatchProcesses(observed)
	t.Fatalf("owned dispatch descendants survived cancellation: %v", observed)
}

func goDispatchOverlayRuns(t *testing.T) []string {
	t.Helper()
	owner := os.Getenv("SEMIO_GO_OVERLAY_OWNER")
	if owner == "" {
		return nil
	}
	entries, err := os.ReadDir(owner)
	if err != nil {
		t.Fatal(err)
	}
	runs := make([]string, 0)
	for _, entry := range entries {
		if strings.HasPrefix(entry.Name(), "run-") {
			runs = append(runs, entry.Name())
		}
	}
	sort.Strings(runs)
	return runs
}

func TestCanonicalGoTestDispatcherCancellation(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("native process-table oracle uses ps; Windows behavior is cross-compiled")
	}
	vector := readGoDispatchVector(t)
	moduleRoot, testFile := materializeGoDispatchVector(t, vector)
	cancellationRoot := filepath.Join(t.TempDir(), "cancellation")
	t.Setenv("SEMIO_GO_CANCELLATION_ROOT", cancellationRoot)
	t.Setenv("SEMIO_GO_DISPATCH_MARKERS", filepath.Join(t.TempDir(), "markers"))
	t.Setenv("SEMIO_TEST_BUDGET_MS", "30000")
	t.Setenv("GOWORK", "off")
	beforeOverlays := goDispatchOverlayRuns(t)
	ctx, cancel := context.WithCancel(context.Background())
	var stdout, stderr bytes.Buffer
	cmd := &command.Command{}
	cmd.SetContext(ctx)
	cmd.SetOut(&stdout)
	cmd.SetErr(&stderr)
	result := make(chan error, 1)
	go func() {
		result <- runTestScope(testScope{Kind: testScopeDefinition, FilePath: testFile, BundleRoot: moduleRoot, TestName: strings.ToLower(vector.Cancellation.TestName), Language: "go"}, cmd)
	}()
	waitForGoDispatchFile(t, filepath.Join(cancellationRoot, vector.Cancellation.Ready), 20*time.Second)
	observed := goDispatchOwnedProcesses(goDispatchProcessTable(t), os.Getpid())
	if len(observed) < 3 {
		cleanupGoDispatchProcesses(observed)
		t.Fatalf("expected dispatcher, go tool, and native test descendants; got %v", observed)
	}
	started := time.Now()
	cancel()
	var err error
	select {
	case err = <-result:
	case <-time.After(5 * time.Second):
		cleanupGoDispatchProcesses(observed)
		select {
		case <-result:
		case <-time.After(2 * time.Second):
		}
		t.Fatal("command context cancellation did not return within five seconds")
	}
	if err == nil || !strings.Contains(err.Error(), context.Canceled.Error()) {
		t.Fatalf("context cancellation error = %v", err)
	}
	elapsed := time.Since(started)
	if elapsed > 5*time.Second {
		t.Fatalf("context cancellation returned too slowly: %s", elapsed)
	}
	assertGoDispatchProcessesGone(t, observed)
	marker := filepath.Join(cancellationRoot, vector.Cancellation.Marker)
	before, readErr := os.ReadFile(marker)
	if readErr != nil {
		t.Fatal(readErr)
	}
	time.Sleep(200 * time.Millisecond)
	after, readErr := os.ReadFile(marker)
	if readErr != nil {
		t.Fatal(readErr)
	}
	if !bytes.Equal(before, after) {
		t.Fatalf("cancellation marker continued changing: %q -> %q", before, after)
	}
	if got := goDispatchOverlayRuns(t); strings.Join(got, ",") != strings.Join(beforeOverlays, ",") {
		t.Fatalf("nested overlay runs survived cancellation: before=%v after=%v", beforeOverlays, got)
	}
	if !strings.Contains(stdout.String(), "Running: bun ") || !strings.Contains(stdout.String(), vector.Cancellation.TestName) {
		t.Fatalf("selected cancellation output was not preserved:\n%s\nstderr:\n%s", stdout.String(), stderr.String())
	}
	t.Logf("[DEBUG] context cancellation returned in %s after observing %d descendants", elapsed, len(observed))
}

func TestCanonicalGoTestDispatcherSpawnError(t *testing.T) {
	cmd := &command.Command{}
	cmd.SetContext(context.Background())
	cmd.SetOut(&bytes.Buffer{})
	cmd.SetErr(&bytes.Buffer{})
	started := time.Now()
	err := runExternalCommand(t.TempDir(), "semio-command-that-does-not-exist", nil, cmd)
	if err == nil {
		t.Fatal("missing child executable returned success")
	}
	if time.Since(started) > 2*time.Second {
		t.Fatalf("spawn error returned too slowly: %s", time.Since(started))
	}
}

func TestCanonicalGoTestDispatcherNestedBudgetProbe(t *testing.T) {
	root := os.Getenv("SEMIO_GO_CANCELLATION_ROOT")
	if root == "" {
		t.Skip("nested budget probe requires an owned artifact root")
	}
	vector := readGoDispatchVector(t)
	moduleRoot, testFile := materializeGoDispatchVectorAt(t, vector, filepath.Join(root, "fixture"))
	cancellationRoot := filepath.Join(root, "cancellation")
	markers := filepath.Join(root, "markers")
	if err := os.MkdirAll(markers, 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv("SEMIO_GO_CANCELLATION_ROOT", cancellationRoot)
	t.Setenv("SEMIO_GO_DISPATCH_MARKERS", markers)
	t.Setenv("SEMIO_TEST_BUDGET_MS", "30000")
	t.Setenv("GOWORK", "off")
	cmd := &command.Command{}
	cmd.SetContext(context.Background())
	cmd.SetOut(os.Stdout)
	cmd.SetErr(os.Stderr)
	if err := runTestScope(testScope{Kind: testScopeDefinition, FilePath: testFile, BundleRoot: moduleRoot, TestName: strings.ToLower(vector.Cancellation.TestName), Language: "go"}, cmd); err != nil {
		t.Fatal(err)
	}
	t.Fatal("nested cancellation probe returned before its owner budget")
}
