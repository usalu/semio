import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { CardAction, CourseApp, CourseCard, HomeScreen, courseText, homePages, initialCourseState, localStore, memoryStorageOrigin, readPreferences, type CourseSession } from "@semio-tech/course-react";
import type { CourseView } from "@semio-tech/course";
import { CourseApp as ShellCourseApp, courseScreen } from "@semio-tech/course-react/app";
import { get } from "lodash";
import fixture from "./🔣️.json";

const course: CourseView = {
  id: "course-shell",
  title: { en: "Test Course", de: "Testkurs" },
  introduction: { title: { en: "Welcome", de: "Willkommen" }, paragraphs: [] },
  quizzes: [],
  badges: [],
};

describe("course shell", () => {
  it("exports the same course product component through the focused app entry", () => {
    expect(typeof CourseApp).toBe("function");
    expect(ShellCourseApp).toBe(CourseApp);
  });

  it.each(fixture.screens)("loads only the $module module for $screen", ({ screen, module }) => {
    const target = courseScreen(screen as Parameters<typeof courseScreen>[0]);
    expect(target).toBe(module);
    expect(target).toBe(get({ home: "HomeScreen", identity: "HomeScreen", introduction: "HomeScreen", run: "RunScreen", results: "ResultsScreen" }, screen));
  });

  it.each(fixture.overlayCards)("receives footer actions through a passthrough overlay when openable=$openable", async ({ openable, actions, openings }) => {
    const action = vi.fn();
    const open = vi.fn();
    render(<><style>{".pointer-events-auto { pointer-events: auto; }"}</style><div style={{ pointerEvents: "none" }}><CourseCard id="card-title" card="test" title="Card" icon={null} onOpen={openable ? open : undefined} footerRight={<CardAction primary onClick={action}>Action</CardAction>} /></div></>);
    await userEvent.setup().click(screen.getByRole("button", { name: "Action" }));
    expect(action).toHaveBeenCalledTimes(actions);
    expect(open).toHaveBeenCalledTimes(openings);
  });

  it.each(fixture.locales)("opens the $locale course with a dedicated quiz section", ({ locale, home, quizzes }) => {
    const language = locale as "en" | "de";
    const session = { open: vi.fn(), refreshLeaderboard: vi.fn(async () => ({ answered: true })), refreshCrowd: vi.fn(async () => undefined) } as unknown as CourseSession;
    const state = initialCourseState({ introduced: true, course, runs: {} });
    render(<HomeScreen session={session} state={state} text={courseText(language)} locale={language} preferences={readPreferences(localStore(memoryStorageOrigin().tab(), course.id))} onPreferences={() => undefined} />);
    expect(homePages()).toEqual(fixture.pages);
    expect(screen.getByRole("heading", { name: home, level: 1 })).toBeDefined();
    expect(screen.getByRole("heading", { name: quizzes })).toBeDefined();
    expect(document.querySelector('[data-course-grid="root"]')).not.toBeNull();
  });
});
