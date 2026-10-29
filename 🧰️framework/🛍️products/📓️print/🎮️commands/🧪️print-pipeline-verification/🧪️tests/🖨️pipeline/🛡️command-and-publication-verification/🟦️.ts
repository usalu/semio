/** 🛡️ The print command and publication verification: the visualization build, the font staging, the command boundaries, the document catalog, the bundle contract and the API publication. */
import { printArtifactNxOutputV1 } from "../../../../../📦️artifacts/🟦️.ts";
import { rasterPrintPdfV1, type PrintPdfTextV1 } from "../../../../../📦️artifacts/📖️pdf/🟦️.ts";
import assert from "node:assert/strict";
import { mkdirSync, readdirSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { visualizationTemplates } from "../../../../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { printFontDescriptors, withPrintFontsV1 } from "../../../../../🔨️modules/🔤print-font-catalog/🟦️.ts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { readPrintMaterialFileV1 } from "../../../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { withPublishedPrintDocumentV1, printTemplatePdfNames, publishPrintArtifact } from "../../../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { assertPrintPdfGlyphs, latexRoot, openPrintPdf, outputDirectory, pipelineScope, type PrintPipelineVerificationV1, printSchemaModule, productRoot, workspaceRoot, writePipelineEvidence } from "../🟦️.ts";
import { verifyPrintPdfs } from "../📄️macro-and-pdf-verification/🟦️.ts";

/** 🧪️ Verifies every visualization PDF and the authored API text contract. */
export async function verifyPrintVisualizationBuild(context:PrintPipelineVerificationV1): Promise<void> {
  const outputRoot=outputDirectory(context),{execution,control}=pipelineScope(context);
  await verifyPrintPdfs(context,visualizationTemplates(), "visualizations");
}

/** 🔤️ Verifies font publication against the schema and an independent native font loader. */
export async function verifyPrintFontStaging(context:PrintPipelineVerificationV1):Promise<void>{
 const{execution}=pipelineScope(context),require=createRequire(import.meta.url),modulePath=join(productRoot,"🔨️modules/🔤print-font-catalog"),catalog=printFontDescriptors();
 assert.equal(new(require("ajv").default)({strict:false}).validate(printSchemaModule(modulePath,"https://json.schemas.assets.semio-tech.com/print/print-font-catalog/schema.json"),catalog),true);
 await withPrintFontsV1(execution,async fonts=>{const{GlobalFonts}=require("@napi-rs/canvas"),keys:unknown[]=[];try{for(const row of catalog){checkResourceLeaseControl(fonts.control);const target=join(fonts.directory,row.texFilename),source=join(productRoot,"🖼️assets/🔤️font",row.directory,row.filename),bytes=await readPrintMaterialFileV1(target,16777216,fonts.control);assert.deepEqual(bytes,await readPrintMaterialFileV1(source,16777216,fonts.control));assert.ok(bytes.byteLength>=12&&[0x00010000,0x4f54544f].includes(new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength).getUint32(0)),row.family);const key=GlobalFonts.registerFromPath(target,"print-held-font-"+row.family);assert.ok(key,row.family);keys.push(key);}await fonts.recheck();}finally{for(const key of keys)GlobalFonts.remove(key);}});
}

