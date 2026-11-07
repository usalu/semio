package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"path/filepath"
	"unicode/utf16"
)

type extractionRequest struct {
	Version        int    `json:"version"`
	InputsRoot     string `json:"inputsRoot"`
	StageRoot      string `json:"stageRoot"`
	CatalogSHA256  string `json:"catalogSha256"`
	RecipeSHA256   string `json:"recipeSha256"`
	ManifestSHA256 string `json:"manifestSha256"`
}

func parseRequest(data []byte) (extractionRequest, error) {
	var request extractionRequest
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if len(data) > 4096 {
		return request, errors.New("request too large")
	}
	if err := decoder.Decode(&request); err != nil {
		return request, err
	}
	var extra any
	if err := decoder.Decode(&extra); err != io.EOF {
		return request, errors.New("extra request data")
	}
	if request.Version != 1 || request.CatalogSHA256 != catalogDigest || request.RecipeSHA256 != recipeDigest || request.ManifestSHA256 != inventoryDigest {
		return request, errors.New("untrusted request pins")
	}
	for _, path := range []string{request.InputsRoot, request.StageRoot} {
		if !filepath.IsAbs(path) || len(utf16.Encode([]rune(path))) > 256 {
			return request, errors.New("invalid request path")
		}
	}
	return request, nil
}
