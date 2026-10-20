/** 🔶️ The learner s choice tests of the 🐾️ pet companions suite. */
import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { PET_CHOICES, PET_CHOICE_LABELS, PetsSwitch, PreferencesPanel, COURSE_LOCALES, CoursePetsProvider, localStore, memoryStorageOrigin, courseText, readPreferences, switchedPets, withPets, writePreferences } from "@semio-tech/course-react";
import { PREFERENCES, at, fakeSource, fakeStage, text } from "../../🪛️pet-companions-kit/🟦️.tsx";

/** 🔷 Registers the learner s choice tests of the 🐾️ pet companions suite. */
export function registerLearnerSChoiceTests(): void {
  describe("the learner's choice", () => {
    it("is lively until the learner chooses otherwise, and stays on the device", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store).pets).toBe("lively");
      for (const choice of PET_CHOICES) {
        writePreferences(store, { ...readPreferences(store), pets: choice });
        expect(readPreferences(localStore(origin.tab(), "arch")).pets).toBe(choice);
      }
      store.write("preferences", { theme: "dark", pets: "wild" });
      expect(readPreferences(store)).toMatchObject({ theme: "dark", pets: "lively", petsLiveliness: "lively" });
    });

    it("remembers how lively the pets were while they are off, so the switch brings them back as they were", () => {
      const store = localStore(memoryStorageOrigin().tab(), "arch");
      expect(readPreferences(store).petsLiveliness).toBe("lively");
      for (const choice of ["still", "calm", "lively"] as const) {
        const chosen = withPets(readPreferences(store), choice);
        expect(chosen).toMatchObject({ pets: choice, petsLiveliness: choice, petsChosen: true });
        const hidden = withPets(chosen, switchedPets(false, chosen.petsLiveliness));
        expect(hidden).toMatchObject({ pets: "off", petsLiveliness: choice, petsChosen: true });
        writePreferences(store, hidden);
        const read = readPreferences(store);
        expect(read).toMatchObject({ pets: "off", petsLiveliness: choice, petsChosen: true });
        expect(withPets(read, switchedPets(true, read.petsLiveliness))).toMatchObject({ pets: choice, petsLiveliness: choice, petsChosen: true });
      }
      store.write("preferences", { pets: "off", petsLiveliness: "off" });
      expect(readPreferences(store)).toMatchObject({ pets: "off", petsLiveliness: "lively" });
      store.write("preferences", { pets: "lively", petsLiveliness: "still" });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively" });
    });

    it("knows whether the learner chose: a fact of its own that stays on the device, that only a choice for the pets states and that preferences stored without it do not carry", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: false });
      writePreferences(store, { ...readPreferences(store), locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false });
      expect(readPreferences(localStore(origin.tab(), "arch"))).toEqual({ locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false, iconsChosen: false, pets: "lively", petsLiveliness: "lively", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} });
      for (const choice of PET_CHOICES) {
        const device = memoryStorageOrigin();
        const fresh = localStore(device.tab(), "arch");
        writePreferences(fresh, withPets(readPreferences(fresh), choice));
        expect(readPreferences(localStore(device.tab(), "arch"))).toMatchObject({ pets: choice, petsChosen: true });
        writePreferences(fresh, { ...readPreferences(fresh), theme: "light", animateIcons: false });
        expect(readPreferences(localStore(device.tab(), "arch"))).toMatchObject({ theme: "light", pets: choice, petsChosen: true });
      }
      const untouched = readPreferences(localStore(memoryStorageOrigin().tab(), "arch"));
      expect(withPets(untouched, switchedPets(false, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "off", petsLiveliness: "lively", petsChosen: true });
      expect(withPets(untouched, switchedPets(true, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "lively", petsLiveliness: "lively", petsChosen: true });
      const before: readonly Record<string, unknown>[] = [{ pets: "lively", petsLiveliness: "lively" }, { pets: "off", petsLiveliness: "still" }, { theme: "dark" }, { pets: "still", petsChosen: "true" }, { pets: "calm", petsChosen: 1 }, { pets: "calm", petsChosen: null }];
      for (const stored of before) {
        store.write("preferences", stored);
        expect(readPreferences(store).petsChosen, JSON.stringify(stored)).toBe(false);
      }
      store.write("preferences", { pets: "lively", petsChosen: true });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: true });
      store.write("preferences", { pets: "wild", petsChosen: true });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsChosen: true });
    });

    it("is the learner's own only through the pets' row: no other control of the preferences states it", () => {
      const onChange = vi.fn();
      render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={onChange} />);
      const row = screen.getByRole("group", { name: "Pets" });
      const others = [...screen.getAllByRole("button"), ...screen.getAllByRole("checkbox")].filter((control) => !row.contains(control));
      expect(others.length).toBeGreaterThanOrEqual(14);
      for (const control of others) {
        fireEvent.click(control);
        expect(onChange.mock.lastCall?.[0], control.closest("label")?.textContent ?? control.textContent ?? "").toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: false });
      }
      expect(onChange).toHaveBeenCalledTimes(others.length);
      for (const button of within(row).getAllByRole("button")) {
        fireEvent.click(button);
        expect(onChange.mock.lastCall?.[0], button.textContent ?? "").toMatchObject({ petsChosen: true });
      }
    });

    it("offers off, still, calm and lively in both languages", () => {
      expect([...PET_CHOICES]).toEqual(["off", "still", "calm", "lively"]);
      expect(Object.keys(PET_CHOICE_LABELS)).toEqual([...PET_CHOICES]);
      const labels = { en: { group: "Pets", choices: ["Off", "Still", "Calm", "Lively"] }, de: { group: "Tierchen", choices: ["Aus", "Reglos", "Ruhig", "Lebhaft"] } };
      for (const locale of COURSE_LOCALES) {
        const onChange = vi.fn();
        const { unmount } = render(<PreferencesPanel preferences={PREFERENCES} locale={locale} text={courseText(locale)} onChange={onChange} />);
        const buttons = within(screen.getByRole("group", { name: labels[locale].group })).getAllByRole("button");
        expect(buttons.map((button) => button.textContent?.trim())).toEqual(labels[locale].choices);
        expect(buttons.map((button) => button.getAttribute("aria-pressed"))).toEqual(["false", "false", "true", "false"]);
        buttons.forEach((button, index) => {
          fireEvent.click(button);
          expect(onChange).toHaveBeenLastCalledWith({ ...PREFERENCES, pets: PET_CHOICES[index], petsLiveliness: PET_CHOICES[index] === "off" ? "calm" : PET_CHOICES[index], petsChosen: true });
        });
        unmount();
      }
    });

    it("has a switch for every screen: a checkbox named in both languages that the keyboard operates, and none on a site without pets", () => {
      const names = { en: "Show pets", de: "Tierchen anzeigen" };
      for (const locale of COURSE_LOCALES) {
        expect(courseText(locale)("course.preferences.petsShown")).toBe(names[locale]);
        for (const shown of [true, false]) {
          const onChange = vi.fn();
          const { unmount } = render(
            <CoursePetsProvider source={fakeSource()} stage={fakeStage().stage} choice="off" chosen={false} play mischief state={at({ screen: "home" })} locale={locale}>
              <PetsSwitch shown={shown} label={names[locale]} onChange={onChange} />
            </CoursePetsProvider>,
          );
          const box = screen.getByRole("checkbox", { name: names[locale] }) as HTMLInputElement;
          expect(box.checked).toBe(shown);
          expect(box.tabIndex).toBe(0);
          expect(box.closest("label")?.textContent).toBe(names[locale]);
          fireEvent.click(box);
          expect(onChange).toHaveBeenLastCalledWith(!shown);
          unmount();
        }
      }
      const { container } = render(
        <CoursePetsProvider source={undefined} choice="calm" chosen={false} play mischief state={at({ screen: "home" })} locale="en">
          <PetsSwitch shown label="Show pets" onChange={() => undefined} />
        </CoursePetsProvider>,
      );
      expect(container.innerHTML).toBe("");
      expect(render(<PetsSwitch shown label="Show pets" onChange={() => undefined} />).container.innerHTML).toBe("");
    });
  });
}
