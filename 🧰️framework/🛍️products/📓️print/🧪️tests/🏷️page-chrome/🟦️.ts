import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { evaluate } from "mathjs";
import { createHash } from "node:crypto";
import sharp from "sharp";
import Ajv from "ajv";
import fixture from "./🔣️.json";

const definition = (text: string, name: string): string => {
  const start = text.indexOf("\\newcommand{\\" + name + "}");
  assert.ok(start >= 0, name);
  const end = text.indexOf("\n\\newcommand", start + 1);
  return text.slice(start, end < 0 ? undefined : end);
};

/** 🏷️ Checks joined page chrome against neutral vectors and independent arithmetic. */
export function verifyPageChromeBorders(): void {
  verifyPageChromeHeader();
  verifyChipFrameModel();
  const text = readFileSync(new URL("../../🖋️latex/semio-window.sty", import.meta.url), "utf8");
  const projectStart = text.indexOf("\\cs_new_protected:Npn \\semio_chrome_navbar_project_chip:");
  const projectEnd = text.indexOf("\\cs_new_protected:Npn \\semio_chrome_navbar_center_chip:", projectStart);
  assert.ok(projectStart >= 0 && projectEnd > projectStart);
  const project = text.slice(projectStart, projectEnd), navbar = definition(text, "SemioChromeNavbarHead");
  assert.equal((project.match(/\\semio@chrome@chip(?!@)/g) ?? []).length, 1);
  assert.ok(project.includes("\\usebox{\\semio@chrome@navbar@emblem@box}"));
  assert.ok(project.indexOf("\\semio@chrome@chip") < project.indexOf("\\tl_if_empty:NF"));
  assert.ok(project.includes("\\tl_use:N \\l_semio_navbar_project_tl"));
  assert.ok(!navbar.includes("\\semio@chrome@chip{"));
  assert.ok(!navbar.includes("\\semio@chrome@navbar@emblem@box"));
  assert.equal((navbar.match(/\\SemioChromeNavbarProjectChip/g) ?? []).length, 1);
  for (const row of fixture.projectChip.cases) {
    const gap = row.label ? fixture.padding : 0;
    const width = fixture.projectChip.emblemWidth + gap + row.labelWidth + 2 * fixture.padding + 2 * fixture.stroke;
    const oracle = Number(evaluate("emblem + gap + label + 2 * padding + 2 * stroke", {emblem:fixture.projectChip.emblemWidth,gap,label:row.labelWidth,padding:fixture.padding,stroke:fixture.stroke}));
    assert.ok(Math.abs(width - row.width) < 1e-8);
    assert.equal(width, oracle);
    assert.equal(row.borders, 1);
    if (row.label) assert.ok(readFileSync(new URL("./" + row.language + ".tex", import.meta.url), "utf8").includes("left={" + row.label + "}"));
  }
  const chip = definition(text, "semio@chrome@chip");
  assert.ok(!chip.includes("\\fcolorbox") && !chip.includes("\\colorbox") && !chip.includes("chip@rule"));
  assert.ok(chip.includes("\\expandafter\\semio@chip@box\\expandafter{\\semio@chrome@chip@edges}") && text.includes("\\def\\semio@chrome@chip@edges{lr}"));
  assert.ok(!text.includes("\\semio@chrome@chip@rule") && !text.includes("\\semio@chrome@chip@body"));
  for (const name of ["SemioFooterAuthorChips", "SemioFooterLogoChips", "SemioChromeNavbarProjectChip", "SemioChromeNavbarCenterChip", "SemioChromeNavbarLabelsCompute"]) assert.ok(text.includes("\\cs_new_eq:NN \\" + name), name);
  const edgeLetters: Record<string, string> = { top: "t", bottom: "b", left: "l", right: "r" };
  for (const row of fixture.cases) {
    const body = fixture.rowHeight + 2 * fixture.padding, chipHeight = body + fixture.stroke, total = chipHeight + fixture.stroke;
    for (const [actual, expected] of [[body, row.bodyHeight], [chipHeight, row.chipHeight], [total, row.totalHeight]]) assert.ok(Math.abs(actual! - expected!) < 1e-8);
    assert.equal(total, Number(evaluate("rowHeight + 2 * padding + 2 * stroke", {rowHeight: fixture.rowHeight, padding: fixture.padding, stroke: fixture.stroke})));
    assert.equal(row.edges.map((edge) => edgeLetters[edge]).join(""), (row.side === "bottom" ? "t" : "b") + "lr");
    assert.ok(!row.edges.includes(row.side), row.name);
    const bar = definition(text, "semio@chrome@" + row.bar + "@bar");
    assert.ok(!bar.includes("\\semio@spacing@single"), row.name);
    assert.ok(bar.includes("\\semio@chrome@footer" + (row.bar === "footer" ? "true" : "false")));
    const rule = bar.indexOf("\\semio@window@stroke@h"), content = bar.indexOf("\\semio@chrome@band@row{#1}"), closing = bar.lastIndexOf("\\semio@window@stroke@h");
    assert.ok(rule >= 0 && rule < content && content < closing, row.name + ": band closed above and below");
    assert.ok(bar.includes("\\nointerlineskip"));
  }
  assert.ok(definition(text, "semio@chrome@bar@threezone").includes("\\vbox to \\dimexpr#1-\\semio@stroke@hairline\\relax"));
  assert.equal(definition(text, "semio@chrome@band@row").split("\\vrule width\\semio@stroke@hairline height\\ht\\z@ depth\\dp\\z@").length, 3);
  assert.match(definition(text, "SemioChromeNavbarHead"), /\\semio@chrome@navbar@chip@text\{\\semio@chrome@font@chip\}\{%\s*\\semio@chrome@fit@width/);
}

/** 🪟 Checks that every chip is one glass overlay inside a single rectangular frame. */
export function verifyChipFrameModel(): void {
  const text = readFileSync(new URL("../../🖋️latex/semio-window.sty", import.meta.url), "utf8");
  const table = readFileSync(new URL("../../🖋️latex/semio-table.sty", import.meta.url), "utf8");
  for (const name of fixture.frame.removed) assert.ok(!text.includes(name) && !table.includes(name), name);
  assert.ok(!text.includes("toprule=0pt") && !text.includes("toprule~at~break=0pt"));
  assert.equal((text.match(/\\pgfsetfillopacity/g) ?? []).length, 1);
  const fill = definition(text, "semio@chip@fill");
  assert.ok(fill.includes("\\ifsemio@chip@glass") && fill.includes("\\pgfsetfillopacity{\\semio@chip@opacity}"));
  assert.ok(text.includes("\\newif\\ifsemio@chip@glass") && text.includes("\\newcommand{\\semio@chip@opacity}{" + fixture.surface.opacity + "}"));
  assert.ok(text.includes("\\newcommand{\\semio@chip@surface}{" + fixture.surface.opaque + "}"));
  assert.equal(fixture.surface.glass, fixture.surface.opaque);
  assert.ok(fixture.surface.opacity > 0 && fixture.surface.opacity < 1);
  const letters: Record<string, string> = { top: "t", bottom: "b", left: "l", right: "r" };
  const overlay = definition(text, "semio@window@chips@overlay");
  assert.ok(overlay.includes("\\semio@chip@muted{" + fixture.frame.edges.title.map((e: string) => letters[e]).join("") + "}"));
  assert.ok(overlay.includes("\\semio@chip@muted{" + fixture.frame.edges.number.map((e: string) => letters[e]).join("") + "}"));
  assert.ok(overlay.includes("frame.north west") && overlay.includes("frame.north east"));
  for (const hook of ["finish~unbroken", "finish~first", "finish~middle", "finish~last"]) assert.match(text, new RegExp(hook + "=\\{\\\\semio@window@chips@overlay\\}"));
  assert.match(text, /toprule=\\semio@stroke@hairline/);
  assert.match(text, /toprule~at~break=\\semio@stroke@hairline/);
  const heading = definition(text, "semio@heading@frame");
  assert.ok(heading.includes("\\semio@chip@box{" + fixture.frame.edges.heading.title.map((e: string) => letters[e]).join("") + "}"));
  assert.ok(heading.includes("\\semio@chip@box{" + fixture.frame.edges.heading.number.map((e: string) => letters[e]).join("") + "}"));
  const validator = new Ajv({strict:true}).compile({type:"array",uniqueItems:true,items:{enum:["top","bottom","left","right"]}});
  for (const edges of [fixture.frame.edges.title, fixture.frame.edges.number, fixture.frame.edges.heading.title, fixture.frame.edges.heading.number]) assert.equal(validator(edges), true);
  assert.ok(!fixture.frame.edges.title.includes("top") && !fixture.frame.edges.number.includes("top"));
}

/** 🎓️ Verifies the original university assets with an independent image decoder. */
export async function verifyPageChromeLogo(): Promise<void> {
  const root = new URL("../../", import.meta.url), workspace = new URL("../../../", root);
  const read = (path: string) => readFileSync(new URL(path, root));
  const core = read("🖋️latex/semio-core.sty").toString();
  assert.ok(!core.includes("\\clist_set:Nn \\l_semio_footer_logos_clist { " + fixture.footerLogo.images[0]!.asset + " }"));
  assert.ok(readFileSync(new URL("./en.tex", import.meta.url), "utf8").includes("\\footerlogos{" + fixture.footerLogo.images[0]!.asset + "}"));
  assert.match(core, /\\NewDocumentCommand \\footerlogos \{ m \} \{\s*\\clist_set:Nn/);
  const footer = definition(read("🖋️latex/semio-window.sty").toString(), "SemioChromeFooterHead");
  assert.ok(footer.includes("\\let\\Ifsemiochromemirrored\\@secondoftwo"));
  assert.ok(footer.indexOf("\\SemioFooterLogoChips") < footer.indexOf("\\SemioFooterAuthorChips"));
  assert.ok(footer.includes("\\begingroup") && footer.includes("\\endgroup"));
  const catalog = JSON.parse(read("🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json").toString());
  for (const row of fixture.footerLogo.images) {
    assert.ok(catalog.librarySources.some((path: string) => row.asset.startsWith(path + "/")));
    const bytes = read(row.asset), original = readFileSync(new URL(row.source, workspace));
    assert.equal(createHash("sha256").update(bytes).digest("hex"), createHash("sha256").update(original).digest("hex"));
    assert.equal(bytes.readUInt32BE(16), fixture.footerLogo.width);
    assert.equal(bytes.readUInt32BE(20), fixture.footerLogo.height);
    const metadata = await sharp(bytes).metadata();
    assert.equal(metadata.width, fixture.footerLogo.width);
    assert.equal(metadata.height, fixture.footerLogo.height);
    assert.deepEqual(await sharp(bytes).raw().toBuffer(), await sharp(original).raw().toBuffer());
  }
  const logo = read("🖋️latex/semio-logo.sty").toString();
  assert.ok(logo.includes("\\str_if_eq:VnTF \\l_semio_theme_tl { dark }"));
  assert.ok(logo.includes("\\filename@base-dark."));
}

/** 🧭️ Checks authored header zones against the independent schema validator. */
export function verifyPageChromeHeader(): void {
  const text = readFileSync(new URL("../../🖋️latex/semio-window.sty", import.meta.url), "utf8");
  assert.ok(text.includes("\\NewDocumentCommand \\SemioHeaderSetup { m }"));
  assert.ok(text.includes("\\keys_define:nn { semio / header }"));
  const navbar = definition(text, "SemioChromeNavbarHead");
  assert.ok(navbar.includes("\\ifsemio@chrome@navbar@authored"));
  assert.ok(navbar.includes("\\let\\Ifsemiochromemirrored\\@secondoftwo"));
  assert.ok(navbar.includes("\\SemioChromeNavbarRightLabel"));
  const validator = new Ajv({strict:true});
  for (const row of fixture.headers) {
    const source = readFileSync(new URL("./" + row.language + ".tex", import.meta.url), "utf8");
    const declaration = source.match(/\\SemioHeaderSetup\{([^\n]+)\}/)?.[1];
    assert.ok(declaration, row.language);
    const actual = Object.fromEntries([...declaration.matchAll(/(left|center|right)=\{([^}]*)\}/g)].map(([,key,value]) => [key,value]));
    assert.deepEqual(actual, row.labels);
    assert.equal(validator.compile({type:"object",required:["left","center","right"],additionalProperties:false,properties:Object.fromEntries(Object.entries(row.labels).map(([key,value])=>[key,{type:"string",const:value}]))})(actual), true);
    assert.ok(source.includes("course={" + row.labels.left + "}"));
    assert.ok(source.indexOf("\\SemioHeaderSetup") < source.indexOf("\\begin{document}"));
    assert.equal(Object.keys(actual).length, 3);
  }
}
