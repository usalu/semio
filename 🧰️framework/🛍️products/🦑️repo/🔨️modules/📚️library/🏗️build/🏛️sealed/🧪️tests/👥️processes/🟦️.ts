/** 👥️ Contract with real processes: several processes that need the same drifted tree capture it exactly once; the pointer is valid and its snapshot readable at every instant while they promote; a broken edit never changes the pointer and is captured at most once for all of them; a promoter that is killed in the middle (while it holds the lock) neither blocks the next one nor leaves a partial pointer. */
import{afterAll,expect,test}from"bun:test";
import{spawn}from"node:child_process";
import{existsSync,readFileSync,rmSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{readSealedPointerV1,readSealedSnapshotV1,sealedEntryOfV1}from"../../🟦️.ts";
import{resolveSealedImplementationV1}from"../../📤️promotion/🟦️.ts";
import{fixtureCaptureV1,fixtureEntryV1,labelOfTextV1,makeFixtureWorkspaceV1,setFixtureTreeV1,testPolicyV1}from"../🧫️fixtures/🟦️.ts";

const child=fileURLToPath(new URL("./📜️child.ts",import.meta.url)),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const start=(root:string,counter:string,environment:Record<string,string>={})=>{
 const process_=spawn(process.execPath,[child,root,counter],{env:{...process.env,...environment},stdio:["ignore","pipe","pipe"]});let output="",errors="";
 process_.stdout.on("data",data=>output+=data);process_.stderr.on("data",data=>errors+=data);
 return{pid:process_.pid!,kill:()=>process_.kill("SIGKILL"),done:new Promise<{code:number|null;verdict?:{status:string;label:string;captures:number}}>(accept=>process_.on("close",code=>accept({code,verdict:output.trim()?JSON.parse(output.trim().split("\n").pop()!):undefined,...(errors?{errors}:{})})))};
};
const captures=(counter:string)=>existsSync(counter)?readFileSync(counter,"utf8").split("\n").filter(line=>/^\d+$/u.test(line)).length:0;
const prepare=async(label:string)=>{const root=makeFixtureWorkspaceV1(label);roots.push(root);const entry=sealedEntryOfV1(root,fixtureEntryV1,testPolicyV1);await resolveSealedImplementationV1(root,entry,{capture:fixtureCaptureV1(root),policy:testPolicyV1});return{root,entry,counter:join(root,"captures.log")};};

test("six processes that need the same drifted tree capture it once, and a reader sees a valid pointer at every instant",async()=>{
 const w=await prepare("A");setFixtureTreeV1(w.root,"B");
 let reads=0,torn=0,watching=true;
 const watcher=(async()=>{while(watching){const pointer=readSealedPointerV1(w.entry);if(pointer){reads++;if(readSealedSnapshotV1(w.entry,pointer)===undefined)torn++;}await new Promise(accept=>setTimeout(accept,1));}})();
 const runs=Array.from({length:6},()=>start(w.root,w.counter,{HOLD_MS:"150"}));
 const results=await Promise.all(runs.map(run=>run.done));
 watching=false;await watcher;
 expect(results.map(row=>row.code)).toEqual([0,0,0,0,0,0]);
 expect(captures(w.counter)).toBe(1);
 const statuses=results.map(row=>row.verdict!.status).sort();
 expect(statuses.filter(status=>status==="promoted")).toHaveLength(1);
 expect(statuses.every(status=>["promoted","served","fresh"].includes(status))).toBe(true);
 expect(results.every(row=>row.verdict!.label==="B")).toBe(true);
 expect(torn).toBe(0);expect(reads).toBeGreaterThan(10);
 expect(labelOfTextV1(readSealedSnapshotV1(w.entry,readSealedPointerV1(w.entry)!)!)).toBe("B");
},60_000);

test("a broken edit leaves the pointer alone and is captured and validated once for all processes",async()=>{
 const w=await prepare("A");const before=readSealedPointerV1(w.entry)!;setFixtureTreeV1(w.root,"X");
 const runs=Array.from({length:5},()=>start(w.root,w.counter,{LOADER:"1",HOLD_MS:"100"}));
 const results=await Promise.all(runs.map(run=>run.done));
 expect(results.map(row=>row.code)).toEqual([0,0,0,0,0]);
 expect(results.every(row=>row.verdict!.status==="blocked"&&row.verdict!.label==="A")).toBe(true);
 expect(captures(w.counter)).toBe(1);
 expect(readSealedPointerV1(w.entry)).toEqual(before);
},60_000);

test("a promoter killed while it holds the lock does not block the next one and leaves no partial pointer",async()=>{
 const w=await prepare("A");const before=readSealedPointerV1(w.entry)!;setFixtureTreeV1(w.root,"B");
 const holder=start(w.root,w.counter,{HOLD_MS:"60000"});
 for(let attempt=0;attempt<400&&!(existsSync(w.counter)&&readFileSync(w.counter,"utf8").includes("held"));attempt++)await new Promise(accept=>setTimeout(accept,25));
 expect(readFileSync(w.counter,"utf8")).toContain("held "+holder.pid);
 expect(readSealedPointerV1(w.entry)).toEqual(before);
 const waiter=start(w.root,w.counter);
 await new Promise(accept=>setTimeout(accept,200));
 holder.kill();
 const [killed,next]=await Promise.all([holder.done,waiter.done]);
 expect(killed.code).not.toBe(0);
 expect(next.code).toBe(0);expect(next.verdict).toMatchObject({status:"promoted",label:"B"});
 expect(captures(w.counter)).toBe(2);
 expect(labelOfTextV1(readSealedSnapshotV1(w.entry,readSealedPointerV1(w.entry)!)!)).toBe("B");
},60_000);
