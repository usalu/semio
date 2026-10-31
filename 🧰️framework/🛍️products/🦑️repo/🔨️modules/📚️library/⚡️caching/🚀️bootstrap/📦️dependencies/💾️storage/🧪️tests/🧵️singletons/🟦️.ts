import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {createRequire} from "node:module";
import {lstatSync,mkdirSync,mkdtempSync,readdirSync,readlinkSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import * as storage from "../../🟦️.ts";
import {javascriptInstallSchemaV1 as schema} from "../../🧱️schemas/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

const digest=(value:string)=>createHash("sha256").update(value).digest("hex");

/** 🧵️ The policy lives in the install schema and the planner follows the neutral corpus, counted again by an independent SQLite query. */
test("singleton alias plan follows the neutral corpus with Ajv and an independent SQLite count",()=>{
 const policy=schema.$defs.SingletonProtocol.const;expect(policy.singletonOwner).toBe(corpus.policy.singletonOwner);expect(policy.singletonPackages).toEqual(corpus.policy.singletonPackages);
 const validate=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:"#/$defs/SingletonCase"}),db=new Database(":memory:");
 try{for(const row of corpus.planCases){expect(validate(row),row.id).toBe(true);let accepted=true,result:readonly unknown[]=[];try{result=storage.planJavascriptDependencySingletonAliasesV1(row.observation);}catch{accepted=false;}expect(accepted,row.id).toBe(row.accepted);if(!accepted)continue;expect(result,row.id).toEqual((row as {aliases:unknown[]}).aliases);
  const count=db.query(`SELECT count(*) n FROM json_each(?1,'$.members') m,json_each(m.value,'$.packages') p WHERE EXISTS (SELECT 1 FROM json_each(?1,'$.owner.packages') o WHERE json_extract(o.value,'$.name')=json_extract(p.value,'$.name') AND json_extract(o.value,'$.version')=json_extract(p.value,'$.version'))`).get(JSON.stringify(row.observation)) as {n:number};expect(count.n,row.id).toBe(result.length);}}finally{db.close();}
});

/** 🏷️ A non-owner ready record carries one binding row per singleton package to the owner generation, and the owner carries none. */
test("ready records bind exactly the policy packages to the owner generation",()=>{
 const generation=digest("b"),ownerGeneration=digest("."),policy=schema.$defs.SingletonProtocol.const,rows=[...policy.singletonPackages].sort().map((name:string)=>({name,owner:".",ownerGeneration})),
 base=()=>({version:4,purpose:"current-dependency-source-owner-ready",scope:"b",generation,ownerRelationSha256:digest("r"),groupSha256:digest("g"),recipeSha256:digest("c"),members:[],bindings:[{sourceDirectory:"/repo/b/node_modules",targetDirectory:"/cache/j/"+generation.slice(0,20)+"/node_modules"}],ownerReferences:[{scope:".",generation:ownerGeneration}],singletons:rows});
 expect(()=>storage.readJavascriptDependencySourceOwnerReadyV1(base())).not.toThrow();
 const refuse=(change:(value:ReturnType<typeof base>)=>void)=>{const value=base();change(value);expect(()=>storage.readJavascriptDependencySourceOwnerReadyV1(value)).toThrow();};
 refuse(value=>{value.singletons=value.singletons.slice(1);});refuse(value=>{value.singletons[0]!.ownerGeneration=digest("foreign");});refuse(value=>{value.ownerReferences=[];});refuse(value=>{value.singletons=[];});
 const root={...base(),scope:".",ownerReferences:[],bindings:[{sourceDirectory:"/repo/node_modules",targetDirectory:"/cache/j/"+generation.slice(0,20)+"/node_modules"}]};
 expect(()=>storage.readJavascriptDependencySourceOwnerReadyV1({...root,singletons:[]})).not.toThrow();expect(()=>storage.readJavascriptDependencySourceOwnerReadyV1(root)).toThrow();
});

const scopes=corpus.layout.scopes,generationOf=(scope:string)=>(corpus.layout.generations as Record<string,string>)[scope]!.repeat(64);

