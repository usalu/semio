/** 📥️ What the client does when code, texts or material it fetches never arrive — each from a module graph of its own, in which the chunk is gone like after a redeploy: a missing language leaves a fallback in both languages with a way to try again and to reload instead of a blank page, a missing screen says a new version is available with a reload, and a material source that fails is asked again before the client goes on without a deputy.
 *
 * @see ../📥️bounded-load/🟦️.tsx — the bounded load and the fallback themselves
 * @see ../../🎯️targets/⚛️react/🔨️modules/📚️app/🟦️.tsx
 */

import { act, render, screen } from "@testing-library/react";
import { StrictMode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { COURSE_LOCALES, CourseApp, courseText, memoryStorageOrigin, waitingMessage, type CourseConnection } from "@semio-tech/course-react";
import { LOAD_FALLBACK_TEXTS } from "../../🎯️targets/⚛️react/🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx";
import { FakeProctor, MATERIAL, QUIET_PRESENCE, TIMING } from "../🧰️learner-journey-kit/🟦️.tsx";

const ASSIGNMENTS = "../../🎯️targets/⚛️react/🔨️modules/📝️assignments/🟦️.tsx";
const PHRASE = "../../🎯️targets/⚛️react/🔨️modules/🌐️i18n/🏷️phrase/🟦️.ts";
const PERSISTENCE = "../../🎯️targets/⚛️react/🔨️modules/💾️persistence/🟦️.ts";
const PREFERENCES = "../../🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx";
const GERMAN = "../../🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔡️german-bundle/🟦️.ts";

const cleanups: (() => void)[] = [];

/** 🧨️ The client in a module graph of its own in which `gone` — a chunk a redeploy replaced — exports nothing, so every fetch of it fails like a missing file; neither unhandled rejections nor reloads of the page escape the counts. */
async function clientWithout(gone: string, languages: string[] = ["en"]) {
  vi.resetModules();
  vi.doMock(gone, () => {
    throw new Error("chunk gone");
  });
  const testing = await import("@testing-library/react");
  const react = await import("react");
  const app = await import("@semio-tech/course-react/app");
  const persistence = await import(PERSISTENCE);
  const preferences = await import(PREFERENCES);
  const storage = persistence.memoryStorageOrigin().tab();
  const rejections: unknown[] = [];
  const record = (reason: unknown): void => {
    rejections.push(reason);
  };
  process.on("unhandledRejection", record);
  let navigations = 0;
  const virtual = (globalThis as unknown as { jsdom: { virtualConsole: { on: (event: string, listener: (error: Error) => void) => void; off: (event: string, listener: (error: Error) => void) => void } } }).jsdom.virtualConsole;
  const navigated = (error: Error): void => {
    if (/Not implemented: navigation/u.test(error.message)) navigations += 1;
  };
  virtual.on("jsdomError", navigated);
  cleanups.push(() => {
    process.off("unhandledRejection", record);
    virtual.off("jsdomError", navigated);
    testing.cleanup();
  });
  const quiet = new FakeProctor();
  const client = testing.render(
    <react.StrictMode>
      <app.CourseApp proctor="" tenant="load-failure" presence={QUIET_PRESENCE} transport={() => quiet.transport} storage={storage} languages={languages} timing={TIMING} />
    </react.StrictMode>,
  );
  return { testing, client, unhandled: () => rejections, reloads: () => navigations, chosen: () => preferences.readPreferences(persistence.localStore(storage, "load-failure")).locale };
}

 afterEach(() => {
  for (const cleanup of cleanups.splice(0)) cleanup();
  vi.doUnmock(ASSIGNMENTS);
  vi.doUnmock(PHRASE);
  vi.doUnmock(GERMAN);
  vi.resetModules();
});

describe("📥️ load failures", () => {
  it.each(COURSE_LOCALES)("says a new version is available, with a reload, for a screen that could not be fetched, in %s", (locale) => {
    const connection: CourseConnection = { online: true, reachability: "reachable", pending: 0, activity: "idle", deputy: false };
    const text = courseText(locale);
    expect(waitingMessage(connection, undefined, text, true)).toEqual({ message: text("course.app.updated"), retry: false, reload: true });
    expect(waitingMessage({ ...connection, online: false }, undefined, text, true)).toEqual({ message: text("course.app.updated"), retry: false, reload: true });
    expect(waitingMessage(connection, undefined, text)).toEqual({ message: text("course.app.fetching"), retry: false });
    expect(text("course.app.updated")).not.toBe(text("course.app.fetching"));
  });

  it("shows a fallback in both languages with a retry and a reload when the texts of the language never arrive", async () => {
    const { testing, client, unhandled, reloads } = await clientWithout(PHRASE);
    const { container } = client;
    expect(container.textContent).not.toBe("");
    await testing.waitFor(() => expect(container.querySelector("[data-load-fallback='failed']")).not.toBeNull());
    expect([...container.querySelectorAll("section[lang]")].map((part) => part.getAttribute("lang"))).toEqual(["en", "de"]);
    expect(testing.within(container).getAllByRole("alert").map((alert) => alert.textContent)).toEqual([LOAD_FALLBACK_TEXTS.en.failed, LOAD_FALLBACK_TEXTS.de.failed]);
    testing.fireEvent.click(testing.within(container).getByRole("button", { name: "Try again now" }));
    expect(container.querySelector("[data-load-fallback='failed']")).toBeNull();
    await testing.waitFor(() => expect(container.querySelector("[data-load-fallback='failed']")).not.toBeNull());
    expect(reloads()).toBe(0);
    testing.fireEvent.click(testing.within(container).getByRole("button", { name: "Seite neu laden" }));
    expect(reloads()).toBe(1);
    expect(unhandled()).toEqual([]);
  });

  it("tells the learner in the shown language that a switched-to language could not be loaded, keeps the choice unpersisted, and retries or dismisses on request", async () => {
    const { testing, client, unhandled, reloads, chosen } = await clientWithout(GERMAN);
    const { container } = client;
    const english = await testing.waitFor(() => testing.within(container).getByRole("button", { name: "Deutsch" }));
    testing.fireEvent.click(english);
    const sentence = "Deutsch could not be loaded – the course stays in English.";
    const notice = () => testing.within(container).queryByRole("alert");
    await testing.waitFor(() => expect(notice()?.textContent).toContain(sentence));
    expect(notice()?.closest("[lang]")?.getAttribute("lang")).toBe("en");
    expect(document.documentElement.getAttribute("lang")).toBe("en");
    expect(chosen()).toBeUndefined();
    testing.fireEvent.click(testing.within(notice() as HTMLElement).getByRole("button", { name: "Try again now" }));
    expect(notice()).toBeNull();
    await testing.waitFor(() => expect(notice()?.textContent).toContain(sentence));
    testing.fireEvent.click(testing.within(notice() as HTMLElement).getByRole("button", { name: "Dismiss" }));
    expect(notice()).toBeNull();
    expect(chosen()).toBeUndefined();
    expect(reloads()).toBe(0);
    expect(unhandled()).toEqual([]);
  });

  it("says a new version is available, with a reload, when the chunk of a screen is gone", async () => {
    const { testing, client, unhandled, reloads } = await clientWithout(ASSIGNMENTS);
    const { container } = client;
    await testing.waitFor(() => expect(testing.within(container).getByText(/A new version of the course is available/u)).toBeDefined());
    expect(container.querySelector("[data-load-fallback]")).toBeNull();
    expect(reloads()).toBe(0);
    testing.fireEvent.click(testing.within(container).getByRole("button", { name: "Reload page" }));
    expect(reloads()).toBe(1);
    expect(unhandled()).toEqual([]);
  });

  it("asks a failing material source again before it goes on, and seats the deputy once the material arrives", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    let calls = 0;
    const requested = vi.fn(async () => {
      calls += 1;
      if (calls < 3) throw new Error("chunk gone");
      return MATERIAL;
    });
    render(
      <StrictMode>
        <CourseApp proctor="" tenant={MATERIAL.course.id} material={requested} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />
      </StrictMode>,
    );
    expect((await screen.findAllByText(courseText("en")("course.connection.local"))).length).toBeGreaterThan(0);
    expect(requested.mock.calls.length).toBeGreaterThanOrEqual(3);
    await act(async () => undefined);
  });
});
