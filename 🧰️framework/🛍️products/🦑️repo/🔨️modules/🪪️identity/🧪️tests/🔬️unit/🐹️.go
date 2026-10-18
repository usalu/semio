// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unit tests for identifier generation, relative time rendering and the entity-emoji codec.

// #endregion 🧲️Header

package identity

import (
	"regexp"
	"testing"
	"time"
)

// #region 🆔️Identifier

func TestSeededIdentifiersAreReproducible(t *testing.T) {
	left, err := NewFrom(NewSeededEntropy(42))
	if err != nil {
		t.Fatal(err)
	}
	right, err := NewFrom(NewSeededEntropy(42))
	if err != nil {
		t.Fatal(err)
	}
	if left.String() != right.String() {
		t.Fatalf("the same seed produced %q and %q", left, right)
	}
	other, err := NewFrom(NewSeededEntropy(43))
	if err != nil {
		t.Fatal(err)
	}
	if other.String() == left.String() {
		t.Fatal("different seeds must produce different identifiers")
	}
}

func TestIdentifiersCarryTheVersionAndVariantBits(t *testing.T) {
	shape := regexp.MustCompile(`^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$`)
	for seed := uint64(0); seed < 32; seed++ {
		value, err := NewFrom(NewSeededEntropy(seed))
		if err != nil {
			t.Fatal(err)
		}
		if !shape.MatchString(value.String()) {
			t.Fatalf("identifier %q is not a version 4 rendering", value)
		}
	}
	if !shape.MatchString(New().String()) {
		t.Fatal("the platform identifier is not a version 4 rendering")
	}
}

// #endregion 🆔️Identifier

// #region ⏳️RelativeTime

func TestRelativeTimePluralisesAndSigns(t *testing.T) {
	cases := []struct {
		delta time.Duration
		want  string
	}{
		{0, "1 second ago"},
		{30 * time.Second, "30 seconds ago"},
		{2 * time.Minute, "2 minutes ago"},
		{time.Hour, "1 hour ago"},
		{50 * time.Hour, "2 days ago"},
		{40 * 24 * time.Hour, "1 month ago"},
		{800 * 24 * time.Hour, "2 years ago"},
		{-time.Hour, "1 hour from now"},
	}
	for _, item := range cases {
		if got := Duration(item.delta); got != item.want {
			t.Errorf("Duration(%v) = %q, want %q", item.delta, got, item.want)
		}
	}
}

func TestTimeAtUsesTheInjectedClock(t *testing.T) {
	now := time.Date(2026, 9, 6, 12, 0, 0, 0, time.UTC)
	if got := TimeAt(now.Add(-3*time.Hour), FixedClock{Instant: now}); got != "3 hours ago" {
		t.Errorf("TimeAt = %q, want 3 hours ago", got)
	}
}

// #endregion ⏳️RelativeTime

// #region 🔣️EmojiTable

func TestTheEmojiTableLoads(t *testing.T) {
	table, err := LoadEntityEmojiTable()
	if err != nil {
		t.Fatal(err)
	}
	if table.Schema != "semio.repo.identity.entity-emojis/1" {
		t.Fatalf("schema = %q", table.Schema)
	}
	if Entity("goal") == "" || Entity("ticket") == "" || Collection("codebase") == "" {
		t.Fatal("the vocabulary is incomplete")
	}
}

func TestAllEntityEmojisAreDeduplicatedAndNormalised(t *testing.T) {
	emojis := AllEntityEmojis()
	if len(emojis) < 40 {
		t.Fatalf("got %d entity emojis, want at least 40", len(emojis))
	}
	seen := map[string]bool{}
	for _, emoji := range emojis {
		if seen[emoji] {
			t.Fatalf("duplicate entity emoji %q", emoji)
		}
		seen[emoji] = true
		if emoji != EmojiText(emoji) {
			t.Fatalf("entity emoji %q is not normalised", emoji)
		}
	}
}

// #endregion 🔣️EmojiTable

// #region 😀️EmojiCodec

func TestEmojiTextAddsThePresentationSelectorOnlyToTextDefaults(t *testing.T) {
	if got := EmojiText("⚙"); got != "⚙️" {
		t.Errorf("EmojiText(gear) = %q, want the presentation form", got)
	}
	if got := EmojiText("🎯️"); got != "🎯" {
		t.Errorf("EmojiText(target) = %q, want the bare form", got)
	}
	if got := EmojiText("☀︎"); got != "☀️" {
		t.Errorf("EmojiText(sun) = %q, want the presentation form", got)
	}
}

