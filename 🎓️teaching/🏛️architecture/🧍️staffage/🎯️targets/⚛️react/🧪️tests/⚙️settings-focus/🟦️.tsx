/** ⌨️ Keyboard focus in the responsive display settings.
 * @see ./🥒️.feature */
import { act, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, expect, it, vi } from "vitest";
import { mountArchive } from "@teaching/staffage-react/app";
import { memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";

let unmount: (() => void) | undefined;
afterEach(() => {
  act(() => unmount?.());
  unmount = undefined;
  document.body.replaceChildren();
  vi.unstubAllGlobals();
});

async function open(phone: boolean) {
  vi.stubGlobal("matchMedia", (query: string) => ({ matches: phone && query === "(max-width: 767px)", media: query, addEventListener() {}, removeEventListener() {} }));
  const root = document.createElement("div");
  document.body.append(root);
  const user = userEvent.setup();
  act(() => {
    unmount = mountArchive(root, { storage: memoryStorage(), languages: ["en"], catalog: { manifest: () => new Promise(() => undefined), category: async () => null } });
  });
  const trigger = await screen.findByRole("button", { name: "Display settings" });
  act(() => trigger.focus());
  await user.keyboard("{Enter}");
  return { user, trigger, settings: await screen.findByRole("group", { name: "Display settings" }) };
}

it("focuses the selected text size, follows the setting order and restores the trigger on Escape", async () => {
  const { user, trigger, settings } = await open(false);
  expect(settings.getAttribute("data-level")).toBe("menu");
  expect(settings.classList.contains("ui-glass")).toBe(true);
  expect(trigger.closest(".staffage-app")?.getAttribute("data-level")).toBe("base");
  const scoped = within(settings);
  const textSize = within(scoped.getByRole("radiogroup", { name: "Text size" }));
  expect(document.activeElement).toBe(textSize.getByRole("radio", { checked: true }));
  const inactive = textSize.getByRole("radio", { checked: false });
  expect(inactive.closest("[data-level]")).toBe(settings);
  expect(inactive.getAttribute("data-shared-border-cell")).toBe("");
  await user.tab();
  const motion = within(scoped.getByRole("radiogroup", { name: "Motion" }));
  expect(document.activeElement).toBe(motion.getByRole("radio", { checked: true }));
  await user.keyboard("{Escape}");
  expect(screen.queryByRole("group", { name: "Display settings" })).toBeNull();
  expect(document.activeElement).toBe(trigger);
});

it("focuses the first navigation link in the phone settings", async () => {
  const { settings } = await open(true);
  expect(document.activeElement).toBe(within(settings).getByRole("link", { name: "All assets" }));
});
