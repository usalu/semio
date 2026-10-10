#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🖍️ Draw example twins plus the publication-authority law: every dispatchable route is declared once, in every place the framework joins. */
import { join, resolve } from "node:path";
import { runOwnedCommand } from "../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type Lane = "artifact" | "config" | "draft" | "presence" | "transient" | "window-config" | "window-transient" | "child" | "interaction" | "host-only";
type Route = { id: string; lanes: Lane[]; frameworkInjected?: true };
type AppAuthority = { owner: string; toolIdsConstants: string[]; source: string; routes: Route[]; reservedToolIdsConstant:string; reservedRoutes:Route[]; reservedProducerSource:string; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
type Fixture = { schema: string; apps: AppAuthority[] };

type BootstrapYield = { expectedStages: string[]; expectedAllocationDelta: number };
type ScheduledContinuation = () => void | ScheduledContinuation;

function bootstrapYieldOracle(law: BootstrapYield): void {
  const scheduler = createRequire(import.meta.url)("scheduler/unstable_mock") as {
    unstable_NormalPriority: number;
    unstable_scheduleCallback(priority: number, callback: ScheduledContinuation): unknown;
    unstable_flushNumberOfYields(count: number): void;
    unstable_flushAllWithoutAsserting(): boolean;
    unstable_clearLog(): string[];
    unstable_hasPendingWork(): boolean;
    log(value: string): void;
  };
  let allocations = 0;
  scheduler.unstable_scheduleCallback(scheduler.unstable_NormalPriority, () => {
    scheduler.log("yielded");
    return () => { allocations += 1; scheduler.log("resumed"); };
  });
  scheduler.unstable_flushNumberOfYields(1);
  const stages = scheduler.unstable_clearLog();
  if (allocations !== law.expectedAllocationDelta || !scheduler.unstable_hasPendingWork()) throw new Error("React Scheduler lost or advanced the yielded bootstrap continuation");
  scheduler.unstable_flushAllWithoutAsserting();
  stages.push(...scheduler.unstable_clearLog());
  if (allocations !== 1 || scheduler.unstable_hasPendingWork() || JSON.stringify(stages) !== JSON.stringify(law.expectedStages)) throw new Error("React Scheduler failed the bootstrap resume law");
  console.error(`Drawing bootstrap third-party React Scheduler oracle: ${stages.join("→")}; allocations before resume=${law.expectedAllocationDelta}`);
}

/** 🖍️ Every anchor draw's publication apparatus must carry verbatim: the proof catalogs, both owned
 * factories, the one-item artifact store preparation authority, the exact Canvas window config +
 * transient owner registrations (the view lanes since 2026-09-15 — there is no plugin config store),
 * their freshness guards and their incremental close, plus the accessible bilingual UI surface. */
const ANCHORS = [
  "semio_framework_plugin::bounded_first_step_tool_proofs!",
  "factory_type:",
  "ToolExecutionContract::bounded_first_step",
  "ToolExecutionContract::resumable",
  "build_artifact_store_one_item_preparation_factory",
  "fn register_window_config_owners(",
  "fn register_window_transient_owners(",
  "register_tool_job_factories",
  "build_tool_job",
  "request.operation != request.authority.operation()",
  "request.generation != request.authority.generation()",
  "request.base_revision != request.authority.base_revision()",
  "authority.prepare_one_item",
  "fn cancel(&mut self)",
  "fn begin_close(&mut self)",
  "base.return_to_registry()",
  "fn terminal_is_empty(&self)",
  "LocalizedLabel::native",
  "SelectionMode::Multiple",
  ".default_layout(edit::layout())",
];

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

const contractPattern = (id: string): RegExp => new RegExp(`ArtifactToolPublicationContract \\{ tool_id: "${id}", lanes: &\\[[^\\]]*\\] \\},?`);

/** ⚖️ One app's routes must be the same set in its tool-id constants, its publication contracts and its
 * `Migrated` classifications — the exact three-way join `validate_tool_job_rows` demands. A route the
 * framework injects already classified is excluded from the classification side only: it still needs
 * its constant row and its publication contract (draw has none today — the shell's utility swap is a
 * window-transient edit, not a routed tool). */
function appOracle(app: AppAuthority, source: string): boolean {
  const ids = app.toolIdsConstants.flatMap((constant) => [...(source.match(new RegExp(`${constant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? [])].map((match) => match[1]!));
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((match) => `${match[1]}:${[...match[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((lane) => lane[1]).sort().join("+")}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", (?:semio_framework_plugin::)?InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = app.routes.map(({ id }) => id);
  const authored = app.routes.filter((route) => route.frameworkInjected !== true).map(({ id }) => id);
  const reserved=[...(source.match(new RegExp(`${app.reservedToolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`,"s"))?.[1]?.matchAll(/"([^"]+)"/g)??[])].map(match=>match[1]!);
  return Object.values(app.laws).every(Boolean)
    && exact(reserved,app.reservedRoutes.map(route=>route.id))&&app.reservedRoutes.every(route=>!expected.includes(route.id))&&source.includes("fn build_reserved_tool_job(")&&source.includes(`${app.reservedToolIdsConstant}.contains(&request.tool_id.as_str())`)
    && app.ui.locales.join(",") === "en,de" && app.ui.accessibleLabels && app.ui.customizableUi
    && app.routes.every((route) => route.lanes.length > 0 && (!route.lanes.includes("host-only") || route.lanes.length === 1))
    && exact(ids, expected) && exact(contracts, app.routes.map(({ id, lanes }) => `${id}:${[...lanes].map(variant).sort().join("+")}`)) && exact(classifications, authored)
    && ANCHORS.every((anchor) => source.includes(anchor));
}

function oracle(fixture: Fixture, sources: Map<string, string>): boolean {
  return fixture.schema === "semio.app.publication-authority.v1" && fixture.apps.length > 0 && fixture.apps.every((app) => appOracle(app, sources.get(app.owner) ?? ""));
}

/** 🗡️ Four source mutations per app, derived from that app's own routes, that MUST break its oracle. */
function hostileSources(app: AppAuthority, source: string): string[] {
  const last = app.routes[app.routes.length - 1]!;
  const authored = app.routes.filter((route) => route.frameworkInjected !== true);
  const second = authored[Math.min(1, authored.length - 1)]!;
  return [
    source.replace(contractPattern(last.id), ""),
    source.replaceAll("request.base_revision != request.authority.base_revision()", ""),
    source.replace(`.action_interactive_job("${second.id}", semio_framework_plugin::InteractiveJobClassification::Migrated)`, ""),
    source.replace("fn register_window_config_owners(", "fn register_window_config_owners_detached("),
  ];
}

/** 🔎️ Strict checking for the owned path raster pipeline and its framework consumers. */
function typecheckPathRaster(repoRoot:string,raster:string,ownedSources:string[]=[]):void {
  runCmd(process.execPath, [join(repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", join(raster, "🟦️.ts"),...ownedSources], {cwd:repoRoot});
}

/** 🌐️ Official pinned Unicode inputs produce reviewable immutable first-party shaping records. */
async function provisionUnicodeShapeTables(repoRoot:string):Promise<void>{
 const version="16.0.0",base=`https://www.unicode.org/Public/${version}/ucd/`,root=join(repoRoot,"🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/📦️data"),files=["ReadMe.txt","UnicodeData.txt","PropertyValueAliases.txt","DerivedNormalizationProps.txt","DerivedCoreProperties.txt","PropList.txt","Scripts.txt","BidiBrackets.txt","BidiMirroring.txt","extracted/DerivedBidiClass.txt","extracted/DerivedJoiningType.txt","auxiliary/GraphemeBreakProperty.txt","emoji/emoji-data.txt"];
 const inputs=new Map<string,string>(),sources:{file:string;url:string;sha256:string;bytes:number}[]=[];
 const previousFile=Bun.file(join(root,"🔣️provenance.json")),previous=await previousFile.exists()?await previousFile.json()as{sources:typeof sources}:null;
 for(const file of [...files,"License.txt"]){const name=file.replaceAll("/","-"),path=join(root,"📥️sources",name),url=file==="License.txt"?"https://www.unicode.org/license.txt":base+file,existing=Bun.file(path);let bytes:Uint8Array;if(await existing.exists())bytes=new Uint8Array(await existing.arrayBuffer());else{const response=await fetch(url,{signal:AbortSignal.timeout(120000)});if(!response.ok)throw Error(`Unicode input refused ${response.status}: ${url}`);bytes=new Uint8Array(await response.arrayBuffer());await Bun.write(path,bytes);}const sha256=new Bun.CryptoHasher("sha256").update(bytes).digest("hex"),record=previous?.sources.find(row=>row.file===name);if(record&&record.sha256!==sha256)throw Error(`Pinned Unicode source changed: ${name}`);inputs.set(file,new TextDecoder().decode(bytes));sources.push({file:name,url,sha256,bytes:bytes.length});console.log(`[DEBUG] Unicode ${version} original input ${name}: ${bytes.length} bytes, sha256=${sha256}`);}
 if(!inputs.get("ReadMe.txt")!.includes("Version 16.0.0"))throw Error("Pinned Unicode version authority mismatch");
 const categories=["Cn","Lu","Ll","Lt","Lm","Lo","Mn","Mc","Me","Nd","Nl","No","Pc","Pd","Ps","Pe","Pi","Pf","Po","Sm","Sc","Sk","So","Zs","Zl","Zp","Cc","Cf","Cs","Co"],bidiClasses=["L","R","AL","EN","ES","ET","AN","CS","NSM","BN","B","S","WS","ON","LRE","LRO","RLE","RLO","PDF","LRI","RLI","FSI","PDI"],joiningTypes=["U","R","D","C","L","T"],graphemeClasses=["Other","CR","LF","Control","Extend","ZWJ","Regional_Indicator","Prepend","SpacingMark","L","V","T","LV","LVT"];
 const maximum=0x110000,ccc=new Uint8Array(maximum),category=new Uint8Array(maximum),bidi=new Uint8Array(maximum),joining=new Uint8Array(maximum),grapheme=new Uint8Array(maximum),flags=new Uint8Array(maximum),scripts=new Uint32Array(maximum),scriptAliases=new Map<string,string>(),bidiAliases=new Map<string,string>(),joiningAliases=new Map<string,string>();scripts.fill(0x5a7a7a7a);
 const lines=(file:string)=>inputs.get(file)!.split(/\r?\n/).map(line=>line.split("#")[0]!.trim()).filter(Boolean).map(line=>line.split(";").map(word=>word.trim()));
 for(const row of lines("PropertyValueAliases.txt")){if(row[0]==="sc")for(const alias of row.slice(1))scriptAliases.set(alias,row[1]!);if(row[0]==="bc")for(const alias of row.slice(1))bidiAliases.set(alias,row[1]!);if(row[0]==="jt")for(const alias of row.slice(1))joiningAliases.set(alias,row[1]!);}
 const index=(values:string[],name:string)=>{const at=values.indexOf(name);if(at<0)throw Error(`Unknown Unicode property ${name}`);return at;},range=(span:string):readonly[number,number]=>{const words=span.split("..");return[parseInt(words[0]!,16),parseInt(words[1]??words[0]!,16)];};
 const decompositions:{code:number;values:number[];compatibility:boolean}[]=[];let first:number|null=null;
 for(const row of lines("UnicodeData.txt")){const code=parseInt(row[0]!,16);if(row[1]!.endsWith(", First>")){first=code;continue;}const from=first??code;first=null;ccc.fill(Number(row[3]),from,code+1);category.fill(index(categories,row[2]!),from,code+1);bidi.fill(index(bidiClasses,row[4]!),from,code+1);if(row[5]){const compatibility=row[5]!.startsWith("<"),text=row[5]!.replace(/^<[^>]+>\s*/,"");decompositions.push({code,values:text.split(" ").map(word=>parseInt(word,16)),compatibility});}}
 const apply=(file:string,target:Uint8Array,values:string[],aliases?:Map<string,string>)=>{for(const raw of inputs.get(file)!.split(/\r?\n/)){const at=raw.indexOf("@missing:");if(at<0)continue;const row=raw.slice(at+9).split(";").map(word=>word.trim()),[from,to]=range(row[0]!);target.fill(index(values,aliases?.get(row[1]!)??row[1]!),from,to+1);}for(const row of lines(file)){const[from,to]=range(row[0]!);target.fill(index(values,aliases?.get(row[1]!)??row[1]!),from,to+1);}};
 apply("extracted/DerivedBidiClass.txt",bidi,bidiClasses,bidiAliases);apply("extracted/DerivedJoiningType.txt",joining,joiningTypes,joiningAliases);apply("auxiliary/GraphemeBreakProperty.txt",grapheme,graphemeClasses);
 const bit=(file:string,property:string,value:number)=>{for(const row of lines(file)){if(row[1]!==property)continue;const[from,to]=range(row[0]!);for(let code=from;code<=to;code++)flags[code]!|=value;}};
 bit("DerivedCoreProperties.txt","Default_Ignorable_Code_Point",1);bit("emoji/emoji-data.txt","Extended_Pictographic",2);bit("DerivedNormalizationProps.txt","Full_Composition_Exclusion",4);bit("PropList.txt","Variation_Selector",8);for(const row of lines("DerivedCoreProperties.txt")){if(row[1]!=="InCB")continue;const value=row[2]==="Consonant"?16:row[2]==="Extend"?32:row[2]==="Linker"?64:0;if(!value)throw Error("Unsupported Indic conjunct property");const[from,to]=range(row[0]!);for(let code=from;code<=to;code++)flags[code]!|=value;}
 for(const row of lines("Scripts.txt")){const[from,to]=range(row[0]!),tag=scriptAliases.get(row[1]!);if(!tag||tag.length!==4)throw Error(`Unicode script alias missing ${row[1]}`);const value=tag.charCodeAt(0)*16777216+tag.charCodeAt(1)*65536+tag.charCodeAt(2)*256+tag.charCodeAt(3);scripts.fill(value,from,to+1);}
 const ranges:number[][]=[];let start=0;const same=(a:number,b:number)=>ccc[a]===ccc[b]&&category[a]===category[b]&&bidi[a]===bidi[b]&&joining[a]===joining[b]&&grapheme[a]===grapheme[b]&&flags[a]===flags[b]&&scripts[a]===scripts[b];
 for(let code=1;code<=maximum;code++){if(code<maximum&&same(start,code))continue;ranges.push([start,code-1,ccc[start]!,category[start]!,bidi[start]!,joining[start]!,grapheme[start]!,flags[start]!,scripts[start]!]);start=code;}
 const compositions=decompositions.filter(row=>!row.compatibility&&row.values.length===2&&!(flags[row.code]!&4)).map(row=>[row.values[0]!,row.values[1]!,row.code]).sort((a,b)=>a[0]!-b[0]!||a[1]!-b[1]!);const scalars=decompositions.flatMap(row=>row.values),mirrors=lines("BidiMirroring.txt").map(row=>[parseInt(row[0]!,16),parseInt(row[1]!,16)]),brackets=lines("BidiBrackets.txt").map(row=>[parseInt(row[0]!,16),parseInt(row[1]!,16),row[2]==="o"?1:2]);
 const rangeOffset=64,decompositionOffset=rangeOffset+ranges.length*24,compositionOffset=decompositionOffset+decompositions.length*16,scalarOffset=compositionOffset+compositions.length*12,mirrorOffset=scalarOffset+scalars.length*4,bracketOffset=mirrorOffset+mirrors.length*8,byteLength=bracketOffset+brackets.length*12,bytes=new Uint8Array(byteLength),view=new DataView(bytes.buffer),word=(at:number,value:number)=>view.setUint32(at,value,false);
 [0x55434450,0x00100000,ranges.length,rangeOffset,decompositions.length,decompositionOffset,compositions.length,compositionOffset,scalars.length,scalarOffset,mirrors.length,mirrorOffset,brackets.length,bracketOffset,byteLength,0].forEach((value,index)=>word(index*4,value));
 ranges.forEach((row,index)=>{const at=rangeOffset+index*24;word(at,row[0]!);word(at+4,row[1]!);for(let field=2;field<8;field++)bytes[at+6+field]=row[field]!;word(at+16,row[8]!);});let scalarAt=0;decompositions.forEach((row,index)=>{const at=decompositionOffset+index*16;word(at,row.code);word(at+4,scalarAt);word(at+8,row.values.length);word(at+12,Number(row.compatibility));scalarAt+=row.values.length;});compositions.forEach((row,index)=>row.forEach((value,field)=>word(compositionOffset+index*12+field*4,value)));scalars.forEach((value,index)=>word(scalarOffset+index*4,value));mirrors.forEach((row,index)=>row.forEach((value,field)=>word(mirrorOffset+index*8+field*4,value)));brackets.forEach((row,index)=>row.forEach((value,field)=>word(bracketOffset+index*12+field*4,value)));
 const provenance={version,byteOrder:"big-endian",headerBytes:64,rangeBytes:24,decompositionBytes:16,compositionBytes:12,sources,categories,bidiClasses,joiningTypes,graphemeClasses};const schema=await Bun.file(join(root,"../🧬️schema/🔣️.json")).json();if(!new Ajv({strict:true}).compile(schema)(provenance))throw Error("Unicode provenance schema refused");await Bun.write(join(root,"🔣️provenance.json"),JSON.stringify(provenance,null,2)+"\n");await Bun.write(join(root,"🔢️properties.bin"),bytes);console.log(`[DEBUG] Unicode ${version} immutable authority: ranges=${ranges.length}, decompositions=${decompositions.length}, compositions=${compositions.length}, mirrors=${mirrors.length}, brackets=${brackets.length}, bytes=${byteLength}, sha256=${new Bun.CryptoHasher("sha256").update(bytes).digest("hex")}`);
}
class TestScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    const subset = join(this.repoRoot, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any");
    runCmd(process.execPath,["test",join(subset,"🚪️io/📝️text/🔺️diff/🧪️tests/🔬️unit/🟦️.ts")]);
    await (await import(join(subset,"🚪️io/🖼️image/🧪️tests/🟦️.ts"))).testDrawingImageAdmission();
    const raster=join(subset,"🧬️schema/🧮️geometry/📷️raster");
    const sceneRaster=join(subset,"🧬️schema/🎬️scene/📷️raster");
    const scenePrepare=join(subset,"🧬️schema/🎬️scene/📋️prepare");
    const sceneBooleans=join(subset,"🧬️schema/🎬️scene/🔀️booleans");
    const sceneTrace=join(subset,"🧬️schema/🎬️scene/🔍️trace");
    const sceneRetire=join(subset,"🧬️schema/🎬️scene/🧹️retire");
    const sceneIdentity=join(subset,"🧬️schema/🎬️scene/🪪️identity");
    const sceneView=join(subset,"🧬️schema/🎬️scene/👁️view");
    const scenePaint=join(subset,"🧬️schema/🎬️scene/🎨️paint");
    const scenePicking=join(scenePaint,"📋️prepare/🎯️query");
    const selectionStatus=join(subset,"✏️editor/🧮️status");
    if(segments.length) {
      if(segments.length===2&&segments[0]==="font-shape"&&segments[1]==="unicode") {await provisionUnicodeShapeTables(this.repoRoot);return;}
      if(segments.length===1&&segments[0]==="owned-export") {const writer=join(subset,"🚪️io/📤️export/📦️owned");runCmd(process.execPath,["test","--timeout","120000",join(writer,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,join(writer,"🧪️tests"));return;}
      if(segments.length===1&&segments[0]==="scene-text") {const text=join(subset,"🧬️schema/🎬️scene/🔤️text");runCmd(process.execPath,["test","--timeout","120000",join(text,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,text,[join(subset,"🧬️schema/🎬️scene/📋️prepare/🟦️.ts")]);return;}
      if(segments.length===1&&segments[0]==="font-shape") {const shape=join(this.repoRoot,"🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape");runCmd(process.execPath,["test","--timeout","120000",join(shape,"🧪️tests/🟦️.ts"),join(shape,"🌐️unicode/🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,shape,[join(shape,"🌐️unicode/🟦️.ts")]);return;}
      if(segments.length===1&&segments[0]==="font-outline") {const font=join(this.repoRoot,"🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font");runCmd(process.execPath,["test",join(font,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,font,[join(font,"🧵️outline/🟦️.ts"),join(font,"🤝️kerning/🟦️.ts"),join(font,"🧵️run/🟦️.ts"),join(font,"📇️catalog/🟦️.ts")]);return;}
      if(segments.length===1&&segments[0]==="pdf-write") {const writer=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write");runCmd(process.execPath,["test","--timeout","120000",join(writer,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,writer);return;}
      if(segments.length===1&&segments[0]==="svg-write") {const writer=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write");runCmd(process.execPath,["test",join(writer,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,writer);return;}
      if(segments.length===1&&segments[0]==="png-export") {const exported=join(subset,"✏️editor/🎮️commands/📤️export-document");runCmd(process.execPath,["test",join(exported,"🧪️tests/🔬️unit/🟦️.ts")]);typecheckPathRaster(this.repoRoot,exported);return;}
      if(segments.length===1&&segments[0]==="image-import") {const imported=join(subset,"✏️editor/🎮️commands/📥️import-image");runCmd(process.execPath,["test",join(imported,"🧪️tests/🔬️unit/🟦️.ts"),join(imported,"🧵️publication/🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,imported,[join(imported,"🧵️publication/🟦️.ts"),join(imported,"🧵️publication/🧪️tests/🟦️.ts")]);return;}
      if(segments.length===1&&segments[0]==="shape-coordinates") {
        const coordinates=join(subset,"🧬️schema/🔷️shape/✏️coordinates");
        runCmd(process.execPath,["test",join(coordinates,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,coordinates);
        return;
      }
      if(segments.length===1&&segments[0]==="selection-actions") {
        const actions=join(subset,"✏️editor/📌️panels/🔍️properties/🎬️actions");
        runCmd(process.execPath,["test",join(actions,"🧪️tests/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,actions);
        return;
      }
      if(segments.length===1&&segments[0]==="field-history") {
        const patch=join(subset,"✏️editor/🎮️commands/🩹️patch-layer");
        runCmd(process.execPath,["test",join(patch,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,patch);
        return;
      }
      if(segments.length===1&&segments[0]==="affine-rendering") {
        const affine=join(this.repoRoot,"🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/↗️affine");
        runCmd(process.execPath,["test",join(affine,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,affine);
        return;
      }
      if(segments.length===1&&segments[0]==="path-editing") {
        const editing=join(subset,"🧬️schema/🧮️geometry/✏️editing");
        runCmd(process.execPath,["test",join(editing,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,editing);
        return;
      }
      if(segments.length===1&&segments[0]==="clipboard"){
        const clipboard=join(subset,"✏️editor/📋️clipboard");
        runCmd(process.execPath,["test",join(clipboard,"🧪️tests/🔬️unit/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🔀️combine-boolean/🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,clipboard);
        return;
      }
      if(segments.length===1&&segments[0]==="scene-placement"){
        const placement=join(subset,"🧬️schema/🎬️scene/📍️placement");
        runCmd(process.execPath,["test","--timeout","120000",join(placement,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,placement,[join(scenePrepare,"🟦️.ts")]);
        return;
      }      if(segments.length===1&&segments[0]==="image-host"){runCmd(process.execPath,["test","--timeout","120000",join(subset,"../../../../🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts")]);return;}
      if(segments.length!==1||!['path-raster','scene-raster','scene-paint','scene-picking','selection-status','image-output'].includes(segments[0]!)) throw Error("Unknown Draw test selection "+segments.join(" "));
      const selected=segments[0]==='image-output'?join(subset,'🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any'):segments[0]==='scene-raster'?sceneRaster:segments[0]==='scene-paint'?scenePaint:segments[0]==='scene-picking'?scenePicking:segments[0]==='selection-status'?selectionStatus:raster;
      runCmd(process.execPath,["test","--timeout","120000",join(selected,"🧪️tests/🔬️unit/🟦️.ts"),...(selected===scenePaint?[join(scenePaint,"📋️prepare/🧪️tests/🔬️unit/🟦️.ts")]:[]),...(selected===sceneRaster?[join(sceneRaster,"🧪️tests/🔬️unit/🖼️images/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🖼️assets/🟦️.ts"),join(scenePrepare,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneBooleans,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneTrace,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRetire,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneIdentity,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneView,"🧪️tests/🔬️unit/🟦️.ts")]:[])]);
      typecheckPathRaster(this.repoRoot,selected,selected===scenePaint?[join(scenePaint,"📋️prepare/🟦️.ts")]:selected===sceneRaster?[join(subset,"🚪️io/🖼️image/🟦️.ts"),join(scenePrepare,"🟦️.ts"),join(sceneBooleans,"🟦️.ts"),join(sceneTrace,"🟦️.ts"),join(sceneRetire,"🟦️.ts"),join(sceneIdentity,"🟦️.ts"),join(sceneIdentity,"🚦️admission/🟦️.ts"),join(sceneView,"🟦️.ts"),join(scenePaint,"🟦️.ts"),join(scenePaint,"📋️prepare/🟦️.ts"),join(scenePaint,"📋️prepare/🎯️query/🟦️.ts")]:[]);
      return;
    }
    runCmd(process.execPath,["test",join(subset,"✏️editor/📋️clipboard/🧪️tests/🔬️unit/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🔀️combine-boolean/🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,join(subset,"✏️editor/📋️clipboard"));
    const inspectorActions=join(subset,"✏️editor/📌️panels/🔍️properties/🎬️actions");
    runCmd(process.execPath,["test",join(inspectorActions,"🧪️tests/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,inspectorActions);
    runCmd(process.execPath,["test",join(selectionStatus,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,selectionStatus);
    runCmd(process.execPath, ["test","--timeout","120000", join(scenePrepare,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneBooleans,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneTrace,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRetire,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneIdentity,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneView,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🖼️images/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🖼️assets/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/📷️raster/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "../🎨️style/🧬️schema/🧬️mutations/🧩️set-group-isolation/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/📄️document/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/↗️transform/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🎨️fill/🌀️rule/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🛤️path/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🎯️picking/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🎛️handles/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/↗️affine/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🟦️.ts"), join(subset, "../🎨️style/🧬️schema/🧬️mutations/📝️update-text/🧪️tests/🔬️unit/🟦️.ts"), join(subset,"../🎨️style/🧬️schema/🧬️mutations/🖼️update-image/🧪️tests/🔬️unit/🟦️.ts"), join(subset,"../🔀️transform/🧬️schema/🧬️mutations/🔷️shape-coordinate/🧪️tests/🔬️unit/🟦️.ts"), join(subset,"🧬️schema/🔷️shape/✏️coordinates/🧪️tests/🔬️unit/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🩹️patch-layer/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "👁️viewer/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/📷️framing/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️window/🟦️.ts"), join(subset, "✏️editor/🎮️commands/➕️add-layer/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/↔️translation/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🎨️fill/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🖊️stroke/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🎯️selection/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🧪️tests/🔬️canvas-tool/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️drag-layers/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️rotate-layers/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️scale-layers/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️drag-path-points/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/✏️editing/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "../🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🧪️tests/📐️bounds/🟦️.ts"), join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,raster,[join(subset,"🚪️io/🖼️image/🟦️.ts"),join(scenePrepare,"🟦️.ts"),join(sceneBooleans,"🟦️.ts"),join(sceneTrace,"🟦️.ts"),join(sceneRetire,"🟦️.ts"),join(sceneIdentity,"🟦️.ts"),join(sceneIdentity,"🚦️admission/🟦️.ts"),join(sceneView,"🟦️.ts"),join(scenePaint,"🟦️.ts"),join(scenePaint,"📋️prepare/🟦️.ts"),join(scenePaint,"📋️prepare/🎯️query/🟦️.ts"),join(sceneRaster,"🟦️.ts"),join(subset,"🧬️schema/🟦️.ts"),join(subset,"🧬️schema/🧮️geometry/🎯️picking/🎨️paint/🟦️.ts"),join(subset,"🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"),join(subset,"../🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🦠️mutation/🟦️.ts"),join(subset,"../🎨️style/🧬️schema/🧬️mutations/🖼️update-image/🦠️mutation/🟦️.ts"),join(subset,"../🔀️transform/🧬️schema/🧬️mutations/🔷️shape-coordinate/🦠️mutation/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🩹️patch-layer/🟦️.ts")]);
    runCmd(process.execPath,["test",join(subset,"✏️editor/🎮️commands/📥️import-image/🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,join(subset,"✏️editor/🎮️commands/📥️import-image"));
    const placement=join(subset,"🧬️schema/🎬️scene/📍️placement");
    runCmd(process.execPath,["test","--timeout","120000",join(placement,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,placement);    const exported=join(subset,"✏️editor/🎮️commands/📤️export-document");
    runCmd(process.execPath,["test",join(exported,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,exported);
    const pdfWriter=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write");runCmd(process.execPath,["test","--timeout","120000",join(pdfWriter,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,pdfWriter);
    const svgWriter=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write");
    runCmd(process.execPath,["test",join(svgWriter,"🧪️tests/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,svgWriter);
    const font=join(this.repoRoot,"🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font"),sceneText=join(subset,"🧬️schema/🎬️scene/🔤️text");
    runCmd(process.execPath,["test","--timeout","120000",join(font,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,font,[join(font,"🧵️outline/🟦️.ts"),join(font,"🤝️kerning/🟦️.ts"),join(font,"🧵️run/🟦️.ts"),join(font,"📇️catalog/🟦️.ts")]);
    runCmd(process.execPath,["test","--timeout","120000",join(sceneText,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,sceneText,[join(scenePrepare,"🟦️.ts")]);
    const affine=join(this.repoRoot,"🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/↗️affine");
    runCmd(process.execPath,["test",join(affine,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,affine);
    await proveDrawPublicationAuthority(this.root, subset);
  }
}
/** 🔏️ Proves the current Drawing source authority and canonical field-patch outcomes. */
async function proveDrawPublicationAuthority(packageRoot: string, subset: string): Promise<void> {
    const plugin = resolve(packageRoot, "../..");
    const authority = resolve(plugin, "🧫️fixtures/🧪️publication-authority");
    const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
    const ajv = new Ajv({ allErrors: true, strict: true });
    const patchRoot = resolve(subset, "🧬️schema/🧬️mutations/🧫️fixtures/🎛️field-patch");
    const validatePatch = ajv.compile(await Bun.file(resolve(subset, "🧬️schema/🧬️mutations/🎛️field-patch/🧬️schema/🔣️.json")).json());
    const patchCases = await Bun.file(resolve(patchRoot, "🔣️.json")).json() as { patch: unknown; accepted: boolean }[];
    for (const test of patchCases) if (validatePatch(test.patch) !== test.accepted) throw new Error(`Draw field-patch oracle disagrees: ${JSON.stringify(test)}`);
    console.error(`Draw independent Ajv field-patch oracle: ${patchCases.length} cases`);
    const admission = resolve(subset, "../../../../🔨️modules/🏠️host/🧰️owned/🧫️fixtures/🧮️mutation-admission");
    const admissionFixture = await Bun.file(resolve(admission, "🔣️.json")).json() as { cases: unknown[]; bootstrapYield: BootstrapYield };
    bootstrapYieldOracle(admissionFixture.bootstrapYield);
    const sources = new Map<string, string>();
    for (const app of fixture.apps) {sources.set(app.owner, await Bun.file(resolve(plugin, app.source)).text());const producer=await Bun.file(resolve(plugin,app.reservedProducerSource)).text();for(const anchor of ["source:request.snapshot_read","NativeEncodeControl::resume","NativeDecodeControl::resume","DrawingLayerCloneWorkAuthority","fn next_close_capacity_byte_demand","fn terminal_is_empty"]){if(!producer.includes(anchor))throw new Error(`Draw reserved producer lost ${anchor}`);}console.error(`[DEBUG] Draw reserved clipboard original lease, codecs and close facets present: ${app.reservedRoutes.map(route=>route.id).join(",")}`);}
    if (!oracle(fixture, sources)) throw new Error("Draw publication-authority oracle rejected production");
    let hostile = 0;
    for (const app of fixture.apps) {
      for (const candidate of hostileSources(app, sources.get(app.owner)!)) {
        hostile += 1;
        if (candidate === sources.get(app.owner)) throw new Error(`Draw hostile mutation for ${app.owner} did not change its source`);
        if (appOracle(app, candidate)) throw new Error(`Draw oracle accepted a hostile source mutation for ${app.owner}`);
      }
      const hostileFixture: Fixture = { ...fixture, apps: fixture.apps.map((entry) => (entry.owner === app.owner ? { ...entry, routes: entry.routes.slice(1) } : entry)) };
      hostile += 1;
      if (oracle(hostileFixture, sources)) throw new Error(`Draw accepted a hostile fixture mutation for ${app.owner}`);
    }
    console.error(`validated Draw publication authority; apps=${fixture.apps.map((app) => `${app.owner}:${app.routes.length}`).join(",")}; oracle=owned; hostile=${hostile}; mutationAdmission=${admissionFixture.cases.length}`);
}
class PublicationAuthorityAuditScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("publication-authority-audit accepts no arguments");
    await proveDrawPublicationAuthority(this.root, resolve(this.root, "../../🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any"));
  }
}
class UtilityActionPolicyScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw Error("utility-action-policy accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧪️tests/🟦️.ts")],this.repoRoot,"draw-utility-action-policy",45000,{env:process.env});}
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", PublicationAuthorityAuditScript).register("utility-action-policy",UtilityActionPolicyScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