func TestExtractEntityEmojiSplitsTheLeadingGrapheme(t *testing.T) {
	cases := []struct {
		input     string
		emoji     string
		remaining string
	}{
		{"", "", ""},
		{"plain", "", "plain"},
		{"🎯️goal", "🎯️", "goal"},
		{"🧑‍💻dev", "🧑‍💻", "dev"},
		{"1️⃣one", "1️⃣", "one"},
		{"5plain", "", "5plain"},
		{"🇩🇪flag", "🇩🇪", "flag"},
	}
	for _, item := range cases {
		emoji, remaining := ExtractEntityEmoji(item.input)
		if emoji != item.emoji || remaining != item.remaining {
			t.Errorf("ExtractEntityEmoji(%q) = (%q, %q), want (%q, %q)", item.input, emoji, remaining, item.emoji, item.remaining)
		}
	}
}

func TestFlatKeepsOnlyAsciiAlphanumerics(t *testing.T) {
	if got := Flat("AI-OPTIMIZED-REPO"); got != "aioptimizedrepo" {
		t.Errorf("Flat = %q", got)
	}
}

// #endregion 😀️EmojiCodec

// #region 🧱️SemanticId

func TestGoalComposeIdsRoundTrip(t *testing.T) {
	compose := GoalPathToComposeID("AI-OPTIMIZED-REPO/REPO-CLI")
	segments := ComposeIDToGoalSegments(compose)
	if len(segments) != 2 || segments[0] != "aioptimizedrepo" || segments[1] != "repocli" {
		t.Fatalf("segments = %v", segments)
	}
	if GoalPathToComposeID("") != "" {
		t.Error("an empty goal path must produce an empty compose id")
	}
}

func TestContributorComposeIdsRoundTrip(t *testing.T) {
	compose := ContributorToComposeID("Ueli-Saluz")
	if ComposeIDToContributorFlat(compose) != "uelisaluz" {
		t.Fatalf("round trip lost the alias: %q", compose)
	}
	if ContributorToComposeID(compose) != compose {
		t.Error("an already-composed id must be returned unchanged")
	}
	if ContributorToComposeID("unknown") != "unknown" {
		t.Error("unknown is a sentinel and must be preserved")
	}
}

func TestParseSemanticIdsSplitsEverySegment(t *testing.T) {
	compose := EmojiText(Entity("year")) + "26" + EmojiText(Entity("month")) + "09" + EmojiText(Entity("day")) + "06"
	ids := ParseSemanticIds(compose)
	if len(ids) != 3 {
		t.Fatalf("got %d segments: %#v", len(ids), ids)
	}
	if ids[0].Value != "26" || ids[1].Value != "09" || ids[2].Value != "06" {
		t.Fatalf("segments = %#v", ids)
	}
	if ids[0].String() != EmojiText(Entity("year"))+"26" {
		t.Errorf("String = %q", ids[0].String())
	}
}

// #endregion 🧱️SemanticId

// #region 🟦️ArtifactRef

func TestParseArtifactRefClassifiesByPrefix(t *testing.T) {
	cases := []struct {
		input    string
		kind     string
		path     string
		sections []string
	}{
		{"💻️a/b.go", "file", "a/b.go", nil},
		{"🔖️a/b.go#Header", "section", "a/b.go", []string{"Header"}},
		{"🔖️a/b.go#Outer#Inner", "section", "a/b.go", []string{"Outer", "Inner"}},
		{"🛠️a/b.go", "definition", "a/b.go", nil},
		{"🗃️a/b/", "folder", "a/b", nil},
		{"a/b/", "folder", "a/b", nil},
		{"a/b.go", "file", "a/b.go", nil},
		{"a/b.go#Header", "section", "a/b.go", []string{"Header"}},
	}
	for _, item := range cases {
		got := ParseArtifactRef(item.input)
		if got.Kind != item.kind || got.Path != item.path {
			t.Errorf("ParseArtifactRef(%q) = %+v, want kind %q path %q", item.input, got, item.kind, item.path)
		}
		if len(got.SectionParts) != len(item.sections) {
			t.Errorf("ParseArtifactRef(%q) sections = %v, want %v", item.input, got.SectionParts, item.sections)
			continue
		}
		for index := range item.sections {
			if got.SectionParts[index] != item.sections[index] {
				t.Errorf("ParseArtifactRef(%q) sections = %v, want %v", item.input, got.SectionParts, item.sections)
			}
		}
	}
}

func TestNormalizePathUsesForwardSlashes(t *testing.T) {
	if got := NormalizePath(`.\a\b.go`); got != "a/b.go" {
		t.Errorf("NormalizePath = %q", got)
	}
}

// #endregion 🟦️ArtifactRef
