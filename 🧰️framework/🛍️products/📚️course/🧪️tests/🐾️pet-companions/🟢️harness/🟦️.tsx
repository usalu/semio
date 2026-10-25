/** 🟢️ The shared declarations of the 🐾️ pet companions suite tests. */
import { vi } from "vitest";

export function quiet(): readonly ReturnType<typeof vi.spyOn>[] {
  return (["log", "info", "warn", "error", "debug"] as const).map((level) => vi.spyOn(console, level).mockImplementation(() => undefined));
}

export function boxed(element: Element, left: number, top: number): void {
  element.getBoundingClientRect = () => ({ left, top, right: left + 16, bottom: top + 16, width: 16, height: 16, x: left, y: top, toJSON: () => ({}) });
}
