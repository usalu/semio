/** 🗣️ Translation completeness: every chrome key exists in English and German with non-empty labels and the same
 * placeholders, resolves through the shared i18n port in both locales, covers every rejection and task kind, and is
 * both used by the client and the only kind of key the client uses.
 */

import { describe, expect, it } from "vitest";
import { REJECTIONS, TASK_KINDS } from "@semio-tech/quiz";
import { resolveUiLabel, uiI18n } from "@semio-tech/ui-react/i18n";
import { QUIZ_BUNDLE_DE, QUIZ_BUNDLE_EN, QUIZ_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, preferredLocale, quizText, type QuizLabelKey } from "@semio-tech/quiz-react";

type Leaf = { readonly label: { readonly normal: string; readonly beginner: string } };

function leaves(tree: unknown, prefix = ""): Map<string, Leaf> {
  const found = new Map<string, Leaf>();
  for (const [key, value] of Object.entries(tree as Record<string, unknown>)) {
    const path = prefix === "" ? key : `${prefix}.${key}`;
    const label = typeof value === "object" && value !== null ? (value as { readonly label?: unknown }).label : undefined;
    if (typeof label === "object" && label !== null && typeof (label as { readonly normal?: unknown }).normal === "string") found.set(path, value as Leaf);
    else for (const [nested, leaf] of leaves(value, path)) found.set(nested, leaf);
  }
  return found;
}

function placeholders(label: string): string[] {
  return [...label.matchAll(/\{\{(\w+)\}\}/gu)].map((match) => match[1] ?? "").sort();
}

const SOURCES = import.meta.glob<string>(["../../🎯️targets/⚛️react/🟦️.tsx", "../../🎯️targets/⚛️react/🔨️modules/**/🟦️.ts", "../../🎯️targets/⚛️react/🔨️modules/**/🟦️.tsx"], { query: "?raw", import: "default", eager: true });
const english = leaves(QUIZ_BUNDLE_EN);
const german = leaves(QUIZ_BUNDLE_DE);

