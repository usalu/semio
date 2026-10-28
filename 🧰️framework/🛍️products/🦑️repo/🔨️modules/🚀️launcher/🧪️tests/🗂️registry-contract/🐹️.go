// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗂️registry-contract proves the runtime registry: the selection fixture, the strict decoding and the bilingual completeness of its texts.

// #endregion 🧲️Header

package launcher

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 🗂️The registry selects the pinned archive of the neutral selection fixture for every host.
func TestRegistrySelectionFixture(t *testing.T) {
	var fixture struct {
		Selection []struct {
			Platform, Arch string
			Musl           bool
			Archive        *string
		}
	}
	readFixture(t, launcherFixture, &fixture)
	runtimeRow := loadRegistry(t).Runtimes["bun"]
	for _, row := range fixture.Selection {
		got, ok := runtimeRow.Distribution(row.Platform, row.Arch, row.Musl)
		if row.Archive == nil && ok || row.Archive != nil && (!ok || got.Archive != *row.Archive) {
			t.Fatalf("%+v: got %+v %v", row, got, ok)
		}
	}
}

// 🗂️The pinned runtime starts with exactly the declared launch environment, which disables Bun's content-keyed transpiler cache.
func TestRegistryLaunchEnvironment(t *testing.T) {
	var fixture struct {
		CacheIsolation struct {
			LaunchEnvironment map[string]string
		}
	}
	readFixture(t, launcherFixture, &fixture)
	got := loadRegistry(t).Runtimes["bun"].Launch.Environment
	if len(got) != len(fixture.CacheIsolation.LaunchEnvironment) {
		t.Fatalf("got %v want %v", got, fixture.CacheIsolation.LaunchEnvironment)
	}
	for name, want := range fixture.CacheIsolation.LaunchEnvironment {
		if got[name] != want {
			t.Fatalf("%s: got %q want %q", name, got[name], want)
		}
	}
}

// 🗂️Every required text exists in every language with the same placeholders; the registry decodes strictly.
func TestRegistryStrictness(t *testing.T) {
	root := testRoot(t)
	raw, err := os.ReadFile(filepath.Join(root, RegistryPath))
	if err != nil {
		t.Fatal(err)
	}
	if _, err := DecodeRegistry("registry", raw); err != nil {
		t.Fatal(err)
	}
	var generic map[string]any
	_ = json.Unmarshal(raw, &generic)
	mutate := func(change func(map[string]any)) error {
		copied := map[string]any{}
		_ = json.Unmarshal(raw, &copied)
		change(copied)
		bytes, _ := json.Marshal(copied)
		_, failure := DecodeRegistry("registry", bytes)
		return failure
	}
	for name, change := range map[string]func(map[string]any){
		"unknown top-level field": func(r map[string]any) { r["extra"] = 1 },
		"future contract":         func(r map[string]any) { r["contract"] = 2 },
		"malformed digest": func(r map[string]any) {
			r["runtimes"].(map[string]any)["bun"].(map[string]any)["distributions"].([]any)[0].(map[string]any)["sha256"] = "XYZ"
		},
		"plain http source": func(r map[string]any) {
			r["runtimes"].(map[string]any)["bun"].(map[string]any)["download"].(map[string]any)["base"] = "http://example.com/x"
		},
		"missing message":                func(r map[string]any) { delete(r["messages"].(map[string]any), "ready") },
		"placeholder lost in a language": func(r map[string]any) { r["messages"].(map[string]any)["ready"].(map[string]any)["de"] = "bereit" },
		"selected runtime undeclared":    func(r map[string]any) { r["selected"] = "deno" },
	} {
		if mutate(change) == nil {
			t.Fatalf("%s must be refused", name)
		}
	}
	var contract RegistryError
	if err := mutate(func(r map[string]any) { r["contract"] = 2 }); err == nil || !strings.Contains(err.Error(), "contract") {
		t.Fatalf("contract refusal: %v", err)
	} else {
		contract = err.(RegistryError)
		if contract.Found != 2 {
			t.Fatalf("found %d", contract.Found)
		}
	}
}

