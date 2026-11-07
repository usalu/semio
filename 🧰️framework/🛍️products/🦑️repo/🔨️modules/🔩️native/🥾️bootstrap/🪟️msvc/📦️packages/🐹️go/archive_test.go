package main

import (
	"archive/zip"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"os"
	"path/filepath"
	"testing"
)

func TestArchiveExtraction(t *testing.T) {
	for _, row := range []struct {
		name, member string
		reject       bool
	}{{"valid", "Contents/VC/bin/cl.exe", false}, {"traversal", "Contents/VC/../escape", true}} {
		t.Run(row.name, func(t *testing.T) {
			root := t.TempDir()
			archive := filepath.Join(root, "input.zip")
			file, err := os.Create(archive)
			if err != nil {
				t.Fatal(err)
			}
			z := zip.NewWriter(file)
			w, err := z.Create(row.member)
			if err != nil {
				t.Fatal(err)
			}
			if _, err := w.Write([]byte("tool")); err != nil {
				t.Fatal(err)
			}
			if err := z.Close(); err != nil {
				t.Fatal(err)
			}
			if err := file.Close(); err != nil {
				t.Fatal(err)
			}
			bytes, err := os.ReadFile(archive)
			if err != nil {
				t.Fatal(err)
			}
			sum := sha256.Sum256(bytes)
			stage := filepath.Join(root, "stage")
			if err := os.Mkdir(stage, 0700); err != nil {
				t.Fatal(err)
			}
			_, err = extractArchives(context.Background(), []archiveInput{{archive, hex.EncodeToString(sum[:]), int64(len(bytes))}}, stage, nil)
			if row.reject {
				if err == nil {
					t.Fatal("expected refusal")
				}
				return
			}
			if err != nil {
				t.Fatal(err)
			}
			got, err := os.ReadFile(filepath.Join(stage, "VC", "bin", "cl.exe"))
			if err != nil || string(got) != "tool" {
				t.Fatalf("got %q: %v", got, err)
			}
		})
	}
}

func TestArchiveRefusesChangedPayloadAndCancellation(t *testing.T) {
	root := t.TempDir()
	archive := filepath.Join(root, "input.zip")
	file, err := os.Create(archive)
	if err != nil {
		t.Fatal(err)
	}
	z := zip.NewWriter(file)
	w, err := z.Create("Contents/VC/bin/cl.exe")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := w.Write([]byte("tool")); err != nil {
		t.Fatal(err)
	}
	if err := z.Close(); err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	data, err := os.ReadFile(archive)
	if err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(data)
	input := archiveInput{archive, hex.EncodeToString(sum[:]), int64(len(data))}
	for _, row := range []struct {
		name   string
		mutate bool
		cancel bool
	}{{"digest", true, false}, {"cancel", false, true}} {
		t.Run(row.name, func(t *testing.T) {
			stage := filepath.Join(root, row.name)
			if err := os.Mkdir(stage, 0700); err != nil {
				t.Fatal(err)
			}
			trial := input
			if row.mutate {
				trial.SHA256 = "0000000000000000000000000000000000000000000000000000000000000000"
			}
			ctx, cancel := context.WithCancel(context.Background())
			defer cancel()
			if row.cancel {
				cancel()
			}
			if _, err := extractArchives(ctx, []archiveInput{trial}, stage, nil); err == nil {
				t.Fatal("expected refusal")
			}
			entries, err := os.ReadDir(stage)
			if err != nil || len(entries) != 0 {
				t.Fatalf("stage not empty: %v %v", entries, err)
			}
		})
	}
}

func TestArchiveCancellationRemovesOnlyOwnedOutputs(t *testing.T) {
	root := t.TempDir()
	archive := filepath.Join(root, "input.zip")
	file, err := os.Create(archive)
	if err != nil {
		t.Fatal(err)
	}
	z := zip.NewWriter(file)
	for _, name := range []string{"Contents/VC/a", "Contents/VC/b"} {
		w, err := z.Create(name)
		if err != nil {
			t.Fatal(err)
		}
		if _, err := w.Write([]byte("tool")); err != nil {
			t.Fatal(err)
		}
	}
	if err := z.Close(); err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	data, err := os.ReadFile(archive)
	if err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(data)
	stage := filepath.Join(root, "stage")
	if err := os.Mkdir(stage, 0700); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	if _, err := extractArchives(ctx, []archiveInput{{archive, hex.EncodeToString(sum[:]), int64(len(data))}}, stage, func(done, total int) {
		if done == 1 {
			cancel()
		}
	}); err == nil {
		t.Fatal("expected cancellation")
	}
	entries, err := os.ReadDir(stage)
	if err != nil || len(entries) != 0 {
		t.Fatalf("stage not cleaned: %v %v", entries, err)
	}
}

func TestArchiveCancellationAfterFinalProgress(t *testing.T) {
	root := t.TempDir()
	archive := filepath.Join(root, "input.zip")
	file, err := os.Create(archive)
	if err != nil {
		t.Fatal(err)
	}
	z := zip.NewWriter(file)
	w, err := z.Create("Contents/VC/a")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := w.Write([]byte("tool")); err != nil {
		t.Fatal(err)
	}
	if err := z.Close(); err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	data, err := os.ReadFile(archive)
	if err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(data)
	stage := filepath.Join(root, "stage")
	if err := os.Mkdir(stage, 0700); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	if _, err := extractArchives(ctx, []archiveInput{{archive, hex.EncodeToString(sum[:]), int64(len(data))}}, stage, func(done, total int) { cancel() }); err == nil {
		t.Fatal("expected cancellation")
	}
	entries, err := os.ReadDir(stage)
	if err != nil || len(entries) != 0 {
		t.Fatalf("stage not cleaned: %v %v", entries, err)
	}
}

func TestArchiveOwnershipSurvivesLaterFailure(t *testing.T) {
	root := t.TempDir()
	path := filepath.Join(root, "input.zip")
	file, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	z := zip.NewWriter(file)
	w, err := z.Create("Contents/VC/a")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := w.Write([]byte("tool")); err != nil {
		t.Fatal(err)
	}
	if err := z.Close(); err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(data)
	stage := filepath.Join(root, "stage")
	if err := os.Mkdir(stage, 0700); err != nil {
		t.Fatal(err)
	}
	var owned []ownedPath
	if _, err := extractArchivesOwned(context.Background(), []archiveInput{{path, hex.EncodeToString(sum[:]), int64(len(data))}}, stage, nil, &owned); err != nil {
		t.Fatal(err)
	}
	if len(owned) != 2 {
		t.Fatalf("missing creation records: %d", len(owned))
	}
	if err := cleanupOwned(owned); err != nil {
		t.Fatal(err)
	}
	entries, err := os.ReadDir(stage)
	if err != nil || len(entries) != 0 {
		t.Fatalf("stage not cleaned after later failure: %v %v", entries, err)
	}
}