/** 🧱️ One bundled input of a command graph and the import edges it declares. */
export type PrintCommandInputsV1 = Readonly<Record<string, { readonly imports: readonly { readonly path: string; readonly external?: boolean }[] }>>;
/** 🪪️ Names the neutral reviewed source identities of one command graph: the exact print-owned sources and the sorted foreign owner roots the print-owned sources import directly, independent of graph size, enumeration order and the transitive churn of foreign owners. */
export function printCommandSourceIdentitiesV1(inputs: PrintCommandInputsV1): { readonly sources: readonly string[]; readonly owners: readonly string[] } {
  const own = "🧰️framework/🛍️products/📓️print/", normal = (path: string) => path.replaceAll("\\", "/"), mine = Object.entries(inputs).filter(([path]) => normal(path).startsWith(own));
  return { sources: mine.map(([path]) => normal(path)).sort(), owners: [...new Set(mine.flatMap(([, input]) => input.imports).filter(item => !item.external).map(item => normal(item.path)).filter(path => !path.startsWith(own)).map(path => path.split("/").slice(0, 3).join("/")))].sort() };
}
/** 🔒️ Enforces the neutral reviewed source identities of one command graph. */
function assertPrintCommandSourceInputs(actual: PrintCommandInputsV1, expected: ReturnType<typeof printCommandSourceIdentitiesV1>, owner: string): void {
  assert.deepEqual(printCommandSourceIdentitiesV1(actual), expected, `${owner}: exact reviewed source owners`);
}
/** 🪶️ Independently resolves production imports without loading generators or repository tests. */
export async function verifyPrintCommandBoundaries(context:PrintPipelineVerificationV1): Promise<void> {
  const outputRoot=outputDirectory(context),{execution,control}=pipelineScope(context);
  const { build } = await import("esbuild");
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
  const graphs = await Promise.all((contract.entries as string[]).map(async (entry: string, index: number) => {
    const result = await build({ absWorkingDir: workspaceRoot, entryPoints: [join(productRoot, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
    mkdirSync(outputRoot, { recursive: true });
    await writePipelineEvidence(context,`command-boundary-${index}.json`,JSON.stringify({entry,metafile:result.metafile}),control);
    return { entry, result };
  }));
  for (const { entry, result } of graphs) {
    const inputs = result.metafile!.inputs as PrintCommandInputsV1, files = Object.keys(inputs), entryKey = "🧰️framework/🛍️products/📓️print/" + entry;
    for (const forbidden of contract.forbidden) assert.equal(files.some(path => path.includes(forbidden)), false, `${entry} loads ${forbidden}`);
    const expected: ReturnType<typeof printCommandSourceIdentitiesV1> = contract.sourceInputs[entry];
    assertPrintCommandSourceInputs(inputs, expected, entry);
    const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ const: expected });
    assert.equal(validate(printCommandSourceIdentitiesV1(inputs)), true, `${entry}: independent source identity oracle`);
    for (const vector of contract.sourceRefusals) {
      const removed = new Set<string>((vector.remove as string[]).map(path => path === "$entry" ? entryKey : path));
      const kept = Object.entries(inputs).filter(([path]) => !removed.has(path)).map(([path, input]) => [path, path === entryKey ? { imports: [...input.imports, ...(vector.addImports as string[]).map(path => ({ path }))] } : input] as const);
      const candidate: PrintCommandInputsV1 = { ...Object.fromEntries(kept), ...Object.fromEntries((vector.addSources as string[]).map(path => [path, { imports: [] }])) };
      assert.equal(Object.keys(candidate).length - files.length, vector.lengthDelta, `${entry}/${vector.id}: neutral graph cardinality`);
      assert.equal(validate(printCommandSourceIdentitiesV1(candidate)), vector.accepted, `${entry}/${vector.id}: AJV source oracle`);
      assert.equal(vector.accepted, false, `${entry}/${vector.id}: authored refusal`);
      assert.throws(() => assertPrintCommandSourceInputs(candidate, expected, `${entry}/${vector.id}`), assert.AssertionError);
    }
    const external = [...new Set(Object.values(result.metafile!.outputs).flatMap(output => output.imports).filter(item => item.external && !/^(node|bun):/u.test(item.path) && item.path !== "bun").map(item => item.path.startsWith("@") ? item.path.split("/").slice(0, 2).join("/") : item.path.split("/")[0]))].sort();
    assert.deepEqual(external, contract.externalDependencies[entry], `${entry}: external imports`);
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
  assert.deepEqual(api.printLibrarySources(), ["🖋️latex", "🖼️assets/🪧️logo", "🔨️modules/🕸️graph/📐️.tex"]);
  const plugin = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
  const gallery = JSON.parse(readFileSync(join(root, product, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🗺️gallery-identities.json"), "utf8"));
  const documents = api.printDocuments();
  assert.equal(documents.length, 89);
  assert.deepEqual(documents.filter((row: any) => row.collection === "visualizations").map((row: any) => row.id).sort(), Object.keys(gallery).sort());
  assert.equal(new Set(documents.map((row:{id:string})=>row.id)).size,documents.length);
  assert.throws(() => api.printDocument("../report"), /Unknown/);
  const project = JSON.parse(readFileSync(join(root, product, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const targets = await plugin.cacheInternals.printDocumentTargets(project, join(product, "📦️packages/🟦️typescript"), root);
  for (const document of documents) {
    const target = targets[`build-${document.id}`];
    assert.ok(target, document.id);
    assert.equal(target.cache, true);
    assert.deepEqual([...new Set(target.inputs.flatMap((input: any) => input.externalDependencies ?? []))].sort(), ["pdfjs-dist", "sharp"]);
    assert.equal(target.inputs.includes("{workspaceRoot}/bun.lock"), false);
    assert.ok(target.inputs.includes("{workspaceRoot}/🧰️framework/🛍️products/📓️print/🔨️modules/🕸️graph/📐️.tex"));
    assert.deepEqual(target.outputs, [printArtifactNxOutputV1(root,{version:1,kind:"document",mode:"prototype",namespace:"print",id:document.id})]);
    assert.match(target.options.command, /📜️script\.ts build [a-z0-9-]+$/);
    assert.ok(target.dependsOn.includes("fonts"));
    assert.ok(target.dependsOn.includes("deps-tectonic"));
    assert.ok(target.dependsOn.includes("deps-tex"));
  }
}

/** 📚️ Exercises locked range acquisition with language-neutral corruption vectors. */
export async function verifyPrintBundleContract(context:PrintPipelineVerificationV1):Promise<void>{
 const{control}=pipelineScope(context);
 const modulePath=join(productRoot,"🔨️modules/🖨️tectonic-template-compilation/📚️bundle"),require=createRequire(import.meta.url),manifest=JSON.parse(readFileSync(join(modulePath,"🔒️dependencies.json"),"utf8")),vectors=JSON.parse(readFileSync(join(modulePath,"🧫️cases.json"),"utf8"));
 assert.equal(new(require("ajv").default)({strict:false}).validate(printSchemaModule(modulePath,"https://json.schemas.assets.semio-tech.com/print/tectonic-template-compilation/bundle/schema.json"),manifest),true);
 const{printBundlePlanV1,readPrintBundleRangeV1}=await import("../../../../../📦️artifacts/📥️material/🟦️.ts"),plan=printBundlePlanV1();assert.equal(plan.bytes,manifest.files.reduce((sum:number,file:{bytes:number})=>sum+file.bytes,0));assert.equal(plan.files.length,manifest.files.length);assert.equal(new Set(plan.files.map(file=>file.name)).size,plan.files.length);
 for(const vector of vectors.failures){const file=plan.files.find(row=>row.bytes>0);assert.ok(file);const range=`${file.offset}-${file.offset+file.bytes-1}`,headers=new Headers({"content-range":`bytes ${vector.range==="matching"?range:"0-0"}/${plan.archiveBytes}`}),response=new Response(new Uint8Array(file.bytes+(vector.bytes==="oversized"?1:0)),{status:vector.status,headers});await assert.rejects(()=>readPrintBundleRangeV1(response,file,plan.archiveBytes,control),/range refused|pinned length|SHA256 mismatch/,vector.name);}
}

/** 📖️ Publishes and independently measures the complete authored API in every selected language and theme. */
export async function verifyPrintApiPublication(context:PrintPipelineVerificationV1): Promise<void> {
  const outputRoot=outputDirectory(context),{execution}=pipelineScope(context);
  const { createHash } = await import("node:crypto");
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/📖️api-publication/🔣️.json"), "utf8"));
  
  
  const control = contract.vectors as { languages: ("en" | "de")[]; themes: ("light" | "dark")[]; minimumCaptionGapPt: number; boundsTolerancePt: number; renderScale: number; headings: { en: string; de: string }[]; forbiddenContents: string[]; defaultSamples: { table: string; key: string; expected: boolean }[] };
  assert.equal(new Set(control.defaultSamples.map(sample => sample.table)).size, 51);
  const root = join(outputRoot, "api-publication");
  mkdirSync(root, { recursive: true });
  const paths = ["🧾️template/📊️viz-api/🔓️viz-api.tex", "🧬️schema/🔣️.json", "🖼️assets/🔣️viz-api.json", "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🔓️api-freshness.json", "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/📖️api-publication/🔣️.json"];
  const digest = (path: string) => createHash("sha256").update(readFileSync(path)).digest("hex");
  const bindings = Object.fromEntries(paths.map(path => [path, digest(join(productRoot, path))]));
  for (const name of readdirSync(latexRoot).filter(name => /\.(?:sty|cls)$/.test(name))) bindings[`🖋️latex/${name}`] = digest(join(latexRoot, name));
  await writePipelineEvidence(context,"api-publication/source-bindings.json",JSON.stringify(bindings),pipelineScope(context).control);
  const source = readFileSync(join(productRoot, paths[0]!), "utf8"), normalize = (value: string) => value.replace(/\s+/g, "");
  assert.match(source, /\\documentclass\[[^\]]*language=de/);
  const receipts:unknown[]=[],failures:string[]=[];
  const check = (condition: unknown, message: string): void => { if (!condition) failures.push(message); };
  for (const language of control.languages) {
      checkResourceLeaseControl(pipelineScope(context).control);
      const work = join(root, language), tex = `viz-api-${language}.tex`;
      mkdirSync(work, { recursive: true });
      const selectedSource = source.replace(/(\\documentclass\[[^\]]*)language=de/, `$1language=${language}`);
      const id=`api-publication-${language}`;await publishPrintArtifact(execution,{version:1,kind:"generated-document",namespace:"print",id,producer:`@semio-tech/print:build-${id}`,name:tex,source:selectedSource,dark:true,resources:[]});
      await withPublishedPrintDocumentV1(execution,"print",id,`@semio-tech/print:build-${id}`,async artifact=>{
      for (const theme of control.themes) {
        const path=join(artifact.path,printTemplatePdfNames(id+".tex")[theme]),pdfBytes=await readPrintMaterialFileV1(path,67108864,artifact.control),pdf=await openPrintPdf(context,pdfBytes,artifact.control),initialFailures=failures.length;
        type Item=PrintPdfTextV1;
        type Caption = { page: number; y: number; title: string; gap: number; titleBands: number[][]; badgeHeight: number };
        const sameRow = (item: Item, anchor: Item): boolean => Math.max(item.transform[5]!, anchor.transform[5]!) < Math.min(item.transform[5]! + item.height, anchor.transform[5]! + anchor.height);
        const pages = new Map<number, Item[]>(), texts: string[] = [], contents: string[] = [], captions: Caption[] = [];
        const images = new Set<number>([1, 2, 3]);
        try {
          for (let number = 1; number <= pdf.numPages; number++) {
            checkResourceLeaseControl(artifact.control);
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
          const rendered:string[]=[],selected=[...images].filter(number=>number<=pdf.numPages).sort((a,b)=>a-b),scope=pipelineScope(context),rasters=await rasterPrintPdfV1({operation:scope.execution.operation,phaseControl:artifact.control},scope.destination,pdfBytes,selected,control.renderScale);
          for(const image of rasters){checkResourceLeaseControl(artifact.control);const name=`api-publication/${language}/${theme}/pages/page-${String(image.page).padStart(3,"0")}.png`;rendered.push(await writePipelineEvidence(context,name,image.png,artifact.control));}
          receipts.push({ language, theme, document:id,file:printTemplatePdfNames(id+".tex")[theme],sha256:digest(path), pageCount: pdf.numPages, captions, actualDefaults: actual, rendered, failures: failures.slice(initialFailures) });
          await writePipelineEvidence(context,`api-publication/${language}/${theme}/measurements.json`,JSON.stringify(receipts),artifact.control);
        } finally { await pdf.destroy(); }
      }
      await artifact.recheck();});
    }
    assert.deepEqual(failures, [], "Full API publication layout violations");
    for (const [path, hash] of Object.entries(bindings)) assert.equal(digest(join(productRoot, path)), hash, `publication source changed during verification: ${path}`);
}
