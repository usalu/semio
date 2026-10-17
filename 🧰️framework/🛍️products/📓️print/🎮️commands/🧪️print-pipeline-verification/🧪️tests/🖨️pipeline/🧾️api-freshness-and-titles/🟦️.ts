/** 🧾️ The print API verification: freshness of the copied metadata, the printed owner rows, the title contracts and the title rendering. */
import { type PrintPdfTextV1 } from "../../../../../📦️artifacts/📖️pdf/🟦️.ts";
import MarkdownIt from "markdown-it";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { vizGeneratedFiles } from "../../../../../🔨️modules/📊️visualization-gallery/🏭️artifact-generation/🟦️.ts";
import { vizDocumentKeyRegions } from "../../../../../🔨️modules/📊️visualization-gallery/🧾️api-reference/🟦️.ts";
import { vizDeclaredKeyNames, vizImplementedFamilyKeyScopes, vizImplementedKeyPathNames } from "../../../../../🔨️modules/📊️visualization-gallery/🔑️implemented-keys/🟦️.ts";
import { vizPrintedKeyTables } from "../../../../../🔨️modules/📊️visualization-gallery/📋️printed-key-tables/🟦️.ts";
import { verifyVisualizationCoverage, vizPrintedKeyFindings } from "../../../../../🔨️modules/📊️visualization-gallery/🩺️coverage-verification/🟦️.ts";
import { withCompiledPrintProbeV1 } from "../../../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { assertPrintPdfGlyphs, openPrintPdf, outputDirectory, pipelineScope, type PrintPipelineVerificationV1, productRoot } from "../🟦️.ts";
import { verifyPrintApiScopeCases } from "../🎯️api-scope-cases/🟦️.ts";

