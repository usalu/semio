/** 🛟️ Runs the production bootstrap with the DOM event and URL implementations provided by jsdom.
 * @see ./🥒️.feature */
import { JSDOM } from "jsdom";
import { expect, it, vi } from "vitest";
import { STALE_CHUNK_BOOT_SCRIPT, STALE_CHUNK_ENTRY_ID, STALE_CHUNK_MOUNTED_EVENT, STALE_CHUNK_RECOVERY_PATH_PREFIX, STALE_CHUNK_RELOAD_KEY } from "../../🛟️recovery/🟦️.ts";

const original = "https://staffage.example/gall%65ry?letter=%7e&label=a%20b&flag&query=one&query=two#p%20n";
const nonce = "28a19556-407a-4ca2-9b37-09d45839ad0b";
const firstPath = "/__staffage_recovery-1000000-" + nonce;

function remember(window: Pick<Window, "sessionStorage">, returnPath = "/gall%65ry") {
  window.sessionStorage.setItem(STALE_CHUNK_RELOAD_KEY, JSON.stringify({ started: 1_000_000, recoveryPath: firstPath, returnPath }));
}

function page(url = original, now = 1_000_000) {
  const dom = new JSDOM(`<script id="${STALE_CHUNK_ENTRY_ID}" type="module" src="/assets/old.js"></script><img id="image" src="/old.webp">`, { url, runScripts: "outside-only" });
  const { window } = dom;
  const replace = vi.fn();
  const host = {
    addEventListener: window.addEventListener.bind(window),
    get sessionStorage() { return window.sessionStorage; },
    crypto: { randomUUID: () => nonce },
    location: { get href() { return window.location.href; }, replace },
    history: window.history,
  };
  new Function("window", "Date", "URL", STALE_CHUNK_BOOT_SCRIPT)(host, { now: () => now }, window.URL);
  return {
    window,
    replace,
    fail(target: "lazy" | "entry" | "image") {
      const event = new window.Event(target === "lazy" ? "vite:preloadError" : "error", { cancelable: true });
      const element = target === "lazy" ? window : window.document.getElementById(target === "entry" ? STALE_CHUNK_ENTRY_ID : "image")!;
      element.dispatchEvent(event);
      return event.defaultPrevented;
    },
    mount: () => window.dispatchEvent(new window.Event(STALE_CHUNK_MOUNTED_EVENT)),
  };
}

it.each(["lazy", "entry"] as const)("replaces the page with a unique path after a missing %s", (target) => {
  const tab = page();
  expect(tab.fail(target)).toBe(true);
  const expected = original.replace("/gall%65ry", firstPath);
  expect(tab.replace).toHaveBeenCalledExactlyOnceWith(expected);
  const parsed = new tab.window.URL(expected);
  expect(parsed.pathname).toMatch(new RegExp("^" + STALE_CHUNK_RECOVERY_PATH_PREFIX));
  expect(parsed.searchParams.getAll("query")).toEqual(["one", "two"]);
  expect(parsed.hash).toBe("#p%20n");
  expect(JSON.parse(tab.window.sessionStorage.getItem(STALE_CHUNK_RELOAD_KEY)!)).toEqual({ started: 1_000_000, recoveryPath: firstPath, returnPath: "/gall%65ry" });
});

it("restores the original path after startup using the same history entry and state", () => {
  const tab = page(original.replace("/gall%65ry", firstPath), 1_020_000);
  remember(tab.window);
  tab.window.history.replaceState({ selection: "viewer" }, "");
  tab.mount();
  expect(tab.window.location.href).toBe(original);
  expect(tab.window.history.state).toEqual({ selection: "viewer" });
  expect(tab.window.history.length).toBe(1);
  expect(tab.replace).not.toHaveBeenCalled();
});

it("leaves other 404 paths, queries and assets alone", () => {
  const url = "https://staffage.example/missing?__staffage_reload=123#detail";
  const tab = page(url, 1_020_000);
  remember(tab.window);
  expect(tab.fail("image")).toBe(false);
  tab.mount();
  expect(tab.window.location.href).toBe(url);
  expect(tab.replace).not.toHaveBeenCalled();
});

it("does not restore an unmatched or invalid recovery path", () => {
  const url = "https://staffage.example" + firstPath;
  const noAttempt = page(url, 1_020_000);
  noAttempt.mount();
  expect(noAttempt.window.location.href).toBe(url);
  expect(noAttempt.fail("entry")).toBe(false);
  expect(noAttempt.replace).not.toHaveBeenCalled();
  const invalidTarget = page(url, 1_020_000);
  remember(invalidTarget.window, "https://example.org/");
  invalidTarget.mount();
  expect(invalidTarget.window.location.href).toBe(url);
});

it("shares the cooldown across entry and lazy failures and rotates the path when it expires", () => {
  const url = original.replace("/gall%65ry", firstPath);
  const blocked = page(url, 1_059_999);
  remember(blocked.window);
  expect(blocked.fail("entry")).toBe(false);
  expect(blocked.fail("lazy")).toBe(false);
  expect(blocked.replace).not.toHaveBeenCalled();

  const retry = page(url, 1_060_000);
  remember(retry.window);
  expect(retry.fail("lazy")).toBe(true);
  expect(retry.fail("entry")).toBe(false);
  expect(retry.replace).toHaveBeenCalledExactlyOnceWith(original.replace("/gall%65ry", "/__staffage_recovery-1060000-" + nonce));
  expect(JSON.parse(retry.window.sessionStorage.getItem(STALE_CHUNK_RELOAD_KEY)!).returnPath).toBe("/gall%65ry");
});

it("leaves failures alone when reading or writing session storage throws", () => {
  for (const method of ["getItem", "setItem"] as const) {
    const tab = page();
    vi.spyOn(tab.window.Storage.prototype, method).mockImplementation(() => { throw new Error("denied"); });
    expect(tab.fail("entry")).toBe(false);
    expect(tab.fail("lazy")).toBe(false);
    expect(tab.replace).not.toHaveBeenCalled();
    expect(tab.mount).not.toThrow();
  }
});
