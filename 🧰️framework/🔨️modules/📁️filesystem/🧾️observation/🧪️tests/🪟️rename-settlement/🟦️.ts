/** 🪟️ A Windows folder move that a foreign handle blocks is waited for with POSIX semantics, a bounded back-off and cancellation, then refused typed in two languages; an own handle below the source can never be moved over; every outcome equals an independent Python simulation of a Windows-shaped volume (NT statuses, share modes, open-handle table). */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {ArtifactWindowsRelativeRenameRefusedErrorV1,artifactWindowsRelativeHandlesBelowV1,settleArtifactWindowsRelativeRenameV1} from "../../📁️descriptor/🟦️.ts";
import fixture from "./🔣️.json";

type Foreign={pid:number;where:"at"|"below";shareDelete:boolean;releaseAtMs:number|null};
type Scenario=(typeof fixture)["scenarios"][number];
const SHARING=0xc0000043,DENIED=0xc0000022,COLLISION=0xc0000035,INVALID_CLASS=0xc0000003;

function volumeStatus(scenario:Scenario,method:string,clock:number):number{
 const volume=scenario.volume,active=(volume.foreign as Foreign[]).filter(item=>item.releaseAtMs===null||clock<item.releaseAtMs);
 if(method==="posix-ex"&&!volume.posixEx)return INVALID_CLASS;
 if(active.some(item=>item.where==="at"&&!item.shareDelete))return SHARING;
 if(active.some(item=>item.where==="below"&&!item.shareDelete))return DENIED;
 if(volume.readonly&&method!=="posix-ex")return DENIED;
 if(volume.collision)return COLLISION;
 return 0;
}

async function simulate(scenario:Scenario){
 let clock=0,calls=0;const pauses:number[]=[],reports:number[]=[];
 const port={source:fixture.source,target:fixture.target,ownHandles:()=>scenario.own,
  attempt:(method:"posix-ex"|"classic")=>{calls++;return volumeStatus(scenario,method,clock);},
  sleep:async(milliseconds:number)=>{pauses.push(milliseconds);clock+=milliseconds;},now:()=>clock,
  checkpoint:()=>{if(scenario.cancelAfterPauses!==null&&pauses.length>=scenario.cancelAfterPauses)throw Error("cancelled");},
  fail:(status:number)=>Object.assign(Error("failed"),{status}),
  holders:()=>(scenario.volume.foreign as Foreign[]).filter(item=>item.releaseAtMs===null||clock<item.releaseAtMs).map(item=>"process "+item.pid),
  report:(row:{waitedMilliseconds:number})=>{reports.push(row.waitedMilliseconds);}};
 try{const done=await settleArtifactWindowsRelativeRenameV1(port);return{outcome:"moved",method:done.method,attempts:done.attempts,waited:done.waitedMilliseconds,pauses,reports};}
 catch(error){
  if(error instanceof ArtifactWindowsRelativeRenameRefusedErrorV1)return error.reason==="own-handles"?{outcome:"refused-own",attempts:calls,waited:clock,pauses,reports,en:error.texts.en,de:error.texts.de}:{outcome:"refused-foreign",status:error.status,attempts:error.attempts,waited:error.waitedMilliseconds,pauses,reports,holders:[...error.holders],en:error.texts.en,de:error.texts.de};
  const status=(error as{status?:number}).status;
  if(status!==undefined)return{outcome:"failed",status,attempts:calls,waited:clock,pauses,reports};
  if((error as Error).message==="cancelled")return{outcome:"cancelled",attempts:calls,waited:clock,pauses,reports};
  throw error;
 }
}

