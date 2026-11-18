/** 📣️ Contract: the poll for the owners the fleet has not delivered yet looks first at the owners the pools of the fleet say they stored (`pool-published-<wave>.txt` below each origin of the shared store, appended by the pool child), instead of rotating through thousands of names 48 at a time; an owner whose look missed waits `hotRetryMilliseconds` before it is looked at again; files older than the age bound and unreadable stores count for nothing.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{TAKE_POLICY_V1,fleetPublishedNamesV1,takePollV1}from"../../🟨️.mjs";
import childSource from"../../../🌐️owner-pool/🧒️child/🟨️.mjs"with{type:"text"};

const store=()=>mkdtempSync(join(tmpdir(),"r194-"));
const publish=(root:string,origin:string,wave:string,names:string[],ageMs=0)=>{const dir=join(root,"v1","origins",origin);mkdirSync(dir,{recursive:true});const file=join(dir,`pool-published-${wave}.txt`);writeFileSync(file,names.join("\n")+"\n");const at=new Date(Date.now()-ageMs);utimesSync(file,at,at);};

test("the published names of every origin are read, old files and other files are not",()=>{
 const root=store();publish(root,"g1-m01","abc123",["a","b"]);publish(root,"g1-m02","abc123",["b","c"]);publish(root,"g1-m03","def456",["old"],TAKE_POLICY_V1.publishedMaximumAgeMilliseconds+60000);
 writeFileSync(join(root,"v1","origins","g1-m01","pool.json"),"{\"x\":\"zz\"}");
 expect([...fleetPublishedNamesV1(root)].sort()).toEqual(["a","b","c"]);
 expect(fleetPublishedNamesV1(undefined).size).toBe(0);expect(fleetPublishedNamesV1(join(root,"missing")).size).toBe(0);
});

test("the poll looks at the listed owners first, again after the retry time only, and rotates when nothing is listed",async()=>{
 const names=Array.from({length:200},(_,i)=>"n"+i),taken=new Set<string>(),looked:string[]=[];let clock=0,listed=new Set(["n150","n170","n199"]);
 const stopAfter=()=>looked.length>=8;
 const result=await takePollV1({queue:()=>names.filter(name=>!taken.has(name)),attempt:async name=>{looked.push(name);if(name==="n170")return"miss";taken.add(name);return"fetched";},arrived:()=>{},stop:()=>stopAfter(),hot:()=>listed,now:()=>clock,sleep:async()=>{clock+=1000;},policy:{...TAKE_POLICY_V1,hotRetryMilliseconds:5000}});
 expect(looked.slice(0,3).sort()).toEqual(["n150","n170","n199"]);
 expect(looked.filter(name=>name==="n170").length).toBeLessThanOrEqual(2);
 expect(result.found).toBeGreaterThanOrEqual(2);
});

test("a pool child that stored an owner appends its name to the shared published file of its wave",()=>{
 expect(childSource).toContain('"pool-published-"');expect(childSource).toContain("appendFileSync(published, unit.name");
});
