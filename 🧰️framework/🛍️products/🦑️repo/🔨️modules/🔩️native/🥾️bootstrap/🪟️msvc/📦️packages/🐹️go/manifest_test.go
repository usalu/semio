package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestPublisherManifestSerialization(t *testing.T) {
	root := os.Getenv("SEMIO_MSVC_FIXTURE_ROOT")
	if root == "" {
		t.Skip("publisher inventory not provided")
	}
	expected, err := os.ReadFile(filepath.Join(root, "provider-manifest-v1.json"))
	if err != nil {
		t.Fatal(err)
	}
	if sha(expected) != inventoryDigest {
		t.Fatal("publisher inventory digest differs")
	}
	output, err := canonicalManifestFromBytes(expected)
	if err != nil {
		t.Fatal(err)
	}
	if string(output) != string(expected) {
		t.Fatal("canonical inventory bytes differ")
	}
}
