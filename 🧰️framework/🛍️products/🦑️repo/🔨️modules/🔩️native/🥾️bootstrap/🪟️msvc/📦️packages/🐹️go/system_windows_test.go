package main

import (
	"path/filepath"
	"strings"
	"testing"
)

func TestSystemLibraryResolution(t *testing.T) {
	for _, name := range []string{"msi.dll", "setupapi.dll"} {
		dll, err := systemDLL(name)
		if err != nil {
			t.Fatal(err)
		}
		if !filepath.IsAbs(dll.Name) || !strings.EqualFold(filepath.Base(dll.Name), name) || !strings.EqualFold(filepath.Base(filepath.Dir(dll.Name)), "System32") {
			t.Fatalf("unexpected system library %q", dll.Name)
		}
		if err := dll.Load(); err != nil {
			t.Fatal(err)
		}
	}
	for _, name := range []string{"../msi.dll", "other.dll", "C:\\msi.dll"} {
		if _, err := systemDLL(name); err == nil {
			t.Fatalf("accepted %q", name)
		}
	}
}