const build=(root:string,divergent:boolean,nestedVersion:string=corpus.layout.nested.version)=>{
 const workspace=join(root,"s"),cache=join(root,"n"),stage=(scope:string)=>join(cache,"j",generationOf(scope).slice(0,20)),packages={...corpus.layout.packages,...corpus.layout.bystander} as Record<string,string>;
 for(const scope of scopes){const versions={...packages};if(divergent&&scope===corpus.layout.divergent.scope)versions[corpus.layout.divergent.package]=corpus.layout.divergent.version;
  for(const [name,version] of Object.entries(versions)){const directory=join(stage(scope),"node_modules",...name.split("/"));mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"package.json"),JSON.stringify({name,version,main:"index.js"}));writeFileSync(join(directory,"index.js"),"module.exports="+JSON.stringify(scope)+";");}
  mkdirSync(join(workspace,scope==="."?"":scope),{recursive:true});symlinkSync(join(stage(scope),"node_modules"),join(workspace,scope==="."?"":scope,"node_modules"),"dir");}
 const nested=join(stage(corpus.layout.nested.scope),"node_modules",corpus.layout.nested.parent,"node_modules",corpus.layout.nested.package);mkdirSync(nested,{recursive:true});writeFileSync(join(nested,"package.json"),JSON.stringify({name:corpus.layout.nested.package,version:nestedVersion,main:"index.js"}));writeFileSync(join(nested,"index.js"),"module.exports=\"nested\";");
 mkdirSync(join(stage("b"),"w/0/node_modules"),{recursive:true});mkdirSync(join(workspace,"b/pkg"),{recursive:true});symlinkSync(join(stage("b"),"w/0/node_modules"),join(workspace,"b/pkg/node_modules"),"dir");
 const input={version:1,requested:scopes,owners:{version:1,scopes:[{manifest:"package.json",packages:[]},{manifest:"b/package.json",packages:["b/pkg"]},{manifest:"c/package.json",packages:[]}]},generations:scopes.map(scope=>({scope,generation:generationOf(scope)}))};
 return {workspace,cache,stage,input,consumers:[workspace,join(workspace,"b/pkg"),join(workspace,"c"),join(stage(corpus.layout.nested.scope),"node_modules",corpus.layout.nested.parent)],nested};
};
const copies=(consumers:readonly string[],name:string)=>new Set(consumers.map(consumer=>dirname(realpathSync(createRequire(join(consumer,"x.js")).resolve(name+"/package.json")))));
const publish=async(fixture:ReturnType<typeof build>,checkpoint:()=>void=()=>{})=>{
 const kernel=await import("@semio-tech/framework-filesystem/observation/descriptor"),control={signal:new AbortController().signal,publish:async()=>{},yieldContinuation:async()=>{}},context={root:fixture.cache,recheck:async()=>{}};
 return kernel.withArtifactRelativeDirectoryV1(fixture.cache,()=>{},base=>storage.publishJavascriptDependencySingletonsDescriptorV1(base,context as never,control as never,fixture.input,checkpoint));
};
const scratch=()=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"sg-")));

