/** 📄️ The print macro staging and PDF verification: the staged macros, the glyph policy, every template PDF and the long pipeline level. */
import { type PrintPdfTextV1 } from "../../../../../📦️artifacts/📖️pdf/🟦️.ts";
import { loadVizCatalog, parseVizTaxonomyLeaves } from "../../../../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { classifyPackageSource, fileKindIdForSourcePath, loadCatalogTaxonomy, scopedFileKindIdForSourcePath, fixedSourceDispositionDecision, implementationLeafBasenameFinding, taxonomyFileKindIsImplementation } from "@semio-tech/repo-lib/inventory";
import { printCompilerName, printSourceReferenceMapV1, printSourceReferenceTextV1 } from "../../../../../🔨️modules/📥️source-staging/🟦️.ts";
import { readPrintMaterialFileV1 } from "../../../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { withPublishedPrintDocumentV1, printTemplatePdfNames, publishPrintArtifact, registeredPrintTemplates } from "../../../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { assertPrintPdfGlyphs, latexRoot, openPrintPdf, outputDirectory, pipelineScope, type PrintPipelineVerificationV1, productRoot, writePipelineEvidence } from "../🟦️.ts";
import { assertPrintKindPaintOwnership, readPrintKindPaint } from "../🖼️gallery-carrier-and-kind-paint/🟦️.ts";

//#region 🧪️PrintPipelineTests
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
  const references=printSourceReferenceMapV1(Object.keys(fixture.inputs));
  assert.equal("🔨️modules/🕸️graph/📐️.tex".split("/").map(printCompilerName).join("/"),"modules/graph/_.tex");
  for(const[path,content]of Object.entries(fixture.inputs)){const output=path.split("/").map(printCompilerName).join("/");assert.equal(/\.(?:tex|bib|sty|cls)$/.test(path)?printSourceReferenceTextV1(content,references):content,fixture.outputs[output],path);}
  for(const collision of fixture.collisions)assert.throws(()=>printSourceReferenceMapV1([...collision.entries,...Object.keys(collision.inputs).filter(name=>!collision.entries.includes(name))]),/Distinct Print staged Source/,collision.id);

}

/** 🧪️ Compiles the semantic graph owner twice through the production stage and independently reads its PDF. */
export async function verifyPrintMacroStagingNative(context:PrintPipelineVerificationV1):Promise<void>{
 const{execution}=pipelineScope(context),sourceRoot="🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🖨️macro-staging",pdfs:Uint8Array[]=[];
 for(const id of["graph-macro-native-first","graph-macro-native-second"]){await publishPrintArtifact(execution,{version:1,kind:"document",namespace:"probe",id,sourceRoot,texPath:"🧪️control.tex",sources:["🧪️control.tex"],dark:false});await withPublishedPrintDocumentV1(execution,"probe",id,async artifact=>{assert.deepEqual(artifact.files,["🧪️control.pdf"]);pdfs.push(await readPrintMaterialFileV1(join(artifact.path,"🧪️control.pdf"),67108864,artifact.control));await artifact.recheck();});}
 assert.deepEqual(pdfs[0],pdfs[1]);const bytes=pdfs[0];if(!bytes)throw Error("Actual first macro PDF required");const pdf=await openPrintPdf(context,bytes,pipelineScope(context).control);try{const texts:string[]=[];for(let index=1;index<=pdf.numPages;index++){const page=await pdf.getPage(index);texts.push((await page.getTextContent()).items.map(item=>"str"in item?item.str:"").join(" "));page.cleanup();}assert.ok(texts.join(" ").replace(/\s+/g," ").includes("Graph macro staging reached"));}finally{await pdf.destroy();}
}

/** 🧩️ Checks a neutral glyph policy and an optional actual missing-glyph compiler counterexample. */
export async function verifyPrintPdfGlyphPolicy(): Promise<void> {
  const policy = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️pdf-consumption.json"), "utf8")).glyphPolicy as { accepted: string[]; rejectedCodePoints: number[] };
  for (const text of policy.accepted) assert.doesNotThrow(() => assertPrintPdfGlyphs(text, "neutral accepted glyphs"));
  for (const point of policy.rejectedCodePoints) assert.throws(() => assertPrintPdfGlyphs(String.fromCodePoint(point), "neutral rejected glyph"), /invalid published glyph/);
  console.log(`[print] Published glyph policy: ${policy.accepted.length} accepted cases, ${policy.rejectedCodePoints.length} rejected scalars PASS`);
}

