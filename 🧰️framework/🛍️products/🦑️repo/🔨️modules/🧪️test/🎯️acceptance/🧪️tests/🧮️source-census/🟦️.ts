//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧮️ Source-census laws: the placeholder, interactive-job, docstring-opener, debug-tag and interface-import scanners against the language-neutral fixture. The live
// gates (`workspace:verify -- production-placeholders`, `-- interactivity commands`) additionally cross-check every tracked
// Rust source against `git grep`, the third-party oracle.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { debugTagHitsOfText, docstringHitsOfText, interactiveJobsOfText, interfaceImportHitsOfText, placeholderHitsOfText } from "../../📋️orchestration/🟦️.ts";
//#endregion 🔌️Adapters

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "🧮️source-census", "🔣️.json"), "utf8")) as {
  placeholders: { name: string; path: string; text: string; expected: { line: number; macro: "todo" | "unimplemented"; testOnly: boolean; commented: boolean }[] }[];
  interactiveJobs: { name: string; path: string; text: string; expected: { command: string; classification: string; line: number }[]; calls: number; codeCalls: number }[];
  docstrings: { name: string; path: string; text: string; expected: { line: number; rule: "at-emoji" | "no-emoji" }[] }[];
  debugTags: { name: string; path: string; text: string; expected: number[] }[];
  interfaceImports: { name: string; path: string; text: string; expected: number[] }[];
};
const TEST_SEGMENT = /(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u;

describe("production placeholders", () => {
  for (const entry of fixture.placeholders) {
    test(entry.name, () => {
      const hits = placeholderHitsOfText(entry.path, entry.text, TEST_SEGMENT.test(entry.path)).map(({ line, macro, testOnly, commented }) => ({ line, macro, testOnly, commented }));
      expect(hits).toEqual(entry.expected);
    });
  }
});

describe("interactive-job declarations", () => {
  for (const entry of fixture.interactiveJobs) {
    test(entry.name, () => {
      const found = interactiveJobsOfText(entry.path, entry.text, false);
      expect(found.declarations.map(({ command, classification, line }) => ({ command, classification, line }))).toEqual(entry.expected);
      expect(found.calls).toBe(entry.calls);
      expect(found.codeCalls).toBe(entry.codeCalls);
    });
  }
});

describe("docstring openers", () => {
  for (const entry of fixture.docstrings) {
    test(entry.name, () => {
      expect(docstringHitsOfText(entry.path, entry.text).map(({ line, rule }) => ({ line, rule }))).toEqual(entry.expected);
    });
  }
});

describe("debug tags", () => {
  for (const entry of fixture.debugTags) {
    test(entry.name, () => {
      expect(debugTagHitsOfText(entry.path, entry.text).map(({ line }) => line)).toEqual(entry.expected);
    });
  }
});

describe("interface-owned imports", () => {
  for (const entry of fixture.interfaceImports) {
    test(entry.name, () => {
      expect(interfaceImportHitsOfText(entry.path, entry.text).map(({ line }) => line)).toEqual(entry.expected);
    });
  }
});
