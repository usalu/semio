import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {REPO_PROFILE_LEGEND_V1,REPO_PROFILE_PHASE_PATTERN_V1,createRepoProfileV1,readRepoProfileModeV1,renderRepoProfileTableV1,repoProfileSpawnLabelV1,summarizeRepoProfileV1} from "../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const ajv=new Ajv({strict:false}),validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Profile"});
const clock=()=>{let time=0;return {now:()=>time,advance:(milliseconds:number)=>{time+=milliseconds;}};};
const grid=(records:readonly {id:number;parent:number;start:number;end:number}[],record:{id:number;start:number;end:number})=>{let covered=0;for(let t=record.start*2;t<record.end*2;t++)if(records.some(child=>child.parent===record.id&&t>=child.start*2&&t<child.end*2))covered++;return (record.end-record.start)-covered/2;};

test("Self time and coverage follow the fixture rows and an independent half millisecond grid",()=>{
 for(const item of fixture.summaries){
  const summary=summarizeRepoProfileV1(item.records,item.total);
  expect(summary.covered,item.name).toBe(item.expect.covered);expect(summary.uncovered,item.name).toBe(item.expect.uncovered);
  expect(summary.rows.map(row=>[row.phase,row.depth,row.start,row.duration,row.self]),item.name).toEqual(item.expect.rows);
  for(const row of summary.rows)expect(row.self,item.name+" grid "+row.phase).toBe(grid(item.records,item.records.find(record=>record.id===row.id)!));
  expect(validate({pid:1,...summary}),JSON.stringify(validate.errors)).toBe(true);
 }
});

test("A disabled profile returns the body result untouched and records nothing",async()=>{
 const profile=createRepoProfileV1({mode:"off"});
 expect(profile.phase("a.b",()=>7)).toBe(7);expect(await profile.phase("a.b",async()=>8)).toBe(8);
 profile.event("x",1);profile.mark("y");expect(profile.summary().rows).toEqual([]);
});

test("Synchronous, asynchronous and concurrent phases nest by asynchronous context",async()=>{
 const time=clock(),profile=createRepoProfileV1({mode:"json",now:time.now,write:()=>undefined});
 await profile.phase("outer",async()=>{
  time.advance(5);
  await Promise.all([profile.phase("inner.a",async()=>{time.advance(10);await Promise.resolve();}),profile.phase("inner.b",async()=>{await Promise.resolve();})]);
  time.advance(5);
  profile.phase("inner.c",()=>{time.advance(2);});
 });
 const rows=profile.summary().rows;
 expect(rows.map(row=>[row.phase,row.parent===0?"root":rows.find(parent=>parent.id===row.parent)!.phase,row.depth])).toEqual([["outer","root",0],["inner.a","outer",1],["inner.b","outer",1],["inner.c","outer",1]]);
 expect(rows[0]!.duration).toBe(22);expect(rows[0]!.self).toBe(10);
});

test("A failing body still closes its phase and rethrows",async()=>{
 const time=clock(),profile=createRepoProfileV1({mode:"json",now:time.now,write:()=>undefined});
 expect(()=>profile.phase("sync.fail",()=>{time.advance(3);throw new Error("boom");})).toThrow("boom");
 await expect(profile.phase("async.fail",async()=>{time.advance(4);throw new Error("late");})).rejects.toThrow("late");
 expect(profile.summary().rows.map(row=>[row.phase,row.duration])).toEqual([["sync.fail",3],["async.fail",4]]);
});

test("Phase names are neutral identifiers and the environment value is checked in both languages",()=>{
 const profile=createRepoProfileV1({mode:"json"});
 for(const name of ["Leases Open","öffnen","","a b","1a"])expect(()=>profile.phase(name,()=>0),name).toThrow("Erwartet");
 for(const [value,mode] of [[undefined,"off"],["","off"],["0","off"],["off","off"],["1","table"],["TRUE","table"],["tsv","table"],["json","json"]] as const)expect(readRepoProfileModeV1({SEMIO_REPO_PROFILE:value}),String(value)).toBe(mode);
 expect(()=>readRepoProfileModeV1({SEMIO_REPO_PROFILE:"verbose"})).toThrow(/Expected SEMIO_REPO_PROFILE=1\|json\|off.*Erwartet/);
});

