import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

export const PX_SCAN_ROOTS = ["🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react", "🧰️framework/🔨️modules/🖱️ui/🎨️styling", "🧰️framework/🛍️products/💻️os", "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev", "✏️s/🔌️plugins/🌊️flow", "✏️s/🔌️plugins/📐️cad", "✏️s/🔌️plugins/🧩️puzzle", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world", "✏️s/🔌️plugins/🌍️gis"] as const;

const PX_SCAN_SKIP = ["/.🧬semio/", "/node_modules/", "/.storybook/", "/fixture/", "tokens.generated.", "session.json", ".plan.md"];

const PX_PATTERNS: { name: string; re: RegExp }[] = [{ name: "tailwind-arbitrary-px", re: /\[(?!9999px)[-0-9]*\.?[0-9]+px\]/ }];

function isPxScanExemptLine(line: string): boolean {
  if (line.includes("--stroke-hairline: 1px")) {
    return true;
  }
  if (line.includes("expect(") || line.includes("toContain(")) {
    return true;
  }
  if (line.includes("@media") && line.includes("px")) {
    return true;
  }
  if (/\b(h|w|min-h|min-w|max-h|max-w)-px\b/.test(line)) {
    return true;
  }
  if (line.includes("rounded-[9999px]")) {
    return true;
  }
  if (line.includes("cursor:") && line.includes("url(")) {
    return true;
  }
  if (/font=["'`]\d/.test(line) && line.includes("px")) {
    return true;
  }
  if (line.includes("transform:") && line.includes("px")) {
    return true;
  }
  if (line.includes("translate3d") || line.includes("translate(")) {
    return true;
  }
  if (line.includes("patchAutoAnimate") || line.includes("innerHTML")) {
    return true;
  }
  return false;
}

function shouldPxScanFile(path: string): boolean {
  if (!/\.(tsx?|css)$/.test(path)) {
    return false;
  }
  return !PX_SCAN_SKIP.some((skip) => path.includes(skip));
}

export function collectPxViolations(repoRoot: string, roots: readonly string[] = PX_SCAN_ROOTS): { file: string; line: number; kind: string; text: string }[] {
  const violations: { file: string; line: number; kind: string; text: string }[] = [];
  const visitedDirectories = new Set<string>();

  const walk = (dir: string): void => {
    if (visitedDirectories.has(dir)) return;
    visitedDirectories.add(dir);
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === ".🧬semio" || entry.name === ".🧬semio") {
          continue;
        }
        walk(full);
        continue;
      }
      const rel = full.slice(repoRoot.length + 1);
      if (!shouldPxScanFile(rel)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (isPxScanExemptLine(line)) {
          continue;
        }
        for (const { name, re } of PX_PATTERNS) {
          if (re.test(line)) {
            violations.push({ file: rel, line: i + 1, kind: name, text: line.trim() });
            break;
          }
        }
      }
    }
  };

  for (const root of roots) {
    const abs = join(repoRoot, root);
    if (existsSync(abs)) {
      walk(abs);
    }
  }
  return violations;
}

export const COLOR_SCAN_ROOTS = [...PX_SCAN_ROOTS.filter((root) => root !== "🧰️framework/🔨️modules/🖱️ui/🎨️styling"), ".storybook"] as const;

/** @emoji 📋️ Files with pre-existing hardcoded-color usage surfaced by the full-palette/manual-dark-variant patterns
 * and the widened scan roots — tracked for follow-up migration, not fixed here. */
const COLOR_SCAN_LEGACY_ALLOWLIST = [
  "🧰️framework/🛍️products/💻️os/📖️stories/🧭️coordination/🟦️.tsx",
  ".storybook/preview.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/📖️stories/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/📖️stories/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/📖️stories/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/📖️stories/🧪️.story.tsx",
  "✏️s/🔌️plugins/📐️cad/📖️stories/🎭️renderer/🧪️.story.tsx",
  "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
];

const COLOR_SCAN_SKIP = [
  ...PX_SCAN_SKIP.filter((skip) => skip !== "/.storybook/"),
  "/dist/",
  "/.vite/",
  "/.stage/",
  "/renderer-modules/",
  "/🔌️plugin-modules/",
  "generated/",
  "/🔤️tokens/🦀️.rs",
  "/🔤️tokens/🐍️.py",
  "/🎨️palette/🎨️.css",
  ...COLOR_SCAN_LEGACY_ALLOWLIST,
];

const COLOR_PATTERNS: { name: string; re: RegExp }[] = [
  { name: "raw-hex-color", re: /#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?(?:[0-9a-fA-F]{2})?\b/ },
  { name: "raw-rgb-hsl-color", re: /\b(?:rgba?|hsla?)\(\s*[\d.]/ },
  {
    name: "tailwind-palette-color-class",
    re: /\b(?:bg|text|border|ring|fill|stroke|from|via|to|divide|outline|decoration|caret|accent|shadow)-(?:red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose|zinc|gray|slate|neutral|stone)-\d{2,3}\b/,
  },
  { name: "manual-dark-variant-palette", re: /\bdark:(?:bg|text|border|ring|fill|stroke)-/ },
];

function isColorScanExemptLine(line: string): boolean {
  if (line.includes("expect(") || line.includes("toContain(") || line.includes("toBe(")) {
    return true;
  }
  if (/^\s*(\/\/|\*|\/\*)/.test(line)) {
    return true;
  }
  return false;
}

function shouldColorScanFile(path: string): boolean {
  if (!/\.(tsx?|css|rs)$/.test(path)) {
    return false;
  }
  return !COLOR_SCAN_SKIP.some((skip) => path.includes(skip));
}

export function collectColorViolations(repoRootPath: string, roots: readonly string[] = COLOR_SCAN_ROOTS): { file: string; line: number; kind: string; text: string }[] {
  const violations: { file: string; line: number; kind: string; text: string }[] = [];
  const visitedDirectories = new Set<string>();

  const walk = (dir: string): void => {
    if (visitedDirectories.has(dir)) return;
    visitedDirectories.add(dir);
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === ".🧬semio" || entry.name === ".🧬semio") {
          continue;
        }
        walk(full);
        continue;
      }
      const rel = full.slice(repoRootPath.length + 1);
      if (!shouldColorScanFile(rel)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (isColorScanExemptLine(line)) {
          continue;
        }
        for (const { name, re } of COLOR_PATTERNS) {
          if (re.test(line)) {
            violations.push({ file: rel, line: i + 1, kind: name, text: line.trim() });
            break;
          }
        }
      }
    }
  };

  for (const root of roots) {
    const abs = join(repoRootPath, root);
    if (existsSync(abs)) {
      walk(abs);
    }
  }
  return violations;
}
