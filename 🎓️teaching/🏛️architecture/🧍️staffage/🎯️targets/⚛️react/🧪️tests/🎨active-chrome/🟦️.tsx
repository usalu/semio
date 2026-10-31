/** 🎨 The current page uses the primary color, and a pressed toggle uses the active background.
 * @see ./🥒️.feature — the scenarios
 */
import { cleanup, fireEvent, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { CATEGORIES, type Manifest } from "@teaching/staffage";
import { mountArchive } from "@teaching/staffage-react/app";
import { STORAGE_KEY, STORAGE_VERSION, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";

const text = (value: string) => ({ en: value, de: value });
const manifest = { $schema: "", version: "semio.staffage/v2", categories: CATEGORIES.map((id) => ({ id, emoji: "x", title: text(id), description: text(id), folder: id })), collections: [], entries: [] } as unknown as Manifest;

function tokens(element: Element): string[] {
  return element.className.toString().split(/\s+/u).filter((token) => token !== "");
}

function mount(): HTMLElement {
  const storage = memoryStorage();
  storage.setItem(STORAGE_KEY, JSON.stringify({ version: STORAGE_VERSION, state: { preferences: { language: "en" } } }));
  const root = document.createElement("div");
  document.body.append(root);
  mountArchive(root, { storage, languages: ["en"], catalog: { manifest: async () => manifest, category: async () => null } });
  return root;
}

describe("🎨 active chrome", () => {
  afterEach(() => {
    cleanup();
    document.body.innerHTML = "";
    window.location.hash = "#/";
  });

  it("paints the current page in the primary color", async () => {
    const root = mount();
    await waitFor(() => expect(document.documentElement.lang).toBe("en"));
    const home = root.querySelector('a[href="#/"]')!;
    expect(home.getAttribute("aria-current")).toBe("page");
    expect(tokens(home)).toContain("text-primary");
    expect(tokens(home)).not.toContain("underline");
    window.location.hash = "#/all";
    await waitFor(() => expect(root.querySelector('a[href="#/all"]')?.getAttribute("aria-current")).toBe("page"));
    const all = root.querySelector('a[href="#/all"]')!;
    expect(tokens(all)).toContain("text-primary");
    expect(tokens(all)).not.toContain("underline");
    expect(home.hasAttribute("aria-current")).toBe(false);
  });

  it("paints a pressed toggle and every selected segment with the active background", async () => {
    window.location.hash = "#/all";
    const root = mount();
    await waitFor(() => expect([...root.querySelectorAll("button")].some((button) => button.textContent?.includes("Ascending"))).toBe(true));
    const selected = [...root.querySelectorAll('[role="radio"][aria-checked="true"]')];
    expect(selected.length).toBeGreaterThan(0);
    for (const radio of selected) {
      expect(tokens(radio)).toContain("bg-active-base");
      expect(tokens(radio)).not.toContain("bg-foreground");
      expect(tokens(radio)).not.toContain("bg-emphasized");
    }
    const direction = [...root.querySelectorAll("button")].find((button) => button.textContent?.includes("Ascending"))!;
    fireEvent.click(direction);
    expect(direction.getAttribute("aria-pressed")).toBe("true");
    expect(tokens(direction)).toContain("bg-active-base");
    expect(tokens(direction)).not.toContain("bg-foreground");
    expect(tokens(direction)).not.toContain("bg-emphasized");
  });
});
