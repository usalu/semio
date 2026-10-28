/** 🧹️ Contract: a journal state directory of a worktree that no longer runs is removed, one a service holds or that changed recently is not. The decision equals an independent Python model on seeded facts, the real sweep over a cache container removes exactly the directories the model names (and nothing outside the `<container>/<id>/journal` layout), and a starting service sweeps its siblings and reports it in English and German.
 * @see ../../🧹️abandoned/🟦️.ts */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {CHANGE_JOURNAL_FILES_V1,CHANGE_JOURNAL_POLICY_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {runChangeJournalServiceV1,type ChangeWatchFactoryV1} from "../../🟦️.ts";
import {JOURNAL_ABANDONED_POLICY_V1,formatJournalSweepV1,journalStateAbandonedV1,sweepAbandonedJournalStatesV1} from "../../🧹️abandoned/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️.py"),python=process.platform==="win32"?"python":"python3",policy=JOURNAL_ABANDONED_POLICY_V1,DAY=86_400_000;
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"journal-abandoned-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
const model=(request:unknown):boolean[]=>{const run=spawnSync(python,["-I",oracle],{input:JSON.stringify(request),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout).abandoned as boolean[];};
const generator=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};

test("the abandoned decision equals the independent Python model on 500 seeded facts and the boundary of both ages",()=>{
 const next=generator(7),now=1_800_000_000_000,cases=Array.from({length:500},()=>({held:next()<0.3,rootExists:[true,false,null][Math.floor(next()*3)] as boolean|null,newestMs:now-Math.floor(next()*9*DAY)}));
 for(const edge of [policy.abandonedAfterMilliseconds-1,policy.abandonedAfterMilliseconds,policy.rootGoneQuietMilliseconds-1,policy.rootGoneQuietMilliseconds])for(const rootExists of [true,false,null])cases.push({held:false,rootExists,newestMs:now-edge});
 const expected=model({policy,now,cases});
 cases.forEach((row,index)=>expect(journalStateAbandonedV1({held:row.held,rootExists:row.rootExists??undefined,newestMs:row.newestMs},now,policy),JSON.stringify(row)).toBe(expected[index]!));
});

type Kind={name:string;lock:"none"|"dead"|"young"|"old";rootExists:boolean;agesMs:number[]};
const KINDS:Kind[]=[
 {name:"aaaaaaaaaa01",lock:"none",rootExists:true,agesMs:[8*DAY,9*DAY]},
 {name:"aaaaaaaaaa02",lock:"none",rootExists:true,agesMs:[1*DAY,9*DAY]},
 {name:"aaaaaaaaaa03",lock:"dead",rootExists:false,agesMs:[20*60_000,9*DAY]},
 {name:"aaaaaaaaaa04",lock:"none",rootExists:false,agesMs:[60_000,9*DAY]},
 {name:"aaaaaaaaaa05",lock:"young",rootExists:true,agesMs:[9*DAY]},
 {name:"aaaaaaaaaa06",lock:"old",rootExists:true,agesMs:[9*DAY]},
 {name:"aaaaaaaaaa07",lock:"young",rootExists:false,agesMs:[3*DAY]},
];
const build=(container:string,kind:Kind,now:number)=>{
 const journal=join(container,kind.name,"journal"),memo=join(journal,"graph-memo");mkdirSync(memo,{recursive:true});
 const root=join(sandbox,"root-"+kind.name);if(kind.rootExists)mkdirSync(root,{recursive:true});
 writeFileSync(join(journal,CHANGE_JOURNAL_FILES_V1.state),JSON.stringify({pid:2,root}));writeFileSync(join(memo,"x.plan"),"y".repeat(2048));
 if(kind.lock!=="none"){
  mkdirSync(join(journal,CHANGE_JOURNAL_FILES_V1.lock));
  const at=kind.lock==="young"?now:now-9*DAY,pid=kind.lock==="dead"?2_000_000_000:process.pid;
  writeFileSync(join(journal,CHANGE_JOURNAL_FILES_V1.lock,"owner.json"),JSON.stringify({pid,at}));
 }
 const files=[join(journal,CHANGE_JOURNAL_FILES_V1.state),join(memo,"x.plan"),memo,journal,join(container,kind.name)];
 files.forEach((path,index)=>{const age=kind.agesMs[index%kind.agesMs.length]!,at=new Date(now-age);utimesSync(path,at,at);});
 if(kind.lock!=="none"){const lock=join(journal,CHANGE_JOURNAL_FILES_V1.lock),at=new Date(now-(kind.lock==="young"?0:9*DAY));utimesSync(join(lock,"owner.json"),at,at);utimesSync(lock,at,at);}
};

