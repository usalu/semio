/** 🚪️ Contract: the source closure of the authorizer compiler stays inside a declared byte budget with headroom below the hard admission limit, and reaches no module that decides by itself whether it is the process entry: the compiler is one single-file bundle in which every module shares the entry's `import.meta.url`, so such a guard is true for all of them (a replica supervisor once started inside the bundle). The bound forms `isMainModuleV1(import.meta.url)` and `import.meta.main` are rewritten per module and stay allowed. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {censusNxAuthorizerSourcesV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import snapshotPolicy from "../../📦️snapshot/🔣️policy.json";
import fixture from "./🧫️guards.json";

const law=schema.$defs.CompilerClosureLaw.const,here=dirname(fileURLToPath(import.meta.url)),root=resolve(here,"../../../../../../../../../../.."),recipe="\u{1F9F0}️framework/\u{1F6CD}️products/\u{1F991}️repo/\u{1F528}️modules/\u{1F4DA}️library/⚡️caching/\u{1F680}️bootstrap/\u{1F6E0}️tools/";
const guards=law.entryGuards.map(source=>new RegExp(source,"u")),entry=recipe+law.entry;
const bound=new RegExp(law.entryGuardBound,"u"),guarded=(code:string)=>code.split("\n").some(line=>!bound.test(line)&&guards.some(pattern=>pattern.test(line)));
const text=(language:"en"|"de",key:"budget"|"guard",fields:Record<string,string|number>)=>law.texts[language][key].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]));
const python=`import json,re,sys
case=json.load(sys.stdin);patterns=[re.compile(p) for p in case["patterns"]]
bound=re.compile(case["bound"])
print(json.dumps([any(not bound.search(line) and any(p.search(line) for p in patterns) for line in row["code"].split("\\n")) for row in case["vectors"]]))`;

test("the entry guard rule names the vectors as an independent Python reading does",()=>{
 const run=Bun.spawnSync(["python3","-I","-c",python],{stdin:new TextEncoder().encode(JSON.stringify({patterns:law.entryGuards,bound:law.entryGuardBound,vectors:fixture.vectors}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const independent=JSON.parse(run.stdout.toString()) as boolean[];
 fixture.vectors.forEach((row,at)=>{expect(guarded(row.code),row.id).toBe(row.guard);expect(independent[at],row.id+" (python)").toBe(row.guard);});
});

test("the budget leaves the declared headroom below the hard admission limit and both languages state it",()=>{
 expect(law.maximumSourceBytes).toBeLessThanOrEqual(snapshotPolicy.maximumSourceBytes*(10000-law.reservedHeadroomBasisPoints)/10000);
 expect(law.maximumSnapshotBytes).toBeLessThanOrEqual(snapshotPolicy.maximumSnapshotBytes*(10000-law.reservedHeadroomBasisPoints)/10000);
 for(const language of["en","de"] as const){const line=text(language,"budget",{files:1,sourceBytes:2,budgetBytes:3,headroom:4,limitBytes:5});expect(line).not.toContain("{");expect(line).toContain("4");}
});

test("the source closure of the authorizer compiler stays inside its budget and reaches no module that is a process entry by itself",async()=>{
 const census=await censusNxAuthorizerSourcesV1(root,entry,new AbortController().signal);
 const headroom=Math.floor((law.maximumSourceBytes-census.sourceBytes)*100/law.maximumSourceBytes),report=text("en","budget",{files:census.files,sourceBytes:census.sourceBytes,budgetBytes:law.maximumSourceBytes,headroom,limitBytes:snapshotPolicy.maximumSourceBytes});
 console.log("[headroom] "+report+" "+text("de","budget",{files:census.files,sourceBytes:census.sourceBytes,budgetBytes:law.maximumSourceBytes,headroom,limitBytes:snapshotPolicy.maximumSourceBytes}));
 expect(census.sourceBytes,report).toBeLessThanOrEqual(law.maximumSourceBytes);
 expect(census.snapshotBytes,report).toBeLessThanOrEqual(law.maximumSnapshotBytes);
 expect(census.files,report).toBeLessThanOrEqual(law.maximumFiles);
 const found:string[]=[];
 for(const source of census.sources){
  if(source.path===entry||!/\.[cm]?[jt]sx?$/u.test(source.path))continue;
  if(guarded(readFileSync(join(root,source.path),"utf8")))found.push(source.path);
 }
 expect(found.map(path=>text("en","guard",{path})),"modules with their own entry guard").toEqual([]);
},120000);
