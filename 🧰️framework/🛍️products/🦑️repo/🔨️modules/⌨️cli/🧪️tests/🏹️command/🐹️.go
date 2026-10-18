// 🧭️ Tests of the cli domain: command wiring and argument dispatch.

package cli

import (
	bytes "bytes"
	os "os"
	strings "strings"
	testing "testing"

	metricspkg "github.com/usalu/semio/repo/metrics"
)

func TestTestCommandHelp(t *testing.T) {
	stdout, _, err := executeCommand("test", "--help")
	if err != nil {
		t.Fatalf("test --help returned error: %v", err)
	}
	if !strings.Contains(stdout, "test") {
		t.Errorf("test --help output should mention 'test', got: %s", stdout)
	}
}

// 🔢️TestLocWiring asserts the CLI wiring of the `loc` verb; every counting, parsing and rendering
// rule it used to duplicate now lives in 📊️metrics and is covered by that module's own tests and by
// the four language-agnostic cases under 📊️metrics/🧪️tests.
func TestLocWiring(t *testing.T) {
	t.Run("text no ansi for pipe", func(t *testing.T) {
		report := &metricspkg.LocReport{Snapshot: metricspkg.ComposeLocReportSnapshot(metricspkg.Cumulative{}, map[string]int64{"Go": 1}, []string{"Go", "TypeScript", "C#", "Python", "Rust"}, 0)}
		var out strings.Builder
		renderLocText(&out, report, false, false, false)
		if strings.ContainsRune(out.String(), 0x1b) {
			t.Fatal("unexpected ansi")
		}
	})
	t.Run("root has loc", func(t *testing.T) {
		root, _ := NewRootWithConfig(testEngineFactory)
		var found *Command
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

// 🌱️newDispatchRoot returns a root carrying one recording child, plus the recorder.
func newDispatchRoot() (*Command, *[]string) {
	var seen []string
	root := &Command{Use: "repo", Short: "root", SilenceUsage: true, SilenceErrors: true}
	child := &Command{
		Use:   "child",
		Short: "child",
		RunE: func(_ *Command, args []string) error {
			seen = append(seen, "child")
			seen = append(seen, args...)
			return nil
		},
	}
	root.AddCommand(child)
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})
	return root, &seen
}

func TestExecuteFallsBackToProcessArguments(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "alpha"}

	root, seen := newDispatchRoot()
	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if len(*seen) != 2 || (*seen)[0] != "child" || (*seen)[1] != "alpha" {
		t.Fatalf("process arguments were not dispatched, got %v", *seen)
	}
}

func TestSetArgsOverridesProcessArguments(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "from-process"}

	root, seen := newDispatchRoot()
	root.SetArgs([]string{"child", "from-caller"})
	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if len(*seen) != 2 || (*seen)[1] != "from-caller" {
		t.Fatalf("installed arguments lost to the process vector, got %v", *seen)
	}
}

func TestSetArgsEmptyDoesNotFallBackToProcessArguments(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child"}

	root, seen := newDispatchRoot()
	root.SetArgs(nil)
	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if len(*seen) != 0 {
		t.Fatalf("an explicitly empty argument vector fell back to the process vector, got %v", *seen)
	}
}

func TestSubcommandFlagsResolveInheritedPersistentFlags(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "--json"}

	var observed bool
	var lookupErr error
	root := &Command{Use: "repo", Short: "root", SilenceUsage: true, SilenceErrors: true}
	root.PersistentFlags().BoolP("json", "", false, "json output")
	child := &Command{
		Use:   "child",
		Short: "child",
		RunE: func(cmd *Command, _ []string) error {
			observed, lookupErr = cmd.Flags().GetBool("json")
			return nil
		},
	}
	root.AddCommand(child)
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})

	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if lookupErr != nil {
		t.Fatalf("inherited persistent flag not visible to the subcommand: %v", lookupErr)
	}
	if !observed {
		t.Fatal("inherited persistent flag resolved but did not carry its parsed value")
	}
}

func TestSubcommandFlagsReportInheritedChanged(t *testing.T) {
	original := os.Args
	defer func() { os.Args = original }()
	os.Args = []string{"repo", "child", "--format", "json"}

	var changed bool
	var value string
	root := &Command{Use: "repo", Short: "root", SilenceUsage: true, SilenceErrors: true}
	root.PersistentFlags().String("format", "md", "output format")
	child := &Command{
		Use:   "child",
		Short: "child",
		RunE: func(cmd *Command, _ []string) error {
			changed = cmd.Flags().Changed("format")
			value, _ = cmd.Flags().GetString("format")
			return nil
		},
	}
	root.AddCommand(child)
	root.SetOut(&bytes.Buffer{})
	root.SetErr(&bytes.Buffer{})

	if err := root.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if !changed {
		t.Fatal("Changed did not report an inherited persistent flag that was set")
	}
	if value != "json" {
		t.Fatalf("inherited persistent flag value = %q, want %q", value, "json")
	}
}