// 🗂️The registry pins exactly the distributions of the existing Bun release record, so the two cannot drift apart.
func TestRegistryAgreesWithBunReleaseRecord(t *testing.T) {
	var schema struct {
		Definitions struct {
			Release struct {
				Const struct {
					Version       string
					Release       string
					Distributions []struct {
						Platform, Arch, Archive, Sha256 string
						Musl                            bool
						Bytes                           int64
					}
				}
			}
		}
	}
	readFixture(t, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json", &schema)
	pinned := schema.Definitions.Release.Const
	bun := loadRegistry(t).Runtimes["bun"]
	if bun.Version != pinned.Version || bun.Download.Base != pinned.Release || len(bun.Distributions) != len(pinned.Distributions) {
		t.Fatalf("release record differs: %s %s %d", pinned.Version, pinned.Release, len(pinned.Distributions))
	}
	for _, row := range pinned.Distributions {
		got, ok := bun.Distribution(row.Platform, row.Arch, row.Musl)
		if !ok || got.Archive != row.Archive || got.Sha256 != row.Sha256 || got.Bytes != row.Bytes {
			t.Fatalf("%+v differs from %+v", row, got)
		}
	}
}

// 🗂️Every text the launcher can print exists in every language with equal placeholders: the registry texts that failures name, and the built-in texts for the moments without a registry.
func TestEveryPrintableTextIsBilingual(t *testing.T) {
	registry := loadRegistry(t)
	for _, id := range []string{"runtimeUnsupported", "cacheUnavailable", "pathTooLong", "mirrorRefused", "redirectRefused", "downloadFailed", "integrityFailed", "extractFailed", "extractTooLarge", "probeFailed", "cancelled", "waitingForPeer", "provisioning", "downloading", "handOffFailed", "entryUnknown", "entryMissing", "runtimeUnknown", "unexpected"} {
		for _, language := range registry.Languages {
			if registry.Messages[id][language] == "" {
				t.Fatalf("%s lacks %s", id, language)
			}
		}
	}
	for id, text := range Builtin {
		if text["en"] == "" || text["de"] == "" || placeholderSet(text["en"]) != placeholderSet(text["de"]) {
			t.Fatalf("built-in text %s is not bilingual", id)
		}
	}
	for _, kind := range []string{"unreadable", "malformed", "contract", "incomplete", "selected", "runtime", "distribution", "entry", "message"} {
		if Builtin["registry."+kind] == nil {
			t.Fatalf("registry problem %s has no text", kind)
		}
	}
	for _, id := range []string{"rootUnreadable", "rootMalformed", "noRepository", "languageProtocol", "unexpected"} {
		if Builtin[id] == nil {
			t.Fatalf("%s has no built-in text", id)
		}
	}
}

func withoutProse(value any) any {
	switch typed := value.(type) {
	case map[string]any:
		kept := map[string]any{}
		for key, member := range typed {
			if key != "description" && key != "$comment" {
				kept[key] = withoutProse(member)
			}
		}
		return kept
	case []any:
		kept := make([]any, len(typed))
		for index, member := range typed {
			kept[index] = withoutProse(member)
		}
		return kept
	}
	return value
}

// 🗂️The shape of the registry belongs to its contract: the digest of the registry schema (prose left out) is pinned next to the contract it was published with, so a change of the shape that keeps the contract fails here, and a launcher built for the old shape never meets a registry of a shape it does not know under its own contract number.
func TestRegistryShapeChangesBumpTheContract(t *testing.T) {
	var fixture struct {
		RegistryShape struct {
			Contract     int
			SchemaDigest string
		}
	}
	readFixture(t, launcherFixture, &fixture)
	var schema any
	readFixture(t, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🧬️schema/🔣️.json", &schema)
	var canonical bytes.Buffer
	encoder := json.NewEncoder(&canonical)
	encoder.SetEscapeHTML(false)
	if err := encoder.Encode(withoutProse(schema)); err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256(bytes.TrimSuffix(canonical.Bytes(), []byte("\n")))
	if fixture.RegistryShape.Contract != Contract || hex.EncodeToString(sum[:]) != fixture.RegistryShape.SchemaDigest {
		t.Fatalf("the registry schema changed (digest %s) while the contract is still %d: raise Contract in the launcher, the registry and its schema, and pin the new digest with the new contract in the launcher fixture; otherwise every installed launcher refuses the registry as malformed", hex.EncodeToString(sum[:]), Contract)
	}
}

// 🗂️A launcher built before a registry change meets a registry of a later contract with a shape it does not know (the old-binary fixture is a registry of the next contract with a block this build has never seen): it refuses by contract, never as a malformed field, and prints the exact reinstall command in both languages.
func TestRegistryOfALaterContractIsRefusedWithTheInstallCommand(t *testing.T) {
	root := testRoot(t)
	raw, err := os.ReadFile(filepath.Join(root, RegistryPath))
	if err != nil {
		t.Fatal(err)
	}
	later := map[string]any{}
	_ = json.Unmarshal(raw, &later)
	later["contract"] = Contract + 1
	later["blockOfALaterContract"] = map[string]any{"anything": []any{1, "two"}}
	delete(later, "restart")
	bytes, _ := json.Marshal(later)
	path := filepath.Join(root, RegistryPath)
	_, failure := DecodeRegistry(path, bytes)
	var problem RegistryError
	if !errors.As(failure, &problem) || problem.Kind != "contract" || problem.Found != Contract+1 {
		t.Fatalf("a later contract must be refused as such, got %v", failure)
	}
	host, log := recordingHost(t, map[string]string{})
	if code := host.ReportRegistry(failure); code != 2 {
		t.Fatalf("exit code %d", code)
	}
	text := strings.Join(log.err, "\n")
	command := InstallCommand(path)
	for _, want := range []string{"Rebuild and reinstall the launcher with: " + command, "Baue den Launcher neu und installiere ihn mit: " + command} {
		if !strings.Contains(text, want) {
			t.Fatalf("missing %q in %q", want, text)
		}
	}
	if !strings.Contains(command, "📥️install") {
		t.Fatalf("the install command must name the install script of the launcher module: %s", command)
	}
}

// #endregion 🗂️Registry
