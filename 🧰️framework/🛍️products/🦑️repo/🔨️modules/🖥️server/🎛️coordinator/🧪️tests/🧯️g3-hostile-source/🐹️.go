// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Phase 9 G3 owned-source hostile-token contract test.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// #region 🧯️G3HostileSource

func TestG3OwnedSourceHasNoExternalOrLegacyStoreFallback(t *testing.T) {
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatal(err)
	}
	prohibited := []string{"modernc.org", "database/sql", "sqlite", "sql.Open", "CREATE TABLE", "ON CONFLICT", "crdt", "legacy store", "store fallback", "_ = s.db.", "_ = s.bus.publish"}
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".go") || strings.HasSuffix(entry.Name(), "_test.go") {
			continue
		}
		data, readErr := os.ReadFile(entry.Name())
		if readErr != nil {
			t.Fatal(readErr)
		}
		lower := strings.ToLower(string(data))
		for _, token := range prohibited {
			if strings.Contains(lower, strings.ToLower(token)) {
				t.Fatalf("production source %s retains prohibited token %q", entry.Name(), token)
			}
		}
	}
	module, err := os.ReadFile("go.mod")
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(module, []byte("modernc.org")) || bytes.Contains(module, []byte("golang.org/")) {
		t.Fatalf("external module retained:\n%s", module)
	}
	if _, err := os.Stat("go.sum"); !os.IsNotExist(err) {
		t.Fatalf("go.sum must be naturally absent: %v", err)
	}
	windowsDurability, err := os.ReadFile(filepath.Join("..", "..", "..", "..", "..", "🔨️modules", "📁️filesystem", "🛡️durability", "📦️packages", "🐹️go", "🪟️windows-durability", "🐹️.go"))
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Contains(windowsDurability, []byte("syscall.FlushFileBuffers")) || !bytes.Contains(windowsDurability, []byte("moveFileWriteThrough")) || bytes.Contains(windowsDurability, []byte("runtime.GOOS")) {
		t.Fatal("Windows durability must write through replacements, flush metadata, or return its explicit error")
	}
	eventStoreSource, err := os.ReadFile("🗄️event-store/🐹️.go")
	if err != nil {
		t.Fatal(err)
	}
	for _, mutation := range []string{"os.OpenFile(", "os.Remove(", "os.Rename(", "os.Chtimes(", "os.MkdirAll("} {
		if bytes.Contains(eventStoreSource, []byte(mutation)) {
			t.Fatalf("event-store mutation escaped the owned operation boundary: %s", mutation)
		}
	}
	repositorySource, err := os.ReadFile("📚️repository/🐹️.go")
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(repositorySource, []byte("sort.Strings")) || !bytes.Contains(repositorySource, []byte("projectionResultCapacity")) || !bytes.Contains(repositorySource, []byte("sortProjectionStrings")) {
		t.Fatal("projection ordering escaped the retained budgeted cursor")
	}
}

// #endregion 🧯️G3HostileSource
