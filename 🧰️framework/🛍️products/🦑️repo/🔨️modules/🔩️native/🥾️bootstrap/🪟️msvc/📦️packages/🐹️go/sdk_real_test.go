package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

func TestPinnedSdkGraph(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("verified publisher metadata not provided")
	}
	raw, err := os.ReadFile(filepath.Join(root, "sdk-file-plan.json"))
	if err != nil {
		t.Fatal(err)
	}
	var reference []struct {
		Index   int    `json:"index"`
		ID      string `json:"id"`
		Path    string `json:"path"`
		Cabinet string `json:"cabinet"`
		Size    int64  `json:"size"`
	}
	if err := json.Unmarshal(raw, &reference); err != nil {
		t.Fatal(err)
	}
	expected := map[string]sdkFile{}
	for _, row := range reference {
		expected[fmt.Sprintf("%d/%s", row.Index, row.ID)] = sdkFile{row.ID, "sdk/" + row.Path[len("Windows Kits/10/"):], row.Cabinet, row.Size}
	}
	count := 0
	for i := 0; i < 8; i++ {
		data, err := os.ReadFile(filepath.Join(root, fmt.Sprintf("sdk-recheck-%d.json", i)))
		if err != nil {
			t.Fatal(err)
		}
		var tables struct {
			Files       [][5]string `json:"files"`
			Components  [][2]string `json:"components"`
			Directories [][3]string `json:"directories"`
			Media       [][3]string `json:"media"`
		}
		if err := json.Unmarshal(data, &tables); err != nil {
			t.Fatal(err)
		}
		rows, err := sdkFiles(tables.Directories, tables.Components, tables.Files, tables.Media)
		if err != nil {
			t.Fatal(err)
		}
		for _, row := range rows {
			key := fmt.Sprintf("%d/%s", i, row.ID)
			if expected[key] != row {
				t.Fatalf("MSI graph differs: %s", key)
			}
			delete(expected, key)
			count++
		}
	}
	if count != 4453 || len(expected) != 0 {
		t.Fatalf("got %d records, %d missing", count, len(expected))
	}
}
