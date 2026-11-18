import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import authored from "../../🔣️policy.json";
import fixture from "../🧫️fixtures/🧭️sites.json";
import {checkHoldAndWaitV1,checkLeaseOrderV1,classifyLeaseResourceV1,readLeaseOrderPolicyV1} from "../../🟦️.ts";
import {censusOf,listSources,pairsOf} from "./🔎️census.mjs";

const framework=resolve(import.meta.dir,"../../../../../.."),roots=[framework+"/🔨️modules/🏃️process",framework+"/🛍️products/🦑️repo/🔨️modules/📚️library"];
const live=readLeaseOrderPolicyV1(authored),sites=censusOf(roots.flatMap(root=>listSources(root)));
const nameOf=(site:{literal?:string}):string|undefined=>site.literal===undefined?undefined:site.literal.startsWith("json:")?'["'+site.literal.slice(5)+'"':site.literal;
const relative=(file:string):string=>file.slice(framework.length+1);

test("the census finds the acquisition sites of the whole build tooling, and the mapping of a native image is one of them with a resolved name",()=>{
 expect(sites.length).toBe(fixture.sites);
 const mapping=sites.filter(site=>relative(site.file).includes("🔒️mapping/🟦️.ts"));
 expect(mapping.length).toBe(3);for(const site of mapping)expect(nameOf(site)).toBe("native-storage-mapping:");
});

test("every resource name the census can resolve is ranked by the order policy, except the names acknowledged with a reason in both languages",()=>{
 const unranked=[...new Set(sites.map(nameOf).filter((name):name is string=>name!==undefined&&classifyLeaseResourceV1(live,name).id==="unclassified"))].sort();
 expect(unranked).toEqual([...fixture.unclassified.names].sort());
 expect(fixture.unclassified.reason.en.length).toBeGreaterThan(40);expect(fixture.unclassified.reason.de.length).toBeGreaterThan(40);
});

test("the sites whose name only a caller knows are counted per file, so a new one cannot appear unnoticed",()=>{
 const found:Record<string,number>={};
 for(const site of sites)if(site.literal===undefined)found[relative(site.file)]=(found[relative(site.file)]??0)+1;
 expect(Object.fromEntries(Object.entries(found).sort())).toEqual(fixture.dynamic.files);
 expect(fixture.dynamic.reason.en.length).toBeGreaterThan(40);expect(fixture.dynamic.reason.de.length).toBeGreaterThan(40);
});

test("every pair of sites where a later site lies inside the lexical extent of an earlier one is in order: not out of order and not a refused hold-and-wait, unless the pair is declared with a reason",()=>{
 const refused:string[]=[],known=new Set(fixture.knownPairs.map(row=>`${row.file}|${row.held}|${row.heldMode}|${row.wanted}|${row.wantedMode}`));
 const seen=new Set<string>();
 for(const pair of pairsOf(sites)){
  const held=nameOf(pair.held),wanted=nameOf(pair.wanted);
  if(held===undefined||wanted===undefined||pair.held.line===pair.wanted.line)continue;
  const heldMode=pair.held.mode==="shared"?"shared" as const:"exclusive" as const,wantedMode=pair.wanted.mode==="shared"?"shared" as const:"exclusive" as const,directory="/d";
  const rows=[{directory,resource:held+"x",mode:heldMode,atMs:1,task:1}],request={directory,resource:wanted+"y",mode:wantedMode,nonblocking:pair.wanted.nonblocking};
  const verdict=checkLeaseOrderV1(live,rows,request)??checkHoldAndWaitV1(live,rows,request,live.holdAndWait.graceMilliseconds+1);
  const key=`${relative(pair.file)}|${held}|${heldMode}|${wanted}|${wantedMode}`;
  if(verdict!==undefined){seen.add(key);if(!known.has(key))refused.push(`${key} ${verdict.kind}`);}
 }
 expect(refused).toEqual([]);
 expect([...known].filter(key=>!seen.has(key))).toEqual([]);
 for(const row of fixture.knownPairs){expect(row.reason.en.length).toBeGreaterThan(40);expect(row.reason.de.length).toBeGreaterThan(40);}
});

test("a site of a ranked name that waits while an exclusive lease of any other class is held is refused only when the policy declares no edge, ground or inert class for it: the ground class of the mapping is never refused",()=>{
 const names=new Set(sites.map(nameOf).filter((name):name is string=>name!==undefined&&classifyLeaseResourceV1(live,name).id!=="unclassified"));
 const ground=[...names].filter(name=>classifyLeaseResourceV1(live,name).ground!==undefined);
 expect(ground).toEqual(["native-storage-mapping:"]);
 for(const holder of names)expect(checkHoldAndWaitV1(live,[{directory:"/d",resource:holder+"x",mode:"exclusive",atMs:1,task:1}],{directory:"/d",resource:"native-storage-mapping:"+"a".repeat(64),mode:"shared",nonblocking:false},600000),holder).toBeUndefined();
});
