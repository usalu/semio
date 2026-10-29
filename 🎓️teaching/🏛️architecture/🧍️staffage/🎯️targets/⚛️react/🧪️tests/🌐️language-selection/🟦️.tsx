/** 🌐️ The app has no default language: a stored choice wins, then the browser's list, else it asks in every language; bundles match key for key.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { LANGUAGE_NAMES, OFFERED, inEveryLanguage, preferredLanguage, resolveLanguage, translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { STORAGE_KEY, STORAGE_VERSION, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { mountArchive } from "@teaching/staffage-react/app";

describe("resolution", () => {
  it("prefers the stored choice over the browser", () => {
    expect(resolveLanguage("de", ["en-US"])).toEqual({ language: "de", source: "stored" });
  });

  it("takes the first browser language the app speaks", () => {
    expect(preferredLanguage(["fr-CH", "de-AT", "en"])).toBe("de");
    expect(resolveLanguage(undefined, ["fr", "de-DE"])).toEqual({ language: "de", source: "browser" });
  });

  it("assumes nothing when neither matches", () => {
    expect(resolveLanguage(undefined, ["fr", "ja"])).toEqual({ language: undefined, source: "none" });
    expect(resolveLanguage("xx", [])).toEqual({ language: undefined, source: "none" });
  });

  it("offers English first, then German", () => {
    expect(OFFERED).toEqual(["en", "de"]);
    expect(inEveryLanguage("brand.title").map((entry) => entry.language)).toEqual(["en", "de"]);
  });
});

describe("bundles", () => {
  it("have the same keys, no empty text and the same placeholders", () => {
    expect(Object.keys(GERMAN).sort()).toEqual(Object.keys(ENGLISH).sort());
    for (const key of Object.keys(ENGLISH) as (keyof typeof ENGLISH)[]) {
      expect(GERMAN[key].trim(), key).not.toBe("");
      const holes = (text: string): string[] => [...text.matchAll(/\{(\w+)\}/gu)].map((match) => match[1]!).sort();
      expect(holes(GERMAN[key]), key).toEqual(holes(ENGLISH[key]));
    }
  });

  it("fills placeholders and leaves unknown ones visible", () => {
    expect(translator("en")("favorite.add", { title: "Bench" })).toBe("Add Bench to favorites");
    expect(translator("de")("filters.results", { shown: 3, total: 48 })).toBe("Angezeigt: 3 von 48");
    expect(translator("en")("favorite.add")).toContain("{title}");
  });
});

describe("the mounted app", () => {
  afterEach(() => {
    cleanup();
    document.body.innerHTML = "";
    document.documentElement.removeAttribute("lang");
  });
  const mount = (storage = memoryStorage(), languages: readonly string[] = []): HTMLElement => {
    const root = document.createElement("div");
    document.body.append(root);
    mountArchive(root, { storage, languages, catalog: { manifest: () => new Promise(() => undefined), category: async () => null } });
    return root;
  };

  it("asks in both languages when none is known, and sets no document language", async () => {
    const root = mount();
    await waitFor(() => expect(root.querySelector("main")).not.toBeNull());
    expect([...root.querySelectorAll("h1")].map((heading) => heading.lang)).toEqual(["en", "de"]);
    expect([...root.querySelectorAll("button")].map((button) => button.textContent)).toEqual([LANGUAGE_NAMES.en, LANGUAGE_NAMES.de]);
    expect(document.documentElement.hasAttribute("lang")).toBe(false);
  });

  it("remembers the choice and sets the document language", async () => {
    const storage = memoryStorage();
    const root = mount(storage);
    await waitFor(() => expect(root.querySelector("button")).not.toBeNull());
    fireEvent.click([...root.querySelectorAll("button")].find((button) => button.textContent === LANGUAGE_NAMES.de)!);
    await waitFor(() => expect(document.documentElement.lang).toBe("de"));
    expect(JSON.parse(storage.getItem(STORAGE_KEY)!)).toMatchObject({ version: STORAGE_VERSION, state: { preferences: { language: "de" } } });
  });

  it("starts in the stored language without asking", async () => {
    const storage = memoryStorage();
    storage.setItem(STORAGE_KEY, JSON.stringify({ version: STORAGE_VERSION, state: { preferences: { language: "de" } } }));
    const root = mount(storage, ["en"]);
    await waitFor(() => expect(document.documentElement.lang).toBe("de"));
    expect(root.querySelector('[role="radio"][aria-checked="true"]')?.textContent).toBe("DE");
  });

  it("preselects from the browser list without storing it", async () => {
    const storage = memoryStorage();
    mount(storage, ["de-CH"]);
    await waitFor(() => expect(document.documentElement.lang).toBe("de"));
    expect(storage.getItem(STORAGE_KEY)).toBeNull();
  });
});
