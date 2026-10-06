import MarkdownIt from "markdown-it";
import { loadVizCatalog } from "../../../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";

import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { basename, dirname, join } from "node:path";
import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { classifyPackageSource, fileKindIdForSourcePath, fixedSourceDispositionDecision, implementationLeafBasenameFinding, loadCatalogTaxonomy, scopedFileKindIdForSourcePath, taxonomyFileKindIsImplementation } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { loadPrintDesignTokens, renderPrintLatexTokenStylesheet, resolvePrintPanelGlassStyle } from "../../../../🔨️modules/🎨print-design-token-paints/🟦️.ts";
import { renderVizGalleryDocument, parseVizTaxonomyLeaves, verifyVisualizationCoverage, visualizationTemplates, vizGeneratedFiles, vizPrintedKeyFindings, vizPrintedFamilyFindings, vizPrintedScopeFindings, vizDocumentKeyGerman, vizDocumentKeyFamilies, vizDeclaredKeyNames, vizPrintedPackageFindings, vizDocumentKeyRegions, vizImplementedFamilyKeyScopes, vizImplementedKeyPathNames, vizPrintedKeyTables, vizPrintedKeyRows } from "../../../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { stagePrintSources } from "../../../../🔨️modules/📥️source-staging/🟦️.ts";
import { printFontDescriptors, printFontSearchPaths, stagePrintFonts } from "../../../../🔨️modules/🔤print-font-catalog/🟦️.ts";
import { printDocumentOutputDirectory } from "../../../../🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts";
import { compilePrintTexOnce, deriveDarkPrintTexSource, printTemplatePdfNames, publishPrintArtifact, registeredPrintTemplates } from "../../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";

//#region 🧪️PrintPipelineTests
const workspaceRoot = getWorkspaceRoot();
const productRoot = join(workspaceRoot, "🧰️framework", "🛍️products", "📓️print");
const packageRoot = join(productRoot, "📦️packages", "🟦️typescript");
const latexRoot = join(productRoot, "🖋️latex");
const outputRoot = process.env.SEMIO_PRINT_OUTPUT_DIR ?? repoTestArtifactEnvironment(workspaceRoot, "print-pipeline").SEMIO_TEST_ARTIFACT_DIR!;

/** 🧬️ Loads one print scope's owned schema module and asserts its canonical identity. */
function printSchemaModule(modulePath: string, schemaId: string): Record<string, unknown> {
  const schema = JSON.parse(readFileSync(join(modulePath, "🧬️schema/🔣️.json"), "utf8"));
  assert.equal(schema.$schema, "http://json-schema.org/draft-07/schema#", modulePath);
  assert.equal(schema.$id, schemaId, modulePath);
  return schema;
}

/** 🔧️ Verifies the pinned tectonic toolchain manifest against its owner module. */
export function verifyPrintToolchainManifest(): void {
  const require = createRequire(import.meta.url), modulePath = join(productRoot, "🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain");
  const schema = printSchemaModule(modulePath, "https://json.schemas.assets.semio-tech.com/print/tectonic-template-compilation/toolchain/schema.json");
  const manifest = JSON.parse(readFileSync(join(modulePath, "🔣️.json"), "utf8"));
  const validate = new (require("ajv").default)({ strict: false }).compile(schema);
  assert.ok(validate(manifest), JSON.stringify(validate.errors));
  assert.equal(new Set(manifest.platforms.map((row: { platform: string; architecture: string }) => `${row.platform}/${row.architecture}`)).size, manifest.platforms.length);
}


