// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Entity, artifact, diffable kinds, and list ID shapes.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

func TestEntityKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"technology", "bundle", "folder", "file", "line", "range",
		"section", "definition", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "checkpoint", "interaction", "session",
	}
	if len(EntityKinds) != len(expected) {
		t.Fatalf("EntityKinds length: expected %d, got %d", len(expected), len(EntityKinds))
	}
	for i, e := range expected {
		if EntityKinds[i] != e {
			t.Errorf("EntityKinds[%d]: expected %q, got %q", i, e, EntityKinds[i])
		}
	}
}

// 🌪️TestAllEntityEmojisProjectsTheFrameworkCatalog holds the CLI's entity-emoji answer against the
// language-agnostic catalog document every implementation reads, not against the Go projection it is
// built from, so a hand-written list can never drift back in.
func TestAllEntityEmojisProjectsTheFrameworkCatalog(t *testing.T) {
	_, currentFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("failed to resolve current test file path")
	}
	repoRoot := findTestRepoRoot(filepath.Dir(currentFile))
	catalogPath := filepath.Join(repoRoot, "🧰️framework", "🔨️modules", "🧬️schema", "🏷️entity-kinds", "🔣️.json")
	raw, err := os.ReadFile(catalogPath)
	if err != nil {
		t.Fatalf("failed to read the framework entity-kind catalog at %s: %v", catalogPath, err)
	}
	var catalog []struct {
		ID         string `json:"id"`
		Emoji      string `json:"emoji"`
		IconID     string `json:"iconId"`
		Label      string `json:"label"`
		Filterable bool   `json:"filterable"`
	}
	if err := json.Unmarshal(raw, &catalog); err != nil {
		t.Fatalf("failed to decode the framework entity-kind catalog: %v", err)
	}
	if len(catalog) != len(EntityKindCatalog) {
		t.Fatalf("generated Go projection is stale: catalog has %d kinds, EntityKindCatalog has %d", len(catalog), len(EntityKindCatalog))
	}
	for index, kind := range catalog {
		if EntityKindCatalog[index].ID != kind.ID || EntityKindCatalog[index].Emoji != kind.Emoji || EntityKindCatalog[index].IconID != kind.IconID || EntityKindCatalog[index].Label != kind.Label || EntityKindCatalog[index].Filterable != kind.Filterable {
			t.Fatalf("generated Go projection is stale at %d: catalog %q/%q, projection %q/%q", index, kind.ID, kind.Emoji, EntityKindCatalog[index].ID, EntityKindCatalog[index].Emoji)
		}
	}

	seen := map[string]bool{}
	var expected []string
	for _, kind := range catalog {
		normalized := emojiText(kind.Emoji)
		if normalized == "" || seen[normalized] {
			continue
		}
		seen[normalized] = true
		expected = append(expected, normalized)
	}
	actual := AllEntityEmojis()
	if len(actual) != len(expected) {
		t.Fatalf("AllEntityEmojis length: expected %d, got %d", len(expected), len(actual))
	}
	for index, want := range expected {
		if actual[index] != want {
			t.Errorf("AllEntityEmojis[%d]: expected %q, got %q", index, want, actual[index])
		}
	}
	for _, required := range []string{EmojiFileTemplate, EmojiBreachScope, EmojiFileResource} {
		if !seen[emojiText(required)] {
			t.Errorf("AllEntityEmojis is missing %q, which the catalog declares", required)
		}
	}
	if kind, found := EntityKindByEmoji(EmojiTechnologyMono); !found || kind.ID != "technology-mono" {
		t.Errorf("first-wins index: expected technology-mono for the shared seedling emoji, got %q (found=%v)", kind.ID, found)
	}
	if kind, found := EntityKindByEmoji("📝️"); !found || kind.ID != "draft" {
		t.Errorf("first-wins index: expected draft for the shared note emoji, got %q (found=%v)", kind.ID, found)
	}
	t.Logf("[DEBUG] Repo CLI Go entity projection observed: %d complete source rows and FIRST-WINS emoji lookups", len(catalog))

}

