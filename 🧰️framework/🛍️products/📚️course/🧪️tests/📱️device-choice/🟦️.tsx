/** 📱️ The device setting: on a phone there is nothing to choose, and on anything wider desktop and tablet
 * are a persisted choice. Tablet is what the surface chrome paints, so the framework's touch targets apply.
 * @see ../../🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx — the setting and the stored choice
 * @see ../../../../🔨️modules/🖱️ui/📱️device/🟦️.ts — which device a choice paints
 */
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, elementsSurfaceDeviceForLayout } from "@semio-tech/ui-react/chrome";
import { PreferencesPanel, localStore, memoryStorageOrigin, courseText, readPreferences, writePreferences, type CoursePreferences } from "@semio-tech/course-react";

const PREFERENCES: CoursePreferences = { theme: "system", textSize: "normal", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "lively", petsLiveliness: "lively", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} };

function media(matches: { readonly mobile: boolean; readonly tablet: boolean }): void {
  window.matchMedia = ((query: string) => ({
    matches: query === UI_MOBILE_MEDIA_QUERY ? matches.mobile : query === UI_TABLET_MEDIA_QUERY ? matches.tablet : false,
    media: query,
    onchange: null,
    addEventListener() {},
    removeEventListener() {},
    addListener() {},
    removeListener() {},
    dispatchEvent() {
      return false;
    },
  })) as unknown as typeof window.matchMedia;
}

afterEach(() => {
  cleanup();
});

describe("device choice", () => {
  it("follows the measured device until a choice is stored, and a stored choice round-trips", () => {
    expect(elementsSurfaceDeviceForLayout("desktop", undefined)).toBe("desktop");
    expect(elementsSurfaceDeviceForLayout("tablet", undefined)).toBe("tablet");
    expect(elementsSurfaceDeviceForLayout("desktop", "tablet")).toBe("tablet");
    expect(elementsSurfaceDeviceForLayout("mobile", "tablet")).toBe("mobile");
    const origin = memoryStorageOrigin();
    const store = localStore(origin.tab(), "arch");
    expect(readPreferences(store).device).toBeUndefined();
    writePreferences(store, { ...readPreferences(store), device: "tablet" });
    expect(readPreferences(localStore(origin.tab(), "arch")).device).toBe("tablet");
    store.write("preferences", { theme: "dark", device: "phone" });
    expect(readPreferences(store).device).toBeUndefined();
  });

  it("shows desktop and tablet on a desktop window and hides them on a phone", () => {
    media({ mobile: false, tablet: false });
    const calls: CoursePreferences[] = [];
    const { unmount } = render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={(next) => calls.push(next)} />);
    const group = screen.getByRole("group", { name: "Device" });
    expect(pressed(group)).toEqual(["Desktop"]);
    fireEvent.click(screen.getByRole("button", { name: "Tablet" }));
    expect(calls.at(-1)).toEqual({ ...PREFERENCES, device: "tablet" });
    unmount();

    media({ mobile: false, tablet: true });
    render(<PreferencesPanel preferences={PREFERENCES} locale="de" text={courseText("de")} onChange={() => undefined} />);
    expect(pressed(screen.getByRole("group", { name: "Gerät" }))).toEqual(["Tablet"]);

    cleanup();
    media({ mobile: true, tablet: false });
    render(<PreferencesPanel preferences={{ ...PREFERENCES, device: "tablet" }} locale="en" text={courseText("en")} onChange={() => undefined} />);
    expect(screen.queryByRole("group", { name: "Device" })).toBeNull();
  });
});

function pressed(group: HTMLElement): string[] {
  return [...group.querySelectorAll("button")].filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => (button.textContent ?? "").trim());
}
