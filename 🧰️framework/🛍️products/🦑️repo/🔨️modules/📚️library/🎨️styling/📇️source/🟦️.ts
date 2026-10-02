import { spawnSync } from "node:child_process";
import { discoverCatalogPackages, loadCatalogTaxonomy, registryCatalogInputView } from "../../🔍️discovery/🟦️.ts";
import { admitStylingScanScopeV1, type StylingSourceV1 } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/🛡️verification/🟦️.ts";

/** 📇️Supplies explicit current source owners and read-only indexed source candidates. */
export function loadWorkspaceStylingSourceV1(repoRoot: string): StylingSourceV1 {
  const taxonomy = loadCatalogTaxonomy(), view = registryCatalogInputView(repoRoot, taxonomy);
  const owners = [...new Set(discoverCatalogPackages(repoRoot, taxonomy, view).map(owner => owner.ownerRel).filter(Boolean))].sort((left, right) => left.length - right.length || left.localeCompare(right));
  if (view.kind(".storybook") === "directory") owners.push(".storybook");
  const roots = admitStylingScanScopeV1({ roots: owners.filter((root, index) => !owners.slice(0, index).some(parent => root.startsWith(`${parent}/`))) }).roots;
  if (roots.length === 0) return { roots, files: [], readText: () => { throw Error("No styling source owners"); } };
  const patterns = roots.flatMap(root => ["ts", "tsx", "css", "rs"].map(extension => `:(glob)${root}/**/*.${extension}`));
  const candidates = "\\[[-0-9]*\\.?[0-9]+px\\]|#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{4}|[0-9a-fA-F]{3})([^a-zA-Z0-9_]|$)|(rgb|rgba|hsl|hsla)\\([[:space:]]*[0-9.]|dark:(bg|text|border|ring|fill|stroke)-|(bg|text|border|ring|fill|stroke|from|via|to|divide|outline|decoration|caret|accent|shadow)-(red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose|zinc|gray|slate|neutral|stone)-[0-9]{2,3}|\\\\(x[0-9a-fA-F]{2}|u[0-9a-fA-F]{4}|u\\{[0-9a-fA-F]+\\})";
  const files = spawnSync("git", ["-c", "core.quotePath=false", "grep", "-l", "-z", "--untracked", "-E", candidates, "--", ...patterns], { cwd: repoRoot, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (files.status !== 0 && files.status !== 1) throw Error(`Styling source census failed (${files.status}): ${files.stderr}`);
  return { roots, files: files.stdout.split("\0").filter(Boolean), readText: path => view.readText(path) };
}
