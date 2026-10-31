import assert from "node:assert/strict";
import type fixtureData from "./🧫️cases.json";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { cross, shuffler, rollup } from "d3-array";
import { createHash } from "node:crypto";
import Ajv2020 from "ajv/dist/2020";
import { evaluate } from "mathjs";
import { defineTestAdapter } from "@semio-tech/framework/test/adapter";

const fixture: typeof fixtureData = JSON.parse(readFileSync(fileURLToPath(new URL("./🧫️cases.json", import.meta.url)), "utf8"));
const workspaceRoot = new URL("../../../../../", import.meta.url);
const sourceRoot = new URL("../", import.meta.url);
const read = (name: string): string => readFileSync(fileURLToPath(new URL(name, sourceRoot)), "utf8");
type Vector = number[];

/** 🧮 Verifies row multiplicities and the same dimension-sharing pair against D3 grouping. */
function sharedDimensions(values: number[][], forms: string[], useOracle: boolean): boolean {
  const groups = (items: unknown[]): number[] => useOracle
    ? [...rollup(items, entries => entries.length, value => JSON.stringify(value)).values()].sort()
    : [...items.reduce<Map<string, number>>((counts, value) => { const key = JSON.stringify(value); counts.set(key, (counts.get(key) ?? 0) + 1); return counts; }, new Map<string, number>()).values()].sort();
  if (!values.every(row => row.length === 3 && JSON.stringify(groups(row)) === JSON.stringify([1, 2]))) return false;
  const dimensions = [0, 1, 2].map(column => values.map(row => row[column]));
  if (JSON.stringify(groups(dimensions)) !== JSON.stringify([1, 2])) return false;
  const shared = cross([0, 1, 2], [0, 1, 2]).filter(([a, b]) => a! < b! && JSON.stringify(dimensions[a!]) === JSON.stringify(dimensions[b!]));
  return shared.length === 1 && shared.every(([a, b]) => forms[a!] !== forms[b!]);
}

/** 📐 Checks continuous dimensions and categorical endpoint composition independently. */
function mixedForms(useOracle: boolean): unknown {
  const example = fixture.workedExample;
  const subtract = (a: Vector, b: Vector): Vector => useOracle
    ? evaluate("b-a", { a, b })
    : a.map((value, index) => b[index]! - value);
  const add = (rows: Vector[]): Vector => useOracle
    ? evaluate("sum(rows, 1)", { rows })
    : rows[0]!.map((_, index) => rows.reduce((sum, row) => sum + row[index]!, 0));
  const points = example.points;
  assert.equal(sharedDimensions(example.values, example.forms, useOracle), true);
  assert.equal(new Set(points.map((point: any) => point.form)).size, 3);
  assert.ok(points.every((point: any) => point.dimensions.every((value: number) => Number.isFinite(value) && value > 0)));
  assert.deepEqual(example.values, [0,1,2].map(axis => points.map((point: any) => point.dimensions[axis])));
  const transitions = example.edges.map((edge: any) => {
    const from = points[edge.from - 1], to = points[edge.to - 1];
    const numeric = subtract(from.dimensions, to.dimensions);
    assert.deepEqual(numeric, edge.numeric);
    assert.deepEqual([from.form, to.form], edge.category);
    return { numeric, category: [from.form, to.form] };
  });
  const reverse = { numeric: subtract(points[2].dimensions, points[0].dimensions), category: [points[2].form, points[0].form] };
  const categories = [transitions[0].category, transitions[2].category, reverse.category];
  assert.ok(categories.every((pair: string[], index: number) => pair[1] === categories[(index+1)%3][0]));
  const unique = (values: unknown[]): boolean => useOracle
    ? cross(values, values).filter(([a, b]) => JSON.stringify(a) === JSON.stringify(b)).length === values.length
    : new Set(values.map(row => JSON.stringify(row))).size === values.length;
  assert.equal(unique(example.oneHot), true);
  assert.ok(example.oneHot.every((flags: number[]) => flags.length === 3 && flags.filter(value => value === 1).length === 1 && flags.every(value => value === 0 || value === 1)));
  return {
    unique: unique(points.map((point: any) => [...point.dimensions, point.form])),
    omittedFormUnique: unique(points.map((point: any) => point.dimensions)),
    transitions, reverse,
    closedCycle: add([transitions[0].numeric, transitions[2].numeric, reverse.numeric]),
    categoryCycle: categories.map((pair: string[]) => pair[0]).concat(categories[2][1]),
    brokenCycle: add([transitions[0].numeric, transitions[2].numeric, example.wrongDirectNumeric.map((value: number) => -value)]),
    ordinaryChanges: Number(points[1].form !== points[2].form),
    flagChanges: useOracle
      ? Number(evaluate("sum(abs(b-a))", { a: example.oneHot[1], b: example.oneHot[2] }))
      : example.oneHot[1].filter((value: number, index: number) => value !== example.oneHot[2][index]).length,
  };
}

