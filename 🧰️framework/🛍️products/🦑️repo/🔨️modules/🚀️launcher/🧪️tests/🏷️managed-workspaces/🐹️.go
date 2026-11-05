package launcher

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestWorkspaceManaged(t *testing.T) {
	var rows []struct {
		ID, Mutation string
		Accepted     bool
	}
	readFixture(t, workspaceLinksBase+"🧫️fixtures/🏷️managed.json", &rows)
	for _, row := range rows {
		t.Run(row.ID, func(t *testing.T) {
			root, policy := launcherSandbox(t), workspaceLinksPolicy(t)
			contract, err := readWorkspaceManagedContract(root, policy)
			if err != nil {
				t.Fatal(err)
			}
			workspaceLinksFixture(t, root, "p/one", "@example/one")
			if err := os.WriteFile(filepath.Join(root, "package.json"), []byte(`{"workspaces":["p/one"]}`), 0o644); err != nil {
				t.Fatal(err)
			}
			generation := strings.Repeat("a", 64)
			generationRoot := filepath.Join(contract.generationRoot, generation[:contract.prefixUnits])
			if row.Mutation == "path" {
				generationRoot = filepath.Join(root, "foreign", generation[:contract.prefixUnits])
			}
			target := filepath.Join(generationRoot, contract.rootEndpoint)
			provider := filepath.Join(generationRoot, "provider")
			if err := os.MkdirAll(filepath.Join(target, "@example"), 0o755); err != nil {
				t.Fatal(err)
			}
			if err := os.Mkdir(provider, 0o755); err != nil {
				t.Fatal(err)
			}
			if err := createWorkspaceDirectoryLink(provider, filepath.Join(target, "@example/one")); err != nil {
				t.Fatal(err)
			}
			endpoint := filepath.Join(root, policy.Directory)
			if err := createWorkspaceDirectoryLink(target, endpoint); err != nil {
				t.Fatal(err)
			}
			binding := map[string]any{"sourceDirectory": endpoint, "targetDirectory": target}
			ready := map[string]any{"version": contract.readyVersion, "purpose": contract.readyPurpose, "scope": contract.rootScope, "generation": generation, "ownerRelationSha256": strings.Repeat("b", 64), "groupSha256": strings.Repeat("c", 64), "recipeSha256": strings.Repeat("d", 64), "members": []any{}, "bindings": []any{binding}, "ownerReferences": []any{}, "singletons": []any{}}
			switch row.Mutation {
			case "generation":
				ready["generation"] = strings.Repeat("e", 64)
			case "binding-source":
				binding["sourceDirectory"] = filepath.Join(root, "wrong")
			case "binding-target":
				binding["targetDirectory"] = provider
			case "purpose":
				ready["purpose"] = "forged"
			case "version":
				ready["version"] = 0
			case "unknown-field":
				ready["authority"] = true
			case "missing-members":
				delete(ready, "members")
			}
			bytes, _ := json.Marshal(ready)
			receipt := filepath.Join(generationRoot, contract.receiptName)
			if err := os.WriteFile(receipt, bytes, 0o644); err != nil {
				t.Fatal(err)
			}
			before, err := os.Lstat(endpoint)
			if err != nil {
				t.Fatal(err)
			}
			namespaceBefore, err := os.Stat(target)
			if err != nil {
				t.Fatal(err)
			}
			classification, err := ClassifyWorkspaceNamespace(root, policy)
			if (err == nil) != row.Accepted {
				t.Fatalf("accepted=%v error=%v", row.Accepted, err)
			}
			if row.Accepted {
				if classification.Kind != "managed" || classification.Authority || classification.Readiness || classification.ContentObserved {
					t.Fatalf("classification attested authority: %+v", classification)
				}
				if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err != nil {
					t.Fatal(err)
				}
			}
			after, err := os.Lstat(endpoint)
			if err != nil {
				t.Fatal(err)
			}
			namespaceAfter, err := os.Stat(target)
			if err != nil {
				t.Fatal(err)
			}
			if !os.SameFile(before, after) || !before.ModTime().Equal(after.ModTime()) || !namespaceBefore.ModTime().Equal(namespaceAfter.ModTime()) {
				t.Fatal("managed namespace was mutated")
			}
			current, err := os.ReadFile(receipt)
			if err != nil || string(current) != string(bytes) {
				t.Fatalf("managed receipt was mutated: %v", err)
			}
		})
	}
}
