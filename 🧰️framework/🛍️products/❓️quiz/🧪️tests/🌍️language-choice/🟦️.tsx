/** 🌍️ No default language: the browser's list preselects only among the offered languages (shared vectors, judged by
 * `i18next`'s own best-match negotiation as the third-party oracle), a list that names none leads to a chooser that
 * speaks every offered language at once and assumes nothing — no `lang` on the document, the title in every language —
 * and from the first paint on the document's `lang`, its title and every text follow the language that is known.
 *
 * @see ../../🧫️fixtures/🌍️language-choice/🔣️.json
 * @see https://www.w3.org/WAI/WCAG22/Understanding/language-of-page.html
 */

import { act, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import i18next from "i18next";
import { afterEach, describe, expect, it } from "vitest";
import { decodeQueryEnvelope, encodeQueryResult, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import type { CatalogView, Query } from "@semio-tech/quiz";
import { QUIZ_LOCALES, QuizApp, documentTitle, initialQuizState, localStore, memoryStorageOrigin, mountQuiz, preferredLocale, readPreferences, type PresenceConnect, type QuizState, type StorageArea } from "@semio-tech/quiz-react";
import choice from "../../🧫️fixtures/🌍️language-choice/🔣️.json";

type Locale = (typeof QUIZ_LOCALES)[number];

interface Fixture {
  readonly offered: readonly string[];
  readonly preselections: readonly { readonly browser: readonly string[]; readonly locale: string | null }[];
  readonly neutral: { readonly title: string; readonly choices: readonly { readonly locale: string; readonly name: string; readonly invitation: string }[] };
  readonly titles: readonly { readonly screen: string; readonly en: string; readonly de: string }[];
}

const fixture: Fixture = choice;
const text = (en: string, de: string) => ({ en, de });
const TENANT = "test-catalog";
const CATALOG: CatalogView = {
  id: TENANT,
  title: text("Test catalog", "Testkatalog"),
  introduction: { title: text("Welcome to the test catalog", "Willkommen im Testkatalog"), paragraphs: [text("Quizzes about everyday physics.", "Quizze über Alltagsphysik.")] },
  quizzes: [],
  badges: [],
};
const TIMING = { minMs: 1, maxMs: 4 };
const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });
const encoder = new TextEncoder();
const decoder = new TextDecoder();

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** 🛂️ A proctor that knows the catalog and nothing else; `held` keeps it silent until released. */
function catalogProctor(held?: Promise<void>): HttpTransport {
  return {
    send: async (request) => {
      await held;
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      if (request.path !== "/queries") return reply(404, { kind: "notFound", message: request.path });
      const query = JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query;
      if (query.type !== "catalog") return reply(404, { kind: "notFound", message: query.type });
      return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(CATALOG)), frontier: null }));
    },
  };
}

/** 🔮️ What `i18next` negotiates for a browser list among the offered languages, with no fallback language. */
function negotiated(browser: readonly string[], offered: readonly string[]): string | null {
  const instance = i18next.createInstance();
  void instance.init({ resources: {}, fallbackLng: false, supportedLngs: [...offered], nonExplicitSupportedLngs: true, cleanCode: true, initImmediate: false, showSupportNotice: false });
  const utils = instance.services.languageUtils as { readonly getBestMatchFromCodes: (codes: readonly string[]) => string | undefined; readonly getLanguagePartFromCode: (code: string) => string };
  const match = utils.getBestMatchFromCodes(browser);
  return match === undefined ? null : utils.getLanguagePartFromCode(match);
}

function app(storage: StorageArea, languages: readonly string[], transport = catalogProctor()) {
  return <QuizApp proctor="" tenant={TENANT} presence={QUIET_PRESENCE} transport={() => transport} storage={storage} languages={languages} timing={TIMING} />;
}

function stateAt(screen: string): QuizState {
  const base = initialQuizState({ introduced: screen !== "introduction" && screen !== "waiting", learner: undefined, catalog: screen === "waiting" ? undefined : CATALOG, learnerView: undefined, runs: {} });
  return screen === "waiting" ? { ...base, step: { screen: "introduction" } } : base;
}

afterEach(() => {
  document.documentElement.removeAttribute("lang");
  document.title = "";
});

