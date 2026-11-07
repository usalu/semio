package main

import (
	"encoding/json"
	"path/filepath"
	"testing"
)

func TestRequestPinsAndShape(t *testing.T) {
	path, err := filepath.Abs("fixture")
	if err != nil {
		t.Fatal(err)
	}
	request := extractionRequest{1, path, path, catalogDigest, recipeDigest, inventoryDigest}
	data, _ := json.Marshal(request)
	if _, err := parseRequest(data); err != nil {
		t.Fatal(err)
	}
	request.ManifestSHA256 = catalogDigest
	data, _ = json.Marshal(request)
	if _, err := parseRequest(data); err == nil {
		t.Fatal("accepted untrusted inventory")
	}
	if _, err := parseRequest([]byte(`{"version":1,"extra":true}`)); err == nil {
		t.Fatal("accepted extra field")
	}
}
