package main

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedCompleteExtraction(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" || os.Getenv("SEMIO_MSVC_FULL") != "1" {
		t.Skip("full publisher extraction not requested")
	}
	stage, err := os.MkdirTemp(root, "provider-")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(stage)
	recipe, err := filepath.Abs(filepath.Join("..", "..", "🧬️schema", "📌️pins.json"))
	if err != nil {
		t.Fatal(err)
	}
	request := extractionRequest{1, root, stage, catalogDigest, recipeDigest, inventoryDigest}
	if err := extract(context.Background(), recipe, request); err != nil {
		t.Fatal(err)
	}
	inventory, err := os.ReadFile(filepath.Join(stage, ".semio-inventory.json"))
	if err != nil {
		t.Fatal(err)
	}
	if sha(inventory) != inventoryDigest {
		t.Fatal("inventory mismatch")
	}
}
