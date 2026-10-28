// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗣️language-contract proves language resolution against the shared fixture of the TypeScript resolver, and the bilingual completeness of every refusal.

// #endregion 🧲️Header

package launcher

import (
	"regexp"
	"strings"
	"testing"
)

const languageFixture = "🧰️framework/🔨️modules/⚙️runtime/🗣️language/🧪️tests/🔣️.json"

// 🗣️Every resolution row of the shared fixture resolves, or is refused, the same way.
func TestLanguageResolutionFixture(t *testing.T) {
	var fixture struct {
		Languages []string
		Resolve   []struct {
			Name        string
			Args        []string
			Environment map[string]string
			Os          []string
			Expected    *struct{ Language, Source string }
			Refusal     string
		}
		Tags    [][2]string
		Neutral []string
	}
	readFixture(t, languageFixture, &fixture)
	registry := loadRegistry(t)
	protocol, err := LoadLanguageProtocol(testRoot(t), registry)
	if err != nil {
		t.Fatal(err)
	}
	for _, row := range fixture.Resolve {
		got, refusal := protocol.ResolveLanguage(fixture.Languages, row.Args, environment(row.Environment), func(OsLocale) []string { return row.Os })
		if row.Expected != nil {
			if refusal != nil || got.Language != row.Expected.Language || got.Source != row.Expected.Source {
				t.Fatalf("%s: %+v %v", row.Name, got, refusal)
			}
			continue
		}
		if refusal == nil {
			t.Fatalf("%s: expected a refusal", row.Name)
		}
		text := strings.Split(protocol.Refuse(registry.Languages, refusal), "\n")
		if len(text) != len(registry.Languages) {
			t.Fatalf("%s: one line per language, got %d", row.Name, len(text))
		}
		want := map[string][2]string{"none": {"--lang=en|de", "--lang=en|de"}, "flag": {"Unknown language", "Unbekannte Sprache"}, "variable": {"SEMIO_LANGUAGE names a language that is not offered", "nennt eine nicht angebotene Sprache"}}[row.Refusal]
		if !strings.Contains(text[0], want[0]) || !strings.Contains(text[1], want[1]) {
			t.Fatalf("%s: %q", row.Name, text)
		}
	}
	for _, pair := range fixture.Tags {
		if got := protocol.LanguageSubtag(pair[0]); got != pair[1] {
			t.Fatalf("%s: %s", pair[0], got)
		}
	}
	for _, neutral := range fixture.Neutral {
		if got := protocol.LanguageSubtag(neutral); got != "" {
			t.Fatalf("%q must be neutral, got %q", neutral, got)
		}
	}
}

// 🗣️The flag reader stops at the separator and keeps the last value.
func TestLanguageFlag(t *testing.T) {
	protocol, err := LoadLanguageProtocol(testRoot(t), loadRegistry(t))
	if err != nil {
		t.Fatal(err)
	}
	if value, given := protocol.LanguageFlag([]string{"a", "--lang", "de", "--", "--lang=en"}); !given || value != "de" {
		t.Fatal(value)
	}
	if value, _ := protocol.LanguageFlag([]string{"--lang=en", "--lang=de"}); value != "de" {
		t.Fatal(value)
	}
	if _, given := protocol.LanguageFlag(nil); given {
		t.Fatal("no flag")
	}
}

// 🍎️The macOS command output is parsed by the pattern of the protocol.
func TestLanguageSystemPattern(t *testing.T) {
	protocol, err := LoadLanguageProtocol(testRoot(t), loadRegistry(t))
	if err != nil {
		t.Fatal(err)
	}
	pattern := regexp.MustCompile(protocol.OsLocale.Darwin.Pattern)
	var tags []string
	for _, match := range pattern.FindAllStringSubmatch("(\n    \"en-US\",\n    \"de-CH\",\n    \"zh-Hans-CN\"\n)", -1) {
		tags = append(tags, match[1])
	}
	if strings.Join(tags, " ") != "en-US de-CH zh-Hans-CN" {
		t.Fatal(tags)
	}
}

// #endregion 🗣️Language
