import { afterEach, describe, expect, it } from "vitest";
import { createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";
import {
  WGPU_HOST_STORAGE_KEYS,
  WGPU_HOST_STORAGE_KEY_PREFIXES,
  WGPU_HOST_STORAGE_SNAPSHOT_MAX_BYTES,
  WGPU_HOST_STORAGE_VALUE_MAX_BYTES,
  readWgpuHostStorageSnapshot,
  wgpuHostStorageCarriesKey,
} from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";

/** @emoji 🗄️ The PAGE half of packet W5a's preference door, tested where it runs. The Rust half's own
 * laws live in `🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs`; what only this suite can
 * prove is that the servicer really reaches the browser's two stores, really refuses a key outside the
 * census, and that the boot snapshot a page reads is the one the frame Worker is handed.
 *
 * 🩸️ Why the door exists: the wgpu shell lives in the frame Worker, which owns no `localStorage`, so
 * every browser `prefs_get`/`prefs_set` used to be a silent no-op. */
class FakeStorage implements Storage {
  private readonly entries = new Map<string, string>();

  get length(): number {
    return this.entries.size;
  }

  clear(): void {
    this.entries.clear();
  }

  getItem(key: string): string | null {
    return this.entries.get(key) ?? null;
  }

  key(index: number): string | null {
    return [...this.entries.keys()][index] ?? null;
  }

  removeItem(key: string): void {
    this.entries.delete(key);
  }

  setItem(key: string, value: string): void {
    this.entries.set(key, value);
  }
}

const SEEN_KEY = "ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor";
const OS_CONFIG = JSON.stringify({ version: 1, preferences: { "os.config.ui-preferences": JSON.stringify({ version: 1, events: [{ mutation: "setAppearance", appearance: "light" }] }) }, namedLayouts: {}, dockLayouts: { apps: {} }, dockUi: { apps: {} }, windowPanes: { apps: {} } });

type StorageHost = { localStorage?: Storage; sessionStorage?: Storage };

function installStores(): { readonly local: FakeStorage; readonly session: FakeStorage } {
  const local = new FakeStorage();
  const session = new FakeStorage();
  Object.defineProperty(globalThis, "localStorage", { configurable: true, value: local });
  Object.defineProperty(globalThis, "sessionStorage", { configurable: true, value: session });
  return { local, session };
}

async function call(request: unknown): Promise<{ value?: string | null; error?: string }> {
  return JSON.parse(await createWgpuPageHostIo()(JSON.stringify(request), null)) as { value?: string | null; error?: string };
}

afterEach(() => {
  Reflect.deleteProperty(globalThis as unknown as StorageHost, "localStorage");
  Reflect.deleteProperty(globalThis as unknown as StorageHost, "sessionStorage");
});

describe("the wgpu preference door's page half", () => {
  it("reads and writes the carried keys against the page's own localStorage", async () => {
    const { local } = installStores();
    local.setItem("semio.os.config", OS_CONFIG);

    expect(await call({ op: "storage", verb: "get", scope: "local", key: "semio.os.config" })).toEqual({ value: OS_CONFIG });
    expect(await call({ op: "storage", verb: "get", scope: "local", key: SEEN_KEY })).toEqual({ value: null });

    expect(await call({ op: "storage", verb: "set", scope: "local", key: SEEN_KEY, value: "true" })).toEqual({ value: null });
    expect(local.getItem(SEEN_KEY)).toBe("true");
    expect(await call({ op: "storage", verb: "get", scope: "local", key: SEEN_KEY })).toEqual({ value: "true" });

    expect(await call({ op: "storage", verb: "remove", scope: "local", key: SEEN_KEY })).toEqual({ value: null });
    expect(local.getItem(SEEN_KEY)).toBeNull();
  });

  it("addresses sessionStorage only when the call says so", async () => {
    const { local, session } = installStores();
    await call({ op: "storage", verb: "set", scope: "local", key: "ui.compute.workerCount", value: "8" });
    expect(local.getItem("ui.compute.workerCount")).toBe("8");
    expect(session.getItem("ui.compute.workerCount")).toBeNull();
  });

  it("refuses a key outside the census and a value outside the bound, loudly", async () => {
    const { local } = installStores();
    const uncarried = await call({ op: "storage", verb: "set", scope: "local", key: "ui.chrome.appearance", value: "dark" });
    expect(uncarried.error).toContain("not a carried key");
    expect(local.getItem("ui.chrome.appearance")).toBeNull();

    const oversized = await call({ op: "storage", verb: "set", scope: "local", key: "semio.os.config", value: "x".repeat(WGPU_HOST_STORAGE_VALUE_MAX_BYTES + 1) });
    expect(oversized.error).toContain("exceeds");
    expect(local.getItem("semio.os.config")).toBeNull();

    expect((await call({ op: "storage", verb: "get", scope: "local", key: "semio.os.config.evil" })).error).toContain("not a carried key");
  });

  it("answers a realm with no store with a refusal rather than an empty read", async () => {
    expect((await call({ op: "storage", verb: "get", scope: "local", key: "semio.os.config" })).error).toContain("owns no localStorage");
  });

  it("reads one boot snapshot carrying every census member and nothing else", () => {
    const { local } = installStores();
    local.setItem("semio.os.config", OS_CONFIG);
    local.setItem("ui.compute.workerCount", "6");
    local.setItem(SEEN_KEY, "true");
    local.setItem("ui.introduction.seen.other-app", "true");
    local.setItem("ui.chrome.appearance", "dark");
    local.setItem("semio.presence.client", "AAAA");

    const snapshot = readWgpuHostStorageSnapshot(globalThis);
    expect(Object.keys(snapshot)).toEqual(["semio.os.config", SEEN_KEY, "ui.introduction.seen.other-app", "ui.compute.workerCount"].sort());
    expect(snapshot["semio.os.config"]).toBe(OS_CONFIG);
    expect(snapshot[SEEN_KEY]).toBe("true");
    expect(snapshot["ui.chrome.appearance"]).toBeUndefined();
    expect(snapshot["semio.presence.client"]).toBeUndefined();
  });

  it("drops an oversized value and stops at the snapshot ceiling instead of growing with the store", () => {
    const { local } = installStores();
    local.setItem("semio.os.config", "x".repeat(WGPU_HOST_STORAGE_VALUE_MAX_BYTES + 1));
    local.setItem("ui.compute.workerCount", "4");
    expect(readWgpuHostStorageSnapshot(globalThis)).toEqual({ "ui.compute.workerCount": "4" });

    for (let index = 0; index < 4; index += 1) local.setItem(`ui.introduction.seen.app-${index}`, "y".repeat(WGPU_HOST_STORAGE_VALUE_MAX_BYTES));
    const bytes = Object.entries(readWgpuHostStorageSnapshot(globalThis)).reduce((total, [key, value]) => total + key.length + value.length, 0);
    expect(bytes).toBeLessThanOrEqual(WGPU_HOST_STORAGE_SNAPSHOT_MAX_BYTES);
  });

  it("reads nothing, and throws nothing, in a realm whose storage is unavailable", () => {
    expect(readWgpuHostStorageSnapshot({})).toEqual({});
    expect(
      readWgpuHostStorageSnapshot({
        get localStorage(): Storage {
          throw new Error("sandboxed");
        },
      }),
    ).toEqual({});
  });

  it("carries exactly the census, and never the bare family prefix", () => {
    for (const key of WGPU_HOST_STORAGE_KEYS) expect(wgpuHostStorageCarriesKey(key)).toBe(true);
    for (const prefix of WGPU_HOST_STORAGE_KEY_PREFIXES) {
      expect(wgpuHostStorageCarriesKey(prefix)).toBe(false);
      expect(wgpuHostStorageCarriesKey(`${prefix}an-app`)).toBe(true);
    }
    for (const key of ["ui.chrome.appearance", "ui.keybindings.overrides", "ui.themes.custom", "semio.presence.client", ""]) expect(wgpuHostStorageCarriesKey(key)).toBe(false);
  });
});