/** 🧪️ Runs pure deterministic print-pipeline verification. */
export async function verifyPrintPipelineQuick(): Promise<void> {
  verifyPrintApiFreshness();
  await verifyPrintPdfGlyphPolicy();
  await verifyPrintCommandBoundaries();
  await verifyPrintDocumentCatalog();
  await verifyPrintBundleContract();
  verifyPrintToolchainManifest();
  await verifyPrintFontStaging();
  verifyPrintMacroStaging();
  const galleryIdentities = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🗺️gallery-identities.json"), "utf8")) as Record<string, string>;
  assert.deepEqual(Object.fromEntries(visualizationTemplates().map(({ id, texPath }) => [id, basename(texPath)])), galleryIdentities);
  assert.equal(new Set(Object.values(galleryIdentities).map((name) => name.split("viz-")[0]!.replaceAll("\uFE0F", ""))).size, 81);
  const stylesheet = renderPrintLatexTokenStylesheet(loadPrintDesignTokens());
  assert.match(stylesheet, /\\ProvidesPackage\{semio-tokens\}/);
  assert.match(stylesheet, /\\definecolor\{semio-chrome-light-panel\}/);
  assert.match(stylesheet, /\\newcommand\{\\semio@spacing@unit\}/);
  const lightPanel = resolvePrintPanelGlassStyle("light");
  assert.match(lightPanel.tintHex, /^#[0-9a-f]{6}$/);
  assert.ok(lightPanel.alpha > 0 && lightPanel.alpha <= 1);
  assert.ok(lightPanel.blurPixels > 0);
  assert.ok(lightPanel.saturation > 0);

  const dark = deriveDarkPrintTexSource("\\documentclass[a4paper]{article}\n\\includegraphics{asset/logo/mark.png}\n");
  assert.match(dark, /theme=dark/);
  assert.match(dark, /asset\/logo\/mark-dark\.png/);
  const withMagic = deriveDarkPrintTexSource("% !TEX program = tectonic\n% !TEX root = report.tex\n\\documentclass[theme=light]{semio}\n");
  assert.match(withMagic, /^% !TEX program = tectonic\n% !TEX root = report\.tex\n% Generated by print\/script\.ts/);
  assert.throws(() => deriveDarkPrintTexSource("\\documentclass[theme=dark]{article}"));
  assert.throws(() => deriveDarkPrintTexSource("\\documentclass{article}"));

  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️merge-contract.json"), "utf8"));
  const templates = registeredPrintTemplates();
  for (const template of [...templates, ...visualizationTemplates()]) {
    const source = readFileSync(join(productRoot, template.texPath), "utf8");
    const options = source.match(/\\documentclass\[([^\]]*)\]/)?.[1];
    assert.ok(options);
    assert.doesNotMatch(options, /\p{Extended_Pictographic}/u);
  }
  assert.deepEqual(templates.map((template) => template.id), contract.templates);
  for (const vector of contract.markdownCases) assert.deepEqual(parseVizTaxonomyLeaves(vector.source), vector.leaves);
  mkdirSync(outputRoot, { recursive: true });
  const stagingRoot = mkdtempSync(join(outputRoot, ".staging-test-"));
  try {
    const input = join(stagingRoot, "input"), output = join(stagingRoot, "output");
    for (const [path, content] of Object.entries(contract.staging.inputs)) {
      mkdirSync(dirname(join(input, path)), { recursive: true });
      writeFileSync(join(input, path), content as string);
    }
    const staged = stagePrintSources(input, ["📄️document.tex", "📎️appendix.tex", "🖼️assets"], output);
    assert.equal(staged.get("📄️document.tex"), join(output, "document.tex"));
    for (const [path, content] of Object.entries(contract.staging.outputs)) assert.equal(readFileSync(join(output, path), "utf8"), content);
    writeFileSync(join(input, "document.tex"), "collision");
    assert.throws(() => stagePrintSources(input, ["📄️document.tex", "document.tex"], output), /colliding print source/);
    assert.equal(readFileSync(join(output, "document.tex"), "utf8"), contract.staging.outputs["document.tex"]);
  } finally {
    rmSync(stagingRoot, { recursive: true, force: true });
  }
  const taxonomy = readFileSync(join(productRoot, "🖼️assets/📊️viz-taxonomy.md"), "utf8");
  const markdown = new MarkdownIt().parse(taxonomy, {});
  const oracle: string[] = [];
  let section: string | undefined;
  for (let index = 0; index < markdown.length; index++) {
    const token = markdown[index]!;
    if (token.type === "heading_open" && token.tag === "h2") section = String(Number.parseInt(markdown[index + 1]!.content, 10));
    if (token.type !== "inline" || !section) continue;
    const children = token.children ?? [];
    const code = children.findIndex((child) => child.type === "code_inline");
    if (code < 0 || !["mark", "chart", "layout", "axis", "scale"].includes(children[code + 1]?.content.trim() ?? "")) continue;
    oracle.push(`${section}/${children[code]!.content}`);
  }
  assert.equal(oracle.length, contract.vizLeaves);
  assert.deepEqual(parseVizTaxonomyLeaves(taxonomy), oracle);
  assert.deepEqual(printTemplatePdfNames("🧾️template/📋️report/📋️report.tex"), { light: "📋️report.pdf", dark: "📋️report-dark.pdf" });

  // 🎓 The dissertation kind is only a kind if all three parts agree: the class
  // branches on type=phd, the appendix level resolves to chapter like the other
  // chapter-sectioned kind, and semio-phd.sty owns the title page command.
  const classSource = readFileSync(join(latexRoot, "semio.cls"), "utf8");
  assert.match(classSource, /\{ phd \} \{ \\LoadClass\[twoside=true,open=right,cleardoublepage=empty\]\{scrreprt\} \}/);
  assert.match(classSource, /\\RequirePackage\{semio-phd\}/);
  assert.match(readFileSync(join(latexRoot, "semio-core.sty"), "utf8"), /\{ phd \} \{ \\tl_set:Nn \\l_semio_appendix_level_tl \{ chapter \} \}/);
  const phdSource = readFileSync(join(latexRoot, "semio-phd.sty"), "utf8");
  assert.match(phdSource, /\\NewDocumentCommand\{\\makedissertationtitle\}\{\}\{\\semio_phd_title_page:\}/);
  for (const environment of ["Abstract", "Kurzfassung", "Acknowledgements"]) assert.match(phdSource, new RegExp(`\\\\NewDocumentEnvironment\\{${environment}\\}`), environment);

  const loader = readFileSync(join(latexRoot, "semio-viz-charts.sty"), "utf8");
  for (const match of loader.matchAll(/\\RequirePackage\{([^}]+)\}/g)) assert.ok(existsSync(join(latexRoot, `${match[1]}.sty`)), `missing chart package ${match[1]}`);
  const windowSource = readFileSync(join(latexRoot, "semio-window.sty"), "utf8");
  const tableSource = readFileSync(join(latexRoot, "semio-table.sty"), "utf8");
    const headingTrackSource = windowSource.slice(windowSource.indexOf("\\newcount\\semio@chrome@heading@level"), windowSource.indexOf("\\newsavebox{\\semio@window@cap@slot}"));
    assert.match(readFileSync(join(latexRoot, "semio.cls"), "utf8"), /\\RequirePackage\[style=\\semio@citestyle,backend=bibtex,sorting=nyt,backref=true\]\{biblatex\}/);
    assert.match(readFileSync(join(latexRoot, "semio-components.sty"), "utf8"), /\\NewDocumentCommand\{\\makecoverpages\}\{\}\{%[\s\S]*?\\newgeometry\{[^}]+\}\s*\\thispagestyle\{empty\}/);
    assert.match(headingTrackSource, /\\semio@chrome@heading@level=99\\relax/);
    assert.match(headingTrackSource, /\\ifnum\\semio@chrome@heading@candidate@level<\\semio@chrome@heading@level[\s\S]*?\\global\\semio@chrome@heading@level=\\semio@chrome@heading@candidate@level/);
    assert.match(headingTrackSource, /\\ifnum\\semio@chrome@heading@candidate@level=\\semio@chrome@heading@level[\s\S]*?\\semio@chrome@heading@set\{#2\}%[\s\S]*?\\else[\s\S]*?\\global\\let\\semio@nav@short@pending\\relax[\s\S]*?\\expandafter\\markright\\expandafter\{\\semio@chrome@heading\}/);
    assert.match(tableSource, /\\newcommand\{\\semio@table@long@head@copy\}\{%[\s\S]*?\\ifsemio@table@long@pageparts[\s\S]*?\\global\\advance\\semio@table@long@part\\@ne[\s\S]*?\\semio@table@long@part@overlay[\s\S]*?\\copy\\LT@head/);
    assert.match(tableSource, /\\patchcmd\{\\LT@start\}[\s\S]*?\{\\copy\\LT@head\}[\s\S]*?\{\\semio@table@long@head@copy\}/);
    assert.match(tableSource, /\\end\{longtable\}%\s*\\semio@table@long@parts@record/);
    assert.match(tableSource, /\\newcommand\{\\semio@table@long@parts@record\}\{%\s*\\ifsemio@table@long@pageparts[\s\S]*?\\semio@window@break@record/);
    assert.match(tableSource, /\\newcommand\{\\SemioTableLong\}[\s\S]*?\\semio@table@long@pagepartstrue[\s\S]*?\\semio@table@long@render[\s\S]*?\\semio@table@long@pagepartsfalse/);
    assert.match(windowSource, /\\semio_window_vskip_stroke_hairline: \{[\s\S]*?\\vskip\\dimexpr-\\semio@stroke@hairline-5\.75pt\\relax/);
    assert.match(windowSource, /overlay~unbroken=\{\\semio@window@break@record\{1\}/);
    assert.match(windowSource, /overlay~first=\{\\semio@window@frame@bottom@stroke\}/);
    assert.match(windowSource, /overlay~middle=\{[\s\S]*?\\semio@window@frame@bottom@stroke/);
    assert.match(windowSource, /bottomrule~at~break=\\semio@stroke@hairline/);
    assert.match(windowSource, /toprule~at~break=0pt/);
    assert.match(windowSource, /\\semio@window@header@invoke@tcb/);
    assert.match(windowSource, /\\semio@heading@cap@muted@open/);
    assert.match(windowSource, /toprule=0pt/);
    assert.match(tableSource, /\\semio@table@long@title@chrome@row[\s\S]*?\\semio@window@header@invoke/);
    assert.match(tableSource, /\\NewDocumentCommand \\SemioTableHeaderRow \{ m \} \{[\s\S]*?\\semio@table@row@sep/);
    assert.match(tableSource, /\\semio_table_long_header_build:nn #1#2 \{[\s\S]*?\\semio@table@long@header@left@cell[\s\S]*?\\clist_item:nn \{#1\} \{1\}/);
    assert.doesNotMatch(tableSource, /\\newcommand\{\\semio@table@long@header@(repeat|continuation)@three\}\[3\]\{%\s*\\hhline/);

  verifyVisualizationCoverage();
}

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
  console.log(`[DEBUG] API freshness: ${fixture.cases.length} copied metadata cases compared with AJV PASS`);

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
  console.log(`[DEBUG] Printed API: ${printed.cases.length} neutral owner-row cases compared with markdown-it/AJV PASS`);
  for (const vector of (printed as any).declarations) {
    const properties: Record<string, string> = { number: "fp", string: "tl" };
    const body = vector.declarations.map((key: any) => key.name + (key.operation === "typed" ? " ." + properties[key.type] + "_set:N = \\l_value_tl" : " .code:n = {" + ({ store: "\\tl_set:Nn \\l_result_tl {#1}", "prop-store": "\\prop_put:Nnn \\l_result_prop {unknown} {#1}", empty: "", forward: "\\keys_set:nn { other / owner } {#1}" } as Record<string,string>)[key.operation] + "}")).join(", ");
    const tokens = markdown.parse("| Name | Control |\n| --- | --- |\n" + vector.declarations.map((key: any) => "| " + key.name + " | " + key.control + " |").join("\n"), {});
    const cells = tokens.filter((token, index) => token.type === "inline" && tokens[index - 1]?.type === "td_open").map(token => token.content);
    const oracle = cells.filter((_, index) => index % 2 === 0 && cells[index + 1] === "true");
    assert.equal(new (require("ajv").default)().validate({ const: vector.keys }, oracle), true, vector.id + ": independent Markdown/AJV key declarations");
    assert.deepEqual(vizDeclaredKeyNames(body), oracle, vector.id + ": native declared controls");
  }
  console.log("[DEBUG] Stored-result, empty and forwarded unknown declarations compared with markdown-it/AJV PASS");
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
  console.log("[DEBUG] Physical source owner and unsupported comment records compared with markdown-it/AJV PASS");
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
  console.log("[DEBUG] Direct, forwarded and cyclic owner scope identities compared with D3/AJV PASS");
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
  console.log("[DEBUG] Authored-option flow findings: " + JSON.stringify(tokenFindings));
  assert.deepEqual(tokenFindings, [], "Native authored/fixed/residual option paths compared with D3/AJV");
  console.log("[DEBUG] Authored and fixed internal macro option routes compared with D3/AJV PASS");
  const policySource = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8"), policyTables = vizPrintedKeyTables(policySource);
  for (const vector of (printed as any).sourcePolicy) {
    const present = vizImplementedKeyPathNames(vector.scope).includes(vector.key);
    assert.equal(new (require("ajv").default)().validate({ const: vector.allowed }, present), true, vector.scope + ":" + vector.key + ": source-owned control policy");
    if (!vector.allowed) assert.equal(policyTables[vector.table]?.includes(vector.key) ?? false, false, vector.table + ":" + vector.key + ": printed source policy");
  }
  console.log("[DEBUG] Placement/force/network scoped control policy compared with AJV PASS");
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
  console.log("[DEBUG] Source-owned grammar/descriptor findings: " + JSON.stringify(grammarFindings));

  const nativeFindings: string[] = [];
  for (const [name, options] of Object.entries(contracts) as [string, Record<string, { type: string; default?: string }> ][]) {
    const family = families.find((candidate: any) => candidate.name === name);
    for (const [key, expected] of Object.entries(options)) {
      const option = family.options.find((candidate: any) => candidate.name === key);
      const actual = Object.fromEntries(Object.keys(expected).map(property => [property, option?.[property]]));
      if (!new (require("ajv").default)().validate({ const: expected }, actual)) nativeFindings.push(name + ":" + key + ":source-owned-contract");
    }
  }
  console.log("[DEBUG] Native source contract findings: " + JSON.stringify(nativeFindings));
  assert.deepEqual([...grammarFindings, ...nativeFindings], [], "Native grammar and source contracts compared with independent AJV");
  console.log("[DEBUG] Native local vocabulary contracts compared with independent AJV PASS");


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
  console.log(`[DEBUG] API visible caption contract: ${titles.length} scoped EN/de titles, MarkdownIt/AJV and owner admission PASS`);
}

/** 📖️ Compiles the actual document-local title apparatus in both explicitly selected languages. */
export async function verifyPrintApiTitleRendering(): Promise<void> {
  const source = readFileSync(join(productRoot, "🧾️template/📊️viz-api/🔓️viz-api.tex"), "utf8");
  const visual = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.visual;
  const apparatus = source.slice(source.indexOf("%region 🔖️Apparatus"), source.indexOf("%endregion 🔖️Apparatus"));
  mkdirSync(outputRoot, { recursive: true });
  const root = mkdtempSync(join(outputRoot, ".api-title-"));
  for (const language of ["en", "de"] as const) {
    const work = join(root, language), out = join(work, "out"), tex = join(work, "titles.tex");
    mkdirSync(work, { recursive: true });
    const headings = visual.headings.map((heading: any, index: number) => `\\${index === 0 ? "chapter" : "section"}{${heading.tex}}`).join("\n");
    writeFileSync(tex, `\\documentclass[type=report,theme=light,language=${language}]{semio}\n${apparatus}\n\\title{\\ApiText{API title controls}{API-Titelkontrollen}}\n\\author{Semio}\n\\errorcontextlines=100\n\\begin{document}\n\\tableofcontents\n${headings}\n\\ApiScopeSource{api-path-spatial-vector-field}{semio / viz / family / spatial-vector-field}\n\\SemioTableLong{\\ApiTitle{api-path-spatial-vector-field}}{0.18,0.13,0.15,0.54}{\\ApiKey & \\ApiType & \\ApiDefault & \\ApiMeaning}{\\SemioTableRow{\\Key{field} & \\Key{string} & \\Key{shear} & \\ApiText{Named vector field.}{Benanntes Vektorfeld.}}}\n\\end{document}\n`);
    await compilePrintTexOnce(tex, out, work);
    const pdf = await openPrintPdf(join(out, "titles.pdf")), texts: string[] = [], contents: string[] = [];
    let captions = 0;
    try {
      for (let number = 1; number <= pdf.numPages; number++) {
        const page = await pdf.getPage(number), content = await page.getTextContent();
        const items = content.items.filter((item): item is import("pdfjs-dist/types/src/display/api").TextItem => "str" in item);
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
  }
}

/** 🪟️ Measures actual gallery content against its effective PGF frame without changing that frame. */
export async function verifyPrintGalleryCarrier(beforePdfPath?: string): Promise<void> {
  const control = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.galleryCarrier;
  const { extent } = await import("d3-array"), { OPS, Util } = await import("pdfjs-dist/legacy/build/pdf.mjs");
  const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ type: "object", required: ["width", "explicitHeight", "tolerancePt", "cases"], properties: { width: { const: 80 }, explicitHeight: { const: 40 }, tolerancePt: { type: "number", minimum: 0, maximum: 2 }, cases: { type: "array", minItems: 4, items: { type: "object", required: ["slug", "en", "de"], properties: { slug: { type: "string" }, en: { type: "string" }, de: { type: "string" } }, additionalProperties: false } } }, additionalProperties: false });
  assert.ok(validate(control), JSON.stringify(validate.errors));
  const title = { en: "Gallery carrier controls", de: "Galerierahmen-Kontrollen" };
  let source = renderVizGalleryDocument(title, control.cases.map((entry: any) => ({ group: title, leafId: entry.slug, slug: entry.slug })));
  const failures: string[] = [];
  const mark = (name: string) => `\\special{pdf:literal direct /${name} BMC}`;
  const end = "\\special{pdf:literal direct EMC}";
  const reference = (index: number | string) => `\\coordinate (SemioCarrierSW) at (current bounding box.south west);\\coordinate (SemioCarrierNE) at (current bounding box.north east);\n${mark(`SemioFrame${index}`)}\n\\begin{pgfinterruptboundingbox}\\draw[line width=0.01pt] (SemioCarrierSW) rectangle (SemioCarrierNE);\\end{pgfinterruptboundingbox}\n${end}`;
  for (const [index, entry] of control.cases.entries()) source = source.replace(`\\SemioVizChart{${entry.slug}}`, `${mark(`SemioCarrier${index}`)}\n\\SemioVizChart{${entry.slug}}\n${end}\n${reference(index)}`);
  const explicit = `\\begin{VizFigure}[title={\\SemioVizLocalized{Explicit height}{Explizite Höhe}},width=${control.width},height=${control.explicitHeight}]\n${mark("SemioCarrierExplicit")}\n\\draw (0,0) rectangle (10,10);\n${end}\n${reference("Explicit")}\n\\end{VizFigure}`;
  source = source.replace("\\end{document}", explicit + "\n\\end{document}");
  mkdirSync(outputRoot, { recursive: true });
  const root = mkdtempSync(join(outputRoot, ".gallery-carrier-"));
  const box = (points: number[][]) => [...extent(points.map(point => point[0]!)), ...extent(points.map(point => point[1]!))] as [number, number, number, number];
  const documents = beforePdfPath ? [["en", "light"]] as const : [["en", "light"], ["de", "dark"]] as const;
  for (const [language, appearance] of documents) {
    const work = join(root, `${language}-${appearance}`), out = join(work, "out"), tex = join(work, "carrier.tex");
    mkdirSync(work, { recursive: true });
    const documentSource = source.replace("theme=light,language=de", `theme=${appearance},language=${language}`);
    if (beforePdfPath) assert.equal(readFileSync(join(dirname(dirname(beforePdfPath)), "carrier.tex"), "utf8"), documentSource, "retained carrier TeX differs from neutral input");
    else { writeFileSync(tex, documentSource); await compilePrintTexOnce(tex, out, work); }
    const pdf = await openPrintPdf(beforePdfPath ?? join(out, "carrier.pdf"));
    const frames = new Map<string, number[]>(), origins = new Map<string, number>(), pages = new Map<string, number>(), pageWidths = new Map<number, number>(), panels: { page: number; bounds: number[] }[] = [], geometry = new Map<string, number[][]>(), texts = new Map<string, string[]>();
    try {
      for (let number = 1; number <= pdf.numPages; number++) {
        const page = await pdf.getPage(number), operators = await page.getOperatorList(), content = await page.getTextContent({ includeMarkedContent: true });
        pageWidths.set(number, page.getViewport({ scale: 1 }).width);
        const graphics: { matrix: number[]; width: number; join: number; cap: number; limit: number }[] = [], marked: string[] = [];
        let matrix = [1, 0, 0, 1, 0, 0], width = 1, joinStyle = 0, cap = 0, limit = 10;
        for (const [index, id] of operators.fnArray.entries()) {
          const args = operators.argsArray[index];
          if (id === OPS.save || id === OPS.paintFormXObjectBegin) { graphics.push({ matrix: [...matrix], width, join: joinStyle, cap, limit }); if (id === OPS.paintFormXObjectBegin && args[0]) matrix = Util.transform(matrix, args[0]); }
          else if (id === OPS.restore || id === OPS.paintFormXObjectEnd) { const saved = graphics.pop(); assert.ok(saved, "gallery PDF graphics stack underflow"); matrix = saved.matrix; width = saved.width; joinStyle = saved.join; cap = saved.cap; limit = saved.limit; }
          else if (id === OPS.transform) matrix = Util.transform(matrix, args);
          else if (id === OPS.setLineWidth) width = Number(args[0]);
          else if (id === OPS.setLineJoin) joinStyle = Number(args[0]);
          else if (id === OPS.setLineCap) cap = Number(args[0]);
          else if (id === OPS.setMiterLimit) limit = Number(args[0]);
          else if (id === OPS.setGState) { for (const state of args[0]) { if (state[0] === "LW") width = Number(state[1]); else if (state[0] === "LJ") joinStyle = Number(state[1]); else if (state[0] === "LC") cap = Number(state[1]); else if (state[0] === "ML") limit = Number(state[1]); } }
          else if (id === OPS.beginMarkedContent || id === OPS.beginMarkedContentProps) marked.push(String(args[0]?.name ?? args[0]));
          else if (id === OPS.endMarkedContent) marked.pop();
          else if (id === OPS.constructPath && args[2]) {
            const owner = [...marked].reverse().find(name => /^(?:SemioFrame|SemioCarrier)/.test(name)) ?? "", bounds = Array.from(args[2], Number);
            const points = [[bounds[0]!, bounds[1]!], [bounds[2]!, bounds[1]!], [bounds[2]!, bounds[3]!], [bounds[0]!, bounds[3]!]];
            points.forEach(point => Util.applyTransform(point, matrix));
            if (owner.startsWith("SemioFrame")) { assert.ok(!frames.has(owner), `duplicate ${owner}`); frames.set(owner, box(points)); origins.set(owner, matrix[5]!); pages.set(owner, number); }
            else if (owner.startsWith("SemioCarrier")) {
              const stroked = [OPS.stroke, OPS.closeStroke, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(args[0]);
              const reservation = Math.max(joinStyle === 0 ? limit : 1, cap === 2 ? Math.SQRT2 : 1);
              const horizontal = stroked ? reservation * width / 2 * Math.hypot(matrix[0]!, matrix[2]!) : 0, vertical = stroked ? reservation * width / 2 * Math.hypot(matrix[1]!, matrix[3]!) : 0;
              geometry.set(owner, [...(geometry.get(owner) ?? []), ...points.flatMap(point => [[point[0]! - horizontal, point[1]! - vertical], [point[0]! + horizontal, point[1]! + vertical]])]);
            }
            else if ([OPS.fill, OPS.eoFill].includes(args[0])) panels.push({ page: number, bounds: box(points) });
          }
        }
        assert.equal(graphics.length, 0, "unbalanced gallery PDF graphics stack");
        const textMarks: string[] = [];
        for (const item of content.items) {
          if (!("str" in item)) { if (item.type === "beginMarkedContent" || item.type === "beginMarkedContentProps") textMarks.push(String((item as { tag?: string }).tag)); else if (item.type === "endMarkedContent") textMarks.pop(); continue; }
          const owner = [...textMarks].reverse().find(name => name.startsWith("SemioCarrier")) ?? "";
          if (!owner.startsWith("SemioCarrier")) continue;
          texts.set(owner, [...(texts.get(owner) ?? []), item.str]);
          const style = content.styles[item.fontName], ascent = style?.ascent ?? 0.8, descent = style?.descent ?? -0.2;
          const [a, b, c, d, x, y] = item.transform, length = Math.hypot(a!, b!);
          const points = [descent, ascent].flatMap(vertical => [0, item.width].map(horizontal => [x! + a! / length * horizontal + c! * vertical, y! + b! / length * horizontal + d! * vertical]));
          geometry.set(owner, [...(geometry.get(owner) ?? []), ...points]);
        }
      }
      for (const [index, entry] of control.cases.entries()) {
        const frame = frames.get(`SemioFrame${index}`), points = geometry.get(`SemioCarrier${index}`);
        assert.ok(frame && points?.length, `${language}/${entry.slug}: marked frame or content missing`);
        const panel = panels.filter(candidate => candidate.page === pages.get(`SemioFrame${index}`) && candidate.bounds[0]! < frame[0]! && candidate.bounds[1]! > frame[1]! && candidate.bounds[2]! < frame[2]! && candidate.bounds[3]! > frame[3]!).sort((left, right) => (left.bounds[1]! - left.bounds[0]!) * (left.bounds[3]! - left.bounds[2]!) - (right.bounds[1]! - right.bounds[0]!) * (right.bounds[3]! - right.bounds[2]!))[0]?.bounds;
        assert.ok(panel && panel[1]! - panel[0]! < pageWidths.get(pages.get(`SemioFrame${index}`)!)! - 0.1, `${language}/${entry.slug}: actual figure panel missing`);
        const bounds = box(points), tolerance = control.tolerancePt;
        if (bounds[0] < panel[0]! - tolerance || bounds[1] > panel[1]! + tolerance || bounds[2] < panel[2]! - tolerance || bounds[3] > panel[3]! + tolerance) failures.push(`${language}/${entry.slug}: actual content ${JSON.stringify(bounds)} escapes figure body ${JSON.stringify(panel)}`);
        console.log(`[DEBUG] Gallery carrier ${language}/${entry.slug}: frame=${JSON.stringify(frame)} body=${JSON.stringify(panel)} ink=${JSON.stringify(bounds)} textItems=${texts.get(`SemioCarrier${index}`)?.length ?? 0}`);
      }
      const explicitFrame = frames.get("SemioFrameExplicit");
      assert.ok(explicitFrame, "explicit-height reference missing");
      assert.ok(Math.abs(explicitFrame[1]! - explicitFrame[0]! - control.width * 72 / 25.4) < 0.2 && Math.abs(explicitFrame[3]! - explicitFrame[2]! - control.explicitHeight * 72 / 25.4) < 0.2, "explicit width/height contract changed");
      assert.ok(Math.min(...geometry.get("SemioCarrier2")!.map(point => point[1]!)) < origins.get("SemioFrame2")! - 6, "negative legend ink was not measured");
      console.log(`[print] Gallery carrier ${language}/${appearance}: ${control.cases.length} actual frames, marked path/text extents, explicit-height control, ${pdf.numPages} PDF.js pages measured`);
    } finally { await pdf.destroy(); }
  }
  if (failures.length) throw Error(failures.join("\n"));
  assert.doesNotMatch(source, /width=80, height=40/, "gallery must select intrinsic height");
}

type PrintKindPaint = { marker: string; page: number; paths: number[][][]; pathOperations: number[][]; images: number[][][]; glyphs: { text: string; bounds: number[]; font: string; size: number; width: number }[]; bounds: number[]; signature: string; graphicSignature: string; glyphGeometrySignature: string };

/** 🖌️ Extracts only marked chart-body paint through the independent PDF.js operator and glyph readers. */
export async function readPrintKindPaint(pdf: Awaited<ReturnType<typeof openPrintPdf>>): Promise<PrintKindPaint[]> {
  const { OPS, Util } = await import("pdfjs-dist/legacy/build/pdf.mjs"), { extent } = await import("d3-array"), { polygonContains } = await import("d3-polygon"), prefix = "SemioVizLeaf", records = new Map<string, PrintKindPaint>(), seen = new Set<string>();
  const bounds = (points: number[][]): number[] => [...extent(points.map(point => point[0]!)), ...extent(points.map(point => point[1]!))] as number[];
  const filled = (raw: number[], evenOdd: boolean): boolean => {
    const rings: number[][][] = []; let ring: number[][] = [], current: number[] | undefined;
    for (let offset = 0; offset < raw.length;) {
      const operation = raw[offset++]!;
      if (operation === 0) { if (ring.length) rings.push(ring); current = [raw[offset++]!, raw[offset++]!]; ring = [current]; }
      else if (operation === 1) { current = [raw[offset++]!, raw[offset++]!]; ring.push(current); }
      else if (operation === 2 || operation === 3) {
        assert.ok(current, "filled curve requires starting point"); const curve = [current, [raw[offset++]!, raw[offset++]!], ...(operation === 2 ? [[raw[offset++]!, raw[offset++]!]] : []), [raw[offset++]!, raw[offset++]!]];
        for (let sample = 1; sample <= 64; sample++) { const t = sample / 64, u = 1 - t; ring.push([0, 1].map(axis => operation === 2 ? u ** 3 * curve[0]![axis]! + 3 * u * u * t * curve[1]![axis]! + 3 * u * t * t * curve[2]![axis]! + t ** 3 * curve[3]![axis]! : u * u * curve[0]![axis]! + 2 * u * t * curve[1]![axis]! + t * t * curve[2]![axis]!)); } current = curve.at(-1)!;
      } else if (operation === 4) { if (ring.length) rings.push(ring); ring = []; current = undefined; }
      else throw Error("unknown filled path operation " + operation);
    }
    if (ring.length) rings.push(ring);
    const winding = (point: number[]) => rings.reduce((sum, polygon) => sum + polygon.reduce((value, a, index) => { const b = polygon[(index + 1) % polygon.length]!, cross = (b[0]! - a[0]!) * (point[1]! - a[1]!) - (point[0]! - a[0]!) * (b[1]! - a[1]!); return value + (a[1]! <= point[1]! && b[1]! > point[1]! && cross > 0 ? 1 : a[1]! > point[1]! && b[1]! <= point[1]! && cross < 0 ? -1 : 0); }, 0), 0);
    return rings.some(polygon => polygon.some((a, index) => { const b = polygon[(index + 1) % polygon.length]!, dx = b[0]! - a[0]!, dy = b[1]! - a[1]!, length = Math.hypot(dx, dy); if (!length) return false; return [-1, 1].some(sign => { const point = [(a[0]! + b[0]!) / 2 - sign * dy * 1e-7, (a[1]! + b[1]!) / 2 + sign * dx * 1e-7]; return evenOdd ? rings.reduce((count, value) => count + Number(polygonContains(value as [number, number][], point as [number, number])), 0) % 2 !== 0 : winding(point) !== 0; }); }));
  };
  for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber++) {
    const page = await pdf.getPage(pageNumber), operators = await page.getOperatorList(), content = await page.getTextContent({ includeMarkedContent: true }), marked: string[] = [], stack: any[] = [], visibleText = new Set<string>();
    let state = { matrix: [1, 0, 0, 1, 0, 0], width: 1, join: 0, cap: 0, limit: 10, fillAlpha: 1, strokeAlpha: 1, textMode: 0, clip: [-Infinity, Infinity, -Infinity, Infinity] }, pendingClip = false;
    const owner = () => [...marked].reverse().find(name => name.startsWith(prefix));
    for (const [index, id] of operators.fnArray.entries()) {
      const args = operators.argsArray[index];
      if (id === OPS.save || id === OPS.paintFormXObjectBegin) { stack.push({ ...state, matrix: [...state.matrix] }); if (id === OPS.paintFormXObjectBegin && args[0]) state.matrix = Util.transform(state.matrix, args[0]); }
      else if (id === OPS.restore || id === OPS.paintFormXObjectEnd) { assert.ok(stack.length, "kind paint graphics stack underflow"); state = stack.pop(); }
      else if (id === OPS.transform) state.matrix = Util.transform(state.matrix, args);
      else if (id === OPS.setLineWidth) state.width = Number(args[0]);
      else if (id === OPS.setLineJoin) state.join = Number(args[0]);
      else if (id === OPS.setLineCap) state.cap = Number(args[0]);
      else if (id === OPS.setMiterLimit) state.limit = Number(args[0]);
      else if (id === OPS.setTextRenderingMode) state.textMode = Number(args[0]);
      else if (id === OPS.clip || id === OPS.eoClip) pendingClip = true;
      else if (id === OPS.setGState) for (const [key, value] of args[0]) { const field = ({ LW: "width", LJ: "join", LC: "cap", ML: "limit", ca: "fillAlpha", CA: "strokeAlpha" } as Record<string, string>)[key]; if (field) (state as any)[field] = Number(value); }
      else if (id === OPS.beginMarkedContent || id === OPS.beginMarkedContentProps) {
        const name = String(args[0]?.name ?? args[0]); marked.push(name);
        if (name.startsWith(prefix)) { assert.ok(!seen.has(name), `duplicate kind body ${name}`); seen.add(name); records.set(name, { marker: name, page: pageNumber, paths: [], pathOperations: [], images: [], glyphs: [], bounds: [], signature: "", graphicSignature: "", glyphGeometrySignature: "" }); }
      } else if (id === OPS.endMarkedContent) { assert.ok(marked.length, "kind paint marked stack underflow"); marked.pop(); }
      else if ([OPS.showText, OPS.showSpacedText, OPS.nextLineShowText, OPS.nextLineSetSpacingShowText].includes(id)) { const name = owner(), mode = state.textMode % 4; if (name && (mode === 0 && state.fillAlpha > 0 || mode === 1 && state.strokeAlpha > 0 || mode === 2 && Math.max(state.fillAlpha, state.strokeAlpha) > 0)) visibleText.add(name); }
      else if (id === OPS.constructPath && args[2]) {
        const clipPoints = [[args[2][0], args[2][1]], [args[2][2], args[2][1]], [args[2][2], args[2][3]], [args[2][0], args[2][3]]]; clipPoints.forEach(point => Util.applyTransform(point, state.matrix));
        if (pendingClip) { const box = bounds(clipPoints); state.clip = [Math.max(state.clip[0]!, box[0]!), Math.min(state.clip[1]!, box[1]!), Math.max(state.clip[2]!, box[2]!), Math.min(state.clip[3]!, box[3]!)]; pendingClip = false; }
        const name = owner(); if (!name) continue;
        const fill = [OPS.fill, OPS.eoFill, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(args[0]) && state.fillAlpha > 0, stroke = [OPS.stroke, OPS.closeStroke, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(args[0]) && state.strokeAlpha > 0 && state.width > 0;
        if (!fill && !stroke) continue;
        const raw = Array.from(args[1]?.[0] ?? [], Number), points: number[][] = [], pathOperations: number[] = [];
        if (!stroke && !filled(raw, [OPS.eoFill, OPS.eoFillStroke, OPS.closeEOFillStroke].includes(args[0]))) continue;
        for (let offset = 0; offset < raw.length;) { const operation = raw[offset++]!, size = [2, 2, 6, 4, 0][operation]; assert.ok(size !== undefined, `unknown PDF.js path operation ${operation}`); pathOperations.push(operation); for (let pair = 0; pair < size; pair += 2) { const point = [raw[offset + pair]!, raw[offset + pair + 1]!]; Util.applyTransform(point, state.matrix); points.push(point); } offset += size; }
        if (!points.length) continue;
        assert.ok(points.flat().every(Number.isFinite), `${name}: nonfinite painted path`);
        const pathBounds = bounds(points), reservation = Math.max(state.join === 0 ? state.limit : 1, state.cap === 2 ? Math.SQRT2 : 1), horizontal = stroke ? reservation * state.width / 2 * Math.hypot(state.matrix[0]!, state.matrix[2]!) : 0, vertical = stroke ? reservation * state.width / 2 * Math.hypot(state.matrix[1]!, state.matrix[3]!) : 0;
        if (!stroke && (pathBounds[0] === pathBounds[1] || pathBounds[2] === pathBounds[3])) continue;
        const box = [Math.max(pathBounds[0]! - horizontal, state.clip[0]!), Math.min(pathBounds[1]! + horizontal, state.clip[1]!), Math.max(pathBounds[2]! - vertical, state.clip[2]!), Math.min(pathBounds[3]! + vertical, state.clip[3]!)]; if (box[0]! >= box[1]! || box[2]! >= box[3]!) continue;
        const record = records.get(name)!; record.paths.push(points); record.pathOperations.push(pathOperations); record.bounds.push(...box);
      } else if ([OPS.paintImageXObject, OPS.paintInlineImageXObject].includes(id)) {
        const name = owner(); if (!name || state.fillAlpha <= 0) continue;
        const points = [[0, 0], [1, 0], [1, 1], [0, 1]]; points.forEach(point => Util.applyTransform(point, state.matrix)); const imageBounds = bounds(points), box = [Math.max(imageBounds[0]!, state.clip[0]!), Math.min(imageBounds[1]!, state.clip[1]!), Math.max(imageBounds[2]!, state.clip[2]!), Math.min(imageBounds[3]!, state.clip[3]!)]; if (box[0]! >= box[1]! || box[2]! >= box[3]!) continue;
        const record = records.get(name)!; record.images.push(points); record.bounds.push(...box);
      }
    }
    assert.equal(stack.length, 0, "unbalanced kind paint graphics stack"); assert.equal(marked.length, 0, "unbalanced kind paint marked stack");
    const textMarks: string[] = [];
    for (const item of content.items) {
      if (!("str" in item)) { if (item.type === "beginMarkedContent" || item.type === "beginMarkedContentProps") textMarks.push(String((item as { tag?: string }).tag)); else if (item.type === "endMarkedContent") textMarks.pop(); continue; }
      const name = [...textMarks].reverse().find(mark => mark.startsWith(prefix)); if (!name || !visibleText.has(name) || !item.str.trim() || item.width <= 0) continue;
      const style = content.styles[item.fontName], [a, b, c, d, x, y] = item.transform, length = Math.hypot(a!, b!); if (!length) continue;
      const points = [style?.descent ?? -0.2, style?.ascent ?? 0.8].flatMap(vertical => [0, item.width].map(horizontal => [x! + a! / length * horizontal + c! * vertical, y! + b! / length * horizontal + d! * vertical]));
      assert.ok(points.flat().every(Number.isFinite), `${name}: nonfinite glyph ink`);
      records.get(name)!.glyphs.push({ text: item.str, bounds: bounds(points), font: item.fontName, size: length, width: item.width });
    }
    page.cleanup();
  }
  for (const record of records.values()) {
    const boxes = Array.from({ length: record.bounds.length / 4 }, (_, index) => record.bounds.slice(index * 4, index * 4 + 4)), glyphBounds = record.glyphs.map(glyph => glyph.bounds), all = [...boxes, ...glyphBounds];
    record.bounds = all.length ? [Math.min(...all.map(box => box[0]!)), Math.max(...all.map(box => box[1]!)), Math.min(...all.map(box => box[2]!)), Math.max(...all.map(box => box[3]!))] : [];
    const paths = record.paths.flat(), x = paths.length ? Math.min(...paths.map(point => point[0]!)) : record.bounds[0] ?? 0, y = paths.length ? Math.min(...paths.map(point => point[1]!)) : record.bounds[2] ?? 0, round = (value: number) => Math.round(value * 1000) / 1000;
    record.signature = createHash("sha256").update(JSON.stringify({ paths: record.paths.map(path => path.map(point => [round(point[0]! - x), round(point[1]! - y)])), images: record.images.map(image => image.map(point => [round(point[0]! - x), round(point[1]! - y)])), glyphs: record.glyphs.map(glyph => ({ text: glyph.text, bounds: glyph.bounds.map((value, index) => round(value - (index < 2 ? x : y))), size: round(glyph.size), width: round(glyph.width) })) })).digest("hex");
    record.graphicSignature = createHash("sha256").update(JSON.stringify({ operations: record.pathOperations, paths: record.paths.map(path => path.map(point => [round(point[0]! - x), round(point[1]! - y)])), images: record.images.map(image => image.map(point => [round(point[0]! - x), round(point[1]! - y)])) })).digest("hex");
    record.glyphGeometrySignature = createHash("sha256").update(JSON.stringify(record.glyphs.map(glyph => ({ bounds: glyph.bounds.map((value, index) => round(value - (index < 2 ? x : y))), size: round(glyph.size), width: round(glyph.width) })))).digest("hex");
  }
  return [...records.values()];
}

/** 🏷️ Requires exact authored leaf ownership and source-backed graphic or typographic chart-body ink. */
export async function verifyPrintGalleryPaintOwnership(path: string, leaves: readonly { leafId: string; slug: string }[]): Promise<PrintKindPaint[]> {
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🖌️kind-paint/🔣️.json"), "utf8")), pdf = await openPrintPdf(path);
  try {
    const actual = await readPrintKindPaint(pdf); assertPrintKindPaintOwnership(path, leaves, actual, contract);
    return actual;
  } finally { await pdf.destroy(); }
}

/** 🧷️ Joins independently consumed paint with exact authored leaf markers and source-backed paint classes. */
function assertPrintKindPaintOwnership(path: string, leaves: readonly { leafId: string; slug: string }[], actual: PrintKindPaint[], contract: { markerPrefix: string; glyphKinds: { slug: string }[] }): void {
  const expected = leaves.map(leaf => contract.markerPrefix + Buffer.from(leaf.leafId).toString("hex"));
  assert.deepEqual(actual.map(record => record.marker).sort(), [...expected].sort(), `${path}: exact marked chart-body census`);
  for (const [index, leaf] of leaves.entries()) { const record = actual.find(entry => entry.marker === expected[index])!, glyph = contract.glyphKinds.some(entry => entry.slug === leaf.slug); assert.ok(record.paths.length > 0 || record.images.length > 0 || glyph && record.glyphs.length > 0, `${leaf.leafId}/${leaf.slug}: no authored ${glyph ? "graphic/glyph" : "graphic"} body paint`); }
}

/** 🧫️ Compiles positive and negative marked-body controls against independent PDF.js, D3 and AJV. */
export async function verifyPrintKindPaintFixtures(): Promise<void> {
  const root = join(import.meta.dir, "../../🧫️fixtures/🖌️kind-paint"), contract = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")), validate = new (createRequire(import.meta.url)("ajv").default)().compile(JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"))), { extent } = await import("d3-array");
  assert.ok(validate(contract), JSON.stringify(validate.errors));
  for (const entry of contract.glyphKinds) { assert.equal(loadVizCatalog().kinds.find(kind => kind.slug === entry.slug)?.family, entry.family, entry.slug); assert.ok(readFileSync(join(latexRoot, entry.source), "utf8").includes(entry.definition), entry.slug + ": source-backed glyph painter"); }
  const work = mkdtempSync(join(outputRoot, ".kind-paint-"));
  for (const [language, appearance] of [["en", "light"], ["de", "dark"]] as const) {
    const directory = join(work, language + "-" + appearance), tex = join(directory, "paint.tex"), out = join(directory, "out"); mkdirSync(directory, { recursive: true });
    const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas"), canvas = createCanvas(2, 1), context = canvas.getContext("2d"); context.fillStyle = "#3768a8"; context.fillRect(0, 0, 2, 1); writeFileSync(join(directory, "neutral.png"), canvas.toBuffer("image/png"));
    const body = contract.cases.map((entry: any) => `${entry.chart ? "\\begin{VizFigure}[width=80,height=40]" : "\\begin{tikzpicture}[x=1mm,y=1mm]\\draw (-1,-1) rectangle (25,16);"}\\special{pdf:literal direct /${contract.markerPrefix + Buffer.from(entry.id).toString("hex")} BMC}\\begin{scope}${entry.body}\\end{scope}\\special{pdf:literal direct EMC}${entry.chart ? "\\end{VizFigure}" : "\\end{tikzpicture}"}`).join("\\par\n");
    writeFileSync(tex, `\\documentclass[type=report,theme=${appearance},language=${language}]{semio}\\title{Paint}\\author{Semio}\\date{}\\begin{document}\\chapter{Paint}${body}\\end{document}`); await compilePrintTexOnce(tex, out, directory);
    const pdf = await openPrintPdf(join(out, "paint.pdf"));
    try {
      const actual = await readPrintKindPaint(pdf);
      for (const entry of contract.cases) {
        const record = actual.find(row => row.marker === contract.markerPrefix + Buffer.from(entry.id).toString("hex")); assert.ok(record, entry.id + ": neutral marker missing");
        const glyphClass=entry.catalogueSlug?contract.glyphKinds.some((kind:any)=>kind.slug===entry.catalogueSlug):entry.class==="glyph";
        assert.equal(record.paths.length > 0 || record.images.length > 0 || glyphClass && record.glyphs.length > 0, entry.valid, `${language}/${entry.id}: independently extracted body paint class`);
        if (entry.expectedData) { const { scaleLinear } = await import("d3-scale"), x = scaleLinear(extent<number>(entry.expectedData.map((row: number[]) => row[0]!)) as [number, number], [8, 72]), y = scaleLinear(extent<number>(entry.expectedData.map((row: number[]) => row[1]!)) as [number, number], [8, 32]), centers = record.glyphs.map(glyph => [(glyph.bounds[0]! + glyph.bounds[1]!) / 2, (glyph.bounds[2]! + glyph.bounds[3]!) / 2]); assert.deepEqual(record.glyphs.map(glyph => glyph.text), entry.expectedData.map((row: number[]) => String(row[1]))); for (const [index, row] of entry.expectedData.entries()) { assert.ok(Math.abs((centers[index]![0]! - centers[0]![0]!) - (x(row[0]) - x(entry.expectedData[0][0])) * 72 / 25.4) < contract.tolerancePt, entry.id + ": D3 bound horizontal glyph displacement"); assert.ok(Math.abs((centers[index]![1]! - centers[0]![1]!) - (y(row[1]) - y(entry.expectedData[0][1])) * 72 / 25.4) < contract.tolerancePt, entry.id + ": D3 bound vertical glyph displacement"); } }
        if (entry.expectedBoundsMm) { const points = record.paths.flat(), expected = [entry.expectedBoundsMm[0], entry.expectedBoundsMm[1], entry.expectedBoundsMm[2], entry.expectedBoundsMm[3]], dx = extent(points.map(point => point[0]!)), dy = extent(points.map(point => point[1]!)); assert.ok(Math.abs((dx[1]! - dx[0]!) - (expected[1]! - expected[0]!) * 72 / 25.4) < contract.tolerancePt && Math.abs((dy[1]! - dy[0]!) - (expected[3]! - expected[2]!) * 72 / 25.4) < contract.tolerancePt, entry.id + ": authored D3 transformed dimensions"); }
      }
      console.log(`[DEBUG] Kind paint ${language}/${appearance}: ${contract.cases.length} actual graphic/glyph/empty/outer-only/transparent/clip controls PASS`);
    } finally { await pdf.destroy(); }
  }
}

/** 🧪️ Adjudicates semantic rows and explicit inherited controls against independent markdown-it tables. */
function verifyPrintApiScopeCases(printed: any): void {
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
  console.log("[DEBUG] Package whole-row source ownership compared with markdown-it/AJV PASS");
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
  console.log("[DEBUG] Printed API semantic/inheritance: " + inherited.cases.length + " independent markdown-it/AJV cases PASS");
  for (const vector of printed.scalars) {
    const row = { name: "values", type: vector.renderedType ?? vector.type, default: vector.rendered, meaning: { en: "Values.", de: "Werte." } };
    const visible = oracleRows({ rows: [row] })[0]!;
    const list = (value: string): readonly string[] => value.split(",").map(part => part.trim().toLowerCase());
    const kinds = (value: string): readonly string[] => value.split("|").map(part => printed.typeClasses?.[part.trim()] ?? part.trim().toLowerCase()).sort();
    const expected = [...(JSON.stringify(list(visible.default)) === JSON.stringify(list(vector.expected)) ? [] : ["example:values:default"]), ...(JSON.stringify(kinds(visible.type)) === JSON.stringify(kinds(vector.type)) ? [] : ["example:values:type"])].sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.findings }, expected), true, vector.id + ": independent scalar semantics");
    assert.deepEqual(vizPrintedScopeFindings("\\subsection{\\Key{example}}" + tableTex({ name: "example", rows: [row] }), { example: { values: { type: vector.type, default: vector.expected } } }, {}, {}), expected, vector.id);
  }
  console.log("[DEBUG] Printed list spacing, order and scalar union contracts compared with markdown-it/AJV PASS");
  for (const vector of printed.groupedSpecs) {
    const visible = oracleRows({ rows: vector.keys.map((key: any, index: number) => ({ name: key.name, type: printed.typeClasses[vector.types[index]] ?? vector.types[index], default: vector.defaults[index], meaning: { en: "Geometry and caption.", de: "Geometrie und Beschriftung." } })) });
    const expected = vector.keys.flatMap((key: any, index: number) => [...(visible[index].type === key.type ? [] : ["example:" + key.name + ":type"]), ...(visible[index].default === key.default ? [] : ["example:" + key.name + ":default"])]).sort();
    assert.equal(new (require("ajv").default)().validate({ const: vector.findings }, expected), true, vector.id + ": independent grouped type/default semantics");
    const tokens = (values: readonly string[]): string => values.map(value => "\\Key{" + value + "}").join(", ");
    const source = "\\subsection{\\Key{example}}\\SemioTableLong{\\Key{example}}{1,1,1,1}{\\ApiKey & Type & Default & Meaning}{\\SemioTableRow{" + tokens(vector.keys.map((key: any) => key.name)) + " & " + tokens(vector.types) + " & " + tokens(vector.defaults) + " & \\ApiText{Geometry and caption.}{Geometrie und Beschriftung.}}}";
    assert.deepEqual(vizPrintedScopeFindings(source, { example: Object.fromEntries(vector.keys.map((key: any) => [key.name, { type: key.type, default: key.default }])) }, {}, {}), expected, vector.id);
  }
  console.log("[DEBUG] Mixed grouped key types/defaults compared with markdown-it/AJV PASS");
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
  console.log("[DEBUG] Shared translation reachability and local precedence compared with D3/AJV PASS");
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
  console.log("[DEBUG] Printed shared source contracts: " + printed.sharedSourceContracts.scopes.reduce((total: number, scope: any) => total + scope.keys.length, 0) + " exact owner/type/default/bilingual rows PASS");
  const packageFindings = vizPrintedPackageFindings(familySource);
  mkdirSync(outputRoot, { recursive: true });
  writeFileSync(join(outputRoot, "printed-api-package-findings.json"), JSON.stringify(packageFindings, null, 2));
  console.log("[DEBUG] Printed API actual package findings: " + JSON.stringify(packageFindings));
  assert.deepEqual(packageFindings, [], "Every actual native owner control has a complete rendered contract");
  const familyFindings = vizPrintedFamilyFindings(familySource, printed.scopeBindings);
  mkdirSync(outputRoot, { recursive: true });
  writeFileSync(join(outputRoot, "printed-api-semantic-findings.json"), JSON.stringify({ translations: { expected, actual }, families: familyFindings, packages: packageFindings }, null, 2));
  console.log("[DEBUG] Printed API actual candidate: " + familyFindings.length + " scoped family findings; " + actual.filter((value: string, index: number) => value !== expected[index]).length + " translation scope failures");
  assert.deepEqual({ translations: actual, families: familyFindings, packages: packageFindings }, { translations: expected, families: [], packages: [] }, "Current owner-scoped semantic API reference");
}

/** 📥️ Verifies canonical macro materialization and pre-mutation collision rejection. */
export function verifyPrintMacroStaging(): void {
  const fixtureRoot = join(import.meta.dir, "../../🧫️fixtures/🖨️macro-staging");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
    readonly entries: readonly string[];
    readonly inputs: Readonly<Record<string, string>>;
    readonly outputs: Readonly<Record<string, string>>;
    readonly sourceKind: { readonly canonicalPath: string; readonly scopedKindId: string; readonly hostileBasenames: readonly string[]; readonly neighborPath: string; readonly neighborFileKindId: string };
    readonly shimGrammar: { readonly contractId: string; readonly grammarId: string; readonly acceptedRole: string; readonly rejections: readonly { readonly id: string; readonly suffix?: string; readonly source?: string; readonly expectedRole: string }[] };
    readonly collisions: readonly { readonly id: string; readonly entries: readonly string[]; readonly inputs: Readonly<Record<string, string>> }[];
  };
  const taxonomy = loadCatalogTaxonomy(), scopedFileKindId = scopedFileKindIdForSourcePath(fixture.sourceKind.canonicalPath, taxonomy);
  assert.equal(scopedFileKindId, fixture.sourceKind.scopedKindId);
  assert.equal(taxonomyFileKindIsImplementation(`scoped:${scopedFileKindId}`, taxonomy), true);
  assert.equal(implementationLeafBasenameFinding(fixture.sourceKind.canonicalPath, taxonomy), null);
  for (const basename of fixture.sourceKind.hostileBasenames) {
    const path = `${dirname(fixture.sourceKind.canonicalPath)}/${basename}`;
    assert.equal(scopedFileKindIdForSourcePath(path, taxonomy), fixture.sourceKind.scopedKindId, basename);
    assert.deepEqual(implementationLeafBasenameFinding(path, taxonomy), { breachId: "taxonomy/kind-only-basename", path, fileKindId: `scoped:${fixture.sourceKind.scopedKindId}`, actualBasename: basename, expectedBasename: "📐️.tex", exemptionAuthorityId: null });
  }
  assert.equal(scopedFileKindIdForSourcePath(fixture.sourceKind.neighborPath, taxonomy), null);
  assert.equal(fileKindIdForSourcePath(fixture.sourceKind.neighborPath, taxonomy), fixture.sourceKind.neighborFileKindId);
  assert.equal(taxonomyFileKindIsImplementation(fixture.sourceKind.neighborFileKindId, taxonomy), false);
  const grammar = taxonomy.packageGlueGrammar[fixture.shimGrammar.grammarId]!, shim = readFileSync(join(latexRoot, "semio-graph.sty"), "utf8");
  assert.equal(classifyPackageSource(shim, grammar).role, fixture.shimGrammar.acceptedRole);
  assert.equal(fixedSourceDispositionDecision(fixture.shimGrammar.contractId, shim, taxonomy)?.finding, null);
  for (const rejection of fixture.shimGrammar.rejections) {
    const source = rejection.source ?? shim + rejection.suffix!;
    assert.equal(classifyPackageSource(source, grammar).role, rejection.expectedRole, rejection.id);
    assert.equal(fixedSourceDispositionDecision(fixture.shimGrammar.contractId, source, taxonomy)?.finding, "fixed-source-disposition-unresolved", rejection.id);
  }
  assert.equal(classifyPackageSource(readFileSync(join(productRoot, "🔨️modules/🕸️graph/📐️.tex"), "utf8"), grammar).role, "implementation");
  mkdirSync(outputRoot, { recursive: true });
  const root = mkdtempSync(join(outputRoot, ".macro-staging-test-"));
  try {
    const input = join(root, "input"), output = join(root, "output");
    for (const [path, content] of Object.entries(fixture.inputs)) {
      mkdirSync(dirname(join(input, path)), { recursive: true });
      writeFileSync(join(input, path), content);
    }
    const staged = stagePrintSources(input, fixture.entries, output);
    assert.equal(staged.get("🔨️modules/🕸️graph/📐️.tex"), join(output, "modules/graph/_.tex"));
    for (const [path, content] of Object.entries(fixture.outputs)) assert.equal(readFileSync(join(output, path), "utf8"), content, path);
    for (const collision of fixture.collisions) {
      const source = join(root, collision.id), destination = join(root, `${collision.id}-output`);
      for (const [path, content] of Object.entries(collision.inputs)) {
        mkdirSync(dirname(join(source, path)), { recursive: true });
        writeFileSync(join(source, path), content);
      }
      mkdirSync(destination, { recursive: true });
      writeFileSync(join(destination, "sentinel"), collision.id);
      assert.throws(() => stagePrintSources(source, collision.entries, destination), /colliding print source/);
      assert.equal(readFileSync(join(destination, "sentinel"), "utf8"), collision.id);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🧪️ Compiles the semantic graph owner twice through the production stage and independently reads its PDF. */
export async function verifyPrintMacroStagingNative(): Promise<void> {
  mkdirSync(outputRoot, { recursive: true });
  const root = mkdtempSync(join(outputRoot, ".macro-staging-native-")), sourceRoot = join(import.meta.dir, "../../🧫️fixtures/🖨️macro-staging");
  try {
    const outputs = [join(root, "first"), join(root, "second")];
    for (const [index, output] of outputs.entries()) await publishPrintArtifact({ id: `graph-macro-native-${index + 1}`, sourceRoot, texPath: "🧪️control.tex", sources: ["🧪️control.tex"], output, owner: `@semio-tech/print:test-macro-staging-${index + 1}`, dark: false });
    const pdfs = outputs.map(output => readFileSync(join(output, "🧪️control.pdf")));
    assert.deepEqual(pdfs[0], pdfs[1]);
    const canvas = createRequire(join(workspaceRoot, "node_modules/pdfjs-dist/legacy/build/pdf.mjs"))("@napi-rs/canvas") as typeof import("@napi-rs/canvas");
    (globalThis as { DOMMatrix?: unknown }).DOMMatrix ??= canvas.DOMMatrix;
    const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs"), pdf = await getDocument({ data: new Uint8Array(pdfs[0]!) }).promise;
    try {
      const text: string[] = [];
      for (let index = 1; index <= pdf.numPages; index++) {
        const page = await pdf.getPage(index);
        text.push((await page.getTextContent()).items.map(item => "str" in item ? item.str : "").join(" "));
        page.cleanup();
      }
      assert.ok(text.join(" ").replace(/\s+/g, " ").includes("Graph macro staging reached"));
    } finally { await pdf.destroy(); }
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🪪️ Requires published text to contain no replacement markers or Unicode noncharacters. */
function assertPrintPdfGlyphs(text: string, source: string): void {
  for (const glyph of text) {
    const point = glyph.codePointAt(0)!;
    if (point === 0xFFFD || point >= 0xFDD0 && point <= 0xFDEF || (point & 0xFFFF) >= 0xFFFE) assert.fail(`${source}: invalid published glyph U+${point.toString(16).toUpperCase().padStart(4, "0")}`);
  }
}

/** 🔎️ Opens an authored PDF with the independent reader and its native matrix provider. */
async function openPrintPdf(path: string) {
  const canvas = createRequire(join(workspaceRoot, "node_modules/pdfjs-dist/legacy/build/pdf.mjs"))("@napi-rs/canvas") as typeof import("@napi-rs/canvas");
  (globalThis as { DOMMatrix?: typeof canvas.DOMMatrix }).DOMMatrix ??= canvas.DOMMatrix;
  const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs");
  return await getDocument({ data: new Uint8Array(readFileSync(path)) }).promise;
}

/** 🧩️ Checks a neutral glyph policy and an optional actual missing-glyph compiler counterexample. */
async function verifyPrintPdfGlyphPolicy(): Promise<void> {
  const policy = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️pdf-consumption.json"), "utf8")).glyphPolicy as { accepted: string[]; rejectedCodePoints: number[] };
  const before = process.env.SEMIO_PRINT_GLYPH_REGRESSION_PDF;
  if (before) {
    const pdf = await openPrintPdf(before);
    let observed = 0;
    try {
      for (let page = 1; page <= pdf.numPages; page++) {
        const current = await pdf.getPage(page), text = (await current.getTextContent()).items.map(item => "str" in item ? item.str : "").join(" ");
        if (text.includes("\uFFFF")) { observed++; assert.throws(() => assertPrintPdfGlyphs(text, `${basename(before)}:${page}`), /invalid published glyph/); }
        current.cleanup();
      }
      assert.ok(observed > 0, "actual compiler counterexample has no observed U+FFFF");
      console.log(`[print] PDF.js missing-glyph counterexample: ${observed} pages rejected PASS`);
    } finally { await pdf.destroy(); }
  }
  for (const text of policy.accepted) assert.doesNotThrow(() => assertPrintPdfGlyphs(text, "neutral accepted glyphs"));
  for (const point of policy.rejectedCodePoints) assert.throws(() => assertPrintPdfGlyphs(String.fromCodePoint(point), "neutral rejected glyph"), /invalid published glyph/);
  console.log(`[print] Published glyph policy: ${policy.accepted.length} accepted cases, ${policy.rejectedCodePoints.length} rejected scalars PASS`);
}

/** 📖️ Consumes each restored document through the independent PDF.js page and text reader. */
async function verifyPrintPdfs(templates: readonly { id: string; texPath: string }[], collection: "templates" | "visualizations"): Promise<void> {
  const expected = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️pdf-consumption.json"), "utf8"));
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️merge-contract.json"), "utf8"));
  const visual = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.visual;
  const painted: any[] = [], paintContract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🖌️kind-paint/🔣️.json"), "utf8"));
  let count = 0;
  for (const template of templates) for (const name of Object.values(printTemplatePdfNames(template.texPath))) {
    const pdf = await openPrintPdf(join(printDocumentOutputDirectory(template.id), name));
    try {
      assert.ok(pdf.numPages >= expected.minimumPages, `${name}: empty document`);
      const texts: string[] = [];
      let contents = 0, captions = 0;
      for (let index = 1; index <= pdf.numPages; index++) {
        const page = await pdf.getPage(index);
        const items = (await page.getTextContent()).items.filter((item): item is import("pdfjs-dist/types/src/display/api").TextItem => "str" in item);
        const text = items.map(item => item.str).join(" ");
        assertPrintPdfGlyphs(text, `${name}:${index}`);
        if (template.id === "viz-api") {
          if (/(?:Inhaltsverzeichnis|Table\s+of\s+contents)/i.test(text)) {
            for (const token of visual.forbiddenContents) assert.ok(!text.replace(/\s+/g, "").includes(token), `${name}:${index}: raw contents token ${token}`);
            contents++;
          }
          for (const badge of items.filter(item => /^(?:Table|Tabelle):/.test(item.str))) {
            const row = items.filter(item => Math.abs(item.transform[5] - badge.transform[5]) < 2 && item.transform[4] < badge.transform[4]);
            if (!visual.titles.some((title: string[]) => title.slice(1).some(value => row.map(item => item.str).join(" ").includes(value)))) continue;
            assert.ok(Math.max(...row.map(item => item.transform[4] + item.width)) + visual.minimumCaptionGapPt <= badge.transform[4], `${name}:${index}: visible API caption overlaps number badge`);
            captions++;
          }
        }
        texts.push(text);
        page.cleanup();
      }
      assert.ok(texts.join(" ").trim(), `${name}: no readable text`);
      if (template.id === "viz-api") {
        for (const text of contract.vizText) assert.ok(texts.join(" ").includes(text), `${name}: missing ${text}`);
        assert.ok(contents > 0 && captions > 0, `${name}: missing measured contents/caption pages`);
        console.log(`[print] API visible titles ${name}: ${contents} contents pages, ${captions} measured captions PASS`);
      } else if (collection === "visualizations") {
        const source = readFileSync(join(productRoot, template.texPath), "utf8"), leaves = [...source.matchAll(/% viz-covers: ([^\r\n]+)[\s\S]*?\\SemioVizChart\{([^}]+)\}/g)].map(match => ({ leafId: match[1]!, slug: match[2]! }));
        assert.ok(leaves.length, `${template.id}: authored leaf/chart mapping missing`);
        const records = await readPrintKindPaint(pdf), catalog = loadVizCatalog(); assertPrintKindPaintOwnership(join(printDocumentOutputDirectory(template.id), name), leaves, records, paintContract);
        for (const leaf of leaves) { const kind = catalog.kinds.find(kind => kind.slug === leaf.slug); assert.ok(kind, leaf.slug); painted.push({ theme: name.includes("-dark") ? "dark" : "light", document: template.id, leafId: leaf.leafId, slug: leaf.slug, family: kind.family, options: kind.options, paintClass: paintContract.glyphKinds.some((entry: any) => entry.slug === leaf.slug) ? "glyph" : "graphic", ...records.find(record => record.marker === paintContract.markerPrefix + Buffer.from(leaf.leafId).toString("hex")) }); }
        writeFileSync(join(outputRoot, `kind-painted-${template.id}-${name.includes("-dark") ? "dark" : "light"}.json`), JSON.stringify(painted.slice(-leaves.length), null, 2));
        console.log(`[DEBUG] Painted chart-body census ${template.id}/${name}: ${leaves.length} exact leaf instances PASS`);
      }
      count++;
      console.log(`[print] Consumed ${collection} ${template.id}/${name}: ${pdf.numPages} pages PASS (${count}/${expected[collection]})`);
    } finally { await pdf.destroy(); }
  }
  assert.equal(count, expected[collection]);
  if (collection === "visualizations") {
    const taxonomy = parseVizTaxonomyLeaves(readFileSync(join(productRoot, "🖼️assets/📊️viz-taxonomy.md"), "utf8")), catalog = loadVizCatalog();
    for (const theme of ["light", "dark"]) { const rows = painted.filter(row => row.theme === theme); assert.equal(rows.length, paintContract.leaves, `${theme}: exact leaf paint count`); assert.deepEqual(rows.map(row => row.leafId).sort(), [...taxonomy].sort(), `${theme}: exact taxonomy paint set`); assert.equal(new Set(rows.map(row => row.slug)).size, paintContract.kinds, `${theme}: unique painted kinds`); assert.deepEqual([...new Set(rows.map(row => row.slug))].sort(), catalog.kinds.map(kind => kind.slug).sort(), `${theme}: exact kind paint set`); }
    writeFileSync(join(outputRoot, "kind-painted-coverage.json"), JSON.stringify(painted, null, 2));
    const groups = new Map<string, any[]>(); for (const row of painted) { const key = [row.theme, row.family, row.paintClass === "glyph" ? row.glyphGeometrySignature : row.graphicSignature].join("/"); groups.set(key, [...(groups.get(key) ?? []), { leafId: row.leafId, slug: row.slug, options: row.options }]); }
    writeFileSync(join(outputRoot, "kind-painted-equal-geometry.json"), JSON.stringify([...groups].filter(([, rows]) => new Set(rows.map(row => row.slug)).size > 1).map(([group, rows]) => ({ group, rows })), null, 2));
    console.log(`[DEBUG] Painted chart-body coverage: ${paintContract.leaves} exact leaves/${paintContract.kinds} unique kinds in each theme PASS; equal geometry retained for source/feature adjudication`);
  }
  console.log(`[print] PDF.js consumption ${collection}: ${count} PDFs PASS`);
}

/** 🧪️ Verifies the light and dark PDFs prepared by explicit Nx document prerequisites. */
export async function verifyPrintPipelineLong(): Promise<void> {
  await verifyPrintPdfs(registeredPrintTemplates(), "templates");
}
//#endregion 🧪️PrintPipelineTests

/** 🧪️ Verifies every visualization PDF and the authored API text contract. */
export async function verifyPrintVisualizationBuild(): Promise<void> {
  await verifyPrintPdfs(visualizationTemplates(), "visualizations");
}

/** 🔤️ Verifies font publication against the schema and an independent native font loader. */
export async function verifyPrintFontStaging(output = outputRoot): Promise<void> {
  const require = createRequire(import.meta.url), modulePath = join(productRoot, "🔨️modules/🔤print-font-catalog");
  const catalog = JSON.parse(readFileSync(join(modulePath, "🔣️.json"), "utf8"));
  assert.equal(new (require("ajv").default)({ strict: false }).validate(printSchemaModule(modulePath, "https://json.schemas.assets.semio-tech.com/print/print-font-catalog/schema.json"), catalog), true);
  assert.deepEqual(printFontDescriptors(), catalog);
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "print-fonts-")), product = "🧰️framework/🛍️products/📓️print";
  try {
    for (const row of catalog) {
      const path = join(product, "🖼️assets/🔤️font", row.directory, row.filename);
      mkdirSync(dirname(join(root, path)), { recursive: true }); copyFileSync(join(workspaceRoot, path), join(root, path));
    }
    await stagePrintFonts(root);
    const staged = printFontSearchPaths(root)[0]!;
    assert.equal(staged, join(root, product, "📦️packages/🟦️typescript/dist/fonts"));
    const { GlobalFonts } = require("@napi-rs/canvas");
    for (const row of catalog) {
      const path = join(staged, row.texFilename);
      assert.deepEqual(readFileSync(path), readFileSync(join(root, product, "🖼️assets/🔤️font", row.directory, row.filename)));
      const key = GlobalFonts.registerFromPath(path, "nx-font-fixture-" + row.family);
      assert.ok(key, row.family); GlobalFonts.remove(key);
    }
    const repeated = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔁️font-publication.json"), "utf8"));
    const identity = () => catalog.map((row: any) => { const stat = statSync(join(staged, row.texFilename), { bigint: true }); return [stat.ino.toString(), stat.birthtimeNs.toString()]; });
    const original = identity(), registrations = catalog.map((row: any) => GlobalFonts.registerFromPath(join(staged, row.texFilename), "nx-live-font-fixture-" + row.family));
    try {
      assert.ok(registrations.every(Boolean));
      for (let round = 0; round < repeated.rounds; round++) await Promise.all(Array.from({ length: repeated.concurrent }, () => stagePrintFonts(root)));
      assert.deepEqual(identity(), original, "Identical live fonts must retain file identity");
    } finally { for (const key of registrations) if (key) GlobalFonts.remove(key); }
    writeFileSync(join(staged, "stale.ttf"), "stale");
    await stagePrintFonts(root);
    assert.equal(existsSync(join(staged, "stale.ttf")), false);
    assert.deepEqual(JSON.parse(readFileSync(join(staged, ".nx-artifact.json"), "utf8")).files, catalog.map((row: any) => row.texFilename).sort());
    const prior = readFileSync(join(staged, catalog[0].texFilename));
    const invalidSource = join(root, product, "🖼️assets/🔤️font", catalog[0].directory, catalog[0].filename);
    writeFileSync(invalidSource, "invalid");
    await assert.rejects(() => stagePrintFonts(root), { message: `Print font is not OpenType: ${invalidSource}` });
    assert.deepEqual(readFileSync(join(staged, catalog[0].texFilename)), prior);
  } finally { rmSync(root, { recursive: true }); }
}

/** 🧱️ Enforces the neutral reviewed source identities independent of graph size and enumeration order. */
function assertPrintCommandSourceInputs(actual: readonly string[], expected: readonly string[], owner: string): void {
  assert.deepEqual(actual.map(path => path.replaceAll("\\", "/")).sort(), expected.map(path => path.replaceAll("\\", "/")).sort(), `${owner}: exact reviewed source owners`);
}
/** 🪶️ Independently resolves production imports without loading generators or repository tests. */
export async function verifyPrintCommandBoundaries(): Promise<void> {
  const { build } = await import("esbuild");
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
  const graphs = await Promise.all((contract.entries as string[]).map(async (entry: string, index: number) => {
    const result = await build({ absWorkingDir: workspaceRoot, entryPoints: [join(productRoot, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
    mkdirSync(outputRoot, { recursive: true });
    writeFileSync(join(outputRoot, `command-boundary-${index}.json`), JSON.stringify({ entry, metafile: result.metafile }, null, 2));
    return { entry, result };
  }));
  for (const { entry, result } of graphs) {
    const files = Object.keys(result.metafile!.inputs);
    for (const forbidden of contract.forbidden) assert.equal(files.some(path => path.includes(forbidden)), false, `${entry} loads ${forbidden}`);
    const expected: string[] = contract.sourceInputs[entry];
    assertPrintCommandSourceInputs(files, expected, entry);
    const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ const: expected.slice().sort() });
    assert.equal(validate(files.slice().sort()), true, `${entry}: independent source identity oracle`);
    for (const vector of contract.sourceRefusals) {
      const candidate = files.slice();
      candidate.splice(vector.at, vector.remove, ...vector.add);
      assert.equal(candidate.length - files.length, vector.lengthDelta, `${entry}/${vector.id}: neutral graph cardinality`);
      assert.equal(validate(candidate.slice().sort()), vector.accepted, `${entry}/${vector.id}: AJV source oracle`);
      assert.equal(vector.accepted, false, `${entry}/${vector.id}: authored refusal`);
      assert.throws(() => assertPrintCommandSourceInputs(candidate, expected, `${entry}/${vector.id}`), assert.AssertionError);
    }
    const external = [...new Set(Object.values(result.metafile!.outputs).flatMap(output => output.imports).filter(item => item.external && !/^(node|bun):/u.test(item.path) && item.path !== "bun").map(item => item.path.startsWith("@") ? item.path.split("/").slice(0, 2).join("/") : item.path.split("/")[0]))].sort();
    assert.deepEqual(external, contract.externalDependencies[entry], `${entry}: external imports`);
    console.log(`[DEBUG] Print command boundary ${entry}: ${files.length} exact reviewed source owners, ${contract.sourceRefusals.length} neutral refusals/AJV and external imports PASS`);
  }
}

/** 📄️ Verifies document ownership against schema, gallery fixtures and Nx inference. */
export async function verifyPrintDocumentCatalog(): Promise<void> {
  const root = workspaceRoot, product = "🧰️framework/🛍️products/📓️print";
  const modulePath = join(product, "🔨️modules/🖨️tectonic-template-compilation/📇️catalog");
  const require = createRequire(import.meta.url), catalog = JSON.parse(readFileSync(join(root, modulePath, "🔣️.json"), "utf8"));
  const validate = new (require("ajv").default)({ strict: false }).compile(printSchemaModule(join(root, modulePath), "https://json.schemas.assets.semio-tech.com/print/tectonic-template-compilation/catalog/schema.json"));
  assert.ok(validate(catalog), JSON.stringify(validate.errors));
  const api = await import(join(root, modulePath, "🟦️.ts"));
  assert.deepEqual(api.printLibrarySources(), catalog.librarySources);
  assert.deepEqual(api.printLibrarySources(), ["🖋️latex", "🔨️modules/🕸️graph/📐️.tex"]);
  const plugin = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"));
  const gallery = JSON.parse(readFileSync(join(root, product, "🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🗺️gallery-identities.json"), "utf8"));
  const documents = api.printDocuments();
  assert.equal(documents.length, 88);
  assert.deepEqual(documents.filter((row: any) => row.collection === "visualizations").map((row: any) => row.id).sort(), Object.keys(gallery).sort());
  assert.equal(new Set(documents.map((row: any) => api.printDocumentOutputDirectory(row.id, root))).size, documents.length);
  assert.throws(() => api.printDocument("../report"), /Unknown/);
  const project = JSON.parse(readFileSync(join(root, product, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const targets = plugin.cacheInternals.printDocumentTargets(project, join(product, "📦️packages/🟦️typescript"), root);
  for (const document of documents) {
    const target = targets[`build-${document.id}`];
    assert.ok(target, document.id);
    assert.equal(target.cache, true);
    assert.deepEqual([...new Set(target.inputs.flatMap((input: any) => input.externalDependencies ?? []))].sort(), ["pdfjs-dist", "sharp"]);
    assert.equal(target.inputs.includes("{workspaceRoot}/bun.lock"), false);
    assert.ok(target.inputs.includes("{workspaceRoot}/🧰️framework/🛍️products/📓️print/🔨️modules/🕸️graph/📐️.tex"));
    assert.deepEqual(target.outputs, [`{projectRoot}/dist/documents/${document.id}`]);
    assert.match(target.options.command, /📜️script\.ts build [a-z0-9-]+$/);
    assert.ok(target.dependsOn.includes("fonts"));
    assert.ok(target.dependsOn.includes("deps-tectonic"));
    assert.ok(target.dependsOn.includes("deps-tex"));
  }
  console.log(`[DEBUG] Print catalog schema, independent gallery identities and ${documents.length} exclusive Nx document owners PASS`);
}

/** 📚️ Exercises locked range acquisition with language-neutral corruption vectors. */
export async function verifyPrintBundleContract(output = outputRoot): Promise<void> {
  const modulePath = join(productRoot, "🔨️modules/🖨️tectonic-template-compilation/📚️bundle");
  const require = createRequire(import.meta.url), manifest = JSON.parse(readFileSync(join(modulePath, "🔒️dependencies.json"), "utf8"));
  assert.equal(new (require("ajv").default)({ strict: false }).validate(printSchemaModule(modulePath, "https://json.schemas.assets.semio-tech.com/print/tectonic-template-compilation/bundle/schema.json"), manifest), true);
  const vectors = JSON.parse(readFileSync(join(modulePath, "🧫️cases.json"), "utf8"));
  const api = await import("../../../../🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "print-bundle-contract-")), original = globalThis.fetch;
  try {
    const controller = new AbortController(); controller.abort();
    await assert.rejects(() => api.preparePrintBundle(root, controller.signal), /abort/i);
    for (const vector of vectors.failures) {
      let requests = 0;
      globalThis.fetch = (async (_url, options) => {
        requests++;
        const range = new Headers(options?.headers).get("range")!.slice(6);
        const file = manifest.files.find((file: any) => range === `${file.offset}-${file.offset + file.bytes - 1}`);
        assert.ok(file);
        return new Response(Buffer.alloc(file.bytes + (vector.bytes === "oversized" ? 1 : 0)), { status: vector.status, headers: { "content-range": `bytes ${vector.range === "matching" ? range : "0-0"}/${manifest.archiveBytes}` } });
      }) as typeof fetch;
      await assert.rejects(() => api.preparePrintBundle(root), new RegExp(vector.error), vector.name);
      assert.ok(requests > 0);
      assert.equal(existsSync(api.printBundleDirectory(root)), false);
      assert.equal(readdirSync(dirname(api.printBundleDirectory(root))).some(name => name.startsWith(".prepare-")), false);
    }
  } finally { globalThis.fetch = original; rmSync(root, { recursive: true }); }
}

/** 📖️ Publishes and independently measures the complete authored API in every selected language and theme. */
export async function verifyPrintApiPublication(): Promise<void> {
  const require = createRequire(import.meta.url), { createHash } = await import("node:crypto");
  const canvas = createRequire(join(workspaceRoot, "node_modules/pdfjs-dist/legacy/build/pdf.mjs"))("@napi-rs/canvas") as typeof import("@napi-rs/canvas");
  Object.assign(globalThis, { DOMMatrix: canvas.DOMMatrix, Path2D: canvas.Path2D, ImageData: canvas.ImageData });
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/📖️api-publication/🔣️.json"), "utf8"));
  const validate = new (require("ajv").default)({ strict: false }).compile(contract.schema);
  assert.ok(validate(contract.vectors), JSON.stringify(validate.errors));
  const control = contract.vectors as { languages: ("en" | "de")[]; themes: ("light" | "dark")[]; minimumCaptionGapPt: number; boundsTolerancePt: number; renderScale: number; headings: { en: string; de: string }[]; forbiddenContents: string[]; defaultSamples: { table: string; key: string; expected: boolean }[] };
  assert.equal(new Set(control.defaultSamples.map(sample => sample.table)).size, 51);
  const root = join(outputRoot, "api-publication");
  mkdirSync(root, { recursive: true });
  const paths = ["🧾️template/📊️viz-api/🔓️viz-api.tex", "🧬️schema/🔣️.json", "🖼️assets/🔣️viz-api.json", "🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json", "🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/📖️api-publication/🔣️.json"];
  const digest = (path: string) => createHash("sha256").update(readFileSync(path)).digest("hex");
  const bindings = Object.fromEntries(paths.map(path => [path, digest(join(productRoot, path))]));
  for (const name of readdirSync(latexRoot).filter(name => /\.(?:sty|cls)$/.test(name))) bindings[`🖋️latex/${name}`] = digest(join(latexRoot, name));
  const retained = process.env.SEMIO_PRINT_API_PUBLICATION_REUSE;
  if (retained) assert.deepEqual(JSON.parse(readFileSync(join(retained, "source-bindings.json"), "utf8")), bindings, "retained API publication source bindings differ");
  writeFileSync(join(root, "source-bindings.json"), JSON.stringify(bindings, null, 2));
  const source = readFileSync(join(productRoot, paths[0]!), "utf8"), normalize = (value: string) => value.replace(/\s+/g, "");
  assert.match(source, /\\documentclass\[[^\]]*language=de/);
  const abort = new AbortController(), cancel = () => abort.abort(new Error("API publication cancelled"));
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  const receipts: unknown[] = [], failures: string[] = [], retainedBytes: { source: string; target: string; sha256: string }[] = [];
  const check = (condition: unknown, message: string): void => { if (!condition) failures.push(message); };
  try {
    for (const language of control.languages) {
      abort.signal.throwIfAborted();
      const work = join(root, language), output = join(work, "published"), tex = `viz-api-${language}.tex`;
      mkdirSync(work, { recursive: true });
      const selectedSource = source.replace(/(\\documentclass\[[^\]]*)language=de/, `$1language=${language}`);
      writeFileSync(join(work, tex), selectedSource);
      console.log(`[DEBUG] Full API publication ${language}: compiling light and dark from bound authored source`);
      const retainedWork = retained ? join(retained, language) : undefined;
      if (retainedWork && existsSync(join(retainedWork, "published", printTemplatePdfNames(tex).light))) {
        assert.equal(readFileSync(join(retainedWork, tex), "utf8"), selectedSource, "retained authored API input differs");
        for (const name of Object.values(printTemplatePdfNames(tex))) {
          const retainedPdf = join(retainedWork, "published", name);
          assert.equal(readFileSync(retainedPdf).subarray(0, 5).toString(), "%PDF-", "retained publication is not a PDF");
          const hash = digest(retainedPdf), target = join(output, name);
          mkdirSync(output, { recursive: true }); copyFileSync(retainedPdf, target);
          assert.equal(digest(target), hash, "retained PDF copy byte digest differs");
          assert.equal(digest(retainedPdf), hash, "retained producer PDF changed during copy");
          retainedBytes.push({ source: retainedPdf, target, sha256: hash });
          writeFileSync(join(root, "retained-consumer-sha256.json"), JSON.stringify(retainedBytes, null, 2));
        }
        console.log(`[DEBUG] Full API ${language}: reusing source-bound consumer owner PDFs`);
      } else await publishPrintArtifact({ id: `api-publication-${language}`, sourceRoot: work, texPath: tex, sources: [tex], output, owner: "@semio-tech/print:test", dark: true }, abort.signal);
      for (const theme of control.themes) {
        const path = join(output, printTemplatePdfNames(tex)[theme]), pdf = await openPrintPdf(path), initialFailures = failures.length;
        type Item = import("pdfjs-dist/types/src/display/api").TextItem;
        type Caption = { page: number; y: number; title: string; gap: number; titleBands: number[][]; badgeHeight: number };
        const sameRow = (item: Item, anchor: Item): boolean => Math.max(item.transform[5]!, anchor.transform[5]!) < Math.min(item.transform[5]! + item.height, anchor.transform[5]! + anchor.height);
        const pages = new Map<number, Item[]>(), texts: string[] = [], contents: string[] = [], captions: Caption[] = [];
        const images = new Set<number>([1, 2, 3]);
        try {
          for (let number = 1; number <= pdf.numPages; number++) {
            abort.signal.throwIfAborted();
            const page = await pdf.getPage(number), content = await page.getTextContent(), viewport = page.getViewport({ scale: 1 });
            const items = content.items.filter((item): item is Item => "str" in item), text = items.map(item => item.str).join(" ");
            pages.set(number, items); texts.push(text);
            try { assertPrintPdfGlyphs(text, `${language}/${theme} page ${number}`); } catch (error) { failures.push(String(error)); images.add(number); }
            if (/(?:Inhaltsverzeichnis|Table\s+of\s+contents)/i.test(text)) { contents.push(text); images.add(number); }
            for (const item of items.filter(item => item.str.trim())) {
              const x = item.transform[4]!, y = item.transform[5]!, tolerance = control.boundsTolerancePt;
              const bounded = x >= -tolerance && x + item.width <= viewport.width + tolerance && y >= -tolerance && y + item.height <= viewport.height + tolerance;
              if (!bounded) images.add(number);
              check(bounded, `${language}/${theme} page ${number}: text outside page: ${item.str}`);
            }
            for (const badge of items.filter(item => /^(?:Table|Tabelle):/.test(item.str))) {
              const row = items.filter(item => item.str.trim() && sameRow(item, badge) && item.transform[4]! < badge.transform[4]!).sort((a, b) => a.transform[4]! - b.transform[4]!);
              if (!row.length) { failures.push(`${language}/${theme} page ${number}: caption has no visible title`); images.add(number); continue; }
              const gap = badge.transform[4]! - Math.max(...row.map(item => item.transform[4]! + item.width));
              if (gap < control.minimumCaptionGapPt) images.add(number);
              check(gap >= control.minimumCaptionGapPt, `${language}/${theme} page ${number}: caption gap ${gap} below ${control.minimumCaptionGapPt}: ${row.map(item => item.str).join(" ")}`);
              captions.push({ page: number, y: badge.transform[5]!, title: row.map(item => item.str).join(" "), gap, titleBands: row.map(item => [item.transform[5]!, item.height]), badgeHeight: badge.height });
            }
          }
          const text = normalize(texts.join(" ")), toc = normalize(contents.join(" "));
          for (const heading of control.headings) {
            check(text.includes(normalize(heading[language])), `${language}/${theme}: unresolved heading ${heading[language]}`);
            check(toc.includes(normalize(heading[language])), `${language}/${theme}: missing contents heading ${heading[language]}`);
          }
          for (const token of control.forbiddenContents) check(!text.includes(normalize(token)), `${language}/${theme}: raw title token ${token}`);
          captions.sort((a, b) => a.page - b.page || b.y - a.y);
          const actual = control.defaultSamples.map(sample => {
            const indices = captions.flatMap((caption, index) => normalize(caption.title) === normalize(sample.table) ? [index] : []);
            if (!indices.length) { failures.push(`${language}/${theme}: missing table caption ${sample.table}`); return { ...sample, missing: true }; }
            for (const index of indices) {
              const start = captions[index]!, end = captions[index + 1]; images.add(start.page);
              for (let number = start.page; number <= (end?.page ?? pdf.numPages); number++) {
                const owned = pages.get(number)!.filter(item => (number > start.page || item.transform[5]! < start.y) && (!end || number < end.page || item.transform[5]! > end.y));
                for (const key of owned.filter(item => normalize(item.str) === sample.key)) {
                  const row = owned.filter(item => sameRow(item, key) && item.transform[4]! > key.transform[4]! + key.width).sort((a, b) => a.transform[4]! - b.transform[4]!);
                  if (normalize(row[0]?.str ?? "") === "boolean" && normalize(row[1]?.str ?? "") === String(sample.expected)) { images.add(number); return { ...sample, page: number, keyX: key.transform[4], typeX: row[0]!.transform[4], defaultX: row[1]!.transform[4] }; }
                }
              }
            }
            failures.push(`${language}/${theme}: missing scoped boolean default ${sample.table}/${sample.key}=${sample.expected}`); return { ...sample, missing: true };
          });
          const imageRoot = join(work, theme, "pages"); mkdirSync(imageRoot, { recursive: true });
          const rendered: string[] = [];
          for (const number of [...images].filter(number => number <= pdf.numPages).sort((a, b) => a - b)) {
            abort.signal.throwIfAborted();
            const page = await pdf.getPage(number), viewport = page.getViewport({ scale: control.renderScale }), image = canvas.createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
            await page.render({ canvasContext: image.getContext("2d") as unknown as CanvasRenderingContext2D, viewport, canvas: image as unknown as HTMLCanvasElement }).promise;
            const imagePath = join(imageRoot, `page-${String(number).padStart(3, "0")}.png`); writeFileSync(imagePath, image.toBuffer("image/png")); rendered.push(imagePath); page.cleanup();
          }
          receipts.push({ language, theme, path, sha256: digest(path), pageCount: pdf.numPages, captions, actualDefaults: actual, rendered, failures: failures.slice(initialFailures) });
          writeFileSync(join(root, "measurements.json"), JSON.stringify(receipts, null, 2));
          console.log(`[DEBUG] Full API ${language}/${theme}: ${pdf.numPages} independent PDF.js pages, ${captions.length} measured captions, ${actual.length} defaults in 51 tables, ${rendered.length} review images, ${failures.length - initialFailures} failures ${failures.length === initialFailures ? "PASS" : "FAIL"}`);
        } finally { await pdf.destroy(); }
      }
    }
    assert.deepEqual(failures, [], "Full API publication layout violations");
    for (const [path, hash] of Object.entries(bindings)) assert.equal(digest(join(productRoot, path)), hash, `publication source changed during verification: ${path}`);
  } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
}