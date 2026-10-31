import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { cross } from "d3-array";
import Ajv2020 from "ajv/dist/2020";
import { evaluate } from "mathjs";
import { defineTestAdapter } from "@semio-tech/framework/test/adapter";

const fixture: any = JSON.parse(readFileSync(fileURLToPath(new URL("./🧫️cases.json", import.meta.url)), "utf8"));
const workspaceRoot = new URL("../../../../../", import.meta.url);
const sourceRoot = new URL("../", import.meta.url);
const read = (name: string): string => readFileSync(fileURLToPath(new URL(name, sourceRoot)), "utf8");
const modules = [["middle", "Mittelmodul"], ["corner", "Eckmodul"]] as const;

/** 🧮 Derives group composition and per-person workload from the fixture or with mathjs. */
function composition(useOracle: boolean): unknown {
  const { members, perMember } = fixture.group;
  const per = (position: "corner" | "middle"): number => useOracle ? evaluate("members * count", { members, count: perMember[position] }) : members * perMember[position];
  const variants = fixture.variants.length, ownTypes = perMember.corner + perMember.middle;
  return {
    corner: per("corner"), middle: per("middle"), total: per("corner") + per("middle"),
    facadeSides: useOracle ? evaluate("c * sc + m * sm", { c: per("corner"), sc: fixture.facadeSides.corner, m: per("middle"), sm: fixture.facadeSides.middle }) : per("corner") * fixture.facadeSides.corner + per("middle") * fixture.facadeSides.middle,
    axonometriesPerMember: useOracle ? evaluate("t * v * m", { t: ownTypes, v: variants, m: fixture.submission.imageModels.length }) : ownTypes * variants * fixture.submission.imageModels.length,
    baseRoomAreaMm2: useOracle ? evaluate("w * d", { w: fixture.grid.widthMm, d: fixture.grid.depthMm }) : fixture.grid.widthMm * fixture.grid.depthMm,
  };
}

/** 🔤 Decodes example module names by splitting or by an independent D3 enumeration of the naming scheme. */
function naming(useOracle: boolean): unknown[] {
  const { position, kind, format } = fixture.scheme;
  if (!useOracle) return fixture.example.map(({ name }: { name: string }) => {
    const [p, k, windows, formats] = name.split("-");
    return { name, position: position[p!], kind: kind[k!], windows: Number(windows), formats: [...formats!].map(code => format[code]) };
  });
  const formatCodes = Object.keys(format), sequences = [...formatCodes, ...cross(formatCodes, formatCodes).filter(([a, b]) => a !== b).map(([a, b]) => a + b)];
  const catalog = new Map(cross(Object.keys(position), Object.keys(kind), [1, 2, 3, 4], sequences).map(([p, k, w, f]) => [[p, k, w, f].join("-"), { position: position[p], kind: kind[k], windows: w, formats: [...f].map(code => format[code]) }]));
  return fixture.example.map(({ name }: { name: string }) => ({ name, ...catalog.get(name)! }));
}

/** 📦 Expands the individual submission into its file count and variant file names. */
function submission(useOracle: boolean): unknown {
  const rows = fixture.submission.rows as { format: string; count: number; files: string[] }[];
  const names = useOracle
    ? cross(fixture.submission.imageModels as string[], modules.map(([, label]) => label), fixture.variants as number[]).map(([model, label, variant]) => `Nachname_${label}${model}_Variante${variant}.png`)
    : fixture.submission.imageModels.flatMap((model: string) => modules.flatMap(([, label]) => fixture.variants.map((variant: number) => `Nachname_${label}${model}_Variante${variant}.png`)));
  return {
    files: useOracle ? evaluate("sum(counts)", { counts: rows.map(row => row.count) }) : rows.reduce((sum, row) => sum + row.count, 0),
    images: names,
    ranges: rows.filter(row => row.format === "PNG").map(row => row.files),
  };
}

