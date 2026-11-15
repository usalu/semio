// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️fleet-provision proves that a second machine takes a pinned runtime archive from the group store and downloads nothing, and that a corrupt store file or a missing store leaves the launch on the upstream.

// #endregion 🧲️Header

package launcher

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
	"os"
	"path/filepath"
	"testing"
)

func fleetRequest(t *testing.T, archive []byte, fake *synthetic, store string) ProvisionRequest {
	t.Helper()
	registry, rt, _ := syntheticRegistry(t, archive, "bun-linux-x64")
	values := map[string]string{"SEMIO_RUNTIME_CACHE": t.TempDir(), "SEMIO_RUNTIME_MIRROR": fake.server.URL}
	if store != "" {
		values["SEMIO_FLEET_STORE_ROOT"] = store
	}
	return ProvisionRequest{Root: t.TempDir(), Name: "bun", Registry: registry, Runtime: rt, Platform: "linux", Arch: "x64", Env: environment(values)}
}

// 🛰️The first machine downloads and publishes by SHA-256; the second installs from the store with no request to the upstream; the stored bytes equal an independent SHA-256.
func TestFleetStoreServesTheSecondMachine(t *testing.T) {
	skipWithoutShell(t)
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	fake, store := serveArchive(t, archive, http.StatusOK, nil), t.TempDir()
	first := fleetRequest(t, archive, fake, store)
	if _, err := first.Ensure(context.Background()); err != nil {
		t.Fatal(err)
	}
	if fake.requests.Load() != 1 {
		t.Fatalf("first machine: expected one download, got %d", fake.requests.Load())
	}
	sum := sha256.Sum256(archive)
	stored, err := os.ReadFile(fleetBlobPath(store, hex.EncodeToString(sum[:])))
	if err != nil || hex.EncodeToString(func() []byte { s := sha256.Sum256(stored); return s[:] }()) != hex.EncodeToString(sum[:]) {
		t.Fatalf("store blob: %v", err)
	}
	second := fleetRequest(t, archive, fake, store)
	install, err := second.Ensure(context.Background())
	if err != nil || install.Executable == "" {
		t.Fatalf("%+v %v", install, err)
	}
	if fake.requests.Load() != 1 {
		t.Fatalf("second machine: expected no further download, got %d", fake.requests.Load())
	}
}

// 🛰️A corrupt store file is refused and the upstream answers; a launch without a store behaves as before.
func TestFleetStoreFallsBackToTheUpstream(t *testing.T) {
	skipWithoutShell(t)
	archive := zipOf(t, "bun-linux-x64/bun", syntheticRuntime(syntheticVersion))
	fake, store := serveArchive(t, archive, http.StatusOK, nil), t.TempDir()
	sum := sha256.Sum256(archive)
	blob := fleetBlobPath(store, hex.EncodeToString(sum[:]))
	if err := os.MkdirAll(filepath.Dir(blob), 0o755); err != nil {
		t.Fatal(err)
	}
	damaged := append([]byte{}, archive...)
	damaged[len(damaged)/2] ^= 0xff
	if err := os.WriteFile(blob, damaged, 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := fleetRequest(t, archive, fake, store).Ensure(context.Background()); err != nil || fake.requests.Load() != 1 {
		t.Fatalf("corrupt store: %v requests=%d", err, fake.requests.Load())
	}
	if _, err := fleetRequest(t, archive, fake, "").Ensure(context.Background()); err != nil || fake.requests.Load() != 2 {
		t.Fatalf("no store: %v requests=%d", err, fake.requests.Load())
	}
	if fleetStoreDirectory(t.TempDir(), environment(map[string]string{"SEMIO_FLEET_FETCH": "off", "SEMIO_FLEET_STORE_ROOT": store})) != "" {
		t.Fatal("SEMIO_FLEET_FETCH=off must disable the tier")
	}
}