test("the real sweep removes exactly the state directories the model names and leaves every other entry of the container alone",()=>{
 const container=join(sandbox,"c"+counter++,"repo"),now=Date.now();mkdirSync(container,{recursive:true});
 const own=join(container,"ffffffffff00","journal");mkdirSync(own,{recursive:true});
 KINDS.forEach(kind=>build(container,kind,now));
 mkdirSync(join(container,"not-an-identifier","journal"),{recursive:true});mkdirSync(join(container,"bbbbbbbbbb01"),{recursive:true});writeFileSync(join(container,"note.txt"),"keep");
 for(const path of [join(container,"not-an-identifier","journal"),join(container,"bbbbbbbbbb01")]){const old=new Date(now-30*DAY);utimesSync(path,old,old);}
 const expected=model({policy,now,cases:KINDS.map(kind=>({lock:kind.lock,rootExists:kind.rootExists,agesMs:kind.agesMs}))});
 const sweep=sweepAbandonedJournalStatesV1(own,now,policy);
 expect(sweep.removed).toEqual(KINDS.filter((_kind,index)=>expected[index]).map(kind=>kind.name));
 for(const [index,kind] of KINDS.entries())expect(existsSync(join(container,kind.name)),kind.name).toBe(!expected[index]);
 for(const kept of ["ffffffffff00","not-an-identifier","bbbbbbbbbb01","note.txt"])expect(existsSync(join(container,kept)),kept).toBe(true);
 expect(sweep.bytes).toBeGreaterThan(0);
});

test("a directory outside the cache layout is never swept: the siblings of a custom journal directory stay",()=>{
 const base=join(sandbox,"c"+counter++),now=Date.now();
 const own=join(base,"j");mkdirSync(own,{recursive:true});
 build(base,KINDS[0]!,now);
 expect(sweepAbandonedJournalStatesV1(own,now,policy).removed).toEqual([]);
 expect(sweepAbandonedJournalStatesV1(join(base,"aaaaaaaaaa01","other"),now,policy).removed).toEqual([]);
 expect(existsSync(join(base,KINDS[0]!.name))).toBe(true);
});

test("a starting service removes its abandoned siblings once and says so in English and German",async()=>{
 const container=join(sandbox,"c"+counter++,"repo"),now=Date.now(),own=join(container,"ffffffffff01","journal");mkdirSync(container,{recursive:true});
 build(container,KINDS[0]!,now);build(container,KINDS[1]!,now);
 const factory:ChangeWatchFactoryV1=()=>({async start(){return{backend:"fsevents",trust:"exact"};},async stop(){}});
 const lines:string[]=[],root=join(sandbox,"service-root");mkdirSync(root,{recursive:true});
 const service=await runChangeJournalServiceV1({root,directory:own,fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},createSupervisor:factory,residentBytes:()=>0,log:(kind,detail)=>{lines.push(kind+" "+detail);}});
 await service.stop("test");
 expect(existsSync(join(container,KINDS[0]!.name))).toBe(false);expect(existsSync(join(container,KINDS[1]!.name))).toBe(true);
 const swept=lines.find(line=>line.startsWith("sweep "));
 expect(swept).toBeDefined();
 expect(swept).toContain("removed 1 abandoned change journal state director(ies)");
 expect(swept).toContain("1 verwaiste Änderungsjournal-Zustandsverzeichnis(se)");
});

test("the report line is bilingual and states the count and the freed size",()=>{
 const sweep={removed:["a","b"],bytes:3*1048576+524288};
 expect(formatJournalSweepV1(sweep,"en")).toBe("[repo journal] removed 2 abandoned change journal state director(ies) (3.5 MiB) of worktrees that no longer run");
 expect(formatJournalSweepV1(sweep,"de")).toBe("[repo journal] 2 verwaiste Änderungsjournal-Zustandsverzeichnis(se) (3.5 MiB) von nicht mehr laufenden Arbeitsverzeichnissen entfernt");
 expect(CHANGE_JOURNAL_POLICY_V1.heartbeatStaleMilliseconds).toBeLessThan(policy.rootGoneQuietMilliseconds);
});