/** 🌐 Verifies both languages present the same assignment with the same tables, files and figures. */
function languages(): unknown {
  const shapes = fixture.languages.map((language: "en" | "de") => {
    assert.equal(read(language + ".tex"), `\\documentclass[theme=light,language=${language}]{assignment}\n\\input{${language}.content.tex}\n`);
    const content = read(language + ".content.tex"), contract = fixture.content[language];
    assert.equal(content.match(/\\title\{([^}]+)\}/)?.[1], contract.title);
    assert.deepEqual([...content.matchAll(/\\footerlogos\{([^}]+)\}/g)].map(match => match[1]), fixture.footerLogos);
    assert.ok(content.indexOf("\\footerlogos{") < content.indexOf("\\begin{document}") && content.includes("\\begin{document}\n\\makeassignmentcover\n") && !content.includes("\\maketitle"));
    const setup = content.match(/\\SemioAssignmentSetup\{(.*)\}\n/)?.[1] ?? "";
    assert.ok(setup.includes("cover-image={" + fixture.cover.image + "}") && setup.includes("logo={" + fixture.cover.logo + "}"));
    assert.equal((content.match(/\\SemioAssignmentCoverFact\{/g) ?? []).length, fixture.cover.facts);
    assert.deepEqual([...content.matchAll(/\\SemioAssignmentCoverPartner\{(\w+)\}/g)].map(match => match[1]), fixture.cover.partners);
    assert.deepEqual([...content.matchAll(/\\(?:sub)?section\{([^}]+)\}/g)].map(match => match[1]), contract.headings);
    const tables = [...content.matchAll(/\\begin\{Table\}\[title=([^,\]]+)[^\]]*\]\s*\\begin\{SemioTable\}\{([^}]+)\}([\s\S]*?)\\end\{SemioTable\}\s*\\end\{Table\}/g)];
    assert.deepEqual(tables.map(table => table[1]), contract.tables);
    for (const table of tables) assert.ok(Math.abs(table[2]!.split(",").reduce((sum, value) => sum + Number(value), 0) - 1) < 1e-9);
    assert.ok(content.includes(contract.date));
    const monospace = new Set([...content.matchAll(/\\texttt\{([^}]+)\}/g)].map(match => match[1]!.replaceAll("\\_", "_")));
    for (const file of [fixture.submission.group, fixture.submission.archive, fixture.submission.example, ...fixture.submission.rows.flatMap((row: { files: string[] }) => row.files), ...fixture.example.map((row: { name: string }) => row.name)]) assert.ok(monospace.has(file), language + " " + file);
    const images = [...content.matchAll(/\\AssignmentContrastImage\[[^\]]*\]\{([^}]+)\}/g)].map(match => match[1]);
    assert.deepEqual(images, fixture.example.map((row: { image: string }) => row.image));
    const workflow = [...tables[0]![3]!.matchAll(/\\SemioTableRow\{([A-Z]) & (dMA|NGS) &/g)].map(match => match[1]! + match[2]!);
    assert.deepEqual(workflow, fixture.parts.map((part: { id: string; subject: string }) => part.id + part.subject));
    assert.equal(/\\(?:qty|SI|enquote)\b/.test(content), false);
    return tables.map(table => (table[3]!.match(/\\SemioTable(?:Header)?Row\{/g) ?? []).length);
  });
  assert.deepEqual(shapes[0], shapes[1]);
  assert.deepEqual(shapes[0], fixture.tableRows);
  return { rows: shapes[0] };
}

