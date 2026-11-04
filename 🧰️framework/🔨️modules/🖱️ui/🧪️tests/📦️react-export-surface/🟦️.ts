/** 📦️ The React package surface (entry and generated `source` barrel) exports the panel fit and the context menu trigger. */
// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, it } from "vitest";
import * as entry from "@semio-tech/ui-react";

const source = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/📥️source/🟦️.ts"), "utf8");

it("exports panelFitV1 and ContextMenuTriggerButton from the entry and the source barrel", () => {
  expect(typeof entry.panelFitV1).toBe("function");
  expect(entry.panelFitV1(1280, 600)).toEqual({ widthPx: 576, collapsed: false });
  expect(typeof entry.ContextMenuTriggerButton).toBe("function");
  for (const name of ["panelFitV1", "ContextMenuTriggerButton"]) expect(source, name).toMatch(new RegExp(`\\b${name}\\b`));
});
