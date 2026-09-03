import { describe, expect, it } from "vitest";
import { stripExecutableShebang } from "./🧪️tests/🧹️executable-source/🟦️.ts";

describe("executable source transformation", () => {
  it.each([
    ["#!/usr/bin/env bun\nexport const value = 1;\n", "export const value = 1;\n"],
    ["#!/usr/bin/env bun\r\nexport const value = 1;\r\n", "export const value = 1;\r\n"],
    ["export const value = '#!/usr/bin/env bun';\n", "export const value = '#!/usr/bin/env bun';\n"],
  ])("removes only a leading Bun shebang before Vite injects imports", (source, expected) => {
    expect(stripExecutableShebang(source)).toBe(expected);
  });
});