const oracle=()=>{
 const run=spawnSync("python3",[fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url)),fileURLToPath(new URL("../../📁️descriptor/🧬️schema/🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(run.status).toBe(0);return JSON.parse(run.stdout) as {scenarios:Record<string,unknown>[];tables:number[]};
};

test("every scenario of the Windows-shaped volume ends as the hand-authored expectation says",async()=>{
 for(const scenario of fixture.scenarios){const got=await simulate(scenario);expect(got,scenario.name).toMatchObject(Object.fromEntries(Object.entries(scenario.expect).filter(([key])=>key!=="firstReports"&&key!=="statusName")));
  const expectation=scenario.expect as Record<string,unknown>;
  if(expectation.firstReports)expect(got.reports.slice(0,(expectation.firstReports as number[]).length),scenario.name).toEqual(expectation.firstReports as number[]);
  if(expectation.statusName){expect((got as{en:string}).en,scenario.name).toContain(expectation.statusName as string);expect((got as{de:string}).de,scenario.name).toContain(expectation.statusName as string);}}
});

test("every scenario equals the independent Python simulation, texts included",async()=>{
 const expected=oracle().scenarios;
 for(const [index,scenario] of fixture.scenarios.entries())expect(JSON.parse(JSON.stringify(await simulate(scenario))),scenario.name).toEqual(expected[index]);
});

test("a refusal names source, target, NT status, words, attempts and holders in English and German and never an internal error",async()=>{
 const forever=fixture.scenarios.find(item=>item.name.startsWith("a foreign file below that never closes"))!,got=await simulate(forever) as{en:string;de:string};
 for(const text of [got.en,got.de]){expect(text).toContain(fixture.source);expect(text).toContain(fixture.target);expect(text).toContain("0xc0000022");expect(text).toContain("36");expect(text).toContain("process 4242");expect(text).not.toMatch(/Internal error|Interner Fehler/u);}
 expect(got.en).toContain("access denied");expect(got.de).toContain("Zugriff verweigert");expect(got.de).toContain("Halter");
});

test("an error thrown by the refusal carries both texts and the boundary name", async()=>{
 const forever=fixture.scenarios.find(item=>item.name.startsWith("an own handle"))!;
 let caught:unknown;try{await settleArtifactWindowsRelativeRenameV1({source:fixture.source,target:fixture.target,ownHandles:()=>forever.own,attempt:()=>0,sleep:async()=>{},now:()=>0,checkpoint:()=>{},fail:()=>Error("x")});}catch(error){caught=error;}
 expect(caught).toBeInstanceOf(ArtifactWindowsRelativeRenameRefusedErrorV1);const error=caught as ArtifactWindowsRelativeRenameRefusedErrorV1;
 expect(error.name).toBe("WindowsRenameRefused");expect(error.message).toBe(error.texts.en);expect(error.texts.de).toContain("dieser Befehl selbst");
 expect(readFileSync(fileURLToPath(new URL("../../../../🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts",import.meta.url)),"utf8")).toContain('"WindowsRenameRefused"');
});

function tableOf(row:(typeof fixture)["tables"][number]){
 const nodes=new Map<string,object>(),links=new Map<object,{parent:object;name:string}>(),parent={},source={};nodes.set("parent",parent);nodes.set("source",source);links.set(source,{parent,name:"gen"});
 for(const item of row.handles){if(item.id==="parent")continue;const node={};nodes.set(item.id,node);}
 for(const item of row.handles){if(item.id==="parent")continue;links.set(nodes.get(item.id)!,{parent:nodes.get(item.parent!)!,name:item.name!});}
 return{open:[...nodes.values()],links,source};
}
test("the own-handle census counts aliases and everything below the entry, nothing else",()=>{
 for(const row of fixture.tables){const table=tableOf(row);expect(artifactWindowsRelativeHandlesBelowV1(table.open,item=>table.links.get(item),table.source),row.name).toBe(row.expect);}
 const expected=oracle().tables;expect(fixture.tables.map(row=>{const table=tableOf(row);return artifactWindowsRelativeHandlesBelowV1(table.open,item=>table.links.get(item),table.source);})).toEqual(expected);
});

test("the platform layer wires the settlement: no direct rename of the preservation, share-delete on every relative open, handle lineage recorded",()=>{
 const source=readFileSync(fileURLToPath(new URL("../../📁️descriptor/🟦️.ts",import.meta.url)),"utf8");
 expect(source).toContain("await settleArtifactWindowsRelativeRenameV1({");
 expect(source).not.toMatch(/NtSetInformationFile!\(source\.value,io,packet,packet\.length,c\.renameClass\)as number\);if\(status!==0\|\|io\.readBigUInt64LE\(8\)!==0n\)throw failure\("preservation rename"/u);
 expect(source).toContain("role===\"directory\"||role===\"ancestor\"?p.shareRead|p.shareWrite|p.shareDelete:p.shareRead|p.shareDelete");
 expect(source).toContain("isDirectory?p.shareRead|p.shareWrite|p.shareDelete:p.shareRead|p.shareDelete");
 expect(source).toContain("lineage.set(owned,{parent,name})");expect(source).toContain("lineage.set(result,{parent:owned,name})");
});
