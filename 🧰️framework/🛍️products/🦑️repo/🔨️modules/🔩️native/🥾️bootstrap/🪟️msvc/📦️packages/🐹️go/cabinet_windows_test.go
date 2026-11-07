package main

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedCabinet(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("verified publisher cabinet not provided")
	}
	name := "26ea25236f12b23db661acf268a70cfa.cab"
	input := cabinetInput{name, archiveInput{filepath.Join(root, name), "a6d81007e50b1881d58e802ef4402ff16c62ff4960b41c4263057e3173981919", 93999}}
	plan := []sdkFile{{"cat26ea25236f12b23db661acf268a70cfa", "sdk/Catalogs/cat26ea25236f12b23db661acf268a70cfa.cat", name, 11124}, {"fil21adcb655557a8d1eb06109c707690b4", "sdk/bin/10.0.26100.0/x86/Microsoft.Diagnostics.Tracing.EventSource.dll", name, 166232}}
	for _, row := range []struct {
		name   string
		cancel bool
	}{{"extract", false}, {"cancel", true}, {"custody", false}} {
		t.Run(row.name, func(t *testing.T) {
			stage := t.TempDir()
			ctx, cancel := context.WithCancel(context.Background())
			defer cancel()
			output, err := extractCabinets(ctx, []cabinetInput{input}, plan, stage, func(done, total int) {
				if row.name == "custody" {
					path := filepath.Join(stage, filepath.FromSlash(plan[0].Path))
					if err := os.Rename(path, path+".replacement"); err == nil {
						t.Error("renamed held CAB output")
					}
				}
				if row.cancel {
					cancel()
				}
			})
			if row.cancel {
				if err == nil {
					t.Fatal("expected cancellation")
				}
				entries, e := os.ReadDir(stage)
				if e != nil || len(entries) != 0 {
					t.Fatalf("stage not cleaned: %v %v", entries, e)
				}
				return
			}
			if err != nil || len(output) != 2 {
				t.Fatalf("got %v (%v)", output, err)
			}
			expected := map[string]string{"sdk/Catalogs/cat26ea25236f12b23db661acf268a70cfa.cat": "4c9d6d408fb9018d9e62db63f568ad89fdd57e2e3de4fbea866591da97b8f968", "sdk/bin/10.0.26100.0/x86/Microsoft.Diagnostics.Tracing.EventSource.dll": "80303bd90154211aa7aa032d5993481ba70eaf49062a42c8fbde0040cb7bd6db"}
			for _, file := range output {
				if expected[file.Path] != file.SHA256 {
					t.Fatalf("unexpected digest for %s", file.Path)
				}
			}
		})
	}
}

func TestCabinetTargetRejectsExtraLink(t *testing.T) {
	root := t.TempDir()
	path := filepath.Join(root, "target")
	if err := os.WriteFile(path, []byte("tool"), 0600); err != nil {
		t.Fatal(err)
	}
	if err := os.Link(path, filepath.Join(root, "link")); err != nil {
		t.Fatal(err)
	}
	file, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	defer file.Close()
	if err := singleLink(file); err == nil {
		t.Fatal("accepted multiple links")
	}
}