//#region 🧪️PrintPipelineTests
/** 🔓️ Rejects copied stale API metadata against the generator and independent AJV equality. */
export function verifyPrintApiFreshness(): void {
  verifyPrintApiTitleContracts();
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")) as { readonly cases: readonly { readonly id: string; readonly path?: readonly (string | number)[]; readonly value?: unknown; readonly suffix?: string; readonly fresh: boolean }[] };
 
  const printed = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed as { required: Record<string, string[]>; scopeBindings: Record<string, string>; cases: { id: string; tables: { name: string; rows: string[]; groupedRows?: string[][]; nestedRows?: string[]; otherCells?: string[]; prose?: string[]; comments?: string[] }[]; missing: string[] }[] };
  verifyPrintApiScopeCases(printed);
  const expected = vizGeneratedFiles().find(file => file.path.endsWith("/🔣️viz-api.json"))!.content;
  const require = createRequire(import.meta.url), validate = new (require("ajv").default)({ strict: false }).compile({ const: expected });
  for (const vector of fixture.cases) {
    const document = JSON.parse(expected);
    if (vector.path) {
      let parent = document;
      for (const part of vector.path.slice(0, -1)) parent = parent[part];
      parent[vector.path.at(-1)!] = vector.value;
    }
    const candidate = `${JSON.stringify(document, null, 2)}\n${vector.suffix ?? ""}`;
    assert.equal(validate(candidate), vector.fresh, `${vector.id}: independent AJV equality`);
    if (vector.fresh) assert.doesNotThrow(() => verifyVisualizationCoverage(candidate), vector.id);
    else assert.throws(() => verifyVisualizationCoverage(candidate), /stale|unmarked/, vector.id);
  }

  const markdown = new MarkdownIt();
  for (const vector of printed.cases) {
    const tex = vector.tables.map(table => `\\SemioTableLong{\\Key{${table.name}}}{1}{\\ApiKey}{${[...table.rows.map(key => `\\SemioTableRow{\\Key{${key}} & string & ${(table.otherCells ?? []).map(other => `\\Key{${other}}`).join(", ")}}`), ...(table.groupedRows ?? []).map(keys => `\\SemioTableRow{${keys.map(key => `\\Key{${key}}`).join(", ")} & string}`), ...(table.nestedRows ?? []).map(key => `\\SemioTableRow{\\ApiText{\\Key{${key}}}{\\Key{${key}}} & string}`)].join("\n")}\n${(table.prose ?? []).map(key => `\\Key{${key}}`).join("\n")}\n${(table.comments ?? []).map(key => `% \\SemioTableRow{\\Key{${key}} & string}`).join("\n")}\n}`).join("\n");
    const oracle: Record<string, string[]> = {};
    for (const table of vector.tables) {
      const rows = [...table.rows.map(key => [key]), ...(table.groupedRows ?? []), ...(table.nestedRows ?? []).map(key => [key])];
      const tokens = markdown.parse(`| Key | Type |\n| --- | --- |\n${rows.map(keys => `| ${keys.join(", ")} | string |`).join("\n")}`, {});
      oracle[table.name] = tokens.filter((token, index) => token.type === "inline" && tokens[index - 1]?.type === "td_open").map(token => token.content).filter((_, index) => index % 2 === 0).flatMap(cell => cell.split(", "));
    }
    const missing = Object.entries(printed.required).flatMap(([owner, keys]) => keys.filter(key => !oracle[owner]?.includes(key)).map(key => `${owner}:${key}`)).sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.missing }, missing), true, `${vector.id}: independent markdown-it/AJV rows`);
    assert.deepEqual(vizPrintedKeyFindings(tex, printed.required), missing, vector.id);
  }
  const source = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8");
  assert.deepEqual(vizPrintedKeyFindings(source, printed.required), [], "Current printed owner-scoped API key rows");
  for (const vector of (printed as any).declarations) {
    const properties: Record<string, string> = { number: "fp", string: "tl" };
    const body = vector.declarations.map((key: any) => key.name + (key.operation === "typed" ? " ." + properties[key.type] + "_set:N = \\l_value_tl" : " .code:n = {" + ({ store: "\\tl_set:Nn \\l_result_tl {#1}", "prop-store": "\\prop_put:Nnn \\l_result_prop {unknown} {#1}", empty: "", forward: "\\keys_set:nn { other / owner } {#1}" } as Record<string,string>)[key.operation] + "}")).join(", ");
    const tokens = markdown.parse("| Name | Control |\n| --- | --- |\n" + vector.declarations.map((key: any) => "| " + key.name + " | " + key.control + " |").join("\n"), {});
    const cells = tokens.filter((token, index) => token.type === "inline" && tokens[index - 1]?.type === "td_open").map(token => token.content);
    const oracle = cells.filter((_, index) => index % 2 === 0 && cells[index + 1] === "true");
    assert.equal(new (require("ajv").default)().validate({ const: vector.keys }, oracle), true, vector.id + ": independent Markdown/AJV key declarations");
    assert.deepEqual(vizDeclaredKeyNames(body), oracle, vector.id + ": native declared controls");
  }
  for (const vector of (printed as any).regions) {
    const expected = vector.regions.map((region: any) => {
      const tokens = markdown.parse("| Name | Declared |\n| --- | --- |\n" + region.keys.map((key: any) => "| " + key.name + " | " + key.declared + " |").join("\n"), {});
      const cells = tokens.filter((token, index) => token.type === "inline" && tokens[index - 1]?.type === "td_open").map(token => token.content);
      return { scope: region.scope, keys: Array.from({ length: cells.length / 2 }, (_, index) => cells[index * 2 + 1] === "true" ? cells[index * 2] : undefined).filter(value => value !== undefined) };
    });
    assert.equal(new (require("ajv").default)().validate({ const: vector.expected }, expected), true, vector.id + ": independent Markdown/AJV physical scope records");
    const source = vector.regions.map((region: any) => "%region " + region.name + "\n" + region.keys.map((key: any) => "%   " + key.name + "  (" + key.type + ", omitted)  Control.").join("\n") + "\n\\keys_define:nn {" + region.scope + "} {" + region.keys.filter((key: any) => key.declared).map((key: any) => key.name + " ." + ({ string: "tl", boolean: "bool", number: "fp" } as Record<string, string>)[key.type] + "_set:N = \\l_value_tl").join(",") + "}\n%endregion").join("\n");
    assert.deepEqual(vizDocumentKeyRegions(source).map(region => ({ scope: region.scopes.join(","), keys: region.keys.map(key => key.name) })), expected, vector.id + ": source-owned metadata records");
  }
  for (const vector of (printed as any).routing) {
    const nodes = require("d3-hierarchy").hierarchy(vector.tree).descendants().map((node: any) => node.data);
    const expected = nodes.map((node: any) => node.scope).sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.expected }, expected), true, vector.id + ": D3 scope traversal");
    const source = nodes.map((node: any) => {
      const forward = node.forward ?? node.children?.[0]?.scope;
      return "\\keys_define:nn {" + node.scope + "} {" + node.key + " .tl_set:N = \\l_value_tl" + (forward ? ", unknown .code:n = {\\keys_set:nn {" + forward + "} {#1}}" : "") + "}";
    }).join("\n") + "\n\\SemioVizFamily{example}{\\keys_set:nn {semio / viz / family / example} {#1}}";
    assert.deepEqual(Object.keys(vizImplementedFamilyKeyScopes([source]).example ?? {}).sort(), expected, vector.id + ": native source forwarding scope identity");
  }
  const tokenFindings: { id: string; actual: readonly string[]; expected: readonly string[] }[] = [];
  for (const vector of (printed as any).tokenRoutes) {
    const own = "semio / viz / family / example", inner = "neutral / inner";
    const tree = { scope: own, authored: true, children: [{ scope: inner, authored: vector.forward }] };
    const expected = require("d3-hierarchy").hierarchy(tree).descendants().filter((node: any) => node.data.authored).map((node: any) => node.data.scope).sort();
    assert.equal(new (require("ajv").default)().validate({ const: [...vector.expected].sort() }, expected), true, vector.id + ": independent D3 authored-option routes");
    const parameter = "#" + vector.parameter, call = vector.parameter === 1 ? "{#1}{fixed}" : "{fixed}{#1}";
    const unknown = vector.route === "long-forward" ? ", unknown .code:n = {\\tl_put_right:Nx \\l_forwarded_unknown_options_tl {\\l_keys_key_str={\\exp_not:n{#1}},}\\exp_args:Nnx \\keys_set:nn {" + inner + "} {\\l_keys_key_str={\\exp_not:n{#1}}}}" : "";
    const alias = vector.route === "stored" || vector.route === "overwritten" ? "\\tl_set:Nn \\l_pass_tl {" + parameter + "}" + (vector.route === "overwritten" ? "\\tl_set:Nn \\l_pass_tl {cap=butt}" : "") : "";
    const setter = vector.route === "known" ? "\\keys_set_known:nnN {" + own + "} {" + parameter + "} \\l_pass_tl" : "\\keys_set:nn {" + own + "} {" + parameter + "}";
    const option = ["stored", "known", "overwritten"].includes(vector.route) ? "\\l_pass_tl" : vector.forward && vector.route !== "long-forward" ? parameter : "cap=butt";
    const source = "\\keys_define:nn {" + own + "} {width .fp_set:N = \\l_width_fp" + unknown + "}\n\\keys_define:nn {" + inner + "} {cap .tl_set:N = \\l_cap_tl}\n\\cs_new_protected:Npn \\semio_viz_probe:nn #1#2 {" + alias + setter + "\\semio_viz_inner:n {" + option + "}}\n\\cs_new_protected:Npn \\semio_viz_inner:n #1 {\\keys_set:nn {" + inner + "} {#1}}\n\\SemioVizFamily{example}{\\semio_viz_probe:nn" + call + "}";
    const actual = Object.keys(vizImplementedFamilyKeyScopes([source]).example ?? {}).sort();
    if (JSON.stringify(actual) !== JSON.stringify(expected)) tokenFindings.push({ id: vector.id, actual, expected });
  }
  assert.deepEqual(tokenFindings, [], "Native authored/fixed/residual option paths compared with D3/AJV");
  const policySource = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8"), policyTables = vizPrintedKeyTables(policySource);
  for (const vector of (printed as any).sourcePolicy) {
    const present = vizImplementedKeyPathNames(vector.scope).includes(vector.key);
    assert.equal(new (require("ajv").default)().validate({ const: vector.allowed }, present), true, vector.scope + ":" + vector.key + ": source-owned control policy");
    if (!vector.allowed) assert.equal(policyTables[vector.table]?.includes(vector.key) ?? false, false, vector.table + ":" + vector.key + ": printed source policy");
  }
  const contracts = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.nativeContracts;
  const families = JSON.parse(vizGeneratedFiles().find(file => file.path.endsWith("/🔣️viz-api.json"))!.content).families;
  const schemaFamilies = JSON.parse(readFileSync(join(productRoot, "🧬️schema/🔣️.json"), "utf8"))["x-semio-family-options"], grammarFindings: string[] = [];
  const neutral = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed;
  for (const vector of neutral.absentDescriptors) {
    const family = families.find((candidate: any) => candidate.name === vector.family);
    assert.ok(family && schemaFamilies[vector.family], vector.family + ": absence contract requires both actual owners");
    const actual = { schema: schemaFamilies[vector.family].options[vector.key] !== undefined, api: family.options.some((option: any) => option.name === vector.key) };
    assert.equal(new (require("ajv").default)().validate({ const: { schema: false, api: false } }, actual), true, vector.family + ":" + vector.key + ": inactive descriptor absent from canonical schema and generated API");
  }
  for (const vector of neutral.syntaxContracts) {
    const option = families.find((family: any) => family.name === vector.family)?.options.find((key: any) => key.name === vector.key);
    const actual = Object.fromEntries(Object.keys(vector.spec).map(property => [property, option?.[property]]));
    if (!new (require("ajv").default)().validate({ const: vector.spec }, actual)) grammarFindings.push(vector.family + ":" + vector.key + ":metadata-syntax");
  }
  const acceptedScopes = vizImplementedFamilyKeyScopes(), nativePaths = new Map<string, readonly string[]>();
  for (const vector of neutral.inheritedContracts) {
    if (!nativePaths.has(vector.scope)) nativePaths.set(vector.scope, vizImplementedKeyPathNames(vector.scope));
    assert.ok(nativePaths.get(vector.scope)!.includes(vector.key), vector.scope + ":" + vector.key + ": actual declared control");
    assert.equal(acceptedScopes[vector.family]?.[vector.scope]?.includes(vector.key) ?? false, vector.allowed, vector.family + ":" + vector.key + ": authored option transport");
    const option = schemaFamilies[vector.family]?.options[vector.key];
    const actual = Object.fromEntries(Object.keys(vector.spec).map(property => [property, option?.[property]]));
    if (vector.allowed ? !new (require("ajv").default)().validate({ const: vector.spec }, actual) : option !== undefined) grammarFindings.push(vector.family + ":" + vector.key + ":inherited-descriptor");
  }
  for (const vector of neutral.forbiddenDescriptors) {
    assert.equal(Object.values(acceptedScopes[vector.family] ?? {}).some(keys => keys.includes(vector.key)), false, vector.family + ":" + vector.key + ": fixed internal control");
    if (schemaFamilies[vector.family]?.options[vector.key] !== undefined) grammarFindings.push(vector.family + ":" + vector.key + ":phantom-descriptor");
  }

  const nativeFindings: string[] = [];
  for (const [name, options] of Object.entries(contracts) as [string, Record<string, { type: string; default?: string }> ][]) {
    const family = families.find((candidate: any) => candidate.name === name);
    for (const [key, expected] of Object.entries(options)) {
      const option = family.options.find((candidate: any) => candidate.name === key);
      const actual = Object.fromEntries(Object.keys(expected).map(property => [property, option?.[property]]));
      if (!new (require("ajv").default)().validate({ const: expected }, actual)) nativeFindings.push(name + ":" + key + ":source-owned-contract");
    }
  }
  assert.deepEqual([...grammarFindings, ...nativeFindings], [], "Native grammar and source contracts compared with independent AJV");


}

