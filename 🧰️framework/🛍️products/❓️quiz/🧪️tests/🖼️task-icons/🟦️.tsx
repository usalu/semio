/** 🖼️ Every task, item, category and dimension can carry an icon — one emoji and one of the looping microanimations (shared
 * vectors: the sheets of `icon-demo`): the glyph is hidden from assistive
 * technology, a task without an icon keeps the icon of its kind, every icon stands in a host of its own, and every motion
 * has its keyframes and plays only on a hovered (desktop), focused or tapped (tablet, phone) host where the client
 * switched the icons on — the learner's choice, kept on the device, and until they choose every device but one that
 * asks for reduced motion.
 *
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html
 */

import { fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { MOTIONS, type Sheet } from "@semio-tech/quiz";
import { ICON_HOSTS, ICON_LOOP_MS, IconLabel, PreferencesPanel, TaskGlyph, TaskView, effectiveIconMotion, playTappedIcons, localStore, memoryStorageOrigin, quizText, readPreferences, textPresentation, withIcons } from "@semio-tech/quiz-react";
import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");
const glyph = (container: HTMLElement): HTMLElement | null => container.querySelector<HTMLElement>(".quiz-glyph");

describe("task icons", () => {
  it("shows the emoji of the task as a hidden glyph playing its motion", () => {
    for (const motion of MOTIONS) {
      const { container, unmount } = render(<TaskGlyph task={{ kind: "sorting", icon: { emoji: "🔋", motion } }} />);
      const shown = glyph(container)!;
      expect(shown.getAttribute("aria-hidden")).toBe("true");
      expect(shown.dataset.motion).toBe(motion);
      expect(shown.textContent).toBe("🔋");
      unmount();
    }
  });

  it("asks the monochrome emoji face for sequences with a colour presentation", () => {
    const { container } = render(<TaskGlyph task={{ kind: "matching", icon: { emoji: "❄️", motion: "spin" } }} />);
    expect(glyph(container)!.textContent).toBe("❄︎");
  });

  it("keeps the icon of its kind for a task without an icon", () => {
    const { container } = render(<TaskGlyph task={{ kind: "classification" }} />);
    expect(glyph(container)).toBeNull();
    expect(container.querySelector("svg")).not.toBeNull();
  });

  it("names keyframes for every motion and plays them only on a hovered, focused or tapped host where the client switched the icons on", () => {
    for (const motion of MOTIONS) {
      expect(css).toContain(`@keyframes quiz-icon-${motion} {`);
      expect(css).toContain(`.quiz-glyph[data-motion="${motion}"] {\n  --quiz-icon-animation: quiz-icon-${motion};`);
    }
    const hosts = ':is([data-icon-host], [data-slot="quiz-card-title-chip"])';
    const plays = [...css.matchAll(/^\s*([^{}\n]+) \{\n\s*animation: var\(--quiz-icon-animation\) 3\.2s ease-in-out (infinite|1);/gmu)].map(([, selector, count]) => `${selector!.trim()} ${count}`);
    expect(plays).toEqual([
      `.quiz-app[data-icon-motion="on"] ${hosts}:hover .quiz-glyph[data-motion] infinite`,
      `.quiz-app[data-icon-motion="on"] ${hosts}:is(:focus-visible, :has(:focus-visible)) .quiz-glyph[data-motion] infinite`,
      '.quiz-app[data-icon-motion="on"] [data-icon-tapped] .quiz-glyph[data-motion] 1',
    ]);
    expect(css.match(/animation: var\(--quiz-icon-animation\)/g)).toHaveLength(plays.length);
    expect(css).not.toMatch(/(?<![-\w])animation(-name)?:\s*quiz-icon-/u);
    expect(css).toMatch(/@media \(hover: hover\) \{\n\s*\.quiz-app\[data-icon-motion="on"\] :is\(\[data-icon-host\], \[data-slot="quiz-card-title-chip"\]\):hover /u);
    expect(ICON_HOSTS).toBe('[data-icon-host], [data-slot="quiz-card-title-chip"]');
  });

  it("shows the icon of every item, category and value card before its label, each in a host of its own, and keeps the labels as they are, with the keys shown and hidden", () => {
    const tasks = ["icons-1", "icons-1-hard"].flatMap((id) => (sheets.sheets.find((vector) => vector.id === id)!.sheet as unknown as Sheet).tasks);
    for (const task of tasks) {
      const { container, unmount } = render(<TaskView task={task} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />);
      const shown = [...container.querySelectorAll<HTMLElement>(".quiz-label-icon")];
      const cards = task.kind === "matching" ? task.dimensions.flatMap((dimension) => (dimension.icon === undefined ? [] : [dimension.icon, ...(dimension.cards ?? []).map(() => dimension.icon!)])) : [];
      const categories = task.kind === "classification" ? task.categories.flatMap((category) => category.icon ?? []) : [];
      const items = task.items.flatMap((item) => item.icon ?? []).flatMap((icon) => (task.kind === "matching" ? task.dimensions.map(() => icon) : [icon]));
      const expected = [...cards, ...categories, ...items].map((icon) => `${textPresentation(icon.emoji)} ${icon.motion}`).sort();
      expect(expected.length).toBeGreaterThan(0);
      expect(shown.map((icon) => `${icon.textContent} ${icon.dataset.motion}`).sort()).toEqual(expected);
      for (const icon of shown) expect(icon.getAttribute("aria-hidden")).toBe("true");
      for (const icon of shown) expect(icon.closest(ICON_HOSTS), icon.textContent ?? "").not.toBeNull();
      expect([...container.querySelectorAll(ICON_HOSTS)].filter((host) => host.parentElement?.closest(ICON_HOSTS) !== null)).toEqual([]);
      for (const item of task.items) expect(screen.getAllByText(item.label.en).length).toBeGreaterThan(0);
      unmount();
    }
  });

  it("plays the icons of a tapped host once, from the start on every tap, and leaves the mouse to hovering", () => {
    vi.useFakeTimers();
    try {
      const { container } = render(
        <ul>
          <li data-icon-host="">
            <IconLabel icon={{ emoji: "🔋", motion: "bounce" }}>
              <b>Battery</b>
            </IconLabel>
          </li>
          <li data-icon-host="">
            <b>No icon</b>
          </li>
        </ul>,
      );
      const [host, bare] = [...container.querySelectorAll("li")];
      playTappedIcons(screen.getByText("Battery"), "mouse");
      expect(host!.hasAttribute("data-icon-tapped")).toBe(false);
      playTappedIcons(screen.getByText("No icon"), "touch");
      expect(bare!.hasAttribute("data-icon-tapped")).toBe(false);
      playTappedIcons(screen.getByText("Battery"), "touch");
      expect(host!.hasAttribute("data-icon-tapped")).toBe(true);
      vi.advanceTimersByTime(ICON_LOOP_MS - 1);
      playTappedIcons(screen.getByText("Battery"), "pen");
      vi.advanceTimersByTime(ICON_LOOP_MS - 1);
      expect(host!.hasAttribute("data-icon-tapped")).toBe(true);
      vi.advanceTimersByTime(1);
      expect(host!.hasAttribute("data-icon-tapped")).toBe(false);
    } finally {
      vi.useRealTimers();
    }
  });

  it("follows the device until the learner chooses, and the learner's choice after that", () => {
    expect(effectiveIconMotion(true, false, false)).toBe(true);
    expect(effectiveIconMotion(true, false, true)).toBe(false);
    expect(effectiveIconMotion(false, false, false)).toBe(true);
    expect(effectiveIconMotion(true, true, true)).toBe(true);
    expect(effectiveIconMotion(false, true, false)).toBe(false);
  });

  it("keeps the learner's choice on the device and marks it as chosen", () => {
    const store = localStore(memoryStorageOrigin().tab(), "arch");
    expect(readPreferences(store)).toMatchObject({ animateIcons: true, iconsChosen: false });
    store.write("preferences", { theme: "dark", textSize: "large", animateIcons: false, iconsChosen: true });
    expect(readPreferences(store)).toMatchObject({ animateIcons: false, iconsChosen: true });
    const onChange = vi.fn();
    render(<PreferencesPanel preferences={readPreferences(store)} locale="en" text={quizText("en")} onChange={onChange} />);
    const box = screen.getByRole("checkbox", { name: "Animate icons" }) as HTMLInputElement;
    expect(box.checked).toBe(false);
    fireEvent.click(box);
    expect(onChange).toHaveBeenLastCalledWith(expect.objectContaining({ animateIcons: true, iconsChosen: true }));
  });

  it("says why the icons stand still on a device that asks for reduced motion, until the learner chooses", () => {
    const matchMedia = window.matchMedia;
    window.matchMedia = ((query: string) => ({ matches: query === "(prefers-reduced-motion: reduce)", media: query, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined })) as unknown as typeof window.matchMedia;
    try {
      const store = localStore(memoryStorageOrigin().tab(), "arch");
      const first = render(<PreferencesPanel preferences={readPreferences(store)} locale="en" text={quizText("en")} onChange={() => undefined} />);
      expect((screen.getByRole("checkbox", { name: "Animate icons" }) as HTMLInputElement).checked).toBe(false);
      expect(screen.getByText("Your device asks for less motion, so the icons stay still until you choose.")).toBeTruthy();
      first.unmount();
      render(<PreferencesPanel preferences={withIcons(readPreferences(store), true)} locale="en" text={quizText("en")} onChange={() => undefined} />);
      expect((screen.getByRole("checkbox", { name: "Animate icons" }) as HTMLInputElement).checked).toBe(true);
      expect(screen.queryByText("Your device asks for less motion, so the icons stay still until you choose.")).toBeNull();
    } finally {
      window.matchMedia = matchMedia;
    }
  });
});
