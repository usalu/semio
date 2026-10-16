import type{GalleryRenderEvidence,GalleryVariantMeasurement}from"./🧬️schema/🟦️.ts";
import{createHash}from"node:crypto";
import{join}from"node:path";
import{writeFile}from"node:fs/promises";
import{VIZ_GALLERY_THEMES,VIZ_LANGUAGES,type VizGalleryTheme,type VizLanguage}from"../../../../🧬️schema/🟦️.ts";
import{loadVizCatalog,visualizationTemplates}from"../../🟦️.ts";
import{withCompiledPrintProbeV1}from"../../../🖨️tectonic-template-compilation/🟦️.ts";
import{withPrintScratchV1,withPrintStorageTaskV1,type PrintExecutionV1}from"../../../../📦️artifacts/🎛️operation/🟦️.ts";
import{readPrintMaterialFileV1}from"../../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import{printArtifactLayoutV1}from"../../../../📦️artifacts/🟦️.ts";
import{withRepositoryArtifactDestinationV1,stageRepositoryArtifacts}from"../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{checkResourceLeaseControl}from"@semio-tech/process/leases";
import{measurePrintPdfV1}from"./📥️pdf/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import intentSchema from"../../../🖨️tectonic-template-compilation/📋️intent/🧬️schema/🔣️.json"with{type:"json"};
const product="🧰️framework/🛍️products/📓️print",compilerPolicy=intentSchema.definitions.Policy.const,measurementPolicy=schema.$defs.GalleryMeasurementPolicy.const,maximumEvidenceBytes=measurementPolicy.maximumEvidenceBytes;
/** 🧭️ One cell of the gallery matrix: a generated section rendered in one theme and one language. */
export type PrintGalleryVariant = Readonly<{ id: string; section: string; texPath: string; theme: VizGalleryTheme; language: VizLanguage }>;

/** 🧭️ Every generated gallery section x {light, dark} x {en, de}, optionally narrowed to the named sections. */
export function printGalleryMatrix(sections?: readonly string[]): readonly PrintGalleryVariant[] {
  const definitions=visualizationTemplates();if(definitions.length>measurementPolicy.maximumSections||(sections?.length??0)>measurementPolicy.maximumSections)throw Error("Finite gallery matrix required");
  const galleries = definitions.filter(({ texPath }) => texPath.startsWith("🧾️template/📊️viz-gallery/"));
  const unknown = (sections ?? []).filter((section) => !galleries.some(({ id }) => id === section));
  if (unknown.length > 0) throw new Error(`unknown gallery section(s): ${unknown.join(", ")}`);
  return galleries
    .filter(({ id }) => sections === undefined || sections.length === 0 || sections.includes(id))
    .flatMap(({ id, texPath }) => VIZ_GALLERY_THEMES.flatMap((theme) => VIZ_LANGUAGES.map((language) => ({ id: `${id}/${theme}/${language}`, section: id, texPath, theme, language }))));
}

/** 📝️ The section source with its document class naming this variant's theme and language, so no default ever applies. */
export function printGallerySource(variant: PrintGalleryVariant, source:string): string {
  if(Buffer.byteLength(source)>compilerPolicy.maximumProbeBytes)throw Error("Bounded gallery TeX Source required");
  const options = /\\documentclass\[([^\]]*)\]\{semio\}/.exec(source);
  if (options === null) throw new Error(`${variant.texPath}: no \\documentclass[…]{semio} line to derive the variant from`);
  const kept = options[1]!.split(",").map((option) => option.trim()).filter((option) => option !== "" && !/^(theme|language)=/.test(option));
  return source.replace(options[0], `\\documentclass[${[...kept, `theme=${variant.theme}`, `language=${variant.language}`].join(",")}]{semio}`);
}

/** 🏷️ The catalogue kinds a section draws, in document order, with their titles in the variant's language. */
function sectionKinds(source: string, language: VizLanguage): readonly { slug: string; title: string }[] {
  const kinds=loadVizCatalog().kinds;if(kinds.length>measurementPolicy.maximumKinds)throw Error("Finite gallery kind catalogue required");const titles=new Map(kinds.map(kind=>[kind.slug,kind.title[language]])),rows:{slug:string;title:string}[]=[];for(const match of source.matchAll(/\\SemioVizChart\{([^}]+)\}/gu)){if(rows.length>=measurementPolicy.maximumKinds)throw Error("Finite gallery kind projection required");const slug=match[1],title=slug===undefined?undefined:titles.get(slug);if(slug===undefined||title===undefined)throw Error(`Undeclared gallery kind ${slug}`);rows.push({slug,title});}return Object.freeze(rows);
}

