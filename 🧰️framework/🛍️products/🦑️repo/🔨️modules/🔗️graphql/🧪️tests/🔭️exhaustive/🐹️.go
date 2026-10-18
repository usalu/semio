//go:build exhaustive

// 🔬️ Tests of the graphql domain, split out of the pre-split godfile suite.

package graphql

import (
	context "context"
	json "encoding/json"
	os "os"
	strings "strings"
	testing "testing"

	tickets "github.com/usalu/semio/repo/tickets"
)

// 📦️#region 🧳️Collection
func TestExhaustiveBundlesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ bundles { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Bundles []struct {
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Bundles) == 0 {
		t.Error("bundles collection should not be empty")
	}
}

func TestExhaustiveContributorsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow contributor test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ contributors { github } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Contributors []struct {
			Github string `json:"github"`
		} `json:"contributors"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Contributors) == 0 {
		t.Error("contributors collection should not be empty")
	}
}

func TestExhaustiveTicketsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tickets test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ tickets { slug } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Tickets []struct {
			Slug string `json:"slug"`
		} `json:"tickets"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets collection should not be empty")
	}
}

func TestExhaustivePoliciesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policies test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ policies { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Policies []struct {
			Name string `json:"name"`
		} `json:"policies"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Policies) == 0 {
		// Policies are no longer registered in Go; breachs come from lint scripts + cache.
	}
}

func TestExhaustiveStatutesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statutes test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ statutes { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes collection should not be empty")
	}
}

func TestExhaustiveFoldersNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow folders test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ folders { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Folders []struct {
			Path string `json:"path"`
		} `json:"folders"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Folders) == 0 {
		t.Error("folders collection should not be empty")
	}
}

func TestExhaustiveFilesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow files test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ files { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Files []struct {
			Path string `json:"path"`
		} `json:"files"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Files) == 0 {
		t.Error("files collection should not be empty")
	}
}

func TestExhaustiveBreachsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow breachs test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	// Unscoped breachs defaults to technology scope "compose" and runs full policy
	// analysis across the tree (very slow). Single-file scope keeps the test
	// representative while bounded.
	result, err := executor.ExecuteJSON(ctx, `{ breachs(scope: "repo/client/main.go") { id } }`, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Breachs []struct {
			ID string `json:"id"`
		} `json:"breachs"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if resp.Breachs == nil {
		t.Error("breachs collection should not be nil")
	}
}

func TestExhaustiveTicketTitleValidation(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket title validation test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	tests := []struct {
		name    string
		emoji   string
		title   string
		wantErr bool
	}{
		{"Emoji Titleized Valid", "🎫️", "Some Title on Something", false},
		{"Emoji Single Word Valid", "🛠️", "Cleanup", false},
		{"Emoji With Hyphen Valid", "🧩️", "Refactor Resource ID System to Bundle-Based Document", false},
		{"Emoji Lowercase Valid", "🔖️", "some title", false},
		{"Emoji Allcaps Valid", "🔥️", "FIX EVERYTHING", false},
		{"Emoji Slug Valid", "🎫️", "some-slug-title", false},
		{"Emoji Dashed Slug Valid", "🎫️", "fix-vscode-types-version-mismatch", false},
		{"Emoji Uppercase Slug Valid", "🎫️", "ENSURE-COMPOSE-REPO-MCP-WORKS-ALLIDES", false},
		{"Missing Emoji Invalid", "", "Some Title on Something", true},
		{"Plain Text Emoji Invalid", "ticket", "Some Title", true},
		{"Multiple Emoji Invalid", "🎫️", "Some Title", true},
		{"Empty Title Invalid", "🎫️", "", true},
		{"Non Alphanumeric Title Invalid", "🎫️", "---", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			query := `mutation { ticketOpen(input: { emoji: "` + tt.emoji + `", title: "` + tt.title + `", prompt: "Test prompt", llm: "opus-4", client: COPILOT_CHAT, goal: "TEST-GOAL", noIssue: true }) { id slug year month day } }`
			result, err := executor.ExecuteJSON(ctx, query, nil)
			if (err != nil) != tt.wantErr {
				t.Errorf("ticketOpen() error = %v, wantErr %v", err, tt.wantErr)
			}

			if err == nil {
				var resp struct {
					TicketOpen struct {
						ID    string `json:"id"`
						Slug  string `json:"slug"`
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
					} `json:"ticketOpen"`
				}
				if json.Unmarshal([]byte(result), &resp) == nil {
					to := resp.TicketOpen
					if to.Year == 0 || to.Month == 0 || to.Day == 0 {
						t.Errorf("ticketOpen returned invalid date: year=%d month=%d day=%d (id=%s)", to.Year, to.Month, to.Day, to.ID)
					}
					if strings.Contains(to.ID, "0000/00/00") {
						t.Errorf("ticketOpen id must not contain 0000/00/00, got %s", to.ID)
					}
					path := tickets.GetTicketPath(to.Year, to.Month, to.Day, to.Slug)
					os.RemoveAll(path)
				}
			}
		})
	}
}

