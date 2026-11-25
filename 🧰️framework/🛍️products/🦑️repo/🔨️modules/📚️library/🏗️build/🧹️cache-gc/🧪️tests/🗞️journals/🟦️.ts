/** 🗞️ Law: the graph reclaim retires the dead build journals of the generation store (every key but the newest one that rests longer than the grace, and the newest one too once it rests beyond the grace without its header), in a dry run only reports them, removes the header last and never touches the newest or a fresh one. A Python oracle computes the same selection from the same groups. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {graphStaleBuildJournalsV1,readBuildJournalGroupsV1,reclaimGraphOptionalV1,staleBuildJournalGroupsV1,type BuildJournalGroupV1,type GraphReclaimPortsV1} from "../../🚰️reclaim/🟦️.ts";
import {CACHE_GC_LIMITS_V1} from "../../🧬️schema/🟦️.ts";

const GIB=1024**3,HOUR=3600000,ledger=cacheLedgerV1(),target=ledger.areas.find(area=>area.id==="graph-inference")!.reclaim!.targetAvailableBytes,key=(letter:string)=>letter.repeat(64);
const ORACLE="import json,sys;g=json.load(sys.stdin);n=max(g['groups'],key=lambda x:x['newestMs']);print(json.dumps([x['key'] for x in g['groups'] if (x is not n or x['key']+'.json' not in x['files']) and g['now']-x['newestMs']>g['grace']]))";
let root="",directory="";
const write=(letter:string,ageMs:number,rows:number,header:boolean)=>{
 const names=[...Array.from({length:rows},(_,at)=>`${key(letter)}.${at}.rows`),...header?[`${key(letter)}.json`]:[]],time=(Date.now()-ageMs)/1000;
 for(const name of names){writeFileSync(join(directory,name),"x".repeat(1000));utimesSync(join(directory,name),time,time);}
};
const ports=(log:string[]):GraphReclaimPortsV1=>({
 exclusive:async()=>({release:()=>{log.push("release");}}),open:async()=>({close:async()=>{log.push("close");}}),free:async()=>target+GIB,
 selected:async()=>{log.push("selected");},optional:async()=>{log.push("optional");},
 journals:async(_image,dryRun)=>graphStaleBuildJournalsV1(root,dryRun,new AbortController().signal)
});
beforeAll(()=>{root=mkdtempSync(join(tmpdir(),"graph-journals-"));directory=join(root,"generations","j");mkdirSync(directory,{recursive:true});});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("the planner keeps the newest journal and every fresh one and agrees with the Python oracle",()=>{
 const groups:BuildJournalGroupV1[]=[{key:"a",files:["a"],newestMs:1*HOUR,bytes:5},{key:"b",files:["b"],newestMs:5*HOUR,bytes:7},{key:"c",files:["c"],newestMs:8*HOUR,bytes:9},{key:"d",files:["d"],newestMs:9.5*HOUR,bytes:11}],now=10*HOUR,grace=CACHE_GC_LIMITS_V1.journalGraceMilliseconds;
 const mine=staleBuildJournalGroupsV1(groups,now,grace).map(group=>group.key);
 const oracle=spawnSync("python3",["-I","-c",ORACLE],{input:JSON.stringify({groups,now,grace}),encoding:"utf8"});
 expect(JSON.parse(oracle.stdout)).toEqual(mine);expect(mine).toEqual(["a","b","c"]);
 expect(staleBuildJournalGroupsV1([],now,grace)).toEqual([]);
 expect(staleBuildJournalGroupsV1([{...groups[0]!,files:["a.json","a.0.rows"]}],now,grace)).toEqual([]);
 const headless:BuildJournalGroupV1[]=[{key:"h",files:["h.json","h.0.rows"],newestMs:2*HOUR,bytes:3},{key:"k",files:["k.0.rows","k.1.rows"],newestMs:4*HOUR,bytes:13}];
 const oracleHeadless=spawnSync("python3",["-I","-c",ORACLE],{input:JSON.stringify({groups:headless,now,grace}),encoding:"utf8"});
 expect(staleBuildJournalGroupsV1(headless,now,grace).map(group=>group.key)).toEqual(["h","k"]);expect(JSON.parse(oracleHeadless.stdout)).toEqual(["h","k"]);
 expect(staleBuildJournalGroupsV1(headless,4.5*HOUR,grace).map(group=>group.key)).toEqual(["h"]);
});

test("a dry run reports the dead journals in both languages and removes nothing",async()=>{
 write("a",8*HOUR,3,true);write("b",6*HOUR,2,true);write("c",2*HOUR,4,false);write("d",600000,1,false);writeFileSync(join(directory,"unrelated.txt"),"x");
 const before=readdirSync(directory).length,log:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"en",ports(log))).toContain("Would remove 3 dead build journals of the generation store (");
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"de",ports(log))).toContain("Würde 3 tote Bau-Journale des Generationsspeichers entfernen (");
 expect(readdirSync(directory).length).toBe(before);expect(readBuildJournalGroupsV1(root).map(group=>group.key[0])).toEqual(["a","b","c","d"]);
});

test("a run removes every dead journal with its header, keeps the newest and the fresh one and finds nothing the second time",async()=>{
 const log:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",ports(log))).toContain("Removed 3 dead build journals of the generation store (");
 expect(readBuildJournalGroupsV1(root).map(group=>group.key[0])).toEqual(["d"]);expect(existsSync(join(directory,"unrelated.txt"))).toBe(true);
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"de",ports(log))).not.toContain("Bau-Journale");
 expect((await graphStaleBuildJournalsV1(root,false,new AbortController().signal)).kept).toBe(1);
});

test("a cancelled run stops before it removes the next journal and an absent store is empty",async()=>{
 write("e",20*HOUR,1,true);write("f",30*HOUR,1,true);
 const abort=new AbortController();abort.abort();
 expect(await graphStaleBuildJournalsV1(root,false,abort.signal)).toMatchObject({candidates:2,removed:0,cancelled:true});
 expect(await graphStaleBuildJournalsV1(join(root,"missing"),false,new AbortController().signal)).toEqual({candidates:0,removed:0,bytes:0,kept:0,cancelled:false});
});
