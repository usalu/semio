/** 🪧️ Contract: a request that waits as its front door alone says who holds the door (pid, command, start, code, phase and owners done of total) and how long it has waited since its first start, refreshed at every progress notice, in English and German; the ticket carries the door it queued in. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {graphQueueYieldPolicyV1,waitGraphQueueYieldV1,yieldGraphQueueV1} from "../../🟨️.mjs";
import {rememberDoorV1} from "../../../🪧️door-board/🟨️.mjs";

const names=graphQueueYieldPolicyV1().environment,roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const door=()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"yield-door-")));roots.push(root);const directory=join(root,"s"),token="a".repeat(24);mkdirSync(directory);
 writeFileSync(join(directory,token+".json"),JSON.stringify({version:1,token,ticket:1,owners:[],structural:true,state:"held",queuedMs:1,holder:{pid:process.pid,command:"script.ts repo show projects",startedMs:Date.UTC(2026,10,17,16,0,0),digest:"c".repeat(64)}}));
 writeFileSync(join(directory,token+".progress.json"),JSON.stringify({phase:"owners",done:120,total:482,advancedMs:Date.now()}));
 return{root,directory};
};

test("the waiting front door names the holder with its phase and owners and the time since the first start, in both languages, and refreshes it at every notice",async()=>{
 const{root,directory}=door(),ticket=join(root,"t.json"),lines:string[]=[];
 writeFileSync(ticket,JSON.stringify({version:1,generation:7,pointer:"/p/current.json",requestedAt:1_000_000,round:1,at:1,door:directory}));
 let clock=1_000_000+240_000;const start=clock;
 await waitGraphQueueYieldV1({environment:{[names.ticket]:ticket,SEMIO_LANGUAGE:""},now:()=>clock,sleep:async milliseconds=>{clock+=milliseconds;},write:text=>lines.push(text),read:()=>clock-start>=70_000?8:7});
 expect(lines.length).toBeGreaterThanOrEqual(3);
 const first=lines[0]!;
 expect(first).toContain("a peer builds the graph");expect(first).toContain("ein Peer baut den Graphen");
 expect(first).toContain("holder: pid "+process.pid);expect(first).toContain("Halter: pid "+process.pid);expect(first).toContain("120 of 482");expect(first).toContain("120 von 482");expect(first).toContain("cccccccccccc");expect(first).toContain("script.ts repo show projects");
 expect(first).toContain("(270 s,");
 expect(lines[1]!).toContain("(300 s,");expect(lines.at(-1)!).toContain("generation 8");
});

test("a door that is unknown or empty adds nothing to the line; the ticket written by a yielding run names the door it queued in and the run tells who holds it",()=>{
 const{root,directory}=door(),ticket=join(root,"w.json"),lines:string[]=[];let code:number|undefined;
 yieldGraphQueueV1({generation:3,pointer:"/p",requestedAt:1700000000000,queuedMs:16000,environment:{[names.ticket]:ticket,SEMIO_LANGUAGE:""},write:text=>lines.push(text),exit:value=>{code=value;},door:directory});
 expect(code).toBe(75);expect(JSON.parse(readFileSync(ticket,"utf8")).door).toBe(directory);expect(lines.join("\n")).toContain("holder: pid "+process.pid);expect(lines.join("\n")).toContain("Halter: pid "+process.pid);
 const none:string[]=[];yieldGraphQueueV1({generation:3,pointer:"/p",requestedAt:1,environment:{[names.ticket]:join(root,"n.json"),SEMIO_LANGUAGE:""},write:text=>none.push(text),exit:()=>{},door:join(root,"absent")});
 expect(none.join("\n")).not.toContain("holder");expect(JSON.parse(readFileSync(join(root,"n.json"),"utf8")).door).toBe(join(root,"absent"));
 rememberDoorV1(directory);const remembered:string[]=[];yieldGraphQueueV1({generation:3,pointer:"/p",requestedAt:1,environment:{[names.ticket]:join(root,"r.json"),SEMIO_LANGUAGE:""},write:text=>remembered.push(text),exit:()=>{}});
 expect(JSON.parse(readFileSync(join(root,"r.json"),"utf8")).door).toBe(directory);expect(remembered.join("\n")).toContain("pid "+process.pid);
});
