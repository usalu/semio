package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedRecipeAndCatalog(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("verified publisher catalog not provided")
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
	if len(plan.archives) != 5 || len(plan.msis) != 8 || len(plan.cabinets) != 34 {
		t.Fatalf("unexpected plan %d/%d/%d", len(plan.archives), len(plan.msis), len(plan.cabinets))
	}
	recipe[0] ^= 1
	if _, err := makeInputPlan(recipe, catalog, root); err == nil {
		t.Fatal("accepted modified recipe")
	}
}