/** 🌍️ Binds visible reference captions to neutral locales while preserving scoped row identities. */
function verifyPrintApiTitleContracts(): void {
  const visual = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.visual;
  const source = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8");
  const titles = [...source.matchAll(/\\ApiTitleDefine\{([^}]+)\}\{([^}]+)\}\{([^}]+)\}/g)].map(match => match.slice(1));
  const tokens = new MarkdownIt().parse("| Scope | English | German |\n| --- | --- | --- |\n" + visual.titles.map((row: string[]) => "| " + row.join(" | ") + " |").join("\n"), {});
  const cells = tokens.filter((token, index) => token.type === "inline" && tokens[index - 1]?.type === "td_open").map(token => token.content);
  const oracle = Array.from({ length: cells.length / 3 }, (_, index) => cells.slice(index * 3, index * 3 + 3));
  const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ const: oracle });
  assert.ok(validate(titles), "Source-owned API caption locales: " + JSON.stringify(validate.errors));
  for (const candidate of [titles.slice(1), titles.map((row, index) => index ? row : [row[0], row[1], ""]), titles.map((row, index) => index ? row : ["other-owner", row[1], row[2]])]) assert.equal(validate(candidate), false);
  const tex = "\\SemioTableLong{\\ApiTitle{neutral-owner}}{1}{\\ApiKey & \\ApiType}{\\SemioTableRow{\\Key{value} & string}}";
  assert.deepEqual(vizPrintedKeyTables(tex), { "neutral-owner": ["value"] }, "Localized captions retain table owner and first-cell keys");
  assert.doesNotMatch(source, /\\u00(?:E4|FC|DF)/, "Reference prose uses actual Unicode characters");
}