function normalizedText(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

/** 📏️ Measures one declared variant while actual compiler, input readers, PDF worker and scratch remain held. */
export function measurePrintGalleryVariant(execution:PrintExecutionV1,variant:PrintGalleryVariant):Promise<GalleryVariantMeasurement>{const selected=printGalleryMatrix().find(row=>row.id===variant.id);if(!selected||JSON.stringify(selected)!==JSON.stringify({id:variant.id,section:variant.section,texPath:variant.texPath,theme:variant.theme,language:variant.language}))throw Error("Declared immutable gallery variant required");const captured=Object.freeze(selected);return withPrintScratchV1(execution,"gallery/"+captured.section,async scratch=>{const path=join(scratch.workspace,product,captured.texPath),original=await readPrintMaterialFileV1(path,compilerPolicy.maximumProbeBytes,scratch.control),identity=createHash("sha256").update(original).digest("hex"),source=printGallerySource(captured,new TextDecoder("utf-8",{fatal:true}).decode(original));return withCompiledPrintProbeV1(scratch.execution,{version:1,kind:"probe",name:"gallery.tex",source},async probe=>{const measured=await measurePrintPdfV1({operation:scratch.execution.operation,phaseControl:probe.control},scratch.destination,await probe.read("gallery.pdf",67108864)),kinds:Record<string,{page:number;text:string}>={};let bytes=0;let processed=0;for(const{slug,title}of sectionKinds(source,captured.language)){checkResourceLeaseControl(probe.control);if(++processed%16===0)await probe.control.yieldContinuation();const index=measured.pages.findIndex(text=>text.includes(normalizedText(title))),text=measured.pages[index];if(index<0||text===undefined)throw Error(`${captured.id}: kind ${slug} (${title}) appears on no page`);bytes+=Buffer.byteLength(JSON.stringify({slug,page:index+1,text}));if(bytes>maximumEvidenceBytes)throw Error("Gallery variant report aggregate exceeded");kinds[slug]={page:index+1,text};}if(createHash("sha256").update(await readPrintMaterialFileV1(path,compilerPolicy.maximumProbeBytes,probe.control)).digest("hex")!==identity)throw Error("Consumed gallery Source changed");await probe.recheck();return Object.freeze({pages:measured.pages.length,kinds:Object.freeze(kinds),hash:measured.hash});});});}
/** 🖼️ Publishes bounded selected-mode evidence while keeping the committed oracle as read-only Source DATA. */
export function writePrintGalleryEvidence(execution:PrintExecutionV1,sections:readonly string[]):Promise<GalleryRenderEvidence>{if(sections.length>measurementPolicy.maximumSections)throw Error("Finite gallery section selection required");const selected=Object.freeze([...sections]),matrix=printGalleryMatrix(selected);return withPrintScratchV1(execution,"gallery-report/evidence",async scratch=>{const source=await readPrintMaterialFileV1(join(scratch.workspace,product,"🧪️tests/🧫️fixtures/🖼️gallery-render/🖼️gallery-render.json"),2097152,scratch.control),committed:unknown=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(source));if(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/GalleryRenderEvidence"},committed).length)throw Error("Closed committed gallery evidence required");const oracle=committed as GalleryRenderEvidence,variants:Record<string,GalleryVariantMeasurement>=selected.length?{...oracle.variants}:{};let bytes=Buffer.byteLength(JSON.stringify(variants));const started=performance.now();for(const[index,variant]of matrix.entries()){checkResourceLeaseControl(scratch.control);const measured=await measurePrintGalleryVariant(scratch.execution,variant),old=variants[variant.id],delta=Buffer.byteLength(JSON.stringify(measured))-(old?Buffer.byteLength(JSON.stringify(old)):0)+Buffer.byteLength(variant.id)+4;if(delta>maximumEvidenceBytes-bytes)throw Error("Gallery evidence aggregate exceeded");bytes+=delta;variants[variant.id]=measured;await scratch.control.publish({resource:`print:gallery:${index+1}/${matrix.length}`,mode:"exclusive",elapsedMs:performance.now()-started});}const fixture:GalleryRenderEvidence={schemaVersion:1,generatedBy:oracle.generatedBy,variants:Object.fromEntries(Object.keys(variants).sort().map(id=>[id,variants[id]!] ))},data=JSON.stringify(fixture)+"\n";if(Buffer.byteLength(data)>maximumEvidenceBytes)throw Error("Finite serialized gallery evidence required");const file=join(scratch.directory,"gallery-evidence.json");await writeFile(file,data,{flag:"wx"});await withPrintStorageTaskV1(scratch.execution.operation,scratch.control,async(_operation,context)=>{const products=context.store("products"),directory=printArtifactLayoutV1(products.root,{version:1,kind:"measurement",mode:context.mode,id:"gallery-evidence"}).directory;await withRepositoryArtifactDestinationV1({workspace:context.workspace,invocation:context.invocation,storage:products.storage,control:products.control,phaseControl:context.phaseControl,directory},destination=>stageRepositoryArtifacts(destination,"@semio-tech/print:gallery-evidence",new Map([["gallery-evidence.json",file]])));});return fixture;});}
