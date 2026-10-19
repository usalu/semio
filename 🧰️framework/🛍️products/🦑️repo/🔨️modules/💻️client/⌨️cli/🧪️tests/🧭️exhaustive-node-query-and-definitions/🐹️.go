// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exhaustive node query, section edges, and definition edges.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"testing"
)

func TestExhaustiveNodeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow node query test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()

	bundleResult, err := executor.ExecuteJSON(ctx, "{ bundles { id name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var bundleResp struct {
		Bundles []struct {
			ID   string `json:"id"`
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(bundleResult), &bundleResp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(bundleResp.Bundles) == 0 {
		t.Fatal("fixture repository has no bundles to test node query")
	}

	testID := bundleResp.Bundles[0].ID
	nodeResult, err := executor.ExecuteJSON(ctx, `query($id: ID!) { node(id: $id) { __typename } }`, map[string]interface{}{"id": testID})
	if err != nil {
		t.Fatalf("node query failed: %v", err)
	}
	var nodeResp struct {
		Node struct {
			Typename string `json:"__typename"`
		} `json:"node"`
	}
	if err := json.Unmarshal([]byte(nodeResult), &nodeResp); err != nil {
		t.Fatalf("failed to parse node response: %v", err)
	}
	if nodeResp.Node.Typename != "Bundle" {
		t.Errorf("node query returned wrong type for %s: got %s, want Bundle (%s)", testID, nodeResp.Node.Typename, nodeResult)
	}
}

func TestExhaustiveSectionsEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow sections edges test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()

	query := `{
		files {
			id
			path
			sections {
				id
				name
				path
				file { id }
				parent { id }
				children { id }
				definitions { id name }
				breachs { id }
				range { start end }
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID       string `json:"id"`
			Path     string `json:"path"`
			Sections []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Path string `json:"path"`
				File struct {
					ID string `json:"id"`
				} `json:"file"`
				Parent *struct {
					ID string `json:"id"`
				} `json:"parent"`
				Children []struct {
					ID string `json:"id"`
				} `json:"children"`
				Definitions []struct {
					ID   string `json:"id"`
					Name string `json:"name"`
				} `json:"definitions"`
				Breachs []struct {
					ID string `json:"id"`
				} `json:"breachs"`
				Range struct {
					Start int `json:"start"`
					End   int `json:"end"`
				} `json:"range"`
			} `json:"sections"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	sectionsFound := false
	for _, file := range resp.Files {
		for _, section := range file.Sections {
			sectionsFound = true
			if section.ID == "" {
				t.Errorf("section %s in file %s has empty id", section.Name, file.Path)
			}
			if section.File.ID == "" {
				t.Errorf("section %s has file with empty id", section.Name)
			}
			if section.File.ID != file.ID {
				t.Errorf("section %s file id mismatch: got %s, want %s", section.Name, section.File.ID, file.ID)
			}
		}
	}
	if !sectionsFound {
		t.Skip("no sections found in any file - may be expected for test repository")
	}
}

func TestExhaustiveDefinitionsEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow definitions edges test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()

	query := `{
		files {
			id
			path
			definitions {
				id
				name
				kind
				file { id }
				section { id name }
				breachs { id }
				range { start end }
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID          string `json:"id"`
			Path        string `json:"path"`
			Definitions []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Kind string `json:"kind"`
				File struct {
					ID string `json:"id"`
				} `json:"file"`
				Section *struct {
					ID   string `json:"id"`
					Name string `json:"name"`
				} `json:"section"`
				Breachs []struct {
					ID string `json:"id"`
				} `json:"breachs"`
				Range struct {
					Start int `json:"start"`
					End   int `json:"end"`
				} `json:"range"`
			} `json:"definitions"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	definitionsFound := false
	for _, file := range resp.Files {
		for _, def := range file.Definitions {
			definitionsFound = true
			if def.ID == "" {
				t.Errorf("definition %s in file %s has empty id", def.Name, file.Path)
			}
			if def.File.ID == "" {
				t.Errorf("definition %s has file with empty id", def.Name)
			}
			if def.File.ID != file.ID {
				t.Errorf("definition %s file id mismatch: got %s, want %s", def.Name, def.File.ID, file.ID)
			}
			if def.Kind == "" {
				t.Errorf("definition %s has empty kind", def.Name)
			}
		}
	}
	if !definitionsFound {
		t.Skip("no definitions found in any file - may be expected for test repository")
	}
}

func TestExhaustiveDefinitionKind(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow definition kind test in short mode")
	}
	executor := newExecutorFor(t, withExhaustiveRepo(t))
	ctx := context.Background()

	query := `{
		files {
			id
			path
			definitions {
				id
				name
				kind
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID          string `json:"id"`
			Path        string `json:"path"`
			Definitions []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	definitionsFound := false
	validKinds := map[string]bool{
		"IMPLEMENTATION": true,
		"INTERFACE":      true,
		"CONSTANT":       true,
		"TEST":           true,
	}

	for _, file := range resp.Files {
		for _, def := range file.Definitions {
			definitionsFound = true
			if def.Kind == "" {
				t.Errorf("definition %s in file %s has empty kind", def.Name, file.Path)
			}
			if !validKinds[def.Kind] {
				t.Errorf("definition %s has invalid kind: %s (expected implementation, interface, constant, or test)", def.Name, def.Kind)
			}
		}
	}
	if !definitionsFound {
		t.Skip("no definitions found in any file - may be expected for test repository")
	}
}
