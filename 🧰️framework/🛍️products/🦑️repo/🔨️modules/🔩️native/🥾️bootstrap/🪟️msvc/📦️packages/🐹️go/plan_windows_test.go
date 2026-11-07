package main

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedSdkPlan(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("publisher inputs not provided")
	}
	recipe, err := os.ReadFile(filepath.Join("..", "..", "🧬️schema", "📌️pins.json"))
	if err != nil {
		t.Fatal(err)
	}
	catalog, err := os.ReadFile(filepath.Join(root, "bazel-msvc-manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	plan, err := makeInputPlan(recipe, catalog, root)
	if err != nil {
		t.Fatal(err)
	}
	files, err := loadSdkPlan(context.Background(), plan, nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 4453 {
		t.Fatal("incomplete SDK graph")
	}
	plan.tableHashes[0] = plan.tableHashes[1]
	if _, err := loadSdkPlan(context.Background(), plan, nil); err == nil {
		t.Fatal("accepted modified table digest")
	}
}