/** 🏛 Verifies the authored leaf, its course publication and its build targets. */
export function verifyTypologyOwnership(): unknown {
  const contract = fixture.ownership, leaf = new URL(contract.sourceRoot + "/", workspaceRoot);
  assert.deepEqual(readdirSync(leaf).filter(name => !name.startsWith("🧪️")).sort(), [...contract.files].sort());
  assert.deepEqual([...readFileSync(new URL(fixture.cover.image, leaf)).subarray(0, 3)], [255, 216, 255]);
  assert.ok(readFileSync(new URL(fixture.cover.image, leaf)).length < 512 * 1024);
  for (const image of [...fixture.example.map((row: { image: string }) => row.image), fixture.cover.logo, fixture.cover.logo.replace(".png", "-dark.png")]) {
    const bytes = readFileSync(new URL(image, leaf));
    assert.deepEqual([...bytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10]);
    assert.ok(bytes.length < 512 * 1024 && bytes.readUInt32BE(16) <= 1024);
  }
  const course = JSON.parse(readFileSync(new URL("🎓️teaching/🏛️architecture/📚️course/🔣️.json", workspaceRoot), "utf8"));
  const assignment = course.assignments.find((entry: { id: string }) => entry.id === contract.assignment);
  assert.equal(assignment.topic, contract.topic);
  assert.equal(course.topics.filter((entry: { id: string }) => entry.id === contract.topic).length, 1);
  assert.deepEqual(assignment.documents, contract.documents);
  const project = JSON.parse(readFileSync(new URL("🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json", workspaceRoot), "utf8"));
  for (const language of fixture.languages) {
    const producer = contract.documents[language].producer, target = producer.split(":")[1];
    assert.equal(project.targets[target].options.command, "repo ./📜️script.ts " + target);
    for (const consumer of ["dev", "dev-site", "build"]) assert.ok(project.targets[consumer].dependsOn.includes(producer));
  }
  assert.ok(project.namedInputs.assignment.includes("{workspaceRoot}/" + contract.sourceRoot + "/**/*"));
  return { sourceRoot: contract.sourceRoot, files: contract.files, assignment: assignment.id, topic: assignment.topic, documents: assignment.documents };
}

/** 🧪 Runs semantic and source contracts of the typology assignment without starting a TeX compiler. */
export function verifyTypologyAssignmentContentV1(): void {
  const validate = new Ajv2020({ allErrors: true }).compile<typeof fixture>(JSON.parse(readFileSync(new URL("./🧬️cases.schema.json", import.meta.url), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  for (const mutate of [
    (entry: any) => { entry.version = 2; },
    (entry: any) => { entry.languages = ["en"]; },
    (entry: any) => { entry.parts[0].subject = "BIM"; },
    (entry: any) => { entry.example[0].name = "mb1q"; },
    (entry: any) => { entry.submission.rows[0].format = "JPG"; },
    (entry: any) => { entry.ownership.topic = "energy"; },
    (entry: any) => { entry.ownership.documents.en.namespace = "print"; },
    (entry: any) => { delete entry.content.de; },
  ]) {
    const invalid = structuredClone(fixture);
    mutate(invalid);
    assert.equal(validate(invalid), false);
  }
  assert.deepEqual(composition(false), composition(true));
  const counts = composition(false) as Record<string, number>;
  assert.deepEqual([counts.corner, counts.middle], [fixture.moduleTypes.corner, fixture.moduleTypes.middle]);
  assert.deepEqual(naming(false), naming(true));
  assert.deepEqual(naming(false), fixture.example.map(({ image: _, ...row }: Record<string, unknown>) => row));
  assert.deepEqual(submission(false), submission(true));
  const expanded = submission(false) as { images: string[]; ranges: string[][] };
  assert.deepEqual(expanded.ranges, expanded.ranges.map((_, index) => [expanded.images[index * fixture.variants.length], expanded.images[(index + 1) * fixture.variants.length - 1]]));
  assert.equal(expanded.images.length, fixture.submission.imageModels.length * modules.length * fixture.variants.length);
  languages();
  verifyTypologyOwnership();
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    composition: { oracle: () => ({ projection: composition(true) }), subject: () => ({ projection: composition(false) }) },
    naming: { oracle: () => ({ projection: naming(true) }), subject: () => ({ projection: naming(false) }) },
    submission: { oracle: () => ({ projection: submission(true) }), subject: () => ({ projection: submission(false) }) },
    languages: { oracle: () => ({ projection: { rows: fixture.tableRows } }), subject: () => ({ projection: languages() }) },
    courseOwnership: {
      oracle: () => ({ projection: { sourceRoot: fixture.ownership.sourceRoot, files: fixture.ownership.files, assignment: fixture.ownership.assignment, topic: fixture.ownership.topic, documents: fixture.ownership.documents } }),
      subject: () => ({ projection: verifyTypologyOwnership() }),
    },
  },
});
