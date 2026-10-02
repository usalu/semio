/** 🖼️ Every task can carry an icon — one emoji and one of the looping microanimations: the glyph is hidden from assistive
 * technology, a task without an icon keeps the icon of its kind, every motion has its keyframes and plays only where the
 * learner neither prefers reduced motion nor switched the animation off, and that switch is kept on the device.
 *
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html
 */

import { fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { MOTIONS } from "@semio-tech/quiz";
import { PreferencesPanel, TaskGlyph, localStore, memoryStorageOrigin, quizText, readPreferences } from "@semio-tech/quiz-react";

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

  it("has keyframes and a rule for every motion, all behind the reduced-motion and preference gates", () => {
    const gated = css.slice(css.indexOf("@media (prefers-reduced-motion: no-preference)"));
    for (const motion of MOTIONS) {
      expect(css).toContain(`@keyframes quiz-icon-${motion} {`);
      expect(gated).toContain(`.quiz-glyph[data-motion="${motion}"] {\n    animation-name: quiz-icon-${motion};`);
    }
    expect(css.match(/animation-name:/g)).toHaveLength(MOTIONS.length);
    expect(gated).toContain('.quiz-app:not([data-icon-motion="off"])');
  });

  it("keeps the animation preference on the device, on unless switched off", () => {
    const store = localStore(memoryStorageOrigin().tab(), "arch");
    expect(readPreferences(store).animateIcons).toBe(true);
    store.write("preferences", { theme: "dark", textSize: "large", animateIcons: false });
    expect(readPreferences(store).animateIcons).toBe(false);
    const onChange = vi.fn();
    render(<PreferencesPanel preferences={readPreferences(store)} locale="en" text={quizText("en")} onChange={onChange} />);
    const box = screen.getByRole("checkbox", { name: "Animate task icons" }) as HTMLInputElement;
    expect(box.checked).toBe(false);
    fireEvent.click(box);
    expect(onChange).toHaveBeenLastCalledWith(expect.objectContaining({ animateIcons: true }));
  });
});
