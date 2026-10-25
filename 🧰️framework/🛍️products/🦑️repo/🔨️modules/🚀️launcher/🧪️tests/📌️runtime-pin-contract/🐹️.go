// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📌️runtime-pin-contract proves that the Go launcher and the TypeScript runtime guard read the one launcher registry the same way: the pinned archive of every host and the install that counts.

// #endregion 🧲️Header

package launcher

import (
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"testing"
)

const runtimePinFixturePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧫️fixtures/📌️runtime-pin/🔣️.json"

type runtimePinStore struct{ Store, Executable, Receipt string }

type runtimePinFixture struct {
	Registry Registry
	Select   []struct {
		ID, Platform, Arch string
		Musl               bool
		Archive            *string
	}
	Locate []struct {
		ID, Platform, Arch string
		Musl               bool
		Stores             []runtimePinStore
		Origin             *string
	}
	Cache []struct {
		ID, Platform string
		Environment  map[string]string
		Home         string
		Directory    *string
	}
}

func hostPlatformName() string {
	if runtime.GOOS == "windows" {
		return "win32"
	}
	return runtime.GOOS
}

func pinRequest(t *testing.T, fixture *runtimePinFixture, root, platform, arch string, musl bool, values map[string]string) ProvisionRequest {
	t.Helper()
	return ProvisionRequest{Root: root, Name: "bun", Registry: &fixture.Registry, Runtime: fixture.Registry.Runtimes["bun"], Platform: platform, Arch: arch, Musl: musl, Env: environment(values)}
}

// 📌️The launcher selects the archive and finds the verified install of the neutral runtime-pin fixture, as the TypeScript guard does.
func TestRuntimePinSelectionAndLocation(t *testing.T) {
	var fixture runtimePinFixture
	readFixture(t, runtimePinFixturePath, &fixture)
	for _, row := range fixture.Select {
		got, ok := (&Runtime{Distributions: fixture.Registry.Runtimes["bun"].Distributions}).Distribution(row.Platform, row.Arch, row.Musl)
		if row.Archive == nil && ok || row.Archive != nil && (!ok || got.Archive != *row.Archive) {
			t.Fatalf("%s: got %+v %v", row.ID, got, ok)
		}
	}
	checked := 0
	for _, row := range fixture.Locate {
		if row.Platform != hostPlatformName() {
			continue
		}
		checked++
		root := t.TempDir()
		request := pinRequest(t, &fixture, root, row.Platform, row.Arch, row.Musl, map[string]string{fixture.Registry.Cache.Environment: filepath.Join(root, "cache")})
		distribution, err := request.row()
		if err != nil {
			if row.Origin != nil {
				t.Fatalf("%s: %v", row.ID, err)
			}
			continue
		}
		tools, user, err := request.StoreDirectories(distribution)
		if err != nil {
			t.Fatal(err)
		}
		places := map[string]string{"tools": tools, "user": user}
		name := request.Runtime.ExecutableName()
		for _, store := range row.Stores {
			directory := places[store.Store]
			if err := os.MkdirAll(directory, 0o755); err != nil {
				t.Fatal(err)
			}
			if store.Executable != "absent" {
				mode := os.FileMode(0o644)
				if store.Executable == "present" {
					mode = 0o755
				}
				if err := os.WriteFile(filepath.Join(directory, name), []byte("12345"), mode); err != nil {
					t.Fatal(err)
				}
			}
			if store.Receipt != "absent" {
				receipt := map[string]any{"archive": "0000000000000000000000000000000000000000000000000000000000000000"}
				if store.Receipt == "ok" || store.Receipt == "size-mismatch" || store.Receipt == "ok-sized" {
					receipt["archive"] = distribution.Sha256
				}
				if store.Receipt == "size-mismatch" {
					receipt["bytes"] = 99
				}
				if store.Receipt == "ok-sized" {
					receipt["bytes"] = 5
				}
				raw, _ := json.Marshal(receipt)
				if err := os.WriteFile(filepath.Join(directory, receiptName), raw, 0o644); err != nil {
					t.Fatal(err)
				}
			}
		}
		install, found, err := request.Locate()
		if err != nil {
			t.Fatalf("%s: %v", row.ID, err)
		}
		if row.Origin == nil && found || row.Origin != nil && (!found || install.Origin != *row.Origin) {
			t.Fatalf("%s: got %+v %v want %v", row.ID, install, found, row.Origin)
		}
	}
	if checked == 0 {
		t.Skipf("no fixture row for %s", hostPlatformName())
	}
}

// 📌️The user cache is the operating system's cache directory unless the registry's cache variable names one.
func TestRuntimePinUserCacheDirectory(t *testing.T) {
	var fixture runtimePinFixture
	readFixture(t, runtimePinFixturePath, &fixture)
	for _, row := range fixture.Cache {
		if row.Platform != hostPlatformName() {
			continue
		}
		t.Setenv("HOME", row.Home)
		t.Setenv("XDG_CACHE_HOME", row.Environment["XDG_CACHE_HOME"])
		t.Setenv("LocalAppData", row.Environment["LocalAppData"])
		distribution := fixture.Registry.Runtimes["bun"].Distributions[0]
		for _, candidate := range fixture.Registry.Runtimes["bun"].Distributions {
			if candidate.Platform == row.Platform {
				distribution = candidate
			}
		}
		_, user, err := pinRequest(t, &fixture, t.TempDir(), row.Platform, distribution.Arch, false, nil).StoreDirectories(distribution)
		if row.Directory == nil {
			if err == nil {
				t.Fatalf("%s: got %q, want a refusal", row.ID, user)
			}
			continue
		}
		want := filepath.Join(append(append([]string{*row.Directory}, fixture.Registry.Cache.User...), "bun", distribution.Sha256)...)
		if err != nil || user != want {
			t.Fatalf("%s: got %q %v want %q", row.ID, user, err, want)
		}
	}
}