test("The table output has one tab separated row per phase, a comment header and a total row",()=>{
 const time=clock(),profile=createRepoProfileV1({mode:"table",now:time.now,write:()=>undefined});
 profile.phase("a",()=>{time.advance(4);profile.mark("a.mark","detail");});time.advance(1);
 const lines=renderRepoProfileTableV1(profile.summary(),42).trimEnd().split("\n");
 expect(lines[0]).toBe("#repo-profile\tversion=1\tpid=42\tunit=ms");expect(lines[1]).toBe("#seq\tdepth\tphase\tstart\tduration\tself\tdetail");
 expect(lines.slice(2,-1).map(line=>line.split("\t").length)).toEqual([7,7]);expect(lines.at(-1)).toBe("#total\t5.0\tcovered\t4.0\tuncovered\t1.0");
 for(const key of ["seq","depth","phase","start","duration","self","detail"] as const){expect(REPO_PROFILE_LEGEND_V1[key].en.length).toBeGreaterThan(0);expect(REPO_PROFILE_LEGEND_V1[key].de).not.toBe(REPO_PROFILE_LEGEND_V1[key].en);expect(schema["x-legend"][key]).toEqual(REPO_PROFILE_LEGEND_V1[key]);}
});

test("A real process prints a schema valid JSON profile on exit and counts the child processes it starts",()=>{
 const module=new URL("../🟨️.mjs",import.meta.url).href,runtime=new URL("../../../../../../../🔨️modules/⚙️runtime/🟦️.ts",import.meta.url).href,program=`const m=await import(${JSON.stringify(module)}),r=await import(${JSON.stringify(runtime)});await m.profilePhaseV1("test.outer",async()=>{r.runtimeSpawnSyncV1([process.execPath,"-e","0"]);await r.runtimeSleepV1(5);});`;
 const run=spawnSync(process.execPath,["-e",program],{encoding:"utf8",env:{...process.env,SEMIO_REPO_PROFILE:"json"}});
 expect(run.status,run.stderr).toBe(0);
 const profile=JSON.parse(run.stderr.trim().split("\n").at(-1)!);
 expect(validate(profile),JSON.stringify(validate.errors)).toBe(true);
 expect(profile.rows.map((row:{phase:string})=>row.phase)).toEqual(["test.outer",expect.stringMatching(/^spawn\.bun/)]);
 expect(profile.rows[1].parent).toBe(profile.rows[0].id);expect(profile.rows[0].duration).toBeGreaterThanOrEqual(5);
 const off=spawnSync(process.execPath,["-e",program],{encoding:"utf8",env:{...process.env,SEMIO_REPO_PROFILE:"0"}});
 expect(off.status,off.stderr).toBe(0);expect(off.stderr).toBe("");
});

test("Child process events are named after the program in a neutral spelling that is a valid phase name",()=>{
 for(const [command,label] of [["/usr/bin/vm_stat","spawn.vm-stat"],["sysctl","spawn.sysctl"],[["/usr/bin/hdiutil","info"],"spawn.hdiutil"],["C:\\\\tools\\\\Python3.11.exe","spawn.python3-11-exe"],["","spawn.unknown"],[[],"spawn.unknown"],["/opt/7z","spawn.x7z"]] as const){
  expect(repoProfileSpawnLabelV1(command),String(command)).toBe(label);expect(REPO_PROFILE_PHASE_PATTERN_V1.test(label),label).toBe(true);
 }
});

test("the profile names the phase started last also while it is off", async () => {
  const { profileCurrentPhaseV1, profilePhaseV1, profileMarkV1 } = await import("../🟨️.mjs");
  profilePhaseV1("lane.progress-first", () => 1);
  expect(profileCurrentPhaseV1()).toBe("lane.progress-first");
  await profilePhaseV1("lane.progress-second", async () => { await Bun.sleep(1); });
  expect(profileCurrentPhaseV1()).toBe("lane.progress-second");
  profileMarkV1("lane.progress-mark");
  expect(profileCurrentPhaseV1()).toBe("lane.progress-mark");
});
