import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {superviseGraphChildV1} from "../../👷️agent/🟦️.ts";

/** 🔇️ Contract: the agent that supervises the readiness child ends it when its ready.json is silent (a wedged child is alive and never exits by itself), leaves a bilingual note in the store and the child starts again; a fresh record leaves it alone. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const world=(updatedAgoMs:number)=>{
 const root=mkdtempSync(join(tmpdir(),"ready-silence-"));scratch.push(root);
 const directory=join(root,"store","v1","origins","g1-m05");mkdirSync(directory,{recursive:true});
 writeFileSync(join(directory,"ready.json"),JSON.stringify({ready:false,state:"running",updatedMs:Date.now()-updatedAgoMs,beatMilliseconds:30000}));
 const starts=join(root,"starts"),entry=join(root,"entry.ts");
 writeFileSync(entry,`import {appendFileSync} from "node:fs";appendFileSync(${JSON.stringify(starts)},"x");setInterval(()=>{},1000);`);
 return {root,directory,starts,entry,environment:{SEMIO_FLEET_STORE_ROOT:join(root,"store"),SEMIO_FLEET_ORIGIN:"g1-m05"}};
};
const count=(file:string):number=>existsSync(file)?readFileSync(file,"utf8").length:0;

test("a silent record ends the readiness child once, leaves a bilingual note and the child starts again",async()=>{
 const w=world(600_000),controller=new AbortController(),lines:string[]=[];
 const supervisor=superviseGraphChildV1({root:w.root,environment:w.environment,signal:controller.signal,write:(line:string)=>lines.push(line)} as never,w.entry,"ready",{graceMilliseconds:300,silencePollMilliseconds:100,silenceGraceMilliseconds:0});
 for(let wait=0;wait<100&&count(w.starts)<2;wait++)await Bun.sleep(100);
 controller.abort();await supervisor.stop();
 expect(count(w.starts)).toBeGreaterThanOrEqual(2);
 const note=JSON.parse(readFileSync(join(w.directory,"ready-wedge.json"),"utf8"));
 expect(note.en).toContain("wrote nothing");expect(note.de).toContain("schrieb seit");
 expect(lines.some(line=>line.includes("wrote nothing")&&line.includes("schrieb seit"))).toBe(true);
});

test("a fresh record never ends the child",async()=>{
 const w=world(0),controller=new AbortController();
 const refresh=setInterval(()=>{writeFileSync(join(w.directory,"ready.json"),JSON.stringify({ready:false,state:"running",updatedMs:Date.now(),beatMilliseconds:30000}));},50);
 const supervisor=superviseGraphChildV1({root:w.root,environment:w.environment,signal:controller.signal,write:()=>{}} as never,w.entry,"ready",{graceMilliseconds:300,silencePollMilliseconds:100,silenceGraceMilliseconds:0});
 await Bun.sleep(1500);
 clearInterval(refresh);controller.abort();await supervisor.stop();
 expect(count(w.starts)).toBe(1);expect(existsSync(join(w.directory,"ready-wedge.json"))).toBe(false);
});
