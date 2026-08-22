// #region 🧲️Header
// 💻️ framework/ui/modules/🏷️style-variants/component.test.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import { styleVariants, type StyleVariantProps } from "./🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🧪️StyleVariantCompilation
describe("owned style variants", () => {
  it("preserves base-only and caller-supplied class composition", () => {
    const compile = styleVariants("base");
    expect(compile()).toBe("base");
    expect(compile({ class: ["class", false], className: "class-name" })).toBe("base class class-name");
  });

  it.each([
    [undefined, "control default"],
    [{}, "control default"],
    [{ variant: undefined }, "control default"],
    [{ variant: null }, "control"],
    [{ variant: "default" as const }, "control default"],
    [{ variant: "ghost" as const }, "control ghost"],
    [{ variant: "outline" as const }, "control outline"],
  ])("covers the complete current button variant matrix for %j", (selection, expected) => {
    const compile = styleVariants("control", {
      variants: {
        variant: {
          default: "default",
          ghost: "ghost",
          outline: "outline",
        },
      },
      defaultVariants: { variant: "default" },
    });
    expect(compile(selection)).toBe(expected);
  });

  it.each([
    [undefined, "base sm off sm-off any-size"],
    [{ size: "sm" as const, active: true }, "base sm on sm-on any-size caller"],
    [{ size: "lg" as const, active: false }, "base lg off large any-size"],
    [{ size: "lg" as const, active: true }, "base lg on large any-size"],
    [{ size: null, active: true }, "base on"],
  ])("selects defaults, boolean choices, and compound conjunctions for %j", (selection, expected) => {
    const compile = styleVariants("base", {
      variants: {
        size: { sm: "sm", lg: "lg" },
        active: { true: "on", false: "off" },
      },
      defaultVariants: { size: "sm", active: false },
      compoundVariants: [
        { size: "sm", active: false, class: "sm-off" },
        { size: "sm", active: true, className: "sm-on" },
        { size: "lg", class: "large" },
        { size: ["sm", "lg"], class: "any-size" },
      ],
    });
    expect(compile(selection === undefined ? undefined : { ...selection, ...(selection.size === "sm" && selection.active === true ? { className: "caller" } : {}) })).toBe(expected);
  });

  it("exposes only declared selections through StyleVariantProps", () => {
    const compile = styleVariants("base", { variants: { tone: { quiet: "quiet", loud: "loud" } } });
    const selection: StyleVariantProps<typeof compile> = { tone: "quiet" };
    expect(compile(selection)).toBe("base quiet");
  });
});
// #endregion 🧪️StyleVariantCompilation
