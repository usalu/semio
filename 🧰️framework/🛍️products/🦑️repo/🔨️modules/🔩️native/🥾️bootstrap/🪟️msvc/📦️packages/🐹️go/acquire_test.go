package main

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
)

func TestAcquirePayloadBoundsAndCleanup(t *testing.T) {
	fixture, err := os.ReadFile(filepath.Join("..", "..", "🧬️schema", "🧪️acquisition.json"))
	if err != nil {
		t.Fatal(err)
	}
	var sample struct {
		Text, SHA256 string
		Bytes        int
	}
	if err := json.Unmarshal(fixture, &sample); err != nil {
		t.Fatal(err)
	}
	data := []byte(sample.Text)
	if len(data) != sample.Bytes {
		t.Fatal("fixture extent mismatch")
	}
	digest := sample.SHA256
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { w.Write(data) }))
	defer server.Close()
	for _, tc := range []struct {
		name   string
		size   int64
		hash   string
		cancel bool
		ok     bool
	}{
		{"valid", int64(len(data)), digest, false, true},
		{"size", int64(len(data) - 1), digest, false, false},
		{"hash", int64(len(data)), string(make([]byte, 64)), false, false},
		{"cancel", int64(len(data)), digest, true, false},
	} {
		t.Run(tc.name, func(t *testing.T) {
			path := filepath.Join(t.TempDir(), "payload")
			ctx, cancel := context.WithCancel(context.Background())
			defer cancel()
			if tc.cancel {
				cancel()
			}
			err := acquirePayload(ctx, server.Client(), server.URL, path, tc.size, tc.hash)
			if (err == nil) != tc.ok {
				t.Fatalf("unexpected error: %v", err)
			}
			got, statErr := os.ReadFile(path)
			if tc.ok && (statErr != nil || string(got) != string(data)) {
				t.Fatalf("incorrect output: %v", statErr)
			}
			if !tc.ok && !os.IsNotExist(statErr) {
				t.Fatalf("failed output retained: %v", statErr)
			}
		})
	}
}

func TestPublisherURL(t *testing.T) {
	for _, tc := range []struct {
		value string
		ok    bool
	}{
		{"https://download.visualstudio.microsoft.com/download/pr/a", true},
		{"http://download.visualstudio.microsoft.com/a", false},
		{"https://download.visualstudio.microsoft.com.evil.invalid/a", false},
		{"https://evil.invalid@download.visualstudio.microsoft.com/a", false},
		{"https://download.visualstudio.microsoft.com/a#fragment", false},
	} {
		if (publisherURL(tc.value) == nil) != tc.ok {
			t.Errorf("unexpected URL decision for %q", tc.value)
		}
	}
}

func TestOwnershipPreservesReplacement(t *testing.T) {
	path := filepath.Join(t.TempDir(), "payload")
	if err := os.WriteFile(path, []byte("owned"), 0600); err != nil {
		t.Fatal(err)
	}
	file, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	info, err := file.Stat()
	if err != nil {
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}
	if err := os.Rename(path, path+".old"); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte("replacement"), 0600); err != nil {
		t.Fatal(err)
	}
	var owned []ownedPath
	if err := recordOwned(&owned, path, info); err == nil {
		t.Fatal("accepted replacement")
	}
	if len(owned) != 1 || !os.SameFile(owned[0].info, info) {
		t.Fatal("lost creation identity")
	}
	if err := cleanupOwned(owned); err == nil {
		t.Fatal("accepted replacement during cleanup")
	}
	if got, err := os.ReadFile(path); err != nil || string(got) != "replacement" {
		t.Fatal("replacement removed")
	}
}
