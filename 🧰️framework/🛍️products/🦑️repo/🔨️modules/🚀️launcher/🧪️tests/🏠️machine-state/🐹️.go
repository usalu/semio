// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏠️machine-state proves that the Go spelling of the machine state path reads the table of the TypeScript one, moves `.cache` and `.nx` the same way, ignores a relative or overlong declaration, and that the launcher's tools partition and fleet store follow it.

// #endregion 🧲️Header

package launcher

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 🏠️The table, the variable and the limit of the Go spelling equal the schema of the TypeScript one.
func TestMachineStateTableEqualsTheSchema(t *testing.T) {
	raw, err := os.ReadFile("../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🧬️schema/🔣️.json")
	if err != nil {
		t.Skip("the TypeScript schema is not next to this copy")
	}
	var schema struct {
		Definitions struct {
			Protocol struct {
				Const struct {
					Environment string `json:"environment"`
					Moved       []struct {
						Prefix string   `json:"prefix"`
						To     []string `json:"to"`
					} `json:"moved"`
					Maximum int `json:"maximumDirectoryCharacters"`
				} `json:"const"`
			} `json:"Protocol"`
		} `json:"definitions"`
	}
	if json.Unmarshal(raw, &schema) != nil {
		t.Fatal("schema unreadable")
	}
	declared := schema.Definitions.Protocol.Const
	if declared.Environment != machineStateEnvironment || declared.Maximum != machineStateMaximumCharacters || len(declared.Moved) != len(machineStateMoved) {
		t.Fatalf("declared %+v, Go %s %d %v", declared, machineStateEnvironment, machineStateMaximumCharacters, machineStateMoved)
	}
	for at, row := range declared.Moved {
		if row.Prefix != machineStateMoved[at].prefix || strings.Join(row.To, "/") != strings.Join(machineStateMoved[at].to, "/") {
			t.Fatalf("row %d: %+v against %+v", at, row, machineStateMoved[at])
		}
	}
}

// 🏠️A declared state directory takes the paths below `.cache` and `.nx`; nothing else moves; an invalid declaration is none.
func TestMachinePathMovesOnlyTheState(t *testing.T) {
	root, state := filepath.FromSlash("/work/checkout"), filepath.FromSlash("/lab/s")
	declared := environment(map[string]string{machineStateEnvironment: state})
	cases := []struct {
		env      Environment
		segments []string
		want     string
	}{
		{declared, []string{".cache", "b", "t", "m"}, "/lab/s/b/t/m"},
		{declared, []string{".cache"}, "/lab/s"},
		{declared, []string{".nx", "workspace-data"}, "/lab/s/nx/workspace-data"},
		{declared, []string{"node_modules", ".cache", "x"}, "/work/checkout/node_modules/.cache/x"},
		{declared, []string{".cachex", "y"}, "/work/checkout/.cachex/y"},
		{declared, []string{"package.json"}, "/work/checkout/package.json"},
		{environment(nil), []string{".cache", "f", "machine.json"}, "/work/checkout/.cache/f/machine.json"},
		{environment(map[string]string{machineStateEnvironment: "relative/state"}), []string{".cache", "f"}, "/work/checkout/.cache/f"},
		{environment(map[string]string{machineStateEnvironment: "/" + strings.Repeat("a", machineStateMaximumCharacters)}), []string{".cache", "f"}, "/work/checkout/.cache/f"},
	}
	for _, row := range cases {
		if got := MachinePath(row.env, root, row.segments...); got != filepath.FromSlash(row.want) {
			t.Fatalf("%v: got %s want %s", row.segments, got, row.want)
		}
	}
}

// 🏠️The tools partition of a runtime install and the fleet store pointer of a worktree are read below the declared state directory, not below the checkout.
func TestLauncherReadsItsStateBelowTheDeclaredDirectory(t *testing.T) {
	registry, rt, _ := syntheticRegistry(t, []byte("x"), "bun-linux-x64")
	state, err := os.MkdirTemp("", "ms-")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(state)
	checkout := t.TempDir()
	row, _ := rt.Distribution("linux", "x64", false)
	request := ProvisionRequest{Root: checkout, Name: "bun", Registry: registry, Runtime: rt, Platform: "linux", Arch: "x64", Env: environment(map[string]string{machineStateEnvironment: state, "SEMIO_RUNTIME_CACHE": t.TempDir()})}
	tools, _, err := request.StoreDirectories(row)
	if err != nil || !strings.HasPrefix(tools, state+string(filepath.Separator)+"b"+string(filepath.Separator)+"t") || strings.HasPrefix(tools, checkout) {
		t.Fatalf("tools partition %s (%v) is not below %s", tools, err, state)
	}
	store := t.TempDir()
	if err := os.MkdirAll(filepath.Join(state, "f"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(state, "f", "machine.json"), []byte(`{"storeRoot":`+strconvQuote(store)+`}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if got := fleetStoreDirectory(checkout, environment(map[string]string{machineStateEnvironment: state})); got != store {
		t.Fatalf("fleet store %q, want %q", got, store)
	}
	if got := fleetStoreDirectory(checkout, environment(nil)); got != "" {
		t.Fatalf("without a declaration the checkout is read: %q", got)
	}
}


// 🏠️The generations of the managed workspace namespace are looked up below the declared state directory: the launcher refuses a root `node_modules` link into the dependency store of a machine whose state is not in the checkout, unless its store is read from the same place the build writes it.
func TestManagedWorkspaceGenerationsAreReadBelowTheDeclaredState(t *testing.T) {
	sandbox, policy := launcherSandbox(t), workspaceLinksPolicy(t)
	plain, err := readWorkspaceManagedContract(sandbox, policy)
	if err != nil {
		t.Fatal(err)
	}
	relative, err := filepath.Rel(filepath.Join(sandbox, ".cache"), plain.generationRoot)
	if err != nil || strings.HasPrefix(relative, "..") {
		t.Fatalf("without a declaration the store is below the checkout: %s (%v)", plain.generationRoot, err)
	}
	state, err := os.MkdirTemp("", "ms-")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(state)
	t.Setenv(machineStateEnvironment, state)
	moved, err := readWorkspaceManagedContract(sandbox, policy)
	if err != nil {
		t.Fatal(err)
	}
	if want := filepath.Join(state, relative); moved.generationRoot != want {
		t.Fatalf("generation root %s, want %s", moved.generationRoot, want)
	}
}

func strconvQuote(value string) string {
	raw, _ := json.Marshal(value)
	return string(raw)
}
