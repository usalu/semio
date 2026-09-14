// 🔬️ Tests of the contributors domain, split out of the pre-split godfile suite.

package contributors

import (
	os "os"
	filepath "path/filepath"
	testing "testing"

	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestContributorDiscovery(t *testing.T) {

	tmpDir, err := os.MkdirTemp("", "compose-test-discovery")
	if err != nil {
		t.Fatalf("failed to create tmp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	originalRootDir := workspace.GetRootDir()
	workspace.SetRootDir(tmpDir)
	defer workspace.SetRootDir(originalRootDir)

	contributorsDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🧑️‍💻️devs")
	os.MkdirAll(contributorsDir, 0755)

	t.Run("Match and update email", func(t *testing.T) {

		github := "usalu"
		c := model.Contributor{
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
		c := model.Contributor{
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

func TestSessionKindEmoji(t *testing.T) {
	tests := []struct {
		kind SessionKind
		want string
	}{
		{SessionKindRunning, identity.Entity("session-running")},
		{SessionKindCompleted, identity.Entity("session-completed")},
		{SessionKindInterrupted, identity.Entity("session-interrupted")},
	}
	for _, tt := range tests {
		t.Run(string(tt.kind), func(t *testing.T) {
			got := SessionKindEmoji(tt.kind)
			if got != tt.want {
				t.Errorf("SessionKindEmoji(%q) = %q, want %q", tt.kind, got, tt.want)
			}
		})
	}
}

func TestSessionGetID(t *testing.T) {
	uuid := "e753ed61-e8cc-49b7-88f7-dda53b8d5a15"
	uuidFlat := "e753ed61e8cc49b788f7dda53b8d5a15"
	checkpointSHA := "abc123sha"
	checkpointId := model.GetArtifactID("checkpoint", map[string]interface{}{"sha": checkpointSHA})
	cases := []struct {
		name     string
		session  Session
		expected string
	}{
		{
			name:     "with checkpoint as parent",
			session:  Session{UUID: uuid, Year: 26, Month: 2, Day: 15, Checkpoint: checkpointSHA, Kind: SessionKindCompleted},
			expected: checkpointId + model.EmojiText(model.EmojiSession) + uuidFlat,
		},
		{
			name:     "without checkpoint falls back to date document",
			session:  Session{UUID: uuid, Year: 26, Month: 2, Day: 15, Kind: SessionKindCompleted},
			expected: model.EmojiText(model.EmojiYear) + "26" + model.EmojiText(model.EmojiMonth) + "02" + model.EmojiText(model.EmojiDay) + "15" + model.EmojiText(model.EmojiSession) + uuidFlat,
		},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := tc.session.GetID()
			if id != tc.expected {
				t.Errorf("Session.GetID() = %q, want %q", id, tc.expected)
			}
		})
	}
}

func TestSessionGetURI(t *testing.T) {
	s := Session{
		UUID:  "e753ed61-e8cc-49b7-88f7-dda53b8d5a15",
		Year:  26,
		Month: 2,
		Day:   15,
		Kind:  SessionKindCompleted,
	}
	uri := s.GetURI()
	expected := "repo://session/" + model.EmojiText(model.EmojiSession) + "e753ed61e8cc49b788f7dda53b8d5a15"
	if uri != expected {
		t.Errorf("Session.GetURI() = %q, want %q", uri, expected)
	}
}
