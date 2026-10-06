//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧮️ Source-census laws: the placeholder, interactive-job, docstring-opener, docstring-emoji-reuse, debug-tag and interface-import scanners against the
// language-neutral fixture. The live gates (`workspace:verify -- production-placeholders`, `-- interactivity commands`) additionally cross-check every
// tracked Rust source against `git grep`, the third-party oracle; the docstring-emoji-reuse cases are cross-checked against tree-sitter (Rust,
// TypeScript, TSX) here.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Parser from "web-tree-sitter";
import { FRAMEWORK_ARTIFACT_NAMES, TOOL_MACHINE_VOCABULARY, artifactNameHitsOfText, debugTagHitsOfText, docstringEmojiReuseOfText, docstringHitsOfText, interactiveJobsOfText, interfaceImportHitsOfText, placeholderHitsOfText, utilityArtifactsOfSources } from "../../📋️orchestration/🟦️.ts";
//#endregion 🔌️Adapters

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "🧮️source-census", "🔣️.json"), "utf8")) as {
  placeholders: { name: string; path: string; text: string; expected: { line: number; macro: "todo" | "unimplemented"; testOnly: boolean; commented: boolean }[] }[];
  interactiveJobs: { name: string; path: string; text: string; expected: { command: string; classification: string; line: number }[]; calls: number; codeCalls: number }[];
  docstrings: { name: string; path: string; text: string; expected: { line: number; rule: "at-emoji" | "no-emoji" }[] }[];
  docstringEmojiReuse: { name: string; path: string; text: string; expected: { emoji: string; lines: number[] }[] }[];
  debugTags: { name: string; path: string; text: string; expected: number[] }[];
  interfaceImports: { name: string; path: string; text: string; expected: number[] }[];
  artifactNames: { name: string; path: string; text: string; expected: { line: number; name: string }[] }[];
  utilityMachines: { name: string; sources: { path: string; text: string }[]; expected: { artifact: string; utilities: number; machine: string[] }[]; offenders: string[] }[];
};
const TEST_SEGMENT = /(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u;
const MARKER = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)/u;
const GRAPHEMES = new Intl.Segmenter("en", { granularity: "grapheme" });

await Parser.init();
const grammars = join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", ".."))), "out");
const parsers = new Map(await Promise.all((["rust", "typescript", "tsx"] as const).map(async (name) => {
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(grammars, `tree-sitter-${name}.wasm`)));
  return [name, parser] as const;
})));

/** 🌳️ Every node of `node`'s subtree, `node` first. */
function descendants(node: Parser.SyntaxNode): Parser.SyntaxNode[] {
  return [node, ...node.children.flatMap(descendants)];
}

/** 🔮️ The independent oracle: each marker opening more than one docstring of a source, read from the tree-sitter syntax tree — a Rust
 * `///` or `//!` comment run on consecutive lines of one kind opens at its first non-empty line, a `/** … *\/` comment (Rust or TypeScript) at its first
 * non-empty content line — so text inside strings never opens one. */
function oracleEmojiReuse(path: string, text: string): { emoji: string; lines: number[] }[] {
  const parser = parsers.get(path.endsWith(".rs") ? "rust" : path.endsWith(".tsx") ? "tsx" : "typescript")!;
  const comments = descendants(parser.parse(text).rootNode).filter((node) => /comment/u.test(node.type)).sort((left, right) => left.startIndex - right.startIndex);
  const openers: { line: number; content: string }[] = [];
  let run: { row: number; kind: string } | null = null;
  let opened = false;
  for (const node of comments) {
    if (path.endsWith(".rs") && /^\/\/[/!](?!\/)/u.test(node.text)) {
      if (run === null || node.startPosition.row !== run.row + 1 || run.kind !== node.text.slice(0, 3)) opened = false;
      run = { row: node.startPosition.row, kind: node.text.slice(0, 3) };
      const content = node.text.slice(3).trim();
      if (!opened && content.length > 0) {
        openers.push({ line: node.startPosition.row + 1, content });
        opened = true;
      }
      continue;
    }
    run = null;
    if (!node.text.startsWith("/**") || node.text.startsWith("/**/") || node.text.startsWith("/***")) continue;
    const lines = node.text.slice(3, -2).split("\n").map((line) => line.replace(/^\s*\*?/u, "").trim());
    const index = lines.findIndex((line) => line.length > 0);
    if (index >= 0) openers.push({ line: node.startPosition.row + 1 + index, content: lines[index]! });
  }
  const groups = new Map<string, number[]>();
  for (const { line, content } of openers.filter((opener) => MARKER.test(opener.content))) {
    const emoji = GRAPHEMES.segment(content)[Symbol.iterator]().next().value!.segment.replaceAll("\uFE0F", "");
    groups.set(emoji, [...(groups.get(emoji) ?? []), line]);
  }
  return [...groups].filter(([, lines]) => lines.length > 1).map(([emoji, lines]) => ({ emoji, lines }));
}

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

