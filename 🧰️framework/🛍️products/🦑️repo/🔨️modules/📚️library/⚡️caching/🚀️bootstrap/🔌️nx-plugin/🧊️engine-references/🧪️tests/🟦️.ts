/** 🧊️ Contract: the persisted references memo answers a stamped file without reading it, a touched file with equal text from its digest, only unseen text by a scan, and every broken memo as an empty one; its digests equal an independent Python sha256 and its file is valid under its schema; two processes of the real closure agree and the second one scans nothing. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import {nxEngineReferencesDirectoryV1,openNxEngineReferencesMemoV1} from "../🟨️.mjs";

const area=mkdtempSync(join(tmpdir(),"nx-engine-references-"));afterAll(()=>rmSync(area,{recursive:true,force:true}));
const validate=new Ajv({strict:true}).compile(schema);
const scanner=()=>{const seen:string[]=[];return{seen,scan:(text:string)=>{seen.push(text);return[{kind:"module",specifier:"./"+text.trim()}];}};};
const fixture=(name:string)=>{const root=join(area,name,"root"),directory=join(area,name,"memo");mkdirSync(root,{recursive:true});writeFileSync(join(root,"a.ts"),"alpha\n");writeFileSync(join(root,"b.ts"),"beta\n");return{root,directory};};
const pass=(root:string,directory:string,scan:(text:string)=>unknown)=>{const memo=openNxEngineReferencesMemoV1(root,directory),rows=["a.ts","b.ts"].map(name=>memo.references(join(root,name),scan as never));return{rows,saved:memo.save()};};
const python=(path:string)=>{const run=Bun.spawnSync(["python3","-I","-c","import hashlib,sys\nprint(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",path]);expect(run.exitCode,run.stderr.toString()).toBe(0);return run.stdout.toString().trim();};

test("a fresh process scans only text it never saw, a touched file with equal text is answered from its digest",()=>{
 const{root,directory}=fixture("reuse"),first=scanner(),cold=pass(root,directory,first.scan);
 expect(first.seen).toEqual(["alpha\n","beta\n"]);expect(cold.saved).toBe(true);
 const second=scanner(),warm=pass(root,directory,second.scan);expect(second.seen).toEqual([]);expect(warm.rows).toEqual(cold.rows);expect(warm.saved).toBe(false);
 const later=new Date(Date.now()+5000);utimesSync(join(root,"a.ts"),later,later);
 const touched=scanner(),again=pass(root,directory,touched.scan);expect(touched.seen).toEqual([]);expect(again.rows).toEqual(cold.rows);expect(again.saved).toBe(true);
 writeFileSync(join(root,"b.ts"),"gamma\n");const changed=scanner(),edited=pass(root,directory,changed.scan);expect(changed.seen).toEqual(["gamma\n"]);expect(edited.rows[1]).toEqual([{kind:"module",specifier:"./gamma"}]);
});

test("the memo file is valid under its schema, names only the used files and its digests equal an independent sha256",()=>{
 const{root,directory}=fixture("format");pass(root,directory,scanner().scan);writeFileSync(join(root,"c.ts"),"extra\n");
 const names=readdirSync(directory).filter(name=>name.endsWith(".json"));expect(names).toHaveLength(1);
 const value=JSON.parse(readFileSync(join(directory,names[0]!),"utf8"));expect(validate(value),JSON.stringify(validate.errors)).toBe(true);
 expect(Object.keys(value.files)).toEqual(["a.ts","b.ts"]);
 for(const name of Object.keys(value.files))expect(value.files[name].sha256).toBe(python(join(root,name)));
 expect(Object.keys(value.references).sort()).toEqual(Object.values(value.files).map((row:any)=>row.sha256).sort());
 expect(readdirSync(directory).filter(name=>name.endsWith(".tmp"))).toEqual([]);
});

test("a broken, oversized or unwritable memo is an empty one and never fails the caller",()=>{
 for(const broken of ["","{","[]",JSON.stringify({version:2,files:{},references:{}}),JSON.stringify({version:1,files:{"a.ts":{stamp:"x",sha256:"y"}},references:{}}),JSON.stringify({version:1,files:{},references:{["0".repeat(64)]:[{kind:"foreign",specifier:"x"}]}})]){
  const{root,directory}=fixture("broken"+broken.length);pass(root,directory,scanner().scan);const file=join(directory,readdirSync(directory)[0]!);writeFileSync(file,broken);
  const counted=scanner(),result=pass(root,directory,counted.scan);expect(counted.seen).toEqual(["alpha\n","beta\n"]);expect(result.rows).toHaveLength(2);
 }
 const{root}=fixture("unwritable"),blocker=join(area,"unwritable","blocker");writeFileSync(blocker,"file");
 expect(pass(root,join(blocker,"inside"),scanner().scan).saved).toBe(false);
});

test("the directory is the declared one, else the per-user cache of the platform",()=>{
 expect(nxEngineReferencesDirectoryV1({[schema.definitions.Limits.const.environment]:"/x/y"},"linux","/h")).toBe("/x/y");
 expect(nxEngineReferencesDirectoryV1({},"darwin","/Users/u")).toBe("/Users/u/Library/Caches/semio/nx-engine-references");
 expect(nxEngineReferencesDirectoryV1({LOCALAPPDATA:"C:\\L"},"win32","C:\\u")).toContain("nx-engine-references");
 expect(nxEngineReferencesDirectoryV1({XDG_CACHE_HOME:"/c"},"linux","/h")).toBe("/c/semio/nx-engine-references");
 expect(nxEngineReferencesDirectoryV1({},"linux","/h")).toBe("/h/.cache/semio/nx-engine-references");
});

test("two processes of the real engine closure agree and the second answers from the memo",()=>{
 const directory=join(area,"real"),library=join(import.meta.dir,"../../../../../"),run=()=>{const began=Date.now(),child=Bun.spawnSync([process.execPath,"-e",`const api=await import(${JSON.stringify(join(library,"🟨️.mjs"))});console.log(JSON.stringify(api.nxBuildEngineClosureV1()));`],{env:{...process.env,SEMIO_NX_ENGINE_REFERENCES_DIRECTORY:directory,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});expect(child.exitCode,child.stderr.toString()).toBe(0);return{value:JSON.parse(child.stdout.toString().trim().split("\n").at(-1)!),milliseconds:Date.now()-began};};
 const cold=run(),warm=run();expect(warm.value).toEqual(cold.value);expect(cold.value.files.length).toBeGreaterThan(100);
 expect(readdirSync(directory).filter(name=>name.endsWith(".json"))).toHaveLength(1);
 const memo=JSON.parse(readFileSync(join(directory,readdirSync(directory).find(name=>name.endsWith(".json"))!),"utf8"));expect(Object.keys(memo.files).sort()).toEqual(cold.value.files.filter((file:string)=>!file.endsWith(".json")).sort());
},180000);
