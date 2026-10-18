// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Exhaustive tree nodes and edges.

// #endregion 🧲️Header

package client

import (
	"context"
	"encoding/json"
	"testing"
)

func TestExhaustiveNodesAndEdgesQuick(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow nodes/edges quick test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	query := `{
		tickets {
			id
			slug
		}
		policies {
			id
			name
			statutes { id }
		}
		statutes {
			id
		}
		folders {
			id
			path
			parent { id }
			children { id }
		}
		files {
			id
			path
			folder { id }
			sections { id name }
			definitions { id name kind }
		}
		breachs(scope: "repo/client/main.go") {
			id
			file { id }
			folder { id }
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Tickets []struct {
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Statutes []struct {
				ID string `json:"id"`
			} `json:"statutes"`
		} `json:"policies"`
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
		Folders []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Parent *struct {
				ID string `json:"id"`
			} `json:"parent"`
			Children []struct {
				ID string `json:"id"`
			} `json:"children"`
		} `json:"folders"`
		Files []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
			Sections []struct {
				ID string `json:"id"`
			} `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
		Breachs []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
		} `json:"breachs"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	if len(resp.Tickets) == 0 {
		t.Error("tickets should not be empty")
	}
	if len(resp.Policies) == 0 {
		t.Error("policies should not be empty")
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes should not be empty")
	}
	if len(resp.Folders) == 0 {
		t.Error("folders should not be empty")
	}
	if len(resp.Files) == 0 {
		t.Error("files should not be empty")
	}
}

// 🌿️#region 🌈️Nodes and Edges
func TestExhaustiveNodesAndEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow nodes/edges test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		bundles {
			id
			name
			folders { id path }
			files { id path }
			breachs { id }
		}
		folders {
			id
			path
			parent { id }
			children { id }
			files { id }
			bundle { id }
			breachs { id }
		}
		files {
			id
			path
			folder { id }
			bundle { id }
			sections { id name }
			definitions { id name kind }
			breachs { id }
		}
		tickets {
			id
			slug
		}
		policies {
			id
			name
			statutes { id }
		}
		statutes {
			id
		}
		breachs(scope: "repo/client/main.go") {
			id
			file { id }
			folder { id }
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Bundles []struct {
			ID      string `json:"id"`
			Name    string `json:"name"`
			Folders []struct {
				ID string `json:"id"`
			} `json:"folders"`
			Files []struct {
				ID string `json:"id"`
			} `json:"files"`
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"bundles"`
		Folders []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Parent *struct {
				ID string `json:"id"`
			} `json:"parent"`
			Children []struct {
				ID string `json:"id"`
			} `json:"children"`
			Files []struct {
				ID string `json:"id"`
			} `json:"files"`
			Bundle *struct {
				ID string `json:"id"`
			} `json:"bundle"`
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"folders"`
		Files []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
			Bundle *struct {
				ID string `json:"id"`
			} `json:"bundle"`
			Sections []struct {
				ID string `json:"id"`
			} `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"files"`
		Tickets []struct {
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Statutes []struct {
				ID string `json:"id"`
			} `json:"statutes"`
		} `json:"policies"`
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
		Breachs []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
		} `json:"breachs"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	if len(resp.Bundles) == 0 {
		t.Error("bundles should not be empty")
	}
	if len(resp.Folders) == 0 {
		t.Error("folders should not be empty")
	}
	if len(resp.Files) == 0 {
		t.Error("files should not be empty")
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets should not be empty")
	}
	if len(resp.Policies) == 0 {
		t.Error("policies should not be empty")
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes should not be empty")
	}
	if resp.Breachs == nil {
		t.Error("breachs should not be nil")
	}

	for _, bundle := range resp.Bundles {
		if bundle.ID == "" {
			t.Errorf("bundle %s has empty id", bundle.Name)
		}
	}
	for _, folder := range resp.Folders {
		if folder.ID == "" {
			t.Errorf("folder %s has empty id", folder.Path)
		}
	}
	for _, file := range resp.Files {
		if file.ID == "" {
			t.Errorf("file %s has empty id", file.Path)
		}
	}
	for _, ticket := range resp.Tickets {
		if ticket.ID == "" {
			t.Errorf("ticket %s has empty id", ticket.Slug)
		}
	}
	for _, policy := range resp.Policies {
		if policy.ID == "" {
			t.Errorf("policy %s has empty id", policy.Name)
		}
	}
	for _, vk := range resp.Statutes {
		if vk.ID == "" {
			t.Error("statute has empty id")
		}
	}
	for _, v := range resp.Breachs {
		if v.ID == "" {
			t.Errorf("breach has empty id: %+v", v)
		}
	}
}