/** 🕸 Constructs all ordered nonreflexive pairs with the reference Cartesian-product implementation. */
function edgeOracle(): string[] {
  const vertices = Array.from({ length: fixture.graph.vertices }, (_, index) => index + 1);
  return cross(vertices, vertices).filter(([a, b]) => a! !== b!).map(([a, b]) => a + "-" + b);
}

/** 🎲 Repeats the seeded chord selection with the independent d3 shuffle implementation. */
function chordOracle(): string[] {
  let seed = fixture.graph.selectionSeed;
  const shuffle = shuffler(() => {
    seed = (Math.imul(1664525, seed) + 1013904223) >>> 0;
    return seed / 2 ** 32;
  });
  for (let attempt = 1; ; attempt++) {
    const vertices = shuffle(Array.from({length: fixture.graph.vertices}, (_, index) => index + 1)).slice(0, 4).sort((a, b) => a - b);
    const gaps = vertices.map((a, index) => (vertices[(index + 1) % 4]! - a + 16) % 16);
    if (!gaps.every(gap => gap >= 3 && gap <= 5)) continue;
    assert.equal(attempt, fixture.graph.selectionAttempts);
    assert.deepEqual(vertices, fixture.graph.cycleVertices);
    return vertices.map((a, index) => a + "-" + vertices[(index + 1) % 4]);
  }
}

/** 🔄 Checks a clockwise simple inner cycle and independent D3 endpoint degrees. */
function innerCycle(edges: string[], oracle: boolean): boolean {
  if (edges.length !== 4 || new Set(edges).size !== 4) return false;
  const pairs = edges.map(edge => edge.split("-").map(Number));
  const vertices = pairs.map(pair => pair[0]!);
  if (new Set(vertices).size !== 4) return false;
  const gaps = pairs.map(([a, b]) => (b! - a! + 16) % 16);
  if (!gaps.every(gap => gap >= 3 && gap <= 5) || gaps.reduce((a, b) => a + b, 0) !== 16) return false;
  const degrees = oracle
    ? cross(vertices, [0, 1]).every(([vertex, endpoint]) => (rollup(pairs, rows => rows.length, pair => pair[endpoint!]).get(vertex) ?? 0) === 1)
    : vertices.every(vertex => pairs.filter(pair => pair[0] === vertex).length === 1 && pairs.filter(pair => pair[1] === vertex).length === 1);
  return degrees && pairs.every(([a, b], index) => b === pairs[(index + 1) % 4]![0]);
}

