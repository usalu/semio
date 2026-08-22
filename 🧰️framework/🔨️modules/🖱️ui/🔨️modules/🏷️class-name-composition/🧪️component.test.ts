// #region 🧲️Header
// 💻️ framework/ui/modules/🏷️class-name-composition/component.test.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import { cn } from "./🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🧪️ClassNameComposition
describe("owned class-name composition", () => {
  it("recursively flattens inputs and suppresses falsey values", () => {
    expect(cn("relative", ["flex", [false, "h-full", null]], { "pointer-events-none": true, hidden: false }, 0, 2, 2n)).toBe("relative flex h-full pointer-events-none 2");
  });

  it.each([
    ["ui-surface", "ui-glass", "ui-glass"],
    ["ui-glass", "ui-veil", "ui-veil"],
    ["ui-veil", "bg-transparent", "bg-transparent"],
    ["bg-transparent", "ui-surface", "ui-surface"],
  ])("keeps only the last repository surface fill for %s then %s", (first, last, expected) => {
    expect(cn(first, last)).toBe(expected);
  });

  it.each([
    ["px-single px-tiny", "px-tiny"],
    ["p-double px-single px-tiny", "p-double px-tiny"],
    ["px-single p-double", "p-double"],
    ["h-medium h-large", "h-large"],
    ["flex-shrink-0 shrink-0", "shrink-0"],
    ["scroll-my-single scroll-my-double", "scroll-my-double"],
    ["border-normal border-accent", "border-accent"],
    ["rounded-sm rounded-md", "rounded-md"],
    ["text-xs text-element text-sm", "text-element text-sm"],
    ["hover:bg-hover-base hover:bg-active-base", "hover:bg-active-base"],
    ["data-[state=open]:border-normal data-[state=open]:border-accent", "data-[state=open]:border-accent"],
    ["w-auto !w-full", "w-auto !w-full"],
  ])("preserves the source-derived last-winner contract for %s", (classes, expected) => {
    expect(cn(classes)).toBe(expected);
  });

  it.each([
    ["aspect-square", "aspect-auto", "aspect-auto"],
    ["aspect-auto", "aspect-square", "aspect-square"],
  ])("keeps the last source-owned aspect utility for %s then %s", (first, last, expected) => {
    expect(cn(first, last)).toBe(expected);
  });

  it("preserves unclassified application classes", () => {
    expect(cn("introduction-demo-callout", "selection-marquee", "introduction-demo-callout")).toBe("introduction-demo-callout selection-marquee introduction-demo-callout");
  });
});
// #endregion 🧪️ClassNameComposition
