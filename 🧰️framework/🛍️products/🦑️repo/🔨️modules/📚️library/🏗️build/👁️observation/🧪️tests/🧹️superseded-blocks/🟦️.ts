import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{brotliCompressSync}from"node:zlib";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const api=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const hex=(character:string)=>character.repeat(64);

/** 🧪️ One record directory: closed headers naming the scenario's blocks and a block directory whose leaves have the scenario's ages. */
function directory(scenario:typeof fixture.scenarios[number]){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"superseded-"));roots.push(root);
 for(const[name,blocks]of Object.entries(scenario.headers)){
  const header=JSON.stringify({digest:hex("a"),resultSha256:hex("b"),authorityObservations:[],blocks,observations:[]});
  writeFileSync(join(root,name+".json"),brotliCompressSync(Buffer.from(header+"\n{}")));
 }
 mkdirSync(join(root,"ob"));
 for(const block of scenario.blocks){const path=join(root,"ob",block.digest+".br");writeFileSync(path,"x");const when=(fixture.nowMilliseconds-block.ageMilliseconds)/1000;utimesSync(path,when,when);}
 for(const name of(scenario as {foreign?:string[]}).foreign??[])writeFileSync(join(root,"ob",name),"x");
 return root;
}

test("the fixture is closed under its schema",()=>{
 expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
});

test("a sweep removes exactly the superseded blocks older than the grace period and nothing else",async()=>{
 for(const scenario of fixture.scenarios){
  const root=directory(scenario),before=readdirSync(join(root,"ob")).sort();
  const report=await api.sweepSupersededObservationBlocksV1(root,{graceMilliseconds:fixture.graceMilliseconds,now:fixture.nowMilliseconds});
  const after=readdirSync(join(root,"ob")).sort(),removed=before.filter(name=>!after.includes(name)).sort();
  expect(removed,scenario.name).toEqual(scenario.expectedRemoved.map(digest=>digest+".br").sort());
  expect(report.removed,scenario.name).toBe(scenario.expectedRemoved.length);
  expect(readdirSync(root).filter(name=>name.endsWith(".json")).length).toBe(Object.keys(scenario.headers).length);
 }
});

test("a dry run reports what it would remove and removes nothing, a second sweep finds nothing",async()=>{
 const scenario=fixture.scenarios[0]!,root=directory(scenario),options={graceMilliseconds:fixture.graceMilliseconds,now:fixture.nowMilliseconds};
 const before=readdirSync(join(root,"ob")).length,planned=await api.sweepSupersededObservationBlocksV1(root,{...options,dryRun:true});
 expect(planned.removed).toBe(scenario.expectedRemoved.length);expect(readdirSync(join(root,"ob")).length).toBe(before);
 await api.sweepSupersededObservationBlocksV1(root,options);
 expect((await api.sweepSupersededObservationBlocksV1(root,options)).removed).toBe(0);
});

test("below the threshold nothing is read, so a damaged header cannot matter",async()=>{
 const scenario=fixture.scenarios[0]!,root=directory(scenario);writeFileSync(join(root,"c".repeat(24)+".json"),"not a header");
 const report=await api.sweepSupersededObservationBlocksV1(root,{minimumBlocks:1000,now:fixture.nowMilliseconds});
 expect(report.skipped).toBe(true);expect(report.removed).toBe(0);
});

test("a header that is not a closed inference header aborts the sweep before any block is removed",async()=>{
 const scenario=fixture.scenarios[0]!,root=directory(scenario);writeFileSync(join(root,"c".repeat(24)+".json"),brotliCompressSync(Buffer.from('{"blocks":[]}\n{}')));
 const before=readdirSync(join(root,"ob")).length;
 await expect(api.sweepSupersededObservationBlocksV1(root,{graceMilliseconds:fixture.graceMilliseconds,now:fixture.nowMilliseconds})).rejects.toThrow();
 expect(readdirSync(join(root,"ob")).length).toBe(before);
});

test("an aborted signal stops the sweep at the next checkpoint",async()=>{
 const scenario=fixture.scenarios[0]!,root=directory(scenario),control=new AbortController();control.abort();
 await expect(api.sweepSupersededObservationBlocksV1(root,{signal:control.signal,now:fixture.nowMilliseconds})).rejects.toThrow();
 expect(existsSync(join(root,"ob"))).toBe(true);
});

test("the independent Python model removes the same blocks",()=>{
 const result=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});
});
