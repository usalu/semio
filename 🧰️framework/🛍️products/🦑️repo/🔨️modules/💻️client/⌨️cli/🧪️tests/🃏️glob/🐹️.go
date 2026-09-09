package client

import (
	"encoding/json"
	glob "github.com/usalu/semio/repo/client/internal/glob"
	"os"
	"path/filepath"
	"testing"
)

// 🃏️ TestCanonicalFixtureGlob verifies policy scope selection against shared path examples.
func TestCanonicalFixtureGlob(t *testing.T) {
	data, err := os.ReadFile(filepath.Join(GetRootDir(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🃏️glob/🔣️.json"))
	if err != nil {
		t.Fatal(err)
	}
	var fixture struct {
		Cases []struct {
			ID      string
			Pattern string
			Path    string
			Match   bool
		}
	}
	if err := json.Unmarshal(data, &fixture); err != nil {
		t.Fatal(err)
	}
	for _, scenario := range fixture.Cases {
		t.Run(scenario.ID, func(t *testing.T) {
			got, err := glob.Match(scenario.Pattern, scenario.Path)
			if err != nil || got != scenario.Match {
				t.Fatalf("Match(%q, %q) = %v, %v; expected %v", scenario.Pattern, scenario.Path, got, err, scenario.Match)
			}
		})
	}
}