func TestArtifactKinds(t *testing.T) {
	expected := []string{"repo", "technology", "bundle", "folder", "file", "section", "definition"}
	if len(ArtifactKinds) != len(expected) {
		t.Fatalf("ArtifactKinds length: expected %d, got %d", len(expected), len(ArtifactKinds))
	}
	for i, e := range expected {
		if ArtifactKinds[i] != e {
			t.Errorf("ArtifactKinds[%d]: expected %q, got %q", i, e, ArtifactKinds[i])
		}
	}
}

func TestDiffableKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour",
		"technology", "bundle", "folder", "file", "section", "definition",
		"goal", "ticket", "contributor", "checkpoint", "interaction", "session",
	}
	if len(DiffableKinds) != len(expected) {
		t.Fatalf("DiffableKinds length: expected %d, got %d", len(expected), len(DiffableKinds))
	}
	for i, e := range expected {
		if DiffableKinds[i] != e {
			t.Errorf("DiffableKinds[%d]: expected %q, got %q", i, e, DiffableKinds[i])
		}
	}
}

func TestRelatedToFileKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"technology", "bundle", "folder", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "checkpoint", "interaction", "session",
	}
	if len(RelatedToFileKinds) != len(expected) {
		t.Fatalf("RelatedToFileKinds length: expected %d, got %d", len(expected), len(RelatedToFileKinds))
	}
	for i, e := range expected {
		if RelatedToFileKinds[i] != e {
			t.Errorf("RelatedToFileKinds[%d]: expected %q, got %q", i, e, RelatedToFileKinds[i])
		}
	}
}

