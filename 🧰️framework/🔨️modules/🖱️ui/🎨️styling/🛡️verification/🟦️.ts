import { lstatSync, readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import schema from "./🧬️schema/🔣️.json";
import { stylingSourceDataV1 } from "./📖️source-data/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";

export type StylingScanScopeV1 = Readonly<{ roots: readonly string[] }>;
export type StylingSourceManifestV1 = StylingScanScopeV1 & Readonly<{ files: readonly string[] }>;
export type StylingSourceV1 = StylingSourceManifestV1 & Readonly<{ readText(path: string): string }>;
export type StylingViolationKindV1 = "tailwind-arbitrary-px" | "raw-hex-color" | "raw-rgb-hsl-color" | "tailwind-palette-color-class" | "manual-dark-variant-palette";
export type StylingViolationV1 = Readonly<{ file: string; line: number; kind: StylingViolationKindV1; text: string }>;
const OPAQUE_DIRECTORIES = new Set(["node_modules", ".🧬semio", ".git", "dist", "target", ".vite", ".stage", "🤖️generated", "🧪️tests", "🧫️fixtures"]);
const PX_PATTERNS: { name: StylingViolationKindV1; re: RegExp }[] = [
  { name: "tailwind-arbitrary-px", re: /\[[-0-9]*\.?[0-9]+px\]/ },
];
const COLOR_PATTERNS: { name: StylingViolationKindV1; re: RegExp }[] = [
  { name: "raw-hex-color", re: /(?<![&\w])#(?:[0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{4}|[0-9a-fA-F]{3})(?!\w)/ },
  { name: "raw-rgb-hsl-color", re: /\b(?:rgba?|hsla?)\(\s*[\d.]/ },
  { name: "tailwind-palette-color-class", re: /\b(?:bg|text|border|ring|fill|stroke|from|via|to|divide|outline|decoration|caret|accent|shadow)-(?:red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose|zinc|gray|slate|neutral|stone)-\d{2,3}\b/ },
  { name: "manual-dark-variant-palette", re: /\bdark:(?:bg|text|border|ring|fill|stroke)-/ },
];

/** 📍️Admits only explicit, bounded, portable styling owner roots. */
export function admitStylingScanScopeV1(input: unknown): StylingScanScopeV1 {
  if (validateJsonSchemaSubset(schema.$defs.ScopeV1, input, schema).length) throw Error("Invalid styling scan scope");
  const roots = [...new Set((input as StylingScanScopeV1).roots)];
  if (roots.some(path => path.normalize("NFC") !== path) || new Set(roots.map(path => path.toLowerCase())).size !== roots.length) throw Error("Ambiguous styling owner identity");
  return { roots };
}

/** 🗂️Admits bounded source identities and refuses portable filesystem ambiguity. */
export function admitStylingSourceManifestV1(input: unknown): StylingSourceManifestV1 {
  if (validateJsonSchemaSubset(schema.$defs.SourceManifestV1, input, schema).length) throw Error("Invalid styling source manifest");
  const { roots } = admitStylingScanScopeV1({ roots: (input as StylingSourceManifestV1).roots });
  const files = [...new Set((input as StylingSourceManifestV1).files)];
  if (files.some(path => path.normalize("NFC") !== path) || new Set(files.map(path => path.toLowerCase())).size !== files.length) throw Error("Ambiguous styling source identity");
  return { roots, files };
}

/** 🔍️Checks supplied source files without discovering or importing concrete owners. */
export function collectStylingViolationsV1(source: StylingSourceV1, kind: "px" | "color"): readonly StylingViolationV1[] {
  const { roots, files } = admitStylingSourceManifestV1({ roots: source.roots, files: source.files });
  if (typeof source.readText !== "function") throw Error("Styling source has no text reader");
  if (kind !== "px" && kind !== "color") throw Error("Invalid styling rule");
  const violations: StylingViolationV1[] = [];
  for (const file of files) {
    const sourcePath = kind === "px" ? schema.$defs.ApplicationPathV1 : schema.$defs.ColorApplicationPathV1;
    if (validateJsonSchemaSubset(sourcePath, file, schema).length || !roots.some(root => file.startsWith(root + "/")) || !(kind === "px" ? /\.(tsx?|css)$/.test(file) : /\.(tsx?|css|rs)$/.test(file))) continue;
    const text = source.readText(file), lines = text.split("\n");
    const data = stylingSourceDataV1(text, file.endsWith(".css") ? "css" : file.endsWith(".rs") ? "rust" : "typescript").split("\n");
    for (let index = 0; index < lines.length; index++) {
      const line = lines[index]!;
      const pattern = (kind === "px" ? PX_PATTERNS : COLOR_PATTERNS).find(pattern => pattern.re.test(data[index]!));
      if (pattern) violations.push({ file, line: index + 1, kind: pattern.name, text: line.trim() });
    }
  }
  return violations;
}

function filesystemSourceV1(repoRoot: string, roots: readonly string[]): StylingSourceV1 {
  const scope = admitStylingScanScopeV1({ roots });
  const files: string[] = [], visited = new Set<string>();
  const walk = (path: string): void => {
    if (visited.has(path)) return;
    visited.add(path);
    for (const entry of readdirSync(join(repoRoot, path), { withFileTypes: true })) {
      if (OPAQUE_DIRECTORIES.has(entry.name)) continue;
      const file = relative(repoRoot, join(repoRoot, path, entry.name)).replaceAll("\\", "/");
      if (entry.isDirectory()) walk(file);
      else if (entry.isFile() && /\.(tsx?|css|rs)$/u.test(entry.name)) files.push(file);
      else if (entry.isSymbolicLink() && /\.(tsx?|css|rs)$/u.test(entry.name)) throw Error("Styling source is a symbolic link: " + file);
    }
  };
  for (const root of scope.roots) {
    const parts = root.split("/");
    for (let index = 1; index <= parts.length; index++) if (!lstatSync(join(repoRoot, ...parts.slice(0, index))).isDirectory()) throw Error("Styling owner is not a regular directory: " + root);
    walk(root);
  }
  return { roots: scope.roots, files, readText: path => readFileSync(join(repoRoot, path), "utf8") };
}

/** 📏️Checks pixel sizing within the caller's present source owners. */
export function collectPxViolations(repoRoot: string, roots: readonly string[]): readonly StylingViolationV1[] {
  return collectStylingViolationsV1(filesystemSourceV1(repoRoot, roots), "px");
}

/** 🎨️Checks semantic colors within the caller's present source owners. */
export function collectColorViolations(repoRoot: string, roots: readonly string[]): readonly StylingViolationV1[] {
  return collectStylingViolationsV1(filesystemSourceV1(repoRoot, roots), "color");
}
