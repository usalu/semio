/** 🙋️ Contract: an ask is one claim of a read on the one budget of its machine, shaped by the schema, alive while its process lives and its heartbeat is kept, withdrawn on clear, and summed into one claim for the pool. Oracle: Ajv, an independent JSON-schema validator. */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync,mkdirSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{HOST_ASK_POLICY_V1 as P,createHostAskV1,hostAskClaimV1,hostAskDirectoryV1,hostAskLiveV1,readHostAsksV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=()=>{const path=mkdtempSync(join(tmpdir(),"host-asks-"));scratch.push(path);return path;};
const alive=()=>true;

test("the ask schema agrees with Ajv on a good ask and on every broken field",()=>{
 const ajv=new Ajv({strict:true}),validate=ajv.compile({...schema.definitions.Ask,definitions:undefined});
 const good={version:1,pid:7,key:"owner-a",needMegabytes:4096,alone:true,sinceMs:1,updatedMs:2,heartbeatMilliseconds:1000};
 expect(validate(good)).toBe(true);
 const dir=temp(),writer=createHostAskV1({directory:dir,pid:7,now:()=>2});writer.set({key:"owner-a",needMegabytes:4096,alone:true});
 expect(validate(JSON.parse(readFileSync(join(hostAskDirectoryV1(dir),"7.json"),"utf8")))).toBe(true);
 for(const broken of[{...good,version:2},{...good,pid:0},{...good,key:""},{...good,needMegabytes:-1},{...good,alone:"yes"},{...good,extra:1},{...good,heartbeatMilliseconds:0}])expect(validate(broken)).toBe(false);
});

test("a writer publishes, keeps the heartbeat only when due, keeps since while the claim stays, and withdraws",()=>{
 const dir=temp();let now=1000;const writer=createHostAskV1({directory:dir,pid:11,now:()=>now}),file=join(hostAskDirectoryV1(dir),"11.json");
 writer.set({key:"a",needMegabytes:3000,alone:false});const first=JSON.parse(readFileSync(file,"utf8"));
 now+=P.heartbeatMilliseconds-1;writer.set({key:"a",needMegabytes:3000,alone:false});expect(JSON.parse(readFileSync(file,"utf8")).updatedMs,"not rewritten before the beat").toBe(first.updatedMs);
 now+=1;writer.set({key:"a",needMegabytes:3000,alone:false});const beat=JSON.parse(readFileSync(file,"utf8"));expect(beat.updatedMs).toBe(now);expect(beat.sinceMs).toBe(first.sinceMs);
 writer.set({key:"b",needMegabytes:500,alone:true});expect(JSON.parse(readFileSync(file,"utf8"))).toMatchObject({key:"b",alone:true,sinceMs:now});
 writer.clear();expect(existsSync(file)).toBe(false);
});

test("a stale, dead, misshapen or oversized ask counts for nothing, and live asks sum into one claim",()=>{
 const dir=temp(),now=5_000_000,asks=hostAskDirectoryV1(dir);mkdirSync(asks,{recursive:true});
 const row=(pid:number,over:object={})=>({version:1,pid,key:"k"+pid,needMegabytes:1000,alone:false,sinceMs:now-10,updatedMs:now-10,heartbeatMilliseconds:1000,...over});
 writeFileSync(join(asks,"1.json"),JSON.stringify(row(1)));
 writeFileSync(join(asks,"2.json"),JSON.stringify(row(2,{needMegabytes:2500,alone:true})));
 writeFileSync(join(asks,"3.json"),JSON.stringify(row(3,{updatedMs:now-1000*P.missedBeats-1})));
 writeFileSync(join(asks,"4.json"),"{not json");writeFileSync(join(asks,"5.json"),JSON.stringify({...row(5),needMegabytes:"many"}));
 writeFileSync(join(asks,"6.json"),JSON.stringify(row(6,{key:"x".repeat(P.maximumFileBytes)})));writeFileSync(join(asks,"note.txt"),"x");
 const live=readHostAsksV1(dir,now,pid=>pid!==2||true);expect(live.map(ask=>ask.pid).sort()).toEqual([1,2]);
 expect(hostAskClaimV1(live)).toEqual({megabytes:3500,alone:true});
 expect(readHostAsksV1(dir,now,pid=>pid===1).map(ask=>ask.pid),"a dead process holds nothing").toEqual([1]);
 expect(hostAskClaimV1([])).toBeUndefined();
 expect(hostAskLiveV1(row(1) as never,now+1000*P.missedBeats-10,alive)).toBe(true);expect(hostAskLiveV1(row(1) as never,now+1000*P.missedBeats-9,alive)).toBe(false);
 expect(readHostAsksV1(join(dir,"missing"),now,alive)).toEqual([]);
});

test("a writer into an unusable directory never throws",()=>{
 const dir=temp();writeFileSync(join(dir,"asks"),"a file where the directory should be");
 const writer=createHostAskV1({directory:dir,pid:3});expect(()=>{writer.set({key:"a",needMegabytes:1,alone:false});writer.clear();}).not.toThrow();
});
