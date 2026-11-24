/** 🎞️ Assignments and learning clips are two more general cards of the home overview: the pages they add and their labels (shared
 * vectors), no list of assignments above the overview, the assignments page with a card per exercise, the PDF of the learner's language
 * inside an assignment's page once it is opened — with a download, a tab of its own and the way back —, and the clips page
 * with a poster per clip and either the link to the hosted clip or that it is coming soon. The links and the document
 * addresses are read back with `URL`, the platform's own parser, as the independent oracle.
 *
 * @see ../../🧫️fixtures/🎞️assignment-clip-pages/🔣️.json
 * @see ./🥒️.feature */
import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { Assignment, Clip, CourseView } from "@semio-tech/course";
import { ClipPostersProvider, HomeScreen, addressPages, containerKey, courseText, homePages, isHomeGrid, pageLabel, pagesWithin, parentPage, type CourseLocale, type CourseSession } from "@semio-tech/course-react";
import vectors from "../../🧫️fixtures/🎞️assignment-clip-pages/🔣️.json";
import { COURSE, Home, PREFERENCES, STATE, cardHost, pane, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";

const assignments = vectors.assignments as unknown as readonly Assignment[];
const clips = vectors.clips as unknown as readonly Clip[];
const course: CourseView = { ...COURSE, topics: [{ id: "sample-topic", emoji: "📐", title: { en: "Sample Topic", de: "Beispielthema" }, quizzes: ["physics", "heating", "cooling", "demand"] }], assignments, clips };

afterEach(cleanup);

function rendered(page: string | undefined, locale: CourseLocale = "en", session = stubSession()) {
  const view = render(
    <ClipPostersProvider posters={vectors.posters}>
      <HomeScreen session={session as unknown as CourseSession} state={{ ...STATE, step: page === undefined ? { screen: "home" } : { screen: "home", page }, course }} text={courseText(locale)} locale={locale} preferences={PREFERENCES} onPreferences={() => undefined} />
    </ClipPostersProvider>,
  );
  return { session, ...view };
}

describe("🎞️ assignments and learning clips on the home overview", () => {
  it("adds the two pages to the general pages of a course that has them, in the shared order, and none to one without", () => {
    expect(homePages(course)).toEqual(vectors.pages.withBoth);
    expect(homePages({ assignments })).toEqual(vectors.pages.withAssignmentsOnly);
    expect(homePages({ clips })).toEqual(vectors.pages.withClipsOnly);
    expect(homePages({})).toEqual(vectors.pages.withNeither);
    expect(homePages()).toEqual(vectors.pages.withNeither);
    expect(homePages({ assignments: [], clips: [] })).toEqual(vectors.pages.withNeither);
  });

  it("derives the entries, parents, addresses and labels of both pages from the course", () => {
    for (const [page, ids] of Object.entries(vectors.pages.within)) expect(pagesWithin(page, course), page).toEqual(ids);
    for (const [page, parent] of Object.entries(vectors.pages.parents)) expect(parentPage(page, course), page).toBe(parent ?? undefined);
    for (const page of ["assignments", "clips"]) expect(isHomeGrid(page, course), page).toBe(true);
    expect(addressPages(course)).toEqual(expect.arrayContaining([...vectors.pages.withBoth, ...vectors.pages.within.assignments, ...vectors.pages.within.clips]));
    expect(new Set(addressPages(course)).size).toBe(addressPages(course).length);
    for (const [page, names] of Object.entries(vectors.pages.labels)) for (const locale of ["en", "de"] as const) expect(pageLabel(page, { course, learner: STATE.learner }, locale, courseText(locale)), `${page}/${locale}`).toBe(names[locale]);
    expect(["quizzes", "assignments", "clips", "sample-topic"].map(containerKey)).toEqual(["quizzes", "assignments", "clips", "topic:sample-topic"]);
  });

  it("shows the two cards beside the quizzes and no list of assignments above the overview", () => {
    rendered(undefined);
    for (const page of vectors.pages.withBoth) expect(cardHost(page), page).not.toBeNull();
    expect(within(cardHost("assignments")).getByRole("link", { name: "Assignments" }).getAttribute("href")).toBe("#assignments");
    expect(within(cardHost("clips")).getByRole("link", { name: "Learning Clips" }).getAttribute("href")).toBe("#clips");
    const home = document.querySelector(".course-home")!;
    expect(home.querySelector("section[aria-label='Assignments']")).toBeNull();
    const above = [...home.children].slice(0, [...home.children].findIndex((element) => element.hasAttribute("data-course-scrollport")));
    expect(above.filter((element) => element.tagName === "SECTION" || element.querySelector("a[download]") !== null)).toEqual([]);
    expect(within(home as HTMLElement).queryAllByText(/· PDF/u)).toEqual([]);
  });

  it("shows no card of either kind for a course without them, as before", () => {
    render(<Home session={stubSession()} />);
    expect(cardHost("assignments")).toBeNull();
    expect(cardHost("clips")).toBeNull();
  });

  it("lists every assignment as a card with its emoji, title, description, a direct download and the way to its page, in each language", () => {
    for (const locale of ["en", "de"] as const) {
      rendered("assignments", locale);
      for (const assignment of assignments) {
        const card = document.querySelector<HTMLElement>(`[data-card="assignment:${assignment.id}"]`)!;
        expect(within(card).getByRole("link", { name: assignment.title[locale] }).getAttribute("href")).toBe(`#${assignment.id}`);
        expect(card.textContent).toContain(assignment.description[locale]);
        const download = card.querySelector<HTMLAnchorElement>("a[download]")!;
        expect(new URL(download.href, "http://site.test").pathname).toBe(vectors.viewer[assignment.id as keyof typeof vectors.viewer][locale]);
        expect(download.getAttribute("download")).toBe(assignment.documents[locale].file);
      }
      cleanup();
    }
  });

  it("shows the PDF of the learner's language inside the page of an opened assignment, with a download, a tab of its own and the way back", () => {
    for (const locale of ["en", "de"] as const) {
      const { session } = rendered("sample-exercise", locale);
      const page = pane("sample-exercise");
      expect(page.hasAttribute("inert")).toBe(false);
      const frame = page.querySelector<HTMLIFrameElement>("iframe")!;
      const url = new URL(frame.getAttribute("src")!, "http://site.test");
      expect(url.pathname).toBe(vectors.viewer["sample-exercise"][locale]);
      expect(url.origin).toBe("http://site.test");
      expect(frame.getAttribute("title")).toBe(locale === "en" ? "Finger Exercise (PDF)" : "Fingerübung (PDF)");
      expect(frame.getAttribute("lang")).toBe(locale);
      const download = within(page).getByRole("link", { name: locale === "en" ? "Download PDF" : "PDF herunterladen" }) as HTMLAnchorElement;
      expect(download.getAttribute("download")).toBe(`sample-exercise-${locale}.pdf`);
      expect(new URL(download.href).pathname).toBe(vectors.viewer["sample-exercise"][locale]);
      const tab = within(page).getByRole("link", { name: locale === "en" ? "Open in new tab" : "In neuem Tab öffnen" }) as HTMLAnchorElement;
      expect(tab.getAttribute("target")).toBe("_blank");
      expect(tab.getAttribute("rel")).toBe("noopener noreferrer");
      expect(new URL(tab.href).pathname).toBe(vectors.viewer["sample-exercise"][locale]);
      const order = [...page.querySelectorAll("a[href], button, iframe")].map((element) => element.tagName);
      expect(order.indexOf("IFRAME")).toBeGreaterThan(order.lastIndexOf("BUTTON"));
      expect(order.indexOf("IFRAME")).toBeGreaterThan(order.indexOf("A"));
      fireEvent.click(within(page).getByRole("button", { name: locale === "en" ? "Back to assignments" : "Zurück zu den Übungen" }));
      expect(session.open).toHaveBeenCalledWith({ screen: "home", page: "assignments" });
      cleanup();
    }
  });

  it("loads no PDF until its page is opened", () => {
    rendered("assignments");
    expect(document.querySelectorAll("iframe")).toHaveLength(0);
    expect(pane("sample-exercise").querySelector("[data-assignment-viewer='waiting']")).not.toBeNull();
  });

  it("closes an opened assignment with Escape back to the assignments and its card's link", async () => {
    const { session } = rendered("sample-task");
    await act(async () => undefined);
    fireEvent.keyDown(document, { key: "Escape" });
    await act(async () => undefined);
    expect(session.open).toHaveBeenLastCalledWith({ screen: "home", page: "assignments" });
  });

  it("shows a poster for every clip and a coming-soon state for the clip without an address", () => {
    rendered("clips");
    for (const clip of clips) {
      const card = document.querySelector<HTMLElement>(`[data-card="clip:${clip.id}"]`)!;
      expect(within(card).getByRole("link", { name: clip.title.en }).getAttribute("href")).toBe(`#${clip.id}`);
      const poster = card.querySelector<HTMLImageElement>("[data-clip-poster] img");
      const built = (vectors.posters as Record<string, string>)[clip.poster.file];
      if (built === undefined) expect(poster).toBeNull();
      else {
        expect(poster!.getAttribute("src")).toBe(built);
        expect(poster!.getAttribute("alt")).toBe("");
        expect(poster!.getAttribute("loading")).toBe("lazy");
      }
    }
    const soon = document.querySelector<HTMLElement>('[data-card="clip:clip-soon"]')!;
    expect(soon.querySelector("[data-clip-state='soon']")?.textContent?.trim()).toBe("Coming soon");
    expect(soon.querySelector("a[target]")).toBeNull();
    expect(soon.querySelector("[data-clip-poster][data-unavailable] [data-course-unavailable-tint]"), "a clip not hosted yet shows its poster tinted").not.toBeNull();
    for (const clip of clips) {
      const card = document.querySelector<HTMLElement>(`[data-card="clip:${clip.id}"]`)!;
      expect(card.textContent, "a clip card holds its poster and chips, not its description").not.toContain(clip.description.en);
      expect(card.querySelector("[data-clip-poster]")!.hasAttribute("data-unavailable"), clip.id).toBe(clip.urls?.en === undefined);
    }
  });

  it("links a clip to its hosted address for the learner's language in a tab of its own, else says it is coming soon", () => {
    for (const locale of ["en", "de"] as const) {
      rendered("clips", locale);
      for (const clip of clips) {
        const card = document.querySelector<HTMLElement>(`[data-card="clip:${clip.id}"]`)!;
        const expected = vectors.watch[clip.id as keyof typeof vectors.watch][locale];
        const link = card.querySelector<HTMLAnchorElement>("a[target='_blank']");
        if (expected === null) {
          expect(link, `${clip.id}/${locale}`).toBeNull();
          expect(card.querySelector("[data-clip-state='soon']")?.textContent?.trim()).toBe(locale === "en" ? "Coming soon" : "Folgt bald");
        } else {
          expect(link!.href, `${clip.id}/${locale}`).toBe(new URL(expected).href);
          expect(link!.getAttribute("rel")).toBe("noopener noreferrer");
          expect(link!.getAttribute("aria-label")).toContain(clip.title[locale]);
          expect(link!.textContent?.trim()).toBe(locale === "en" ? "Watch clip" : "Clip ansehen");
          expect(card.querySelector("[data-clip-state='soon']")).toBeNull();
        }
      }
      cleanup();
    }
  });

  it("opens a clip's page with its poster, its state and the way back to the clips", () => {
    const { session } = rendered("clip-hosted");
    const page = pane("clip-hosted");
    expect(page.querySelector("[data-clip-poster] img")?.getAttribute("src")).toBe("/assets/Sample_02_Hosted-AbCd1234.png");
    expect(within(page).getByRole("link", { name: /Watch clip/u }).getAttribute("href")).toBe("https://videos.example.org/en/hosted");
    fireEvent.click(within(page).getByRole("button", { name: "Back to learning clips" }));
    expect(session.open).toHaveBeenCalledWith({ screen: "home", page: "clips" });
  });

  it("falls back to a designed placeholder where a clip's poster was not built", () => {
    rendered("clips");
    const card = document.querySelector<HTMLElement>('[data-card="clip:clip-german"]')!;
    expect(card.querySelector("[data-clip-poster] img")).toBeNull();
    expect(card.querySelector("[data-clip-poster]")?.textContent).toContain("🎬");
  });
});