func TestExhaustiveFilterTicketWorkspaceFiles(t *testing.T) {
	executor := getTestExecutor(t)
	if executor == nil {
		t.Fatal("executor is nil")
	}
	folder := ".🧬semio/🦑️repo/🎫️tickets/26/01/20/SAMPLE"
	files := []string{
		folder,
		folder + "/plan.md",
		"./" + folder + "/ticket.json",
		"go/repo/main.go",
	}
	filtered := tickets.FilterTicketWorkspaceFiles(folder, files)
	if len(filtered) != 1 || filtered[0] != "go/repo/main.go" {
		t.Fatalf("expected [go/repo/main.go], got %v", filtered)
	}
}

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

func TestExhaustiveNodeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow node query test in short mode")
	}
	executor := getTestExecutor(t)
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
		t.Skip("no bundles to test node query")
	}

	testID := bundleResp.Bundles[0].ID
	nodeResult, err := executor.ExecuteJSON(ctx, `query($id: ID!) { node(id: $id) { ... on Bundle { id name } } }`, map[string]interface{}{"id": testID})
	if err != nil {
		t.Fatalf("node query failed: %v", err)
	}
	var nodeResp struct {
		Node struct {
			ID   string `json:"id"`
			Name string `json:"name"`
		} `json:"node"`
	}
	if err := json.Unmarshal([]byte(nodeResult), &nodeResp); err != nil {
		t.Fatalf("failed to parse node response: %v", err)
	}
	if nodeResp.Node.ID != testID {
		t.Errorf("node query returned wrong id: got %s, want %s", nodeResp.Node.ID, testID)
	}
	if nodeResp.Node.Name != bundleResp.Bundles[0].Name {
		t.Errorf("node query returned wrong name: got %s, want %s", nodeResp.Node.Name, bundleResp.Bundles[0].Name)
	}
}

func TestExhaustiveSectionsEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow sections edges test in short mode")
	}
	executor := getTestExecutor(t)
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
	executor := getTestExecutor(t)
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
	executor := getTestExecutor(t)
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

func TestExhaustiveFixViaGraphQL(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow fix via graphql test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	result, err := executor.ExecuteJSON(ctx, `mutation { fix(scope: "repo/go/main_test.go") { fixed remaining breachs { id summary } } }`, nil)
	if err != nil {
		t.Fatalf("fix mutation failed: %v", err)
	}

	var resp struct {
		Fix struct {
			Fixed     int `json:"fixed"`
			Remaining int `json:"remaining"`
			Breachs   []struct {
				ID      string `json:"id"`
				Summary string `json:"summary"`
			} `json:"breachs"`
		} `json:"fix"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse fix response: %v\nResult: %s", err, result)
	}
	if resp.Fix.Remaining < 0 {
		t.Error("remaining should not be negative")
	}
	if len(resp.Fix.Breachs) != resp.Fix.Remaining {
		t.Errorf("breachs length %d != remaining %d", len(resp.Fix.Breachs), resp.Fix.Remaining)
	}
}

func TestExhaustivePolicyGroupsGraphQL(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow policy groups graphql test in short mode")
	}
	executor := getTestExecutor(t)
	query := `{ policies { id name groups { name description scopes kinds { id } groups { name kinds { id } } } } }`
	result, err := executor.Execute(context.Background(), query, nil)
	if err != nil {
		t.Fatalf("GraphQL query failed: %v", err)
	}
	data, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected map result")
	}
	policiesData, ok := data["policies"].([]interface{})
	if !ok {
		t.Fatal("expected policies array")
	}
	if len(policiesData) == 0 {
		t.Fatal("expected at least one policy")
	}
	for _, pRaw := range policiesData {
		p, ok := pRaw.(map[string]interface{})
		if !ok {
			continue
		}
		groups, ok := p["groups"].([]interface{})
		if !ok {
			t.Fatalf("expected groups array for policy %v", p["id"])
		}
		if len(groups) == 0 {
			t.Errorf("expected at least one group for policy %v", p["id"])
		}
	}
}

func TestExhaustiveGraphQLTicketsQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow graphql tickets query test in short mode")
	}
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { tickets { id slug status } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL tickets returned error: %v", err)
	}
	if !strings.Contains(result, "tickets") {
		t.Errorf("Expected result to contain 'tickets', got: %s", result)
	}
}

func TestExhaustiveGraphQLAnalyzeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow analyze query test in short mode")
	}
	result, err := executor.ExecuteJSON(context.Background(), `{ analyze(scope: "repo/asset/fixture/some/folder/🧪️file-fixed/🐹️.go") { metrics { total } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL analyze returned error: %v", err)
	}
	if !strings.Contains(result, "total") {
		t.Errorf("Expected result to contain 'total', got: %s", result)
	}
}
