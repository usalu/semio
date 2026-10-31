import Ajv from "ajv";
import {JSDOM} from "jsdom";
import pointer from "../../🧫️fixtures/🔓️access/👆️pointer.json";
import pointerSchema from "../../🧫️fixtures/🔓️access/🧬️schema/👆️pointer.json";
/** 🧭️ Pointer, keyboard, URL, schedule-boundary and question preview behavior. */
import { act, fireEvent, render, renderHook, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { AccessGate, clientAccess, previewAccess, useReleaseClock } from "../../🎯️targets/⚛️react/🔨️modules/🔓️access/🟦️.tsx";
import { courseText, navigationWays, useAddress, type CourseSession, type CourseState } from "@semio-tech/course-react";
import { COURSE, Home, STATE, cardHost, pane, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";
import vectors from "../../🧫️fixtures/🔓️access/🔣️.json";

const locked = { ...COURSE, unlocks: { physics: null, "heating/heating-task": null } };

afterEach(() => { vi.useRealTimers(); window.history.replaceState(null, "", "/"); });

describe("scheduled UI access", () => {
  it("keeps a quiz preview behind a tint, removes links and disables actions for pointer and keyboard", async () => {
    const session = stubSession();
    render(<Home session={session} course={locked} initial={{ screen: "home", page: "quizzes" }} />);
    const host = cardHost("physics");
    const gate = host.querySelector<HTMLElement>('[data-course-locked="physics"]')!;
    expect(gate.querySelector(".course-access-preview[inert][aria-hidden=true]")).toBeTruthy();
    expect(gate.querySelector(".course-access-tint")).toBeTruthy();
    expect(gate.textContent).toContain("About physics.");
    expect(gate.textContent).toContain("Not available yet");
    expect(host.querySelector("a[href]")).toBeNull();
    expect(host.querySelector("button")).toHaveProperty("disabled", true);
    fireEvent.click(host.querySelector("section")!);
    fireEvent.click(host.querySelector("button")!);
    await userEvent.setup().keyboard("{Enter} ");
    expect(session.open).not.toHaveBeenCalled();
    expect(session.startRun).not.toHaveBeenCalled();
    expect(pane("physics").hasAttribute("data-opened")).toBe(false);
  });

  it("locks a topic and all descendant quiz previews while keeping their content mounted", () => {
    const course = { ...COURSE, topics: [{ id: "energy", emoji: "⚡", title: { en: "Energy", de: "Energie" }, quizzes: COURSE.quizzes.map(quiz => quiz.id) }], unlocks: { energy: null } };
    render(<Home session={stubSession()} course={course} initial={{ screen: "home", page: "quizzes" }} />);
    expect(cardHost("energy").querySelector('[data-course-locked="energy"]')).toBeTruthy();
    expect(pane("physics").textContent).toContain("About physics.");
    expect(pane("physics").querySelector('[data-course-locked="physics"]')).toBeTruthy();
  });

  it("does not open an ordinary hash deep link", () => {
    window.history.replaceState(null, "", "/#physics");
    const session = stubSession();
    const state = { ...STATE, course: locked, step: { screen: "home" } } as CourseState;
    renderHook(() => useAddress({ ...session, getSnapshot: () => ({ state }) } as unknown as CourseSession, state));
    expect(session.open).not.toHaveBeenCalled();
  });

  it("disables saved-run actions and history destinations of locked quizzes", () => {
    const session = stubSession();
    const course = { ...COURSE, unlocks: { heating: null, cooling: null } };
    render(<Home session={session} course={course} initial={{ screen: "home", page: "learner" }} />);
    const rows = within(pane("learner")).getAllByRole("row").filter(row => /Quiz (?:heating|cooling)/.test(row.textContent ?? ""));
    expect(rows.length).toBeGreaterThan(0);
    for (const row of rows) for (const button of row.querySelectorAll("button")) expect(button.disabled).toBe(true);
    expect(navigationWays({ ...STATE, course, step: { screen: "home", page: "quizzes" }, trail: { back: [{ screen: "home", page: "heating" }], forward: [] } }).back).toBeUndefined();
  });

  it("requires the exact hidden argument and permits author deep links", () => {
    for (const row of vectors.urls) expect(previewAccess(row.search)).toBe(row.preview);
    window.history.replaceState(null, "", "/?preview=1#physics");
    expect(previewAccess()).toBe(true);
    expect(clientAccess(locked, "physics", 0)).toBe(true);
    const session = stubSession();
    render(<Home session={session} course={locked} initial={{ screen: "home", page: "physics" }} />);
    expect(pane("physics").hasAttribute("data-opened")).toBe(true);
    expect(pane("physics").querySelector("[data-course-locked]")).toBeNull();
  });

  it("unlocks automatically at the exact scheduled instant without a reload", async () => {
    vi.useFakeTimers();
    vi.setSystemTime(1000);
    const course = { ...COURSE, unlocks: { physics: 2000 } };
    render(<Home session={stubSession()} course={course} initial={{ screen: "home", page: "quizzes" }} />);
    expect(cardHost("physics").querySelector("[data-course-locked]")).toBeTruthy();
    await act(async () => { vi.advanceTimersByTime(999); });
    expect(cardHost("physics").querySelector("[data-course-locked]")).toBeTruthy();
    await act(async () => { vi.advanceTimersByTime(1); });
    expect(cardHost("physics").querySelector("[data-course-locked]")).toBeNull();
    expect(cardHost("physics").querySelector("a[href]")).toBeTruthy();
  });

  it("rechecks sleeping tabs on focus and releases its timer and listeners on unmount", () => {
    vi.useFakeTimers();
    let instant = 1000;
    const now = () => instant;
    const { result, unmount } = renderHook(() => useReleaseClock({ ...COURSE, unlocks: { physics: 2000 } }, now));
    expect(result.current).toBe(1000);
    instant = 2000;
    act(() => { window.dispatchEvent(new Event("focus")); });
    expect(result.current).toBe(2000);
    unmount();
    expect(vi.getTimerCount()).toBe(0);
  });

  for (const locale of ["en", "de"] as const) it(`keeps a question preview and speaks its status in ${locale}`, () => {
    render(<AccessGate course={locked} page="heating" task="heating-task" at={1000} locale={locale} text={courseText(locale)}><button>Question preview</button></AccessGate>);
    expect(screen.getByRole("status").textContent?.trim()).toBe(locale === "en" ? "Not available yet" : "Noch nicht freigeschaltet");
    expect(document.querySelector("[inert]")?.textContent).toContain("Question preview");
    expect(screen.queryByRole("button")).toBeNull();
  });
});

/** 👆️ The real locked tint receives ordinary input inside the overview pointer boundary. */
for(const locale of pointer.locales) it(`locked tint receives pointer input without unlocking in ${locale}`,async()=>{
 if(locale!=="en"&&locale!=="de")throw Error("Authored locale required");
 expect(new Ajv({strict:true}).compile(pointerSchema)(pointer)).toBe(true);
 const open=vi.fn(),view=render(<><style>{".pointer-events-none{pointer-events:none}.pointer-events-auto{pointer-events:auto}"}</style><div className="pointer-events-none"><AccessGate course={locked} page="physics" at={1000} locale={locale} text={courseText(locale)}><button onClick={open}>Locked action</button></AccessGate></div></>),tint=view.container.querySelector<HTMLElement>(".course-access-tint")!,preview=view.container.querySelector<HTMLElement>(".course-access-preview")!;
 expect(getComputedStyle(tint).pointerEvents).toBe(pointer.tintPointer);
 await userEvent.setup().click(tint);
 expect(open).not.toHaveBeenCalled();expect(preview.hasAttribute("inert")).toBe(true);expect(preview.getAttribute("aria-hidden")).toBe("true");
 const oracle=new JSDOM(view.container.innerHTML);try{expect(oracle.window.getComputedStyle(oracle.window.document.querySelector(".course-access-tint")!).pointerEvents).toBe(pointer.tintPointer);expect(oracle.window.document.querySelector("[inert][aria-hidden=true]")?.textContent).toBe("Locked action");}finally{oracle.window.close();}
});