func TestTechnologyListIDs(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Fatalf("ToolTechnologyList returned error: %s", result.Error)
	}
	technologies, ok := result.Data.([]Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	expectedIDs := map[string]string{
		"compose": emojiText(EmojiTechnologyUser) + "compose",
		"repo":    emojiText(EmojiTechnologyInfra) + "repo",
		"coda":    emojiText(EmojiTechnologyResearch) + "coda",
	}
	for _, p := range technologies {
		expected, ok := expectedIDs[p.Name]
		if !ok {
			continue
		}
		got := p.GetID()
		if got != expected {
			t.Errorf("technology %q id: expected %q, got %q", p.Name, expected, got)
		}
		delete(expectedIDs, p.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected technology %q not found in list", name)
	}
}

func TestBundleListIDs(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolBundleList()
	if result.Error != "" {
		t.Fatalf("ToolBundleList returned error: %s", result.Error)
	}
	bundles, ok := result.Data.([]Bundle)
	if !ok {
		t.Fatal("ToolBundleList data is not []Bundle")
	}
	expectedIDs := map[string]string{
		"compose/js":  emojiText(EmojiTechnologyUser) + "compose" + emojiText("📜️") + "js",
		"compose/go":  emojiText(EmojiTechnologyUser) + "compose" + emojiText("🐹️") + "go",
		"repo/client": emojiText(EmojiTechnologyInfra) + "repo" + emojiText("⌨️") + "client",
	}
	for _, b := range bundles {
		expected, ok := expectedIDs[b.Name]
		if !ok {
			continue
		}
		got := b.GetID()
		if got != expected {
			t.Errorf("bundle %q id: expected %q, got %q", b.Name, expected, got)
		}
		delete(expectedIDs, b.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected bundle %q not found in list", name)
	}
}

func TestSectionListIDs(t *testing.T) {
	result := ToolSectionList(withSectionFixture(t))
	if result.Error != "" {
		t.Fatalf("ToolSectionList returned error: %s", result.Error)
	}
	sections, ok := result.Data.([]Section)
	if !ok {
		t.Fatal("ToolSectionList data is not []Section")
	}
	if len(sections) == 0 {
		t.Fatal("ToolSectionList returned no sections")
	}
	seenEmojis := make(map[string]string)
	for _, s := range sections {
		localID := s.GetID()
		emoji := s.Emoji
		if emoji == "" {
			t.Errorf("section %q has no emoji", s.Name)
			continue
		}
		flatName := Flat(s.Name)
		expectedID := emojiText(emoji) + flatName
		if localID != expectedID {
			t.Errorf("section %q local id: expected %q, got %q", s.Name, expectedID, localID)
		}
		if prev, exists := seenEmojis[emoji]; exists {
			t.Errorf("section %q has duplicate emoji %q (same as %q)", s.Name, emoji, prev)
		}
		seenEmojis[emoji] = s.Name
	}
}

func TestContributorListIDs(t *testing.T) {
	setupToolTest(t)
	result := ToolContributorList()
	if result.Error != "" {
		t.Fatalf("ToolContributorList returned error: %s", result.Error)
	}
	contributors, ok := result.Data.([]Contributor)
	if !ok {
		t.Fatal("ToolContributorList data is not []Contributor")
	}
	if len(contributors) == 0 {
		t.Fatal("ToolContributorList returned no contributors")
	}
	for _, c := range contributors {
		id := c.GetID()
		expectedPrefix := emojiText(EmojiContributor)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("contributor %q id %q should start with %q", c.Alias, id, expectedPrefix)
		}
		expectedID := expectedPrefix + Flat(c.Alias)
		if id != expectedID {
			t.Errorf("contributor %q id: expected %q, got %q", c.Alias, expectedID, id)
		}
	}
	foundUeli := false
	for _, c := range contributors {
		if c.Alias == "ueli" {
			if c.GetID() != emojiText(EmojiContributor)+"ueli" {
				t.Errorf("ueli contributor id: expected %q, got %q", emojiText(EmojiContributor)+"ueli", c.GetID())
			}
			foundUeli = true
		}
	}
	if !foundUeli {
		t.Error("expected to find contributor 'ueli'")
	}
}

func TestGoalListIDs(t *testing.T) {
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*Goal)
	if !ok {
		t.Fatalf("ToolGoalList data is not []*Goal, got %T", result.Data)
	}
	for _, g := range goals {
		id := g.GetID()
		goalEmoji := emojiText(EmojiGoal)
		if !strings.HasPrefix(id, goalEmoji) {
			t.Errorf("goal %q id %q should start with %q", g.ID, id, goalEmoji)
		}
		var expected strings.Builder
		for _, segment := range strings.Split(g.ID, "/") {
			expected.WriteString(goalEmoji)
			expected.WriteString(Flat(segment))
		}
		if id != expected.String() {
			t.Errorf("goal %q id: expected %q, got %q", g.ID, expected.String(), id)
		}
	}
}

func TestExhaustiveTicketListIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket list ids test in short mode")
	}
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Skipf("ToolTicketList returned error: %s", result.Error)
	}
	tickets, ok := result.Data.([]Ticket)
	if !ok {
		t.Skip("ToolTicketList data is not []Ticket")
	}
	for _, tk := range tickets {
		id := tk.GetID()
		expectedPrefix := emojiText(EmojiTicket)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("ticket %q id %q should start with %q", tk.Slug, id, expectedPrefix)
		}
		expectedID := expectedPrefix + Flat(tk.Slug)
		if id != expectedID {
			t.Errorf("ticket %q id: expected %q, got %q", tk.Slug, expectedID, id)
		}
	}
}

func TestDraftListIDs(t *testing.T) {
	result := ToolDraftList()
	if result.Error != "" {
		t.Skipf("ToolDraftList returned error: %s", result.Error)
	}
	drafts, ok := result.Data.([]*Draft)
	if !ok {
		t.Skip("ToolDraftList data is not []*Draft")
	}
	for _, d := range drafts {
		id := d.GetID()
		expectedPrefix := emojiText(EmojiDraft)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("draft %q id %q should start with %q", d.ID, id, expectedPrefix)
		}
	}
}
