package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"sort"
	"strings"
)

type inventoryDocument struct {
	Version      int          `json:"version"`
	RecipeSHA256 string       `json:"recipeSha256"`
	Files        []outputFile `json:"files"`
}

func canonicalManifest(files []outputFile) ([]byte, error) {
	sort.Slice(files, func(i, j int) bool { return files[i].Path < files[j].Path })
	seen := map[string]bool{}
	for _, file := range files {
		path, err := archivePath(file.Path)
		if err != nil || path != file.Path || !(strings.HasPrefix(path, "VC/") || strings.HasPrefix(path, "sdk/")) || file.Bytes < 0 || file.Bytes > 100000000 || !lowerHash.MatchString(file.SHA256) || seen[strings.ToLower(path)] {
			return nil, errors.New("invalid inventory file")
		}
		seen[strings.ToLower(path)] = true
	}
	var data bytes.Buffer
	encoder := json.NewEncoder(&data)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(inventoryDocument{1, recipeDigest, files}); err != nil {
		return nil, err
	}
	return data.Bytes(), nil
}

func canonicalManifestFromBytes(data []byte) ([]byte, error) {
	var inventory inventoryDocument
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&inventory); err != nil {
		return nil, err
	}
	if inventory.Version != 1 || inventory.RecipeSHA256 != recipeDigest || len(inventory.Files) != 4947 {
		return nil, errors.New("invalid inventory")
	}
	return canonicalManifest(inventory.Files)
}
