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

func TestWorkspaceDetached(t *testing.T) {
	var rows []struct {
		ID, Mutation string
		Accepted     bool
	}
	readFixture(t, workspaceLinksBase+"🧫️fixtures/🔌️detached.json", &rows)
	registry := loadRegistry(t)
	for _, row := range rows {
		t.Run(row.ID, func(t *testing.T) {
			root, policy := launcherSandbox(t), workspaceLinksPolicy(t)
			contract, err := readWorkspaceManagedContract(root, policy)
			if err != nil {
				t.Fatal(err)
			}
			prefix := strings.Repeat("a", contract.prefixUnits)
			store, name := contract.generationRoot, contract.rootEndpoint
			switch row.Mutation {
			case "foreign-store":
				store = filepath.Join(root, "foreign")
			case "wrong-name":
				name = "elsewhere"
			case "wrong-generation":
				prefix = strings.Repeat("z", contract.prefixUnits)
			case "other-store-depth":
				store = filepath.Join(store, "deeper")
			}
			target := filepath.Join(store, prefix, name)
			endpoint := filepath.Join(root, policy.Directory)
			link := target
			if row.Mutation == "relative" {
				if link, err = filepath.Rel(filepath.Dir(endpoint), target); err != nil {
					t.Fatal(err)
				}
			}
			if row.Mutation == "unready-image" {
				if err := os.MkdirAll(target, 0o755); err != nil {
					t.Fatal(err)
				}
			}
			if err := createWorkspaceDirectoryLink(link, endpoint); err != nil {
				t.Fatal(err)
			}
			if _, err := os.Stat(endpoint); row.Mutation != "unready-image" && !os.IsNotExist(err) {
				t.Fatalf("store is attached: %v", err)
			}
			classification, err := ClassifyWorkspaceNamespace(root, policy)
			if (err == nil) != row.Accepted {
				t.Fatalf("accepted=%v error=%v", row.Accepted, err)
			}
			if row.Accepted {
				if classification.Kind != "detached" || classification.Authority || classification.Readiness || classification.ContentObserved {
					t.Fatalf("classification attested authority: %+v", classification)
				}
				if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err != nil {
					t.Fatal(err)
				}
			} else {
				var failure Failure
				if !asFailure(err, &failure) || failure.ID != "namespaceDangling" {
					t.Fatalf("refusal without remedy: %v", err)
				}
				for _, language := range registry.Languages {
					text := registry.Say(language, failure.ID, failure.Values)
					if !strings.Contains(text, endpoint) || !strings.Contains(text, "repo dependencies sync --all") || strings.Contains(text, "{") {
						t.Fatalf("%s refusal does not name the remedy: %s", language, text)
					}
				}
				if err := PrepareWorkspacePackageLinks(context.Background(), root, policy); err == nil {
					t.Fatal("foreign dangling namespace accepted")
				}
			}
			kept, err := os.Readlink(endpoint)
			if err != nil || kept != link {
				t.Fatalf("dangling link was mutated: %q %v", kept, err)
			}
		})
	}
}

func TestWorkspaceDetachedHandsOverToTheImplementation(t *testing.T) {
	setup, sandbox, policy := seededHost(t, map[string]string{"SEMIO_LANGUAGE": "en"}, true), launcherSandbox(t), workspaceLinksPolicy(t)
	contract, err := readWorkspaceManagedContract(sandbox, policy)
	if err != nil {
		t.Fatal(err)
	}
	if err := createWorkspaceDirectoryLink(filepath.Join(contract.generationRoot, strings.Repeat("b", contract.prefixUnits), contract.rootEndpoint), filepath.Join(sandbox, policy.Directory)); err != nil {
		t.Fatal(err)
	}
	setup.host.Prepare = func(ctx context.Context, _ string) error { return PrepareWorkspacePackageLinks(ctx, sandbox, policy) }
	handedOff := false
	setup.host.HandOff = func(string, []string, []string) error { handedOff = true; return nil }
	if code := setup.host.Execute(context.Background(), setup.root, "build", []string{"--help"}); code != 0 || !handedOff {
		t.Fatalf("detached store blocked the implementation: code=%d notices=%q", code, setup.log.err)
	}
}
