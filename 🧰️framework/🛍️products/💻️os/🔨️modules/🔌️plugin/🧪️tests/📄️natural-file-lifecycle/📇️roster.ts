/** 📇️ Exact authored source roster, separate from canonical browser journeys and codec execution. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
const fixtures = join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle");
const roster = JSON.parse(readFileSync(join(fixtures, "📇️surfaces.json"), "utf8")) as { surfaces: { owner: string; editorApp: string; viewerApp: string; naturalFile: string }[] };
const lifecycle = JSON.parse(readFileSync(join(fixtures, "🔣️.json"), "utf8")) as { formats: { appId: string; formatKind: string; extension: string; mediaType: string; binary: boolean }[]; capabilityGaps?: { appId: string; reason: string }[] };
const authored = () => {
  const result: string[] = [];
  const artifacts = join(process.cwd(), "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
  for (const name of readdirSync(artifacts)) {
    const file = join(artifacts, name, "🧩️composition/🔣️.json");
    if (!existsSync(file)) continue;
    const source = JSON.parse(readFileSync(file, "utf8"));
    const editors = source.apps.filter((app: { role: string }) => app.role === "editor");
    const viewers = source.apps.filter((app: { role: string }) => app.role === "viewer");
    expect(editors.length).toBe(viewers.length);
    expect(source.playgrounds.length).toBe(editors.length);
    for (const entry of source.playgrounds) result.push(entry.app);
  }
  result.push("s.energy.model@1/*#editor");
  return result.sort();
};
test("validates language-neutral source surfaces using independent Ajv", () => {
  expect(new Ajv().validate(JSON.parse(readFileSync(join(fixtures, "📇️surfaces.schema.json"), "utf8")), roster)).toBe(true);
  expect(new Ajv().validate(JSON.parse(readFileSync(join(fixtures, "📄️lifecycle.schema.json"), "utf8")), lifecycle)).toBe(true);
});
test("matches every authored source editor and paired viewer exactly once", () => {
  expect(roster.surfaces.map(row => row.editorApp).sort()).toEqual(authored());
  expect(new Set(roster.surfaces.map(row => row.editorApp)).size).toBe(roster.surfaces.length);
  for (const row of roster.surfaces) expect(row.viewerApp).toBe(row.editorApp.replace(/#editor$/u, "#viewer"));
});
test("partitions source surfaces into exact native codec declarations or explicit required gaps", () => {
  const formats = lifecycle.formats.map(row => row.appId), gaps = (lifecycle.capabilityGaps ?? []).map(row => row.appId);
  expect(new Set([...formats, ...gaps]).size).toBe(formats.length + gaps.length);
  expect([...formats, ...gaps].sort()).toEqual(authored());
  expect(gaps.sort()).toEqual(roster.surfaces.filter(row => row.naturalFile === "missing").map(row => row.editorApp).sort());
});
test("requires natural file capability on every supported source editor", () => {
  expect((lifecycle.capabilityGaps ?? []).map(row => row.appId)).toEqual([]);
  expect(roster.surfaces.filter(row => row.naturalFile !== "declared")).toEqual([]);
});

test("matches Semio macro-authored native codec metadata without using stale descriptors", () => {
  const subsets = join(process.cwd(), "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
  const declared: string[] = [];
  for (const name of readdirSync(subsets)) {
    const editor = join(subsets, name, "✏️editor/🦀️.rs");
    if (!existsSync(editor)) continue;
    for (const match of readFileSync(editor, "utf8").matchAll(/semio_natural_file!\([^,]+,\s*"([^"]+)"\)/gu)) declared.push(match[1] + "@v1");
  }
  const formats = lifecycle.formats.filter(row => row.appId.startsWith("s.stdio.semio@"));
  expect(formats.map(row => row.formatKind).sort()).toEqual(declared.sort());
  for (const row of formats) expect({ extension: row.extension, mediaType: row.mediaType, binary: row.binary }).toEqual({ extension: ".semio", mediaType: "application/vnd.semio", binary: true });
});

test("keeps Rust named metadata batches equal to the neutral codec set", () => {
  const source = readFileSync(join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs"), "utf8");
  const ids = [...source.matchAll(/const BATCH_[A-Z]_NATURAL_FILE_APPS: &\[&str\] = &\[([\s\S]*?)\];/gu)].flatMap(match => [...match[1].matchAll(/"([^"]+)"/gu)].map(value => value[1]));
  expect(ids.sort()).toEqual(lifecycle.formats.map(row => row.appId).sort());
  expect((source.match(/BATCH_I_NATURAL_FILE_APPS/gu) ?? []).length).toBe(3);
});
