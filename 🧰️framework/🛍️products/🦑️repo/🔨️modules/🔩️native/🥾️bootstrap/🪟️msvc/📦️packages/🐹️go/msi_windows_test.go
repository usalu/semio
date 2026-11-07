package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedMsiTables(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("verified publisher payloads not provided")
	}
	hashes := []string{"8fb00df405939d60e50a2f1f5d068284153ab3520b6b3657c9ce3796e4a60469", "c9f78ae3e730ea1789a37001e204a970099d2acc555cc537fb45ea0bb23b347c", "be5ee13ccc98b87211b4f3686972b00fa16f20f266fe9dc5572d62f001e0547c", "1381535d2f6b1894a092da67f8a8fb048a4dfe8060ce75b3275affa81b02586e", "f611ce8a9e576e3383917b04b6fbe5ee6bed8363c1a2a8e9d6f8335cbb422675", "d189ca50e5632b546795922e2262794c068d6ff301860feb4522b8b93cbb3ba8", "14667a930569186a25e75eb35614ce43549b7406705e098db71ffe9b31c60faa", "2e956ca1cf17800b0f9811a6249b945f045addf18ee85ffe7aea99ec6c27243a"}
	sizes := []int64{516096, 1060864, 471040, 528384, 589824, 450560, 450560, 528384}
	for i := range hashes {
		tables, err := readMsi(context.Background(), archiveInput{filepath.Join(root, fmt.Sprintf("sdk-%d.msi", i)), hashes[i], sizes[i]})
		if err != nil {
			t.Fatal(err)
		}
		recipe, err := os.ReadFile(filepath.Join(root, "provider-recipe-v1.json"))
		if err != nil {
			t.Fatal(err)
		}
		var pins recipeDocument
		if err := json.Unmarshal(recipe, &pins); err != nil {
			t.Fatal(err)
		}
		encoded, err := canonicalTables(tables)
		if err != nil {
			t.Fatal(err)
		}
		if sha(encoded) != pins.TableHashes[i] {
			t.Fatalf("MSI %d table hash mismatch", i)
		}
		reference, err := os.ReadFile(filepath.Join(root, fmt.Sprintf("sdk-recheck-%d.json", i)))
		if err != nil {
			t.Fatal(err)
		}
		var oracle struct {
			Files       [][5]string `json:"files"`
			Components  [][2]string `json:"components"`
			Directories [][3]string `json:"directories"`
			Media       [][3]string `json:"media"`
		}
		if err := json.Unmarshal(reference, &oracle); err != nil {
			t.Fatal(err)
		}
		a, err := sdkFiles(tables.directories, tables.components, tables.files, tables.media)
		if err != nil {
			t.Fatal(err)
		}
		b, err := sdkFiles(oracle.Directories, oracle.Components, oracle.Files, oracle.Media)
		if err != nil {
			t.Fatal(err)
		}
		x, _ := json.Marshal(a)
		y, _ := json.Marshal(b)
		if string(x) != string(y) {
			t.Fatalf("MSI %d graph mismatch", i)
		}
	}
}
