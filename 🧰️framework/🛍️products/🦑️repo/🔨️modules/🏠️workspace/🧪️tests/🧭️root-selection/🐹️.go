package workspace

import (
 "encoding/json"
 "fmt"
 "os"
 "path/filepath"
 "strings"
 "testing"
 "unicode/utf16"
)

func TestExplicitRepositoryRootUsesProvidedDirectory(t *testing.T) {
 cwd, err := os.Stat("."); if err != nil { t.Fatal(err) }
 raw, err := os.ReadFile(filepath.Join(FixtureOrigin, "..", "..", "🧭️root-selection", "🧪️tests", "🧫️fixtures", "🔣️.json")); if err != nil { t.Fatal(err) }
 var corpus struct { Cases []struct { Name string; Cwd string; Explicit *string; Expected *string }; PrefixCases []struct { Path string; Expected string } }; if err := json.Unmarshal(raw, &corpus); err != nil { t.Fatal(err) }
 for _, row := range corpus.PrefixCases { if got := normalizeRootPath(row.Path); got != row.Expected { t.Fatalf("prefix %q: %q != %q", row.Path, got, row.Expected) } }
 parent := os.Getenv("SEMIO_TEST_ARTIFACT_DIR"); if !filepath.IsAbs(parent) || len(utf16.Encode([]rune(parent))) > 256 { t.Fatal("bounded owned artifacts required") }; if err := os.MkdirAll(parent, 0755); err != nil { t.Fatal(err) }
 owned := ""; for serial := 0; serial < 128; serial++ { candidate := filepath.Join(parent, fmt.Sprintf("rr-go-%d-%d", os.Getpid(), serial)); if len(utf16.Encode([]rune(candidate))) > 256 { t.Fatal("bounded fixture path required") }; err := os.Mkdir(candidate, 0755); if err == nil { owned = candidate; break }; if !os.IsExist(err) { t.Fatal(err) } }; if owned == "" { t.Fatal("finite fixture allocation exhausted") }; defer os.RemoveAll(owned); if err := os.MkdirAll(filepath.Join(owned, ".git"), 0755); err != nil { t.Fatal(err) }
 for _, name := range []string{"first/child", "second"} { directory := filepath.Join(owned, name); if len(utf16.Encode([]rune(directory))) > 256 { t.Fatal("bounded directory required") }; if err := os.MkdirAll(directory, 0755); err != nil { t.Fatal(err) } }
 file := filepath.Join(owned, "first/file.rs"); if len(utf16.Encode([]rune(file))) > 256 { t.Fatal("bounded file required") }; if err := os.WriteFile(file, []byte("pub fn fixture() {}\n"), 0644); err != nil { t.Fatal(err) }
 for _, row := range corpus.Cases { start := filepath.Join(owned, row.Cwd); var explicit *string; if row.Explicit != nil { value := *row.Explicit; if value == "{second}" { value = filepath.Join(owned, "second") }; if value == "{long}" { value = strings.Repeat("x", 300) }; explicit = &value }; selected, err := SelectRepoRoot(start, explicit); if row.Expected == nil { if err == nil { t.Errorf("%s accepted invalid root", row.Name) }; continue }; if err != nil { t.Fatalf("%s: %v", row.Name, err) }; expected := filepath.Join(owned, *row.Expected); if *row.Expected == "discovery" { expected = FindRepoRoot(start) }; expected, err = filepath.EvalSymlinks(expected); if err != nil { t.Fatal(err) }; if selected != expected { t.Errorf("%s: %q != %q", row.Name, selected, expected) }; after, err := os.Stat("."); if err != nil || !os.SameFile(cwd, after) { t.Fatal("cwd changed") } }
}