/** 📝 Reads exact asset resource references independently of the icon rendering macro. */
function authoredTypes(source: string, id: string): unknown[] {
  const start = source.indexOf("\\newcommand{\\AssignmentTypes" + id + "}");
  assert.ok(start >= 0, "missing geometry set " + id);
  const body = source.slice(start, source.indexOf("\n}", start));
  return [...body.matchAll(/#1\{(capsule|ellipsoid-capsule)\}\{([a-zA-Z]+)\}\{([^}]+)\}/g)].map(([, family, variant, image]) => ({ family, variant, image }));
}

/** 🔢 Maps the original table order to one unbroken global vertex numbering. */
function typesFromFixture(): unknown[] {
  return fixture.exercise.tables.flatMap((table: any, index: number) => table.types.map((type: any, column: number) => ({vertex: index * 8 + column + 1, table: table.label, column: column + 1, image: type.image})));
}

/** 🧩 Compares the real drawing and ledger inputs with the language-neutral fixture. */
function exercises(): unknown {
  const source = read("geometry.tex");
  const edgesStart = source.indexOf("\\newcommand{\\AssignmentDirectedEdges}");
  const edges = source.slice(edgesStart, source.indexOf("\n}", edgesStart));
  const pairs = [...edges.matchAll(/#1\{(\d+)\}\{(\d+)\}/g)].map(([, a, b]) => a + "-" + b);
  assert.equal(pairs.length, fixture.graph.edges);
  assert.deepEqual(pairs, edgeOracle());
  assert.equal(pairs.filter(pair => { const [a, b] = pair.split("-").map(Number); return (a! <= 8) !== (b! <= 8); }).length, fixture.graph.crossTableEdges);
  assert.equal(Number(evaluate("n*(n-1)", { n: fixture.graph.vertices })), pairs.length);
  assert.ok(source.includes("\\AssignmentDirectedEdges{\\AssignmentGraphEdge}"));
  assert.ok(source.includes("\\AssignmentSelectedEdges{\\AssignmentBlankLedgerEntry}"));
  assert.ok(source.includes("AssignmentTypes#1"));
  const graph = source.slice(source.indexOf("\\newcommand{\\AssignmentCompleteGraph}"), source.indexOf("\\newcounter{assignmentedge}"));
  assert.ok(!graph.startsWith("\\newcommand{\\AssignmentCompleteGraph}[1]"));
  const picture = source.slice(source.indexOf("\\newcommand{\\AssignmentGraphPicture}"), source.indexOf("\\newcounter{assignmentedge}"));
  assert.ok(picture.indexOf("\\AssignmentTypesOne{\\AssignmentGraphVertex}") < picture.indexOf("\\AssignmentTypesTwo{\\AssignmentGraphVertex}"));
  for (const table of ["One", "Two"]) assert.ok(picture.includes("\\AssignmentTypes" + table + "{\\AssignmentGraphVertex}"));
  assert.ok(source.includes("22.5"));
  assert.ok(source.includes("to[bend left=8]"));
  assert.ok(source.includes("Stealth[length="));
  assert.ok(picture.includes("pgfonlayer"));
  assert.match(source, /\\AssignmentGraphRadiusY\s*=\s*53mm/);
  assert.ok(graph.includes("\\linewidth-\\wd\\AssignmentGraphBox"));
  assert.ok(graph.includes("\\advance\\AssignmentGraphRadiusX"));
  assert.ok(picture.includes("x=\\AssignmentGraphRadiusX,y=\\AssignmentGraphRadiusY"));
  const ledger = source.slice(source.indexOf("\\newcommand{\\AssignmentLedgerEntry}"), source.indexOf("\\newcommand{\\AssignmentBox}"));
  assert.ok(ledger.includes("\\linewidth/2"));
  assert.ok(ledger.includes("\\fontsize{" + fixture.workedExample.ledger.valueFontPt + "}{10}"));
  assert.equal(fixture.workedExample.ledger.rows * fixture.workedExample.ledger.columns, fixture.workedExample.ledger.entries);
  assert.ok(ledger.includes("floor((\\value{assignmentedge}-1)/\\AssignmentLedgerRows)"));
  assert.ok(ledger.includes("mod(\\value{assignmentedge}-1,\\AssignmentLedgerRows)"));
  assert.ok(ledger.includes("-6.3*\\AssignmentRow"));
  const mapping = typesFromFixture();
  assert.deepEqual(mapping, fixture.graph.vertexMapping);
  const selectedStart = source.indexOf("\\newcommand{\\AssignmentSelectedEdges}");
  const selectedBody = source.slice(selectedStart, source.indexOf("\n}", selectedStart));
  const selected = [...selectedBody.matchAll(/#1\{(\d+)\}\{(\d+)\}/g)].map(([, a, b]) => a + "-" + b);
  assert.deepEqual(selected, fixture.graph.selectedEdges);
  assert.equal(new Set(selected).size, fixture.graph.ledgerEntries);
  assert.ok(selected.every(pair => pairs.includes(pair)));
  for (let vertex = 1; vertex <= fixture.graph.vertices; vertex++) {
    assert.equal(pairs.filter(pair => pair.startsWith(vertex + "-")).length, fixture.graph.outDegree);
    assert.equal(pairs.filter(pair => pair.endsWith("-" + vertex)).length, fixture.graph.inDegree);
  }
  assert.ok(selected.every(pair => !selected.includes(pair.split("-").reverse().join("-"))));
  assert.deepEqual(fixture.workedExample.edges.map((edge: any) => edge.from + "-" + edge.to).sort(), cross([1,2,3],[1,2,3]).filter(([a,b]) => a !== b).map(([a,b]) => a + "-" + b).sort());
  const ring = Array.from({length: fixture.graph.vertices}, (_, index) => [index + 1, (index + 1) % fixture.graph.vertices + 1].join("-"));
  assert.deepEqual(selected.slice(0, 16), ring);
  const directedCycle = (edges: string[]): boolean => {
    const adjacency = new Map(edges.map(pair => pair.split("-").map(Number)).map(([a,b]) => [a!,b!]));
    const visited = new Set<number>();
    let current = 1;
    for (let index = 0; index < fixture.graph.vertices; index++) {
      if (visited.has(current) || !adjacency.has(current)) return false;
      visited.add(current);
      current = adjacency.get(current)!;
    }
    return current === 1 && visited.size === fixture.graph.vertices && adjacency.size === fixture.graph.vertices;
  };
  const perimeter = selected.slice(0, 16);
  const oracleDegrees = (edges: string[]): boolean => {
    const endpoints = edges.map(pair => pair.split("-").map(Number));
    return cross(Array.from({length: 16}, (_, index) => index + 1), [0, 1]).every(([vertex, endpoint]) => Number(evaluate("sum(values)", {values: endpoints.map(pair => Number(pair[endpoint!] === vertex))})) === 1);
  };
  assert.equal(directedCycle(perimeter), true);
  assert.equal(oracleDegrees(perimeter), true);
  const flipped = perimeter.map(pair => pair === "16-1" ? "1-16" : pair);
  assert.equal(directedCycle(flipped), false);
  assert.equal(oracleDegrees(flipped), false);
  assert.equal(selected.slice(16).length, 4);
  assert.deepEqual(selected.slice(16), chordOracle());
  for (const oracle of [false, true]) {
    assert.equal(innerCycle(selected.slice(16), oracle), true);
    for (const invalid of [["1-5", "5-10", "10-13", "1-13"], ["1-5", "2-7", "4-8", "9-13"], ["1-5", "5-10", "10-5", "5-1"], ["1-2", "2-7", "7-12", "12-1"]]) assert.equal(innerCycle(invalid, oracle), false);
  }
  assert.ok(source.includes("\\AssignmentSelectedEdges{\\AssignmentSelectedGraphEdge}"));
  assert.equal(fixture.graph.ledgerRows * fixture.graph.ledgerColumns, fixture.graph.ledgerEntries);
  const types = fixture.exercise.tables.map((entry: { id: string }) => authoredTypes(source, entry.id));
  const all = types.flat();
  assert.equal(new Set(all.map(row => JSON.stringify(row))).size, 16);
  for (const set of types) assert.deepEqual(set.map((row: any, index: number) => row.family === (index % 2 ? "ellipsoid-capsule" : "capsule")), Array(8).fill(true));
  const plan = JSON.parse(read("icons.json"));
  assert.deepEqual(plan.resources, fixture.exercise.tables.flatMap((entry: any) => entry.types.map(({icon, image, sha256}: any) => ({source: icon, output: image, sha256, width: (() => { const svg = readFileSync(new URL(icon, workspaceRoot), "utf8"), match = /viewBox="([^"]+)"/u.exec(svg); assert.ok(match); const box = match[1]!.trim().split(/[ ,]+/u).map(Number); assert.equal(box.length, 4); return Math.ceil(Math.min(22, 11 * box[2]! / box[3]!) * 600 / 25.4 / 16) * 16; })()}))));
  for (const row of fixture.exercise.tables.flatMap((entry: any) => entry.types)) {
    const raw = readFileSync(new URL(row.icon, workspaceRoot));
    assert.equal(createHash("sha256").update(raw).digest("hex"), row.sha256);
  }
  assert.ok(source.includes("\\AssignmentContrastImage["));
  assert.equal(source.includes("\\def\\AssignmentRounded"), false);
  return { types, pairs };
}

/** 🌐 Verifies one bilingual exercise with two transposed matrices and one complete graph. */
function languages(): void {
  for (const language of fixture.languages as ("en" | "de")[]) {
    const main = read(language + ".tex");
    assert.ok(main.includes("\\documentclass[theme=light,language=" + language + "]{assignment}"));
    assert.ok(main.includes("\\input{geometry.tex}"));
    const content = read(language + ".content.tex");
    const contract = fixture.content[language];
    assert.deepEqual([...content.matchAll(/\\footerlogos\{([^}]+)\}/g)].flatMap(match => match[1]!.split(",")), contract.footerLogos);
    assert.ok(content.indexOf("\\footerlogos{") < content.indexOf("\\begin{document}"));
    const tables = [...content.matchAll(/\\begin\{AssignmentTable\}\{([^}]*)\}([\s\S]*?)\\end\{AssignmentTable\}/g)];
    assert.deepEqual(tables.map(table => table[1]), ["", ...fixture.content[language].tableLabels]);
    assert.ok(tables[0]?.[2]?.includes("\\AssignmentWorkedExampleTable"));
    assert.deepEqual(tables.slice(1).map(table => table[2]?.match(/\\AssignmentExerciseTable\{(One|Two)\}/)?.[1]), ["One", "Two"]);
    assert.deepEqual([...content.matchAll(/\\(subsection|section)\{([^}]+)\}/g)].map(([,kind,title]) => [kind,title]), [["section", fixture.headings[language][0]], ["subsection", fixture.headings[language][1]], ["subsection", fixture.headings[language][2]]]);
    assert.equal(/\\(?:makeassignmenttitle|subtitle)\b/.test(content), false);
    assert.equal(/\\Delta|Form:|\\AssignmentCriteria|\\AssignmentInstructions/.test(content), false);
    assert.ok(content.includes("\\AssignmentWorkedExampleLedger"));
    for (const [kind, macros] of [["Diagram", ["AssignmentWorkedExampleGraph", "AssignmentCompleteGraph"]], ["Transitions", ["AssignmentWorkedExampleLedger", "AssignmentSelectedLedger"]]] as const) {
      const wrappers = [...content.matchAll(new RegExp("\\\\begin\\{" + kind + "\\}\\[title=\\{\\},break=false\\]([\\s\\S]*?)\\\\end\\{" + kind + "\\}", "g"))];
      assert.equal(wrappers.length, fixture.silhouettes[kind === "Diagram" ? "diagrams" : "transitions"]);
      assert.deepEqual(wrappers.map((wrapper, index) => wrapper[1]?.includes("\\" + macros[index])), [true, true]);
    }
    assert.equal(tables.length, fixture.tableChrome.wrappedTables);
    assert.ok(contract.tableLabels.every((label: string) => label === ""));
    const formNames: Record<string, string> = language === "en" ? {box:"Box",pyramid:"Pyramid","elliptical-cylinder":"elliptic cylinder"} : {box:"Quader",pyramid:"Pyramide","elliptical-cylinder":"ellipt. Zylinder"};
    const ledgerStart = content.indexOf("\\AssignmentWorkedExampleLedger");
    const ledger = content.slice(ledgerStart, content.indexOf("\\end{Transitions}", ledgerStart));
    const labels = [...ledger.matchAll(/\{([^{}]+)\}/g)].map(match => match[1]);
    assert.equal(/\([0-9,]+\)/.test(ledger), false);
    assert.deepEqual(labels.map(label => label.replaceAll("$", "")), fixture.workedExample.edges.map((edge: any) => {
      const numeric = edge.numeric.flatMap((value: number, index: number) => value === 0 ? [] : [fixture.workedExample.ledger.dimensionNames[index] + (value > 0 ? "+" : "") + value]);
      return [...numeric, formNames[edge.category[0]] + " \\to " + formNames[edge.category[1]]].join("; ");
    }));

    assert.equal(/Exercise [12]|Übung [12]|\bindependently\b|\bunabhängig\b|different parameter strategy|andere Parameterstrategie|table row|Tabellenzeile|Zeilennummer|row reference/.test(content), false);
    const pages = content.split("\\clearpage");
    assert.equal(pages.length, fixture.pages);
    assert.equal((pages.slice(1).join("\n").match(/\\section\{/g) ?? []).length, 0);
    for (const macro of ["AssignmentWorkedExampleTable","AssignmentWorkedExampleGraph"]) assert.ok(pages[0]!.includes("\\" + macro));
    assert.ok(pages[1]!.includes("\\AssignmentExerciseTable{One}"));
    assert.ok(pages[1]!.includes("\\AssignmentExerciseTable{Two}"));
    assert.ok(pages[2]!.includes("\\AssignmentCompleteGraph"));
    assert.ok(pages[2]!.includes("\\AssignmentSelectedLedger"));
    assert.equal(/choose a triangle|Wählen Sie ein Dreieck/.test(content), false);
    assert.equal((content.match(/\\AssignmentExerciseTable\{(?:One|Two)\}/g) ?? []).length, 2);
    assert.equal((content.match(/\\AssignmentCompleteGraph/g) ?? []).length, 1);
    assert.equal(/\\AssignmentCompleteGraph\{/.test(content), false);
    assert.equal((content.match(/\\AssignmentSelectedLedger/g) ?? []).length, 1);
    assert.equal(content.includes("\\AssignmentLedgerRange"), false);
    assert.equal(/rounded capsules|flat-ended|runde Kapseln|ebenen Stirnflächen/.test(content), false);
  }
  const geometry = read("geometry.tex");
  const worked = geometry.slice(geometry.indexOf("\\newcommand{\\AssignmentWorkedExampleTable}"));
  assert.ok(worked.includes("type-columns=3,parameter-rows=8,header-height=16mm,row-height=6mm,padding=2mm,label-width=27mm"));
  assert.ok(worked.includes("\\AssignmentParameterMatrixBlankRows{4}"));
  const latex = readFileSync(new URL("🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty", workspaceRoot), "utf8");
  assert.ok(latex.includes("\\NewExpandableDocumentCommand \\AssignmentParameterMatrixBlankRows"));
  assert.ok(latex.includes("\\AssignmentParameterMatrixBlankRows{\\l_semio_assignment_parameter_rows_int}"));
  const header = geometry.slice(geometry.indexOf("\\newcommand{\\AssignmentWorkedExampleHeader}"), geometry.indexOf("\\newcommand{\\AssignmentWorkedExampleTable}"));
  assert.ok(header.includes("\\scalebox{.5}{\\AssignmentWorkedExampleShape{#1}}"));
  assert.equal(/textbf|small|scriptsize/.test(header), false);
  assert.equal(worked.slice(0, worked.indexOf("\\newcommand{\\AssignmentWorkedExampleGraphVertex}")).includes("\\begin{tabular}"), false);
  fixture.workedExample.values.forEach((values: number[], index: number) => assert.ok(worked.includes("\\AssignmentParameterMatrixRow{#" + (index + 1) + "}{" + values.join(",") + "}")));
  assert.ok(worked.includes("\\AssignmentParameterMatrixRow{#4}{#5,#6,#7}"));
  assert.ok(geometry.includes("\\newcommand{\\AssignmentWorkedExampleGraph}"));
  assert.ok(geometry.includes("\\newcommand{\\AssignmentWorkedExampleLedger}[6]"));
  assert.equal(geometry.includes("\\newcommand{\\AssignmentWorkedExampleGraph}[6]"), false);
  const examplePicture = geometry.slice(geometry.indexOf("\\newcommand{\\AssignmentWorkedExampleGraphPicture}"), geometry.indexOf("\\newcommand{\\AssignmentWorkedExampleGraph}"));
  assert.deepEqual([...examplePicture.matchAll(/\\AssignmentSelectedGraphEdge\{(\d+)\}\{(\d+)\}/g)].map(([,from,to]) => [Number(from),Number(to)]), fixture.workedExample.edges.map((edge: any) => [edge.from,edge.to]));
  assert.equal(/node\[midway|Form:|\\Delta/.test(examplePicture), false);

  for (const obsolete of ["AssignmentWorkedBoxTable", "AssignmentWorkedBoxGraph", "AssignmentWorkedBoxLedger"]) assert.equal(geometry.includes(obsolete), false);
  for (const name of ["AssignmentBox", "AssignmentPyramid", "AssignmentEllipticalCylinder"]) {
    assert.ok(geometry.includes("\\newcommand{\\" + name + "}"));
    assert.ok(geometry.includes("\\" + name + "{"));
  }
  const pyramidStart = geometry.indexOf("\\newcommand{\\AssignmentPyramid}");
  const cylinderStart = geometry.indexOf("\\newcommand{\\AssignmentEllipticalCylinder}");
  const pyramid = geometry.slice(pyramidStart, geometry.indexOf("\\newcommand{", pyramidStart + 1));
  const cylinder = geometry.slice(cylinderStart, geometry.indexOf("\\newcommand{", cylinderStart + 1));
  assert.ok(pyramid.includes("--") && pyramid.includes("cycle"));
  assert.ok(/ellipse|arc|plot/.test(cylinder));
  const palette = ["semio-core.sty", "semio-tokens.sty"].map(name => readFileSync(new URL("🧰️framework/🛍️products/📓️print/🖋️latex/" + name, workspaceRoot), "utf8")).join("\n");
  for (const name of new Set(geometry.match(/semio-chrome-[a-z-]+/g))) assert.ok(palette.includes("{" + name + "}"), "undefined geometry color " + name);
  assert.equal(geometry.includes("\\begin{AssignmentParameterTable}"), false);
}

/** 📊 Projects writable cells from authored matrix dimensions or an independent Cartesian product. */
function matrix(useOracle: boolean): unknown {
  const role = (row: number, column: number): string => row === 0 ? (column === 0 ? "parameter-label" : "type-icon") : (column === 0 ? "blank-parameter-name" : "blank-parameter-value");
  if (useOracle) {
    const rows = Array.from({ length: fixture.matrix.totalRows }, (_, index) => index);
    const columns = Array.from({ length: fixture.matrix.totalColumns }, (_, index) => index);
    return cross(rows, columns).map(([row, column]) => ({ row, column, role: role(row!, column!) }));
  }
  const geometry = read("geometry.tex");
  const options = geometry.match(/\\begin\{AssignmentParameterMatrix\}\[([^\]]+)\]\{Parameter\}/)?.[1];
  assert.ok(options, "missing transposed parameter matrix");
  assert.equal(Number(options.match(/(?:^|,)\s*row-height=(\d+(?:\.\d+)?)mm(?:,|$)/)?.[1]), fixture.matrix.rowHeightMm);
  const option = (name: string): number => {
    const value = options.match(new RegExp("(?:^|,)\\s*" + name + "=(\\d+)(?:,|$)"))?.[1];
    assert.ok(value, "missing matrix option " + name);
    return Number(value);
  };
  const cells = [];
  for (let row = 0; row <= option("parameter-rows"); row++) {
    for (let column = 0; column <= option("type-columns"); column++) cells.push({ row, column, role: role(row, column) });
  }
  return cells;
}

/** 🏛 Verifies the domain source owner and course publication contract. */
export function verifyCourseOwnership(): unknown {
  const root = new URL("../../../../../", import.meta.url);
  const contract = fixture.ownership;
  const leaf = new URL(contract.sourceRoot + "/", root);
  const files = readdirSync(leaf);
  assert.ok(contract.files.every((name: string) => files.includes(name)));
  const generated = (folder: URL): string[] => readdirSync(folder, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? generated(new URL(entry.name + "/", folder)) : /\.(pdf|png)$/.test(entry.name) ? [entry.name] : []);
  assert.deepEqual(generated(leaf), []);
  const course = JSON.parse(readFileSync(new URL("🎓️teaching/🏛️architecture/📚️course/🔣️.json", root), "utf8"));
  const assignment = course.assignments.find((entry: any) => entry.id === contract.assignment);
  assert.equal(assignment.topic, contract.topic);
  assert.equal(course.topics.filter((entry: any) => entry.id === contract.topic).length, 1);
  assert.deepEqual(assignment.documents, contract.documents);
  for (const language of fixture.languages) {
    const main = readFileSync(new URL(language + ".tex", leaf), "utf8");
    assert.ok(main.includes("\\input{geometry.tex}"));
    assert.ok(main.includes("\\input{" + language + ".content.tex}"));
  }
  return { sourceRoot: contract.sourceRoot, files: contract.files, assignment: assignment.id, topic: assignment.topic, documents: assignment.documents };
}

/** 🧪 Runs semantic and source contracts without starting a TeX compiler. */
export function verifyParametrizationAssignmentContentV1(): void {
  const schema = JSON.parse(readFileSync(new URL("./🧬️cases.schema.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  for (const mutate of [
    (entry: any) => { entry.headings.de[1] = "Introduction"; },
    (entry: any) => { entry.workedExample.values[0][0] = 9; },
    (entry: any) => { entry.workedExample.points[1].form = "box"; },
    (entry: any) => { entry.workedExample.domains.L = "integer"; },
    (entry: any) => { entry.workedExample.edges[0].category = [0, 1]; },
    (entry: any) => { entry.workedExample.wrongDirectNumeric[0] = 1; },
    (entry: any) => { entry.workedExample.titleBanner = true; },
    (entry: any) => { entry.workedExample.ledger.valueFontPt = 8; },
    (entry: any) => { entry.workedExample.ledger.ruleWidthFractionOfColumn = 1; },
    (entry: any) => { entry.workedExample.ledger.ruleWidthBasis = "outer-textwidth"; },
    (entry: any) => { entry.workedExample.ledger.numericNotation = "tuple"; },
    (entry: any) => { entry.workedExample.ledger.zeroNumericChanges = "printed"; },
    (entry: any) => { entry.workedExample.ledger.dimensionNames = ["X", "Y", "Z"]; },
    (entry: any) => { entry.workedExample.matrix.textVerticalAlignment = "baseline"; },
    (entry: any) => { entry.workedExample.ledger.entries = 5; },
    ...["parameterRows", "filledRows", "blankRows", "headerHeightMm", "headerShapeScale", "headerNumbered", "tableChrome", "maxInkCenterOffsetMm", "centeredFilledCells"].map(key => (entry: any) => { const value = entry.workedExample.matrix[key]; entry.workedExample.matrix[key] = typeof value === "boolean" ? !value : value + 1; }),
    (entry: any) => { entry.exercise.strategy = "independent"; },
    (entry: any) => { entry.exercise.tables[1].id = "One"; },
    (entry: any) => { entry.matrix.totalRows = 10; },
    (entry: any) => { entry.matrix.typeColumns = 7; },
    (entry: any) => { entry.matrix.writingLinesPerRow = 2; },
    (entry: any) => { entry.matrix.rowHeightMm = 12; },
    (entry: any) => { entry.pages = 5; },
    (entry: any) => { entry.graph.vertices = 8; },
    (entry: any) => { entry.graph.edges = 56; },
    (entry: any) => { entry.graph.crossTableEdges = 0; },
    (entry: any) => { entry.graph.verticalRadiusMm = 32; },
    (entry: any) => { entry.graph.ringEdges[15] = "1-16"; },
    (entry: any) => { entry.graph.selectedEdges[15] = "1-16"; },
    (entry: any) => { entry.graph.fullTextWidth = false; },
    (entry: any) => { entry.graph.vertexMapping[8].vertex = 1; },
    (entry: any) => { entry.graph.selectedEdges[1] = entry.graph.selectedEdges[0]; },
    (entry: any) => { entry.graph.ledgerEntries = 120; },
    (entry: any) => { entry.graph.chords[0] = "1-3"; },
    (entry: any) => { entry.graph.chords[3] = "1-13"; },
    (entry: any) => { entry.graph.cycleVertices[3] = 10; },
    (entry: any) => { entry.graph.chords = ["1-5", "2-7", "4-8", "9-13"]; },
    (entry: any) => { entry.graph.chords[0] = "1-2"; },
    (entry: any) => { entry.silhouettes.diagrams = 0; },
    (entry: any) => { entry.silhouettes.transitions = 0; },
    (entry: any) => { entry.silhouettes.titleVisibleName = true; },
    (entry: any) => { entry.silhouettes.rightAutomaticChip = false; },
    (entry: any) => { entry.silhouettes.connectedOutline = false; },
    (entry: any) => { entry.silhouettes.breakable = true; },
    (entry: any) => { entry.silhouettes.diagramNumbers.reverse(); },
    (entry: any) => { entry.silhouettes.transitionNumbers.reverse(); },
    (entry: any) => { entry.tableChrome.titleVisibleName = true; },
    (entry: any) => { entry.tableChrome.rightAutomaticChip = false; },
    (entry: any) => { entry.tableChrome.connectedOutline = false; },
    (entry: any) => { entry.tableChrome.wrappedTables = 0; },
    (entry: any) => { entry.pageOrder.reverse(); },
    (entry: any) => { entry.content.en.footerLogos = []; },
    (entry: any) => { entry.content.de.footerLogos = ["other.png"]; },
    (entry: any) => { entry.ownership.sourceRoot = "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment"; },
    (entry: any) => { entry.ownership.documents.en.namespace = "print"; },
    (entry: any) => { entry.ownership.documents.de.producer = "@print:build"; },
    (entry: any) => { entry.ownership.documents.en.file = "parametrization-en.pdf"; },
  ]) {
    const invalid = structuredClone(fixture);
    mutate(invalid);
    assert.equal(validate(invalid), false);
  }
  for (const oracle of [false, true]) {
    assert.equal(sharedDimensions(fixture.workedExample.values, fixture.workedExample.forms, oracle), true);
    for (let axis = 0; axis < 3; axis++) {
      for (const row of [[1, 1, 1], [1, 2, 3]]) {
        const invalid = structuredClone(fixture.workedExample.values);
        invalid[axis] = row;
        assert.equal(sharedDimensions(invalid, fixture.workedExample.forms, oracle), false);
      }
      const mismatched = structuredClone(fixture.workedExample.values);
      mismatched[axis].reverse();
      assert.equal(sharedDimensions(mismatched, fixture.workedExample.forms, oracle), false);
    }
    assert.equal(sharedDimensions(fixture.workedExample.values, ["box", "box", "elliptical-cylinder"], oracle), false);
  }
  assert.deepEqual(mixedForms(false), mixedForms(true));
  const mixed = mixedForms(false) as any;
  assert.equal(mixed.unique, true);
  assert.equal(mixed.omittedFormUnique, false);
  assert.deepEqual(mixed.closedCycle, [0, 0, 0]);
  assert.deepEqual(mixed.brokenCycle, [-1, 0, 0]);
  assert.deepEqual(mixed.categoryCycle, ['box', 'pyramid', 'elliptical-cylinder', 'box']);
  assert.equal(mixed.ordinaryChanges, 1);
  assert.equal(mixed.flagChanges, 2);
  const result = exercises() as { types: unknown[] };
  assert.deepEqual(result.types, fixture.exercise.tables.map((entry: { types: unknown[] }) => entry.types.map(({ family, variant, image }: any) => ({family, variant, image}))));
  assert.deepEqual(matrix(false), matrix(true));
  assert.equal((matrix(false) as unknown[]).length, 81);
  languages();
  verifyCourseOwnership();
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    courseOwnership: {
      oracle: () => ({ projection: { sourceRoot: fixture.ownership.sourceRoot, files: fixture.ownership.files, assignment: fixture.ownership.assignment, topic: fixture.ownership.topic, documents: fixture.ownership.documents } }),
      subject: () => ({ projection: verifyCourseOwnership() }),
    },
    mixedForms: {
      oracle: () => ({ projection: mixedForms(true) }),
      subject: () => ({ projection: mixedForms(false) }),
    },
    exercises: {
      oracle: () => ({ projection: { types: fixture.exercise.tables.map((entry: { types: unknown[] }) => entry.types.map(({ family, variant, image }: any) => ({family, variant, image}))), pairs: edgeOracle() } }),
      subject: () => ({ projection: exercises() }),
    },
    matrix: {
      oracle: () => ({ projection: matrix(true) }),
      subject: () => ({ projection: matrix(false) }),
    },
    languages: {
      oracle: () => ({ projection: { valid: true } }),
      subject: () => { languages(); return { projection: { valid: true } }; },
    },
  },
}); 
