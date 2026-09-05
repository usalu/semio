import { copyFileSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, extname, join } from "node:path";

const segmenter = new Intl.Segmenter(undefined, { granularity: "grapheme" });

/** 🔤️ Resolves semantic filenames to names accepted by TeX auxiliary-file handling. */
export function printCompilerName(name: string): string {
  const first = segmenter.segment(name)[Symbol.iterator]().next().value?.segment ?? "";
  return /\p{Extended_Pictographic}|\p{Regional_Indicator}/u.test(first) ? name.slice(first.length) : name;
}

/** 📄️ Stages compiler inputs and their references without modifying canonical source files. */
export function stagePrintSources(sourceRoot: string, entries: readonly string[], stageRoot: string): ReadonlyMap<string, string> {
  const files = new Map<string, string>();
  const destinations = new Set<string>();
  const names = new Map<string, string>();
  const visit = (relative: string): void => {
    const parts = relative.split(/[\\/]/);
    const staged = join(stageRoot, ...parts.map(printCompilerName));
    if (destinations.has(staged)) throw new Error(`colliding print source: ${relative}`);
    destinations.add(staged);
    for (const part of parts) if (printCompilerName(part) !== part) {
      names.set(part, printCompilerName(part));
      if (part.endsWith(".tex")) names.set(part.slice(0, -4), printCompilerName(part).slice(0, -4));
    }
    if (statSync(join(sourceRoot, relative)).isFile()) files.set(relative.replaceAll("\\", "/"), staged);
    else for (const entry of readdirSync(join(sourceRoot, relative))) {
      if ([".DS_Store", "dist", ".semio-dark"].includes(entry)) continue;
      visit(join(relative, entry));
    }
  };
  for (const entry of entries) visit(entry);
  const pattern = names.size ? new RegExp([...names.keys()].sort((a, b) => b.length - a.length).map((name) => name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|"), "g") : undefined;
  rmSync(stageRoot, { recursive: true, force: true });
  for (const [relative, staged] of files) {
    const source = join(sourceRoot, relative);
    mkdirSync(dirname(staged), { recursive: true });
    if (![".tex", ".bib"].includes(extname(source))) copyFileSync(source, staged);
    else {
      const content = readFileSync(source, "utf8");
      writeFileSync(staged, pattern ? content.replace(pattern, (name) => names.get(name)!) : content, "utf8");
    }
  }
  return files;
}