describe("🌍️ language choice", () => {
  it("offers English first, then German", () => {
    expect([...QUIZ_LOCALES]).toEqual(fixture.offered);
    expect(fixture.neutral.choices.map((entry) => entry.locale)).toEqual(fixture.offered);
  });

  for (const vector of fixture.preselections) {
    it(`preselects ${vector.locale ?? "nothing"} for a browser that lists ${vector.browser.join(", ") || "nothing"}`, () => {
      expect(preferredLocale(vector.browser) ?? null).toBe(vector.locale);
      expect(negotiated(vector.browser, fixture.offered)).toBe(vector.locale);
    });
  }

  it("asks in every offered language and assumes none when the browser lists no offered language", async () => {
    const origin = memoryStorageOrigin();
    document.documentElement.lang = "en";
    render(app(origin.tab(), ["fr-FR", "it"]));
    await waitFor(() => expect(document.documentElement.hasAttribute("lang")).toBe(false));
    expect(document.title).toBe(fixture.neutral.title);
    const heading = screen.getByRole("heading", { level: 1 });
    expect(heading.textContent).toBe(fixture.neutral.title);
    expect([...heading.querySelectorAll("[lang]")].map((part) => [part.getAttribute("lang"), part.textContent])).toEqual(fixture.neutral.choices.map((entry) => [entry.locale, fixture.neutral.title.split(" · ")[fixture.offered.indexOf(entry.locale)]]));
    const buttons = screen.getAllByRole("button");
    expect(buttons.map((button) => [button.closest("[lang]")?.getAttribute("lang"), button.textContent])).toEqual(fixture.neutral.choices.map((entry) => [entry.locale, entry.name]));
    for (const entry of fixture.neutral.choices) expect(screen.getByText(entry.invitation).closest("[lang]")?.getAttribute("lang")).toBe(entry.locale);
    expect(screen.queryByRole("navigation")).toBeNull();
    expect(document.querySelector(".quiz-app")?.hasAttribute("lang")).toBe(false);
    expect(readPreferences(localStore(origin.tab(), TENANT)).locale).toBeUndefined();

    await userEvent.setup().click(screen.getByRole("button", { name: "Deutsch" }));
    await screen.findByRole("heading", { level: 1, name: "Willkommen im Testkatalog" });
    expect(document.documentElement.lang).toBe("de");
    expect(readPreferences(localStore(origin.tab(), TENANT)).locale).toBe("de");
    await waitFor(() => expect(document.title).toBe(fixture.titles.find((entry) => entry.screen === "introduction")!.de));
  });

  it("speaks the preselected language from the first paint and keeps the stored choice over the browser's list", async () => {
    const origin = memoryStorageOrigin();
    const root = document.body.appendChild(document.createElement("div"));
    let unmount: () => void = () => undefined;
    act(() => {
      unmount = mountQuiz(root, { proctor: "", tenant: TENANT, presence: QUIET_PRESENCE, transport: () => catalogProctor(), storage: origin.tab(), languages: ["de-CH", "fr"], timing: TIMING });
      expect(document.documentElement.lang).toBe("de");
    });
    await within(root).findByRole("heading", { level: 1, name: "Willkommen im Testkatalog" });
    await userEvent.setup().click(within(within(root).getByRole("navigation", { name: "Hauptnavigation" })).getByRole("button", { name: "English" }));
    expect(document.documentElement.lang).toBe("en");
    act(() => unmount());
    root.remove();

    const again = document.body.appendChild(document.createElement("div"));
    act(() => {
      unmount = mountQuiz(again, { proctor: "", tenant: TENANT, presence: QUIET_PRESENCE, transport: () => catalogProctor(), storage: origin.tab(), languages: ["de-CH", "fr"], timing: TIMING });
      expect(document.documentElement.lang).toBe("en");
    });
    await within(again).findByRole("heading", { level: 1, name: "Welcome to the test catalog" });
    act(() => unmount());
    again.remove();
  });

  for (const vector of fixture.titles) {
    it(`titles the document on the ${vector.screen} screen in the language that is spoken`, () => {
      for (const locale of QUIZ_LOCALES) expect(documentTitle(stateAt(vector.screen), locale)).toBe(vector[locale as Locale]);
    });
  }

  it("titles the document by its chooser while no language is known", () => {
    expect(documentTitle(stateAt("introduction"), undefined)).toBe(fixture.neutral.title);
  });

  it("follows the language and the screen with the document title", async () => {
    let release: () => void = () => undefined;
    const held = new Promise<void>((resolve) => {
      release = resolve;
    });
    render(app(memoryStorageOrigin().tab(), ["en-GB"], catalogProctor(held)));
    const title = (screen: string, locale: Locale): string => fixture.titles.find((entry) => entry.screen === screen)![locale];
    await waitFor(() => expect(document.title).toBe(title("waiting", "en")));
    await userEvent.setup().click(within(screen.getByRole("navigation", { name: "Main navigation" })).getByRole("button", { name: "Deutsch" }));
    await waitFor(() => expect(document.title).toBe(title("waiting", "de")));
    release();
    await waitFor(() => expect(document.title).toBe(title("introduction", "de")));
    await userEvent.setup().click(screen.getByRole("button", { name: "Weiter" }));
    await waitFor(() => expect(document.title).toBe(title("identity", "de")));
  });
});