/** 📖️ Compiles the actual document-local title apparatus in both explicitly selected languages. */
export async function verifyPrintApiTitleRendering(context:PrintPipelineVerificationV1): Promise<void> {
  const outputRoot=outputDirectory(context),{execution,control}=pipelineScope(context);
  const source = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8");
  const visual = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.visual;
  const apparatus = source.slice(source.indexOf("%region 🔖️Apparatus"), source.indexOf("%endregion 🔖️Apparatus"));
  for (const language of ["en", "de"] as const) {
    const headings = visual.headings.map((heading: any, index: number) => `\\${index === 0 ? "chapter" : "section"}{${heading.tex}}`).join("\n");
    const generated=`\\documentclass[type=report,theme=light,language=${language}]{semio}\n${apparatus}\n\\title{\\ApiText{API title controls}{API-Titelkontrollen}}\n\\author{Semio}\n\\errorcontextlines=100\n\\begin{document}\n\\tableofcontents\n${headings}\n\\ApiScopeSource{api-path-spatial-vector-field}{semio / viz / family / spatial-vector-field}\n\\SemioTableLong{\\ApiTitle{api-path-spatial-vector-field}}{0.18,0.13,0.15,0.54}{\\ApiKey & \\ApiType & \\ApiDefault & \\ApiMeaning}{\\SemioTableRow{\\Key{field} & \\Key{string} & \\Key{shear} & \\ApiText{Named vector field.}{Benanntes Vektorfeld.}}}\n\\end{document}\n`;
    await withCompiledPrintProbeV1(execution,{version:1,kind:"probe",name:"titles.tex",source:generated},async probe=>{
    const pdf = await openPrintPdf(context,await probe.read("titles.pdf",67108864),probe.control), texts: string[] = [], contents: string[] = [];
    let captions = 0;
    try {
      for (let number = 1; number <= pdf.numPages; number++) {
        const page = await pdf.getPage(number), content = await page.getTextContent();
        const items = content.items.filter((item): item is PrintPdfTextV1 => "str" in item);
        const text = items.map(item => item.str).join(" ");
        assertPrintPdfGlyphs(text, `${language} API-title page ${number}`);
        texts.push(text);
        if (/(?:Inhaltsverzeichnis|Table\s+of\s+contents)/i.test(text)) contents.push(text);
        for (const badge of items.filter(item => /^(?:Table|Tabelle):/.test(item.str))) {
          const row = items.filter(item => Math.abs(item.transform[5] - badge.transform[5]) < 2 && item.transform[4] < badge.transform[4]);
          if (!row.map(item => item.str).join(" ").includes(visual.headings[3][language])) continue;
          assert.ok(Math.max(...row.map(item => item.transform[4] + item.width)) + visual.minimumCaptionGapPt <= badge.transform[4], `${language}: visible caption overlaps number badge`);
          captions++;
        }
      }
      const text = texts.join(" ").replace(/\s+/g, "");
      for (const heading of visual.headings) assert.ok(text.includes(heading[language].replace(/\s+/g, "")), `${language}: missing resolved heading ${heading[language]}`);
      const entries = contents.join(" ").replace(/\s+/g, "");
      for (const heading of visual.headings) assert.ok(entries.includes(heading[language].replace(/\s+/g, "")), `${language}: missing rendered contents entry ${heading[language]}`);
      for (const token of visual.forbiddenContents) assert.ok(!text.includes(token), `${language}: raw title token ${token}`);
      assert.ok(captions > 0, `${language}: no measured table caption`);
      console.log(`[print] API localized title rendering ${language}: ${visual.headings.length} headings, ${captions} measured captions, ${pdf.numPages} PDF.js pages PASS`);
    } finally { await pdf.destroy(); }
    });
  }
}
//#endregion 🧪️PrintPipelineTests