describe("🗣️ translation completeness", () => {
  it("has the same keys in English and German", () => {
    expect([...german.keys()]).toEqual([...english.keys()]);
    expect(english.size).toBeGreaterThan(150);
  });

  it("has non-empty labels with the same placeholders in both languages", () => {
    for (const [key, leaf] of english) {
      const other = german.get(key)!;
      for (const label of [leaf.label.normal, leaf.label.beginner, other.label.normal, other.label.beginner]) expect(label.trim(), key).not.toBe("");
      expect(placeholders(other.label.normal), key).toEqual(placeholders(leaf.label.normal));
      expect(placeholders(other.label.beginner), key).toEqual(placeholders(leaf.label.beginner));
    }
  });

  it("resolves every key through the shared i18n port in both locales", () => {
    for (const locale of QUIZ_LOCALES) {
      const text = quizText(locale);
      const bundle = locale === "en" ? english : german;
      for (const [key, leaf] of bundle) {
        const values = Object.fromEntries(placeholders(leaf.label.normal).map((name) => [name, `‹${name}›`]));
        const resolved = text(key as QuizLabelKey, values);
        expect(resolved, key).toBe(leaf.label.normal.replace(/\{\{(\w+)\}\}/gu, (_, name: string) => `‹${name}›`));
      }
    }
  });

  it("explains every proctor rejection and names every task kind", () => {
    expect(Object.keys(REJECTION_LABELS).sort()).toEqual([...REJECTIONS].sort());
    expect(Object.keys(TASK_KIND_LABELS).sort()).toEqual([...TASK_KINDS].sort());
    for (const key of [...Object.values(REJECTION_LABELS), ...Object.values(TASK_KIND_LABELS)]) expect(english.has(key), key).toBe(true);
  });

  it("uses exactly the registered keys in the client sources", () => {
    expect(Object.keys(SOURCES).length).toBeGreaterThan(15);
    const used = new Set(Object.values(SOURCES).flatMap((source) => [...source.matchAll(/"(quiz\.[a-zA-Z]+\.[a-zA-Z]+)"/gu)].map((match) => match[1] ?? "")));
    expect([...used].filter((key) => !english.has(key))).toEqual([]);
    expect([...english.keys()].filter((key) => !used.has(key))).toEqual([]);
  });

  it("addresses learners informally in German, like the catalog content", () => {
    for (const [key, leaf] of german) expect(`${leaf.label.normal} ${leaf.label.beginner}`, key).not.toMatch(/\b(?:Sie|Ihr|Ihre|Ihren|Ihrem|Ihrer|Ihres|Ihnen)\b/u);
    expect(german.get("quiz.identity.title")?.label.normal).toBe("Wie möchtest du erscheinen?");
    expect(german.get("quiz.results.yourAnswer")?.label.normal).toBe("Deine Antwort");
  });

  it("reaches the design system through its slim `@semio-tech/ui-react/i18n` and `/chrome` subpaths only, never the barrel", () => {
    const imports = Object.values(SOURCES).flatMap((source) => [...source.matchAll(/from "(@semio-tech\/ui-react[^"]*)"/gu)].map((match) => match[1]));
    expect(new Set(imports)).toEqual(new Set(["@semio-tech/ui-react/i18n", "@semio-tech/ui-react/chrome"]));
    expect(resolveUiLabel({ label: { normal: "N", beginner: "B" } }, "normal")).toBe("N");
    expect(resolveUiLabel({ label: { normal: "N", beginner: "B" } }, "beginner")).toBe("B");
    expect(resolveUiLabel({ label: { beginner: "B" } }, "normal")).toBe("B");
    expect(resolveUiLabel({ label: "plain" }, "normal")).toBe("plain");
    expect(resolveUiLabel({ label: {}, normal: "outer" }, "beginner")).toBe("outer");
    expect(resolveUiLabel("text", "normal")).toBe("text");
    expect(resolveUiLabel(42, "normal")).toBeUndefined();
    expect(resolveUiLabel(uiI18n.t("ui.nav.back" as never, { lng: "de" }), "normal")).toBe("Zurück");
  });

  it("preselects the first offered language of the browser's list and none when the list names no offered one", () => {
    expect(preferredLocale(["de-CH", "en-US"])).toBe("de");
    expect(preferredLocale(["fr-FR", "en-GB", "de"])).toBe("en");
    expect(preferredLocale(["fr-FR", "it"])).toBeUndefined();
    expect(preferredLocale([])).toBeUndefined();
  });

  it("writes every count as a labelled number, so no sentence needs a plural form", () => {
    for (const bundle of [english, german])
      for (const [key, leaf] of bundle) {
        expect(leaf.label.normal, key).not.toMatch(/\{\{\w+\}\} (?:points?|answers?|learners?|runs?|Punkte?|Antwort(?:en)?|Lernende|Durchg[aä]nge?)\b/u);
        expect(leaf.label.normal, key).not.toMatch(/\{\{detail\}\}/u);
      }
    expect(english.get("quiz.home.points")?.label.normal).toBe("Points: {{points}}");
    expect(german.get("quiz.connection.reconnectingWaiting")?.label.normal).toBe("Verbindung unterbrochen – neuer Versuch läuft. Auf diesem Gerät gesicherte Antworten: {{waiting}}");
  });

  it("keeps the list role on every list whose markers the styles remove, and hides no status glyph behind a word-less name", () => {
    const lists = Object.entries(SOURCES).flatMap(([path, source]) => [...source.matchAll(/<(?:ul|ol)\b[^>]*>/gu)].map((tag) => [path, tag[0]] as const));
    expect(lists.length).toBeGreaterThan(10);
    for (const [path, tag] of lists) if (/list-none|className=\{chips\}/u.test(tag)) expect(tag, path).toContain('role="list"');
    for (const [path, source] of Object.entries(SOURCES)) expect(source, path).not.toMatch(/`[✓○◐✗] \$\{/u);
  });

  it("makes every box that scrolls the containing block of what it scrolls, so text hidden for assistive technology never widens the page", () => {
    const scrollers = Object.entries(SOURCES).flatMap(([path, source]) => [...source.matchAll(/"[^"\n]*\boverflow(?:-[xy])?-auto\b[^"\n]*"/gu)].map((classes) => [path, classes[0]] as const));
    expect(scrollers.length).toBeGreaterThan(6);
    for (const [path, classes] of scrollers) expect(classes, path).toMatch(/\b(?:relative|absolute|fixed)\b/u);
  });

  it("uses one German word per concept", () => {
    const texts = [...german.values()].map((leaf) => leaf.label.normal).join("\n");
    for (const banned of [/Bestenliste/u, /Klassifikation/u, /zugeordnet zu/u, /gereiht/u, /Anrechnung/u, /\bLäuft\b/u, /\bBegonnen\b/u, /zieht und mischt/u]) expect(texts, String(banned)).not.toMatch(banned);
    expect(german.get("quiz.leaderboard.title")?.label.normal).toBe("Rangliste");
    expect(german.get("quiz.leaderboard.learner")?.label.normal).toBe("Lernende");
    expect(german.get("quiz.home.open")?.label.normal).toBe(german.get("quiz.learner.statusOpen")?.label.normal);
  });
});
