// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Dispatch and flag-resolution laws for the owned command package.

// #endregion 🧲️Header

package command

import (
	"bytes"
	"os"
	"testing"
)

// #region 🧪️Dispatch

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

// #endregion 🧪️Dispatch

// #region 🧪️InheritedFlags

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

// #endregion 🧪️InheritedFlags
