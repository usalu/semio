// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Interaction author shapes, contributor discovery, and MCP bootstrap assets.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

func TestInteractionUnmarshalAuthorShapes(t *testing.T) {
	cases := []struct {
		name         string
		authorJSON   string
		expectedAuth string
	}{
		{
			name:         "string author",
			authorJSON:   `"Ueli Saluz <ueli@semio-tech.com>"`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
		{
			name:         "object author",
			authorJSON:   `{"name":"Ueli Saluz","email":"ueli@semio-tech.com","github":"usalu"}`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			raw := fmt.Sprintf(`{
				"date": "2026-02-06 22:03:11",
				"author": %s,
				"system": "linux",
				"client": "codex",
				"checkpoint": "abc123",
				"prompt": "test",
				"llm": "gpt-5-2-codex"
			}`, tc.authorJSON)

			var interaction Interaction
			if err := json.Unmarshal([]byte(raw), &interaction); err != nil {
				t.Fatalf("unexpected unmarshal error: %v", err)
			}
			if interaction.Author != tc.expectedAuth {
				t.Fatalf("expected author %q, got %q", tc.expectedAuth, interaction.Author)
			}
		})
	}
}

func TestContributorDiscovery(t *testing.T) {

	tmpDir, err := os.MkdirTemp("", "compose-test-discovery")
	if err != nil {
		t.Fatalf("failed to create tmp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	originalRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(originalRootDir)

	contributorsDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🧑️‍💻️devs")
	os.MkdirAll(contributorsDir, 0755)

	t.Run("Match and update email", func(t *testing.T) {

		github := "usalu"
		c := Contributor{
			Github: github,
			Name:   "Ueli Saluz",
			Names:  []string{"Ueli Saluz"},
			Email:  "ueli@semio-tech.com",
			Emails: []string{"ueli@semio-tech.com"},
		}
		if err := SaveContributor(c); err != nil {
			t.Fatalf("failed to save: %v", err)
		}

		authorStr := "Ueli <ueli@semio-tech.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != github {
			t.Errorf("expected github %q, got %q", github, gotGithub)
		}

		updated, err := LoadContributor(github)
		if err != nil {
			t.Fatalf("failed to load: %v", err)
		}
		if len(updated.Names) != 2 || updated.Names[1] != "Ueli" {
			t.Errorf("expected names updated, got %v", updated.Names)
		}
	})

	t.Run("Match and update name", func(t *testing.T) {

		github := "octocat"
		c := Contributor{
			Github: github,
			Name:   "The Octocat",
			Names:  []string{"The Octocat"},
			Email:  "octocat@github.com",
			Emails: []string{"octocat@github.com"},
		}
		SaveContributor(c)

		authorStr := "The Octocat <octo@github.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != github {
			t.Errorf("expected github %q, got %q", github, gotGithub)
		}

		updated, _ := LoadContributor(github)
		found := false
		for _, e := range updated.Emails {
			if e == "octo@github.com" {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("expected emails updated with octo@github.com, got %v", updated.Emails)
		}
	})

	t.Run("No match returns original string", func(t *testing.T) {
		authorStr := "Stranger <stranger@danger.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != authorStr {
			t.Errorf("expected original string, got %q", gotGithub)
		}
	})
}