/** 📖️ Consumes each restored document through the independent PDF.js page and text reader. */
export async function verifyPrintPdfs(context:PrintPipelineVerificationV1,templates: readonly { id: string; texPath: string }[], collection: "templates" | "visualizations"): Promise<void> {
  const{execution}=pipelineScope(context),outputRoot=outputDirectory(context);
  const expected = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️pdf-consumption.json"), "utf8"));
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️merge-contract.json"), "utf8"));
  const visual = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔓️api-freshness.json"), "utf8")).printed.visual;
  const painted: any[] = [], paintContract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🖌️kind-paint/🔣️.json"), "utf8"));
  let count = 0;
  for (const template of templates) await withPublishedPrintDocumentV1(execution,"print",template.id,async artifact=>{for (const name of Object.values(printTemplatePdfNames(template.texPath))) {
    const pdf = await openPrintPdf(context,await readPrintMaterialFileV1(join(artifact.path,name),67108864,artifact.control),artifact.control);
    try {
      assert.ok(pdf.numPages >= expected.minimumPages, `${name}: empty document`);
      const texts: string[] = [];
      let contents = 0, captions = 0;
      for (let index = 1; index <= pdf.numPages; index++) {
        const page = await pdf.getPage(index);
        const items = (await page.getTextContent()).items.filter((item): item is PrintPdfTextV1 => "str" in item);
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
        const records = await readPrintKindPaint(pdf), catalog = loadVizCatalog(); assertPrintKindPaintOwnership(template.id+"/"+name, leaves, records, paintContract);
        for (const leaf of leaves) { const kind = catalog.kinds.find(kind => kind.slug === leaf.slug); assert.ok(kind, leaf.slug); painted.push({ theme: name.includes("-dark") ? "dark" : "light", document: template.id, leafId: leaf.leafId, slug: leaf.slug, family: kind.family, options: kind.options, paintClass: paintContract.glyphKinds.some((entry: any) => entry.slug === leaf.slug) ? "glyph" : "graphic", ...records.find(record => record.marker === paintContract.markerPrefix + Buffer.from(leaf.leafId).toString("hex")) }); }
        await writePipelineEvidence(context,`kind-painted-${template.id}-${name.includes("-dark")?"dark":"light"}.json`,JSON.stringify(painted.slice(-leaves.length)),artifact.control);
      }
      count++;
      console.log(`[print] Consumed ${collection} ${template.id}/${name}: ${pdf.numPages} pages PASS (${count}/${expected[collection]})`);
    } finally { await pdf.destroy(); }
  }await artifact.recheck();});
  assert.equal(count, expected[collection]);
  if (collection === "visualizations") {
    const taxonomy = parseVizTaxonomyLeaves(readFileSync(join(productRoot, "🖼️assets/📊️viz-taxonomy.md"), "utf8")), catalog = loadVizCatalog();
    for (const theme of ["light", "dark"]) { const rows = painted.filter(row => row.theme === theme); assert.equal(rows.length, paintContract.leaves, `${theme}: exact leaf paint count`); assert.deepEqual(rows.map(row => row.leafId).sort(), [...taxonomy].sort(), `${theme}: exact taxonomy paint set`); assert.equal(new Set(rows.map(row => row.slug)).size, paintContract.kinds, `${theme}: unique painted kinds`); assert.deepEqual([...new Set(rows.map(row => row.slug))].sort(), catalog.kinds.map(kind => kind.slug).sort(), `${theme}: exact kind paint set`); }
    await writePipelineEvidence(context,"kind-painted-coverage.json",JSON.stringify(painted),pipelineScope(context).control);
    const groups = new Map<string, any[]>(); for (const row of painted) { const key = [row.theme, row.family, row.paintClass === "glyph" ? row.glyphGeometrySignature : row.graphicSignature].join("/"); groups.set(key, [...(groups.get(key) ?? []), { leafId: row.leafId, slug: row.slug, options: row.options }]); }
    await writePipelineEvidence(context,"kind-painted-equal-geometry.json",JSON.stringify([...groups].filter(([,rows])=>new Set(rows.map(row=>row.slug)).size>1).map(([group,rows])=>({group,rows}))),pipelineScope(context).control);
  }
  console.log(`[print] PDF.js consumption ${collection}: ${count} PDFs PASS`);
}

/** 🖨️ Verifies the light and dark PDFs prepared by explicit Nx document prerequisites. */
export async function verifyPrintPipelineLong(context:PrintPipelineVerificationV1): Promise<void> {
  const outputRoot=outputDirectory(context),{execution,control}=pipelineScope(context);
  await verifyPrintPdfs(context,registeredPrintTemplates(), "templates");
}
//#endregion 🧪️PrintPipelineTests
