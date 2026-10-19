import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { CourseView } from "@semio-tech/course";
import { HomeScreen, IdentityScreen, ProctorUnavailable, CourseApp, CourseIntroduction, evolveCourseState, homePages, initialCourseState, localStore, memoryStorageOrigin, courseIntroduction, courseText, readPreferences, type CourseSession } from "@semio-tech/course-react";
import fixture from "../../🧫️fixtures/🎓️onboarding/🔣️.json";

const course: CourseView = {
  id: "onboarding",
  title: { en: "Quizzes", de: "Quizze" },
  introduction: { title: { en: "Welcome", de: "Willkommen" }, paragraphs: [{ en: "Learn by playing.", de: "Spielend lernen." }] },
  quizzes: [],
  badges: [],
};
const initial = (introduced = false) => initialCourseState({ introduced, course, learner: undefined, learnerView: undefined, runs: {} });
const session = () => ({ open: vi.fn(), identify: vi.fn(async () => undefined), refreshLeaderboard: vi.fn(async () => ({ answered: true })), refreshCrowd: vi.fn(async () => undefined) });

describe("overview onboarding", () => {
  it.each(fixture.visits)("opens home for $introduced / $identified", (visit) => {
    const state = initialCourseState({ ...initial(visit.introduced), learner: visit.identified ? { id: "a".repeat(32) } : undefined });
    expect(state.step.screen).toBe(visit.screen);
  });

  it("removes the introduction page and preserves a visited page on dismissal", () => {
    expect(homePages()).toEqual(fixture.pages);
    const state = { ...initial(), step: { screen: "home" as const, page: "prefs" } };
    expect(evolveCourseState(state, { type: "introduction-read" })).toMatchObject({ introduced: true, step: state.step });
  });

  it.each(["en", "de"] as const)("uses shared anchored introduction chrome in %s", { timeout: 30_000 }, async (locale) => {
    const user = userEvent.setup();
    const dismiss = vi.fn();
    expect(courseIntroduction(course, locale).steps.map(step => step.introduce)).toEqual(fixture.anchors);
    render(<CourseIntroduction course={course} locale={locale} onDismiss={dismiss} />);
    await waitFor(() => expect(document.querySelector('[data-slot="introduction-veil"]')).not.toBeNull(), { timeout: 20_000 });
    expect(document.querySelector('[data-slot="introduction-info-box-title"]')?.textContent).toBe(course.introduction.title[locale]);
    await user.keyboard("{Escape}");
    expect(dismiss).toHaveBeenCalledOnce();
  });

  it("renders the guest overview and opens the identity question", async () => {
    const user = userEvent.setup();
    const client = session();
    render(<HomeScreen session={client as unknown as CourseSession} state={initial(true)} text={courseText("en")} locale="en" preferences={readPreferences(localStore(memoryStorageOrigin().tab(), course.id))} onPreferences={() => undefined} />);
    expect(document.querySelector('[data-course-grid="root"]')).not.toBeNull();
    expect(document.querySelector('[data-layered-card="intro"]')).toBeNull();
    await user.click(await screen.findByRole("button", { name: courseText("en")("course.identity.signIn") }));
    expect(client.open).toHaveBeenCalledWith({ screen: "identity" });
  });

  it("asks identity in an accessible dialog and cancels with Escape", async () => {
    const user = userEvent.setup();
    const client = session();
    render(<IdentityScreen session={client as unknown as CourseSession} text={courseText("en")} />);
    await waitFor(() => expect(screen.getByRole("dialog").getAttribute("aria-modal")).toBe("true"));
    await user.keyboard("{Escape}");
    expect(client.open).toHaveBeenCalledWith({ screen: "home" }, true);
  });

  it("keeps the overview mounted through introduction, login, cancellation and replay", { timeout: 30_000 }, async () => {
    const user = userEvent.setup();
    const storage = memoryStorageOrigin().tab();
    const store = localStore(storage, course.id);
    render(<CourseApp proctor="" tenant={course.id} languages={["en"]} storage={storage} transport={() => ({ send: async () => { throw new ProctorUnavailable("test outage"); } })} material={{ course: { schema: "semio.course/v1", ...course, quizzes: [], badges: [] }, quizzes: [] }} timing={{ minMs: 60_000, maxMs: 60_000 }} />);
    const overview = await screen.findByRole("heading", { name: "Course", level: 1 });
    await waitFor(() => expect(document.querySelector('[data-slot="introduction-veil"]')).not.toBeNull(), { timeout: 20_000 });
    expect(screen.queryByRole("dialog")).toBeNull();
    await user.keyboard("{Escape}");
    expect(store.read("introduced")).toBe(true);
    const signIn = screen.getByRole("button", { name: "Sign in" });
    await user.click(signIn);
    expect(screen.getByRole("dialog").getAttribute("aria-modal")).toBe("true");
    expect(overview.isConnected).toBe(true);
    await user.keyboard("{Escape}");
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(document.activeElement).toBe(signIn);
    await user.click(within(screen.getByRole("navigation", { name: "Main navigation" })).getByRole("button", { name: "Introduction" }));
    await waitFor(() => expect(document.querySelector('[data-slot="introduction-veil"]')).not.toBeNull());
  });
});
