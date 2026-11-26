/** 🧹️ Contract: a machine's directory of the shared store sheds the temporary files of dead writers and the published-name files nobody reads any more, and nothing else; the verdicts equal an independent Python model, and the retention is never below the age at which readers ignore a published file.
 * @see ../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{testPythonV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{TAKE_POLICY_V1}from"../../../📥️take/🟨️.mjs";
import{ORIGIN_FILES_POLICY_V1,retireOriginFilesV1}from"../🟦️.ts";
import{createOwnerPoolStatusV1}from"../../📊️status/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

const scratch:string[]=[],directory=():string=>{const path=mkdtempSync(join(tmpdir(),"origin-files-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const now=Date.now();
const listing=(root:string,prefix:string=""):string[]=>readdirSync(root,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?listing(join(root,entry.name),`${prefix}${entry.name}/`):[`${prefix}${entry.name}`]).sort();
const populate=(root:string,files:readonly{name:string;ageMilliseconds:number}[])=>{for(const file of files){mkdirSync(dirname(join(root,file.name)),{recursive:true});writeFileSync(join(root,file.name),"x");const at=(now-file.ageMilliseconds)/1000;utimesSync(join(root,file.name),at,at);}};

test("the fixture cases are valid against their schema definition",()=>{
 const valid=new Ajv({strict:true,allErrors:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Case"});
 for(const row of fixture.cases)expect(valid(row),JSON.stringify(valid.errors)).toBe(true);
});

test("the files retired equal the independent model, and the files kept are exactly the others",async()=>{
 const python=testPythonV1(),run=Bun.spawnSync([python!,"-I",join(import.meta.dir,"🐍️oracle","🐍️.py")],{stdin:Buffer.from(JSON.stringify({...fixture,policy:ORIGIN_FILES_POLICY_V1}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const expected=JSON.parse(run.stdout.toString()) as string[][];
 for(const [index,row] of fixture.cases.entries()){
  const root=directory();populate(root,row.files);
  const retired=await retireOriginFilesV1({directory:root,now,alive:pid=>(row.alive as number[]).includes(pid),own:fixture.own});
  expect([...retired.temporaries,...retired.published,...retired.appleDouble].sort(),row.name).toEqual(expected[index]!);
  expect(expected[index],row.name).toEqual(row.removed);
  expect(listing(root),row.name).toEqual(row.files.map(file=>file.name).filter(name=>!row.removed.includes(name)).sort());
 }
});

test("the caller's own temporary file is never retired, and a missing directory is no failure",async()=>{
 const root=directory();writeFileSync(join(root,`pool.json.${process.pid}.tmp`),"x");
 expect((await retireOriginFilesV1({directory:root,alive:()=>false})).temporaries).toEqual([]);
 expect(readdirSync(root)).toEqual([`pool.json.${process.pid}.tmp`]);
 expect(await retireOriginFilesV1({directory:join(root,"missing")})).toEqual({temporaries:[],published:[],appleDouble:[]});
});

test("the retention never falls below the age at which readers ignore a published file",()=>{
 expect(ORIGIN_FILES_POLICY_V1.publishedRetentionMilliseconds).toBeGreaterThanOrEqual(TAKE_POLICY_V1.publishedMaximumAgeMilliseconds);
});

test("the pool status writer leaves no temporary file when the rename fails, and retires the dead writers' files at its first write",async()=>{
 const store=directory(),origin="g1-m09",home=join(store,"v1","origins",origin);mkdirSync(join(home,"pool.json"),{recursive:true});writeFileSync(join(home,"pool.json","keep"),"x");
 let dead=2147483000;while(true){try{process.kill(dead,0);dead--;}catch{break;}}
 writeFileSync(join(home,`ready.json.${dead}.tmp`),"");writeFileSync(join(home,`pool-published-abc123.txt`),"a");utimesSync(join(home,"pool-published-abc123.txt"),1,1);
 createOwnerPoolStatusV1({store,origin,role:"puller"}).flush();
 await Bun.sleep(300);
 expect(readdirSync(home).sort()).toEqual(["pool.json"]);
});