describe("docstring emoji reuse", () => {
  for (const entry of fixture.docstringEmojiReuse) {
    test(entry.name, () => {
      expect(docstringEmojiReuseOfText(entry.path, entry.text).map(({ emoji, lines }) => ({ emoji, lines }))).toEqual(entry.expected);
      expect(oracleEmojiReuse(entry.path, entry.text)).toEqual(entry.expected);
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

/** 🧽️ The independent oracle of the artifact-neutrality rule (design §22.31 c): every line of a source that carries an artifact
 * name outside the doc comments tree-sitter finds — a comment node that opens `///`, `//!` or `/*!` (Rust) or `/**` (Rust and
 * TypeScript; `////`, `/***` and `/**\/` are plain comments) is blanked, every other node, strings and plain comments included, is read. */
function oracleArtifactNames(path: string, text: string): { line: number; name: string }[] {
  const rust = path.endsWith(".rs");
  const parser = parsers.get(rust ? "rust" : path.endsWith(".tsx") ? "tsx" : "typescript")!;
  const documented = descendants(parser.parse(text).rootNode).filter((node) => /comment/u.test(node.type) && ((rust && (/^\/\/[/!](?!\/)/u.test(node.text) || node.text.startsWith("/*!"))) || (node.text.startsWith("/**") && !node.text.startsWith("/***") && !node.text.startsWith("/**/"))));
  const units = text.split("");
  for (const node of documented) for (let index = node.startIndex; index < node.endIndex; index += 1) if (units[index] !== "\n") units[index] = " ";
  const pattern = new RegExp(FRAMEWORK_ARTIFACT_NAMES.join("|"), "iu");
  return units.join("").split("\n").flatMap((line, index) => {
    const found = pattern.exec(line);
    return found === null ? [] : [{ line: index + 1, name: found[0].toLowerCase() }];
  });
}

/** 🎻️ The independent oracle of the utility-machine rule (design §22.32 d): per artifact tree, how many identifier nodes of its
 * production Rust sources (tree-sitter; test paths skipped, comments never hold identifiers) are `UtilityDefinition`, and which
 * machine vocabulary words are among them. */
function oracleUtilityArtifacts(sources: readonly { path: string; text: string }[]): { artifact: string; utilities: number; machine: string[] }[] {
  const found = new Map<string, { utilities: number; machine: Set<string> }>();
  for (const { path, text } of sources) {
    const segments = path.split("/");
    const at = segments.indexOf("🗿️artifacts");
    if (at < 0 || at + 2 >= segments.length || TEST_SEGMENT.test(path)) continue;
    const artifact = segments.slice(0, at + 2).join("/");
    const row = found.get(artifact) ?? { utilities: 0, machine: new Set<string>() };
    found.set(artifact, row);
    for (const node of descendants(parsers.get("rust")!.parse(text).rootNode).filter((candidate) => candidate.type === "identifier" || candidate.type === "type_identifier")) {
      if (node.text === "UtilityDefinition") row.utilities += 1;
      if (TOOL_MACHINE_VOCABULARY.includes(node.text)) row.machine.add(node.text);
    }
  }
  return [...found].filter(([, row]) => row.utilities > 0).map(([artifact, row]) => ({ artifact, utilities: row.utilities, machine: [...row.machine].sort() })).sort((left, right) => left.artifact.localeCompare(right.artifact));
}

describe("artifact names in framework code", () => {
  for (const entry of fixture.artifactNames) {
    test(entry.name, () => {
      expect(artifactNameHitsOfText(entry.path, entry.text).map(({ line, name }) => ({ line, name }))).toEqual(entry.expected);
      expect(oracleArtifactNames(entry.path, entry.text)).toEqual(entry.expected);
    });
  }
});

describe("utility artifacts and their machine vocabulary", () => {
  for (const entry of fixture.utilityMachines) {
    test(entry.name, () => {
      const artifacts = utilityArtifactsOfSources(entry.sources);
      expect(artifacts.map((artifact) => ({ ...artifact, machine: [...artifact.machine] }))).toEqual(entry.expected);
      expect(oracleUtilityArtifacts(entry.sources)).toEqual(entry.expected);
      expect(artifacts.filter((artifact) => artifact.machine.length === 0).map((artifact) => artifact.artifact)).toEqual(entry.offenders);
    });
  }
  test("every machine vocabulary word is a public item of the framework tool-machine module", () => {
    const module = readFileSync(join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🔨️modules", "🛠️tool-machine", "🦀️.rs"), "utf8");
    for (const word of TOOL_MACHINE_VOCABULARY) expect(new RegExp(`^pub (?:trait|struct|fn) ${word}\\b`, "mu").test(module), word).toBe(true);
  });
});
