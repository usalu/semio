import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{READY_TREE_POLICY_V1 as P,readyBackgroundReaderV1,readyForegroundBusyV1,readyHolderCustodyV1,readyHolderWaitOfV1,readyHolderWaitTrackerV1,readyScaledTreePolicyV1,readyStallTrackerV1}from"../../🟦️.ts";
import{observeDependencyStoreHoldersV1}from"../../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/🟦️.ts";

/** 🤝️ The custody of a wait on a named holder of the dependency store: who the line names, whether the holder lives and renews its entry, when the waiter is stalled, when a background read of the machine's own graph server gives way, and when a dead holder is reclaimed. Cases are authored, an independent Python oracle (own regular expressions, a fixpoint over the table) answers the same cases. */
type Case={name:string};
const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url));
const fixture=JSON.parse(readFileSync(here("./🔣️.json"),"utf8")) as {parse:(Case&{lines:string[];expected:unknown})[];custody:(Case&{wait:any;entry:any;table:any[];nowMs:number;expected:string})[];reader:(Case&{record:any;holder:number;self:number;table:any[];expected:any})[];foreground:(Case&{status:unknown;nowMs:number;expected:boolean})[]};
const planOf=(row:ReturnType<typeof readyBackgroundReaderV1>)=>row===undefined?null:{root:row.root.pid,plan:[...row.plan].sort((a,b)=>a-b),reason:row.reason};
const ours={
 parse:fixture.parse.map(row=>readyHolderWaitOfV1(row.lines)??null),
 custody:fixture.custody.map(row=>readyHolderCustodyV1({wait:row.wait,entry:row.entry??undefined,table:row.table,nowMs:row.nowMs}).kind),
 reader:fixture.reader.map(row=>planOf(readyBackgroundReaderV1({record:row.record??undefined,holder:row.holder,table:row.table,self:row.self}))),
 foreground:fixture.foreground.map(row=>readyForegroundBusyV1(row.status,row.nowMs)),
};

test("every authored case has the authored answer",()=>{
 fixture.parse.forEach((row,index)=>expect(ours.parse[index],row.name).toEqual(row.expected as never));
 fixture.custody.forEach((row,index)=>expect(ours.custody[index],row.name).toBe(row.expected));
 fixture.reader.forEach((row,index)=>expect(ours.reader[index],row.name).toEqual(row.expected));
 fixture.foreground.forEach((row,index)=>expect(ours.foreground[index],row.name).toBe(row.expected));
});

test("an independent Python implementation gives the same answers on the same cases",()=>{
 const run=spawnSync("python3",["-I",here("./🐍️.py"),here("../../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(JSON.parse(JSON.stringify(ours)));
});

test("a wait behind a live holder that renews its entry is no stall at any length, a silent one and a spent bound are named, a dead one is reclaimed by the waiter",()=>{
 const wait=readyHolderWaitOfV1([fixture.parse[0]!.lines[1]!])!;
 const table=[{pid:30880,ppid:24364,startMs:1794975120000,name:"bun.exe"}],entry=(updated:number)=>({pid:30880,startedAtMs:1794975120000,updatedAtMs:updated,command:"x"});
 const verdict=(seconds:number,updated:number,rows=table)=>readyHolderCustodyV1({wait:{...wait,seconds},entry:entry(updated),table:rows,nowMs:1794975700000}).kind;
 for(const seconds of[10,646,3600,21599])expect(verdict(seconds,1794975690000),`${seconds} s`).toBe("progressing");
 expect(verdict(646,1794975700000-P.holderSilentMilliseconds-1)).toBe("silent");
 expect(verdict(21600,1794975690000)).toBe("bound");
 expect(verdict(646,1794975690000,[{pid:4,ppid:0,startMs:1,name:"System"}])).toBe("gone");
});

test("the tracker still calls the same wait line no progress: only the custody judgement keeps a live holder's waiter running",()=>{
 const tracker=readyStallTrackerV1(),line=(seconds:number)=>`Process 30880 (741d x show projects) holds the dependency store since 2026-11-18T03:28:01.557Z; waiting for it (${seconds} s of at most 21600 s)`;
 expect(tracker.observe([line(346)])).toBe(true);
 for(const seconds of[356,366,376])expect(tracker.observe([line(seconds)])).toBe(false);
 const waiting=readyHolderWaitTrackerV1();expect(waiting.observe([line(346)],0)).toBe(0);expect(waiting.observe([line(356)],70000)).toBe(70000);
});

test("the German wait line is a holder wait for the tracker too, and the new numbers scale with the speed of the machine",()=>{
 const german="Der Prozess 4120 (x show projects) hält den Abhängigkeitsspeicher seit 2026-11-18T03:28:01.557Z; es wird darauf gewartet (77 s von höchstens 21600 s)",tracker=readyHolderWaitTrackerV1();
 tracker.observe([german],0);expect(tracker.observe([german],5000)).toBe(5000);
 const slow=readyScaledTreePolicyV1(3);expect(slow.holderSilentMilliseconds).toBe(3*P.holderSilentMilliseconds);expect(slow.holderYieldMilliseconds).toBe(3*P.holderYieldMilliseconds);
});

test("Windows-shaped liveness: a dead holder is reclaimed without a person (entry removed, named), a live one that renews stays, a reused pid is another process, an unknown start falls back on the heartbeat",()=>{
 const registry=[
  {path:"dead",entry:{version:1 as const,pid:31,startedAtMs:1000,updatedAtMs:1_000_000,command:"x show projects",directory:"C:\\semio\\.cache\\b\\l\\m\\data",holds:[{resource:"native-fixed-image:C:/x",sinceMs:1000,mode:"shared" as const,task:1}],waits:[]}},
  {path:"alive",entry:{version:1 as const,pid:32,startedAtMs:2000,updatedAtMs:1_000_000-10_000,command:"y show projects",directory:"C:\\semio\\.cache\\b\\l\\m\\data",holds:[{resource:"native-fixed-image:C:/x",sinceMs:2000,mode:"shared" as const,task:1}],waits:[]}},
  {path:"reused",entry:{version:1 as const,pid:33,startedAtMs:3000,updatedAtMs:1_000_000-10_000,command:"z show projects",directory:"C:\\semio\\.cache\\b\\l\\m\\data",holds:[{resource:"native-fixed-image:C:/x",sinceMs:3000,mode:"shared" as const,task:1}],waits:[]}},
  {path:"unknown-silent",entry:{version:1 as const,pid:34,startedAtMs:4000,updatedAtMs:1_000_000-400_000,command:"w show projects",directory:"C:\\semio\\.cache\\b\\l\\m\\data",holds:[{resource:"native-fixed-image:C:/x",sinceMs:4000,mode:"shared" as const,task:1}],waits:[]}},
 ],removed:string[]=[];
 const found=observeDependencyStoreHoldersV1("C:\\semio\\.cache\\b\\l\\m\\data","native-fixed-image:C:/x",{entries:()=>registry as never,alive:pid=>pid!==31,osStartedAtMs:pid=>pid===33?3000+600_000:pid===34?undefined:2000,now:()=>1_000_000,ancestors:()=>[],remove:path=>{removed.push(path);},pid:1,startedAtMs:1});
 expect(found.live.map(row=>row.pid)).toEqual([32]);
 expect(found.gone.map(row=>row.pid).sort()).toEqual([31,33,34]);
 expect(removed.sort()).toEqual(["dead","reused","unknown-silent"]);
});
