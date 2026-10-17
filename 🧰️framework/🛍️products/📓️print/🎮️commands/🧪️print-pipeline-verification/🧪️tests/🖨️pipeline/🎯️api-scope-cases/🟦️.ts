/** 🎯️ The print API scope cases: the owner-scoped key rows checked against the printed tables. */
import MarkdownIt from "markdown-it";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { vizDocumentKeyGerman, vizDocumentKeyFamilies } from "../../../../../🔨️modules/📊️visualization-gallery/🧾️api-reference/🟦️.ts";
import { vizImplementedKeyPathNames } from "../../../../../🔨️modules/📊️visualization-gallery/🔑️implemented-keys/🟦️.ts";
import { vizPrintedKeyRows } from "../../../../../🔨️modules/📊️visualization-gallery/📋️printed-key-tables/🟦️.ts";
import { vizPrintedFamilyFindings, vizPrintedScopeFindings, vizPrintedPackageFindings } from "../../../../../🔨️modules/📊️visualization-gallery/🩺️coverage-verification/🟦️.ts";
import { productRoot } from "../🟦️.ts";

//#region 🧪️PrintPipelineTests
/** 🧪️ Adjudicates semantic rows and explicit inherited controls against independent markdown-it tables. */
export function verifyPrintApiScopeCases(printed: any): void {
  const markdown = new MarkdownIt(), require = createRequire(import.meta.url);
  const rowTex = (row: any): string => "\\SemioTableRow{\\Key{" + row.name + "} & \\Key{" + row.type + "} & \\Key{" + row.default + "} & \\ApiText{" + row.meaning.en + "}{" + row.meaning.de + "}}";
  const tableTex = (table: any): string => "\\SemioTableLong{\\Key{" + table.name + "}}{1}{\\ApiKey & \\ApiType & \\ApiDefault & \\ApiMeaning}{" + table.rows.map(rowTex).join("\n") + "}\n";
  const oracleRows = (table: any): any[] => {
    const text = "| Key | Type | Default | EN | DE |\n| --- | --- | --- | --- | --- |\n" + table.rows.map((row: any) => "| " + [row.name, row.type, row.default, row.meaning.en, row.meaning.de].map(value => value.replaceAll("|", "\\|")).join(" | ") + " |").join("\n");
    const tokens = markdown.parse(text, {});
    const cells = tokens.filter((token, index) => token.type === "inline" && tokens[index - 1]?.type === "td_open").map(token => token.content);
    return Array.from({ length: cells.length / 5 }, (_, index) => ({ name: cells[index * 5], type: cells[index * 5 + 1], default: cells[index * 5 + 2], meaning: { en: cells[index * 5 + 3], de: cells[index * 5 + 4] } }));
  };
  for (const vector of printed.packageRows) {
    const source = "%region Keys\n%  " + vector.key + " (" + vector.comment.type + ", " + vector.comment.default + ") " + vector.comment.en + "\n\\keys_define:nn {" + vector.scope + "} {" + vector.key + " ." + vector.setter + " = \\l_neutral_value" + (vector.additional ?? []).map((item: any) => ", " + item.row.name + " ." + item.setter + " = \\l_neutral_extra").join("") + "}\n%endregion\n";
    const rows = [vector.row, ...(vector.additional ?? []).map((item: any) => item.row)];
    const reference = "\\ApiScopeSource{neutral-owner}{" + vector.scope + "}" + tableTex({ name: "neutral-owner", rows }).replace("\\Key{}", vector.defaultTex ?? "\\Key{}");
    const expected = oracleRows({ rows }).map(visible => ({ name: visible.name, type: visible.type, default: visible.default, description: visible.meaning }));
    const actual = vizDocumentKeyFamilies(source, reference).flatMap(family => family.keys);
    assert.equal(new (require("ajv").default)().validate({ const: expected }, actual), true, vector.id + ": source-owner whole-row metadata compared with markdown-it/AJV");
  }
  for (const vector of printed.packageRowCases) {
    const control = printed.packageRows[0];
    const source = "%region Keys\n%  fit (string, linear) Copied comment.\n\\keys_define:nn {" + control.scope + "} {fit .bool_set:N = \\l_neutral_value}\n%endregion\n";
    const rows = oracleRows({ rows: vector.rows });
    const expected = vector.scope === control.scope && rows.every(row => row.type && row.meaning.en && row.meaning.de) && new Set(rows.map(row => JSON.stringify(row))).size === 1;
    assert.equal(new (require("ajv").default)().validate({ const: vector.valid }, expected), true, vector.id + ": independent markdown-it/AJV owner ambiguity policy");
    const reference = vector.rows.map((row: any, index: number) => "\\ApiScopeSource{neutral-owner-" + index + "}{" + vector.scope + "}" + tableTex({ name: "neutral-owner-" + index, rows: [row] })).join("\n");
    let admitted = true;
    try { vizDocumentKeyFamilies(source, reference); } catch { admitted = false; }
    assert.equal(admitted, expected, vector.id);
  }
  for (const vector of printed.packageNamespaceRows) {
    const text = "\\subsection{\\Key{neutral-installed}}\\ApiScopeSource{neutral-installed}{" + vector.scope + "}" + tableTex({ name: "neutral-installed", rows: vector.rows });
    const expected = oracleRows({ rows: vector.rows });
    const actual = vizDocumentKeyFamilies(vector.implementation, text).flatMap(entry => entry.keys).map(key => ({ name: key.name, type: key.type, default: key.default, meaning: key.description }));
    assert.equal(new (require("ajv").default)().validate({ const: expected }, actual), true, vector.id + ": independently parsed installed controls");
  }
  const inherited = printed.inherited;
  for (const vector of inherited.cases) {
    const document = structuredClone(inherited.base);
    if (vector.path) {
      let parent = document;
      for (const part of vector.path.slice(0, -1)) parent = parent[part];
      parent[vector.path.at(-1)] = vector.value;
    }
    const tables = new Map<string, any>(document.tables.filter((table: any) => !table.literal && !table.code).map((table: any) => [table.name, { ...table, rows: oracleRows(table) }]));
    const expected: string[] = [];
    for (const [name, options] of Object.entries(inherited.required) as [string, Record<string, any>][]) {
      const family = document.families.find((item: any) => item.name === name);
      if (!family) { expected.push(name + ":missing-section"); continue; }
      const rows = oracleRows(family);
      for (const ref of family.refs) {
        const table = tables.get(ref), scope = inherited.bindings[ref], accepted = inherited.reachable[name]?.[scope];
        if (!table || !accepted) { expected.push(name + ":invalid-scope:" + ref); continue; }
        if (table.scope !== scope) { expected.push(name + ":invalid-binding:" + ref); continue; }
        for (const row of table.rows) {
          if (accepted.includes(row.name)) rows.push(row);
          else expected.push(name + ":unreachable:" + ref + ":" + row.name);
        }
      }
      for (const [key, spec] of Object.entries(options)) {
        const row = rows.find((item: any) => item.name === key);
        if (!row) { expected.push(name + ":" + key); continue; }
        if (row.type !== spec.type) expected.push(name + ":" + key + ":type");
        if (row.default !== spec.default) expected.push(name + ":" + key + ":default");
        for (const language of ["en", "de"]) {
          if (!row.meaning[language]) expected.push(name + ":" + key + ":" + language);
          else if (spec.unit && !row.meaning[language].includes(spec.unit)) expected.push(name + ":" + key + ":" + language + ":unit");
        }
      }
    }
    expected.sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.findings }, expected), true, vector.id + ": independent markdown-it/AJV scope semantics");
    const shared = document.tables.map((table: any) => {
      const body = "\\ApiScopeSource{" + table.name + "}{" + table.scope + "}\n" + tableTex(table);
      return table.literal ? "\\begin{verbatim}\n" + body + "\\end{verbatim}" : table.code ? "\\begin{SemioCode}\n" + body + "\\end{SemioCode}" : body;
    }).join("\n");
    const source = shared + document.families.map((family: any) => "\\subsection{\\Key{" + family.name + "}}\n" + tableTex(family) + family.refs.map((ref: string) => "\\ApiKeyScope{" + ref + "}").join("\n")).join("\n") + "\\section{Later examples}\n" + (vector.laterRefs ?? []).map((ref: string) => "\\ApiKeyScope{" + ref + "}").join("\n");
    assert.deepEqual(vizPrintedScopeFindings(source, inherited.required, inherited.reachable, inherited.bindings), expected, vector.id);
  }
  for (const vector of printed.scalars) {
    const row = { name: "values", type: vector.renderedType ?? vector.type, default: vector.rendered, meaning: { en: "Values.", de: "Werte." } };
    const visible = oracleRows({ rows: [row] })[0]!;
    const list = (value: string): readonly string[] => value.split(",").map(part => part.trim().toLowerCase());
    const kinds = (value: string): readonly string[] => value.split("|").map(part => printed.typeClasses?.[part.trim()] ?? part.trim().toLowerCase()).sort();
    const expected = [...(JSON.stringify(list(visible.default)) === JSON.stringify(list(vector.expected)) ? [] : ["example:values:default"]), ...(JSON.stringify(kinds(visible.type)) === JSON.stringify(kinds(vector.type)) ? [] : ["example:values:type"])].sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.findings }, expected), true, vector.id + ": independent scalar semantics");
    assert.deepEqual(vizPrintedScopeFindings("\\subsection{\\Key{example}}" + tableTex({ name: "example", rows: [row] }), { example: { values: { type: vector.type, default: vector.expected } } }, {}, {}), expected, vector.id);
  }
  for (const vector of printed.groupedSpecs) {
    const visible = oracleRows({ rows: vector.keys.map((key: any, index: number) => ({ name: key.name, type: printed.typeClasses[vector.types[index]] ?? vector.types[index], default: vector.defaults[index], meaning: { en: "Geometry and caption.", de: "Geometrie und Beschriftung." } })) });
    const expected = vector.keys.flatMap((key: any, index: number) => [...(visible[index].type === key.type ? [] : ["example:" + key.name + ":type"]), ...(visible[index].default === key.default ? [] : ["example:" + key.name + ":default"])]).sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.findings }, expected), true, vector.id + ": independent grouped type/default semantics");
    const tokens = (values: readonly string[]): string => values.map(value => "\\Key{" + value + "}").join(", ");
    const source = "\\subsection{\\Key{example}}\\SemioTableLong{\\Key{example}}{1,1,1,1}{\\ApiKey & Type & Default & Meaning}{\\SemioTableRow{" + tokens(vector.keys.map((key: any) => key.name)) + " & " + tokens(vector.types) + " & " + tokens(vector.defaults) + " & \\ApiText{Geometry and caption.}{Geometrie und Beschriftung.}}}";
    assert.deepEqual(vizPrintedScopeFindings(source, { example: Object.fromEntries(vector.keys.map((key: any) => [key.name, { type: key.type, default: key.default }])) }, {}, {}), expected, vector.id);
  }
  const translated = printed.translations, source = translated.tables.map(tableTex).join("\n");
  const expected = translated.queries.map((query: any) => oracleRows(translated.tables.find((table: any) => table.name === query.scope) ?? { rows: [] }).find((row: any) => row.name === query.key)?.meaning.de ?? "");
  assert.equal(new (require("ajv").default)().validate({ const: translated.queries.map((query: any) => query.de) }, expected), true, "Independent owner-specific translations");
  const actual = translated.queries.map((query: any) => vizDocumentKeyGerman(source, [query.scope]).get(query.key) ?? "");
  for (const vector of printed.sharedTranslations) {
    const tree = { meaning: vector.local ? "Local network data." : "", reachable: true, children: [{ meaning: "Shared native network data.", reachable: vector.reachable }] };
    const expected = require("d3-hierarchy").hierarchy(tree).descendants().find((node: any) => node.data.reachable && node.data.meaning)?.data.meaning ?? "";
    const row = (meaning: string): string => tableTex({ name: "neutral-root", rows: meaning ? [{ name: "data", type: "string", default: "demo", meaning: { en: meaning, de: meaning } }] : [] });
    const text = "\\subsection{\\Key{neutral-root}}\\ApiScopeSource{neutral-root}{" + vector.root + "}" + row(vector.local ? "Local network data." : "") + "\\ApiKeyScope{neutral-target}\\subsection{\\Key{neutral-target}}" + (vector.binding ? "\\ApiScopeSource{neutral-target}{" + vector.target + "}" : "") + tableTex({ name: "neutral-target", rows: [{ name: "data", type: "string", default: "demo", meaning: { en: "Shared native network data.", de: "Shared native network data." } }] });
    assert.equal(new (require("ajv").default)().validate({ const: expected }, vizDocumentKeyGerman(text, [vector.root]).get("data") ?? ""), true, vector.id + ": independent D3/AJV inherited translation ownership");
  }
  const familySource = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8");
  const sharedRows = vizPrintedKeyRows(familySource);
  for (const scope of printed.sharedSourceContracts.scopes) {
    const declared = vizImplementedKeyPathNames(scope.scope);
    const visible = sharedRows[scope.id] ?? [];
    const expected = oracleRows({ rows: scope.keys.map((key: any) => ({ name: key.key, type: key.type, default: key.default, meaning: key.description })) });
    for (const key of scope.keys) assert.ok(declared.includes(key.key), scope.scope + ":" + key.key + ": native declared owner");
    const actual = scope.keys.map((key: any) => {
      const row = visible.find(candidate => candidate.names.includes(key.key));
      return row ? { name: key.key, type: row.type, default: row.default, meaning: row.meaning } : null;
    });
    assert.equal(new (require("ajv").default)().validate({ const: expected }, actual), true, scope.id + ": source-audited bilingual type/default contracts against markdown-it/AJV");
  }
  const packageFindings = vizPrintedPackageFindings(familySource);
  assert.deepEqual(packageFindings, [], "Every actual native owner control has a complete rendered contract");
  const familyFindings = vizPrintedFamilyFindings(familySource, printed.scopeBindings);
  assert.deepEqual({ translations: actual, families: familyFindings, packages: packageFindings }, { translations: expected, families: [], packages: [] }, "Current owner-scoped semantic API reference");
}
//#endregion 🧪️PrintPipelineTests
