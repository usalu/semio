package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

func TestDestinations(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "🧫️fixtures", "🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Cases []struct {
			Name        string      `json:"name"`
			Directories [][3]string `json:"directories"`
			Directory   string      `json:"directory"`
			File        string      `json:"file"`
			Expected    string      `json:"expected"`
			Error       bool        `json:"error"`
		} `json:"cases"`
		ArchiveCases []struct {
			Name  string `json:"name"`
			Path  string `json:"path"`
			Valid bool   `json:"valid"`
		} `json:"archiveCases"`
		SDKCases []struct {
			Name            string      `json:"name"`
			Directories     [][3]string `json:"directories"`
			Components      [][2]string `json:"components"`
			Files           [][5]string `json:"files"`
			Media           [][3]string `json:"media"`
			ExpectedPath    string      `json:"expectedPath"`
			ExpectedCabinet string      `json:"expectedCabinet"`
			Valid           bool        `json:"valid"`
		} `json:"sdkCases"`
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.Cases {
		t.Run(row.Name, func(t *testing.T) {
			got, err := destination(row.Directories, row.Directory, row.File)
			if row.Error {
				if err == nil {
					t.Fatal("expected refusal")
				}
				return
			}
			if err != nil || got != row.Expected {
				t.Fatalf("got %q (%v), want %q", got, err, row.Expected)
			}
		})
	}
	for _, row := range fixture.ArchiveCases {
		t.Run(row.Name, func(t *testing.T) {
			_, err := archivePath(row.Path)
			if (err == nil) != row.Valid {
				t.Fatalf("valid=%v, error=%v", row.Valid, err)
			}
		})
	}
	for _, row := range fixture.SDKCases {
		t.Run(row.Name, func(t *testing.T) {
			got, err := sdkFiles(row.Directories, row.Components, row.Files, row.Media)
			if !row.Valid {
				if err == nil {
					t.Fatal("expected refusal")
				}
				return
			}
			if err != nil || len(got) != 1 || got[0].Path != row.ExpectedPath || got[0].Cabinet != row.ExpectedCabinet {
				t.Fatalf("got %v (%v)", got, err)
			}
		})
	}
}
