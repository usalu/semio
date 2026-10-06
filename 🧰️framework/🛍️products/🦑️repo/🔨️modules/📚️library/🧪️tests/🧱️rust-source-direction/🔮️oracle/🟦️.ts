import { dirname, join, relative, resolve } from "node:path";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
const library = resolve(import.meta.dir, "../../..");
const read = (path: string): any => JSON.parse(readFileSync(join(library, path), "utf8"));

/** 🏘️ Replays the actual policy loader and authored workspace contribution in a private native root. */
export function writeRustLayerOracle(cwd: string, files: Readonly<Record<string, string>>): void {
  const repo = resolve(library, "../../../../.."), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const write = (path: string, content: string): void => { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), content); };
  const boundary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs";
  write(boundary, readFileSync(join(repo, boundary), "utf8"));
  for (const path of ["🔣️taxonomy.json", "🕸️dependencies/🧭️direction/🟦️.ts", "🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs", "🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs", "🗂️workspaces/🟦️bun/🟦️.ts", "🗂️workspaces/🟦️bun/🟨️.cjs", "🗂️workspaces/📦️payload/🟦️.ts", "🗂️workspaces/📦️payload/🟨️.cjs"]) write(`${relative(repo, library)}/${path}`, readFileSync(join(library, path), "utf8"));
  write("nx.json", "{}");
  write("📋️project.json", JSON.stringify({ metadata: { semio: { taxonomy: `${relative(repo, library)}/🔣️taxonomy.json` } } }));
  const members = corpus.contribution.members as string[];
  write("package.json", JSON.stringify({ workspaces: members, semio: { workspace: { schemaVersion: 1, members, owners: [] } } }));
  for (const [path, source] of Object.entries(corpus.contribution.files)) write(path, source as string);
  for (const [path, source] of Object.entries(files)) write(path, source);
}