/** 🔬️ Node's own resolution from three consumer directories finds one physical copy of every singleton package after publication, and still three of every bystander. */
test("published generations resolve to one physical copy per singleton package under the Node realpath oracle",async()=>{
 const root=scratch();try{const fixture=build(root,false),policy=corpus.policy.singletonPackages,bystander=Object.keys(corpus.layout.bystander);
  for(const name of [...policy,...bystander])expect(copies(fixture.consumers,name).size,"before "+name).toBe(name===corpus.layout.nested.package?4:3);
  const changed=await publish(fixture);expect(changed).toBe(2*policy.length+1);
  for(const name of policy){const found=copies(fixture.consumers,name);expect(found.size,"after "+name).toBe(1);expect([...found][0]).toBe(join(fixture.stage("."),"node_modules",...name.split("/")));}
  for(const name of bystander)expect(copies(fixture.consumers,name).size,"bystander "+name).toBe(3);
  for(const scope of ["b","c"])for(const name of policy){const path=join(fixture.stage(scope),"node_modules",...name.split("/"));expect(lstatSync(path).isSymbolicLink()).toBe(true);expect(readlinkSync(path)).toBe(join(fixture.stage("."),"node_modules",...name.split("/")));}
  expect(await publish(fixture)).toBe(0);
  expect(lstatSync(fixture.nested).isSymbolicLink()).toBe(true);
  const stale=join(fixture.stage("b"),"node_modules/react");rmSync(stale);symlinkSync(join(fixture.cache,"j",digest("previous-owner").slice(0,20),"node_modules/react"),stale,"dir");
  expect(await publish(fixture)).toBe(1);expect(readlinkSync(stale)).toBe(join(fixture.stage("."),"node_modules/react"));expect(copies(fixture.consumers,"react").size).toBe(1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🚫️ A scope whose copy has another version is refused before any link is written. */
test("publication refuses a divergent singleton version and leaves every copy untouched",async()=>{
 const root=scratch();try{const fixture=build(root,true);await expect(publish(fixture)).rejects.toThrow("One singleton version required");
  for(const scope of scopes)expect(lstatSync(join(fixture.stage(scope),"node_modules/react")).isSymbolicLink()).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 💥️ A crash at any checkpoint of the swap leaves a state the next run resumes from: one physical copy, no tombstone, every occurrence a link. */
test("publication is crash-atomic: a crash at every checkpoint is recovered by the next run",async()=>{
 const total=await (async()=>{const root=scratch();try{const fixture=build(root,false);let count=0;await publish(fixture,()=>{count++;});return count;}finally{rmSync(root,{recursive:true,force:true});}})();expect(total).toBeGreaterThan(20);
 const leftovers=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.name.startsWith(".semio-singleton-")?[join(directory,entry.name)]:entry.isDirectory()?leftovers(join(directory,entry.name)):[]);
 for(let crash=1;crash<=total;crash++){const root=scratch();try{const fixture=build(root,false);let count=0;await publish(fixture,()=>{if(++count===crash)throw Error("injected crash");}).then(()=>{throw Error("crash "+crash+" never fired");},error=>{expect((error as Error).message,"crash "+crash).toBe("injected crash");});
  await publish(fixture);for(const name of corpus.policy.singletonPackages){const found=copies(fixture.consumers,name);expect(found.size,"crash "+crash+" "+name).toBe(1);}
  expect(leftovers(join(fixture.cache,"j")),"crash "+crash).toEqual([]);expect(lstatSync(fixture.nested).isSymbolicLink(),"crash "+crash).toBe(true);expect(await publish(fixture),"crash "+crash).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}}
},300000);

/** 🪆️ A nested copy with another version is refused and names scope, directory and package. */
test("nested divergent singleton version is refused with its location",async()=>{
 const root=scratch();try{const fixture=build(root,false,corpus.layout.nestedDivergent.version);await expect(publish(fixture)).rejects.toThrow("One singleton version required: b ");
  expect(lstatSync(fixture.nested).isSymbolicLink()).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🪟️ Junction link texts with the extended prefix, a trailing separator or another letter case name the same target on Windows and nothing else. */
test("link target comparison normalizes Windows junction text",()=>{
 const same=storage.sameJavascriptDependencyLinkTargetV1;
 expect(same("\\\\?\\C:\\Cache\\j\\a\\node_modules\\react\\","c:\\cache\\j\\a\\node_modules\\react","win32")).toBe(true);
 expect(same("C:\\cache\\j\\b\\node_modules\\react","C:\\cache\\j\\a\\node_modules\\react","win32")).toBe(false);
 expect(same("/c/j/a/node_modules/react/","/c/j/a/node_modules/react","linux")).toBe(true);expect(same("/c/J/a","/c/j/a","linux")).toBe(false);
});

/** 🐍️ The Python twin rebuilds the layout from the same corpus and resolves it with its own algorithm and with Node. */
test("python twin agrees on the neutral corpus",()=>{
 const python=testPythonV1()??"python3",done=spawnSync(python,["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{encoding:"utf8"});
 expect(done.status,done.stderr).toBe(0);
});
