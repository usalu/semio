/** 🧫️ Shipped layout tables agree with independently parsed OpenType feature and lookup authority. */
import {test,expect} from "bun:test";
import "../📇️face/🧪️tests/🟦️.ts";
import "../🪄️substitute/🧪️tests/🟦️.ts";
import "../🏷️class/🧪️tests/🟦️.ts";
import "../🧵️layout/🧪️tests/🟦️.ts";
import "../📍️mark/🧪️tests/🟦️.ts";
import "../🧩️context/🧪️tests/🟦️.ts";
import Ajv from "ajv";
import opentype from "opentype.js";
import {FontFaceAdmissionJob,type FontOutlineLimits} from "../../🟦️.ts";
import {builtinFontLocations} from "../../📇️catalog/🟦️.ts";
import {FontLookupPlanJob} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import rows from "../🧫️fixtures/🔣️.json";
const limits:FontOutlineLimits={maxFontBytes:67108864,maxTables:128,maxGlyphs:262144,maxPoints:1048576,maxContours:65536,maxComponents:262144,maxDepth:32,maxSegments:1048576,maxWork:1e9};
const features=["ccmp","rlig","liga","clig","calt","kern","mark","mkmk"];
test("shared clusters and original layout plans match independent OpenType",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);
 const sources=[...new Map(["Anta","Kelly Slab","Share Tech Mono","Noto Emoji"].flatMap(family=>builtinFontLocations(family)).map(source=>[source.url.href,source])).values()];
 for(const source of sources){const bytes=new Uint8Array(await Bun.file(source.url).arrayBuffer()),oracle=opentype.parse(bytes.buffer),admission=new FontFaceAdmissionJob(bytes,limits);while(!admission.advance(7).done){}const face=admission.result();
  for(const kind of ["substitution","positioning"]as const){const table=(oracle.tables as any)[kind==="substitution"?"gsub":"gpos"];const script=table?.scripts.find((row:any)=>row.tag==="latn")??table?.scripts.find((row:any)=>row.tag==="DFLT"),lang=script?.script.defaultLangSys;const active=lang?[...(lang.reqFeatureIndex!==65535?[lang.reqFeatureIndex]:[]),...lang.featureIndexes]:[];
   const indices=[...new Set<number>(active.flatMap((index:number)=>{const feature=table.features[index];return index===lang.reqFeatureIndex||features.includes(feature.tag)?feature.feature.lookupListIndexes:[]}))].sort((a,b)=>a-b);
   for(const grant of [1,7,4096]){const job=new FontLookupPlanJob(face,kind,"latn",features,limits.maxWork);let work=0;while(true){const progress=job.advance(grant);expect(progress.work-work).toBeLessThanOrEqual(grant);work=progress.work;if(progress.done)break;expect(()=>job.result()).toThrow();}const actual=job.result();expect(actual.map(lookup=>lookup.index)).toEqual(indices);for(const lookup of actual){expect(lookup.kind).toBe(table.lookups[lookup.index].lookupType);expect(lookup.flags).toBe(table.lookups[lookup.index].lookupFlag);expect(lookup.subtables).toBe(table.lookups[lookup.index].subtables.length);}expect(face.bytes).toBe(bytes);const moved=job.intoRetirement();while(!moved.job.advance(1).done){}expect(moved.output).toBe(actual);}
  }const moved=admission.intoRetirement();while(!moved.job.advance(1).done){}console.log(`[DEBUG] Original font layout ${source.family}/${source.id}: independent OpenType lookup order and flags matched all grants`);
 }
},120000);
test("font lookup interruptions retain original source and withhold private plans",async()=>{const source=builtinFontLocations("Anta")[0]!,bytes=new Uint8Array(await Bun.file(source.url).arrayBuffer()),admission=new FontFaceAdmissionJob(bytes,limits);while(!admission.advance(7).done){}const face=admission.result();for(const stop of [0,1,2,7,20,40,100]){const job=new FontLookupPlanJob(face,"substitution","latn",features,limits.maxWork);if(stop)job.advance(stop);job.cancel();expect(()=>job.result()).toThrow();const moved=job.intoRetirement();expect(moved.output).toBeNull();while(!moved.job.advance(1).done){}expect(face.bytes).toBe(bytes);}const moved=admission.intoRetirement();while(!moved.job.advance(1).done){}console.log("[DEBUG] Original layout seven interruption stages retained borrowed font and private table plan until explicit close");});

test("actual authored ligature components resolve one source glyph and match independent OpenType",async()=>{const {FontLigatureJob}=await import("../🔗️ligature/🟦️.ts");let matched=0;for(const source of builtinFontLocations("Noto Emoji")){const bytes=new Uint8Array(await Bun.file(source.url).arrayBuffer()),oracle=opentype.parse(bytes.buffer),admission=new FontFaceAdmissionJob(bytes,limits);while(!admission.advance(7).done){}const face=admission.result(),plan=new FontLookupPlanJob(face,"substitution","DFLT",["ccmp","rlig","liga"],limits.maxWork);while(!plan.advance(7).done){}for(const lookup of plan.result().filter(row=>row.kind===4&&row.flags===0)){const foreign=(oracle.tables as any).gsub.lookups[lookup.index];for(const table of foreign.subtables){const coverage=(oracle as any).substitution.expandCoverage(table.coverage) as number[];for(const index of new Set([0,Math.floor(coverage.length/2),coverage.length-1])){if(index<0)continue;const set=table.ligatureSets[index];for(const row of [set[0],set.at(-1)]){const glyphs=[coverage[index]!,...row.components];for(const grant of [1,7,4096]){const job=new FontLigatureJob(face,lookup,glyphs,0,limits.maxWork);let work=0;while(true){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;expect(()=>job.result()).toThrow();}expect(job.result()).toEqual({glyph:row.ligGlyph,consumed:glyphs.length});const moved=job.intoRetirement();while(!moved.job.advance(1).done){}expect(moved.output).toEqual({glyph:row.ligGlyph,consumed:glyphs.length});matched++;}}}}}const selected=plan.intoRetirement();while(!selected.job.advance(1).done){}const original=admission.intoRetirement();while(!original.job.advance(1).done){}expect(face.bytes).toBe(bytes);}expect(matched).toBeGreaterThan(0);console.log(`[DEBUG] Actual Noto ligature matches=${matched}: independent OpenType component sets resolved original glyph at grants 1/7/4096`);},120000);
