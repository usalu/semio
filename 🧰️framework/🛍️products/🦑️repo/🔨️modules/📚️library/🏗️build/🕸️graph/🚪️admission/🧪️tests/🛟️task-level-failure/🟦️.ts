/** 🛟️ Contract: the failure of ONE project's configuration (here a project.json whose test target names a route the canonical table does not declare) is a fact about that project. The read seals every other owner, a plan for an unrelated target succeeds, and a plan whose task closure needs the project fails with the named, bilingual, owner-scoped failure (never the unexpected failure of the service). A project that is only a project dependency, or only a `^` task of a target it does not declare, is not in the task closure.
 * @see ../../🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {closeBuildOwnerAdmissionV1,readBuildGraphSelectionV1,type BuildOwnerRegistryV1} from "../../🟦️.ts";
import {isOwnerFailureV1,observeOwnerWithRetryV1} from "../../../📥️load/🛟️owner-isolation/🟦️.ts";
import {canonicalTestAggregatePolicyV1 as aggregate,deriveCanonicalTestAggregateV1,requireCanonicalTestAggregateV1} from "../../../../../⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts";
import {failureRefusalV1,ownerFailureOfV1} from "../../../../🏡️resident/🏛️service/🧯️failure/🟨️.mjs";

const workspace=(()=>{for(let directory=import.meta.dirname;directory!==dirname(directory);directory=dirname(directory))if(existsSync(join(directory,aggregate.manifest)))return directory;throw Error("workspace not found");})();
const directories:string[]=[];
afterAll(()=>{for(const directory of directories)rmSync(directory,{recursive:true,force:true});});

/** The real route tables and the real declaration of the aggregate owner, plus one target whose command names an undeclared route: the real refusal `route-undeclared`. */
const brokenProject=()=>{
 const directory=mkdtempSync(join(tmpdir(),"task-level-failure-"));directories.push(directory);
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),text);};
 for(const path of [aggregate.cacheTable,aggregate.buildTable,aggregate.manifest])put(path,readFileSync(join(workspace,path),"utf8"));
 const manifest=JSON.parse(readFileSync(join(directory,aggregate.manifest),"utf8")),donor=manifest.targets["test-build-test-file"],target=structuredClone(donor);
 target.options.command=String(target.options.command).replace("build-test-file","zz-undeclared-route");
 manifest.targets[aggregate.target]=deriveCanonicalTestAggregateV1({...manifest,root:aggregate.root} as never,path=>readFileSync(join(directory,path),"utf8"));
 manifest.targets["test-zz-undeclared-route"]=target;
 put(aggregate.manifest,JSON.stringify(manifest,null,2)+"\n");
 const read=(path:string)=>readFileSync(join(directory,path),"utf8");
 return{directory,assemble:()=>requireCanonicalTestAggregateV1(JSON.parse(read(aggregate.manifest)),aggregate.root,read)};
};

interface Row {dependencies?:string[];targets:Record<string,string[]>;files?:string[]}
const table:Record<string,Row>={
 app:{dependencies:["lib","repo"],targets:{build:["^build"],bundle:["build"]}},
 lib:{dependencies:["core"],targets:{build:["^build"]}},
 core:{targets:{build:[]}},
 repo:{targets:{test:[]}},
 other:{targets:{build:[]}},
};
const registry=():BuildOwnerRegistryV1=>({version:1,owners:Object.keys(table).map(name=>({name,root:name,manifest:name+"/📋️project.json",packages:[]})),catalogProviders:[]});
const factsOf=(name:string)=>({project:{name,targets:Object.fromEntries(Object.entries(table[name]!.targets).map(([target,dependsOn])=>[target,{dependsOn}]))},files:table[name]!.files??[],dependencies:table[name]!.dependencies??[]});

const walk=async(tasks:{project:string;target:string}[],failing:string,assemble:()=>void,declares?:(owner:string,target:string)=>boolean|undefined)=>{
 const observed:string[]=[],reports:{owner:string;needed:boolean}[]=[];
 const run=()=>closeBuildOwnerAdmissionV1(readBuildGraphSelectionV1({kind:"tasks",tasks}),registry(),async owner=>{
  observed.push(owner.name);
  return await observeOwnerWithRetryV1({owner:owner.name,pause:async()=>{},hold:async()=>true,attempt:async()=>{if(owner.name===failing)assemble();return factsOf(owner.name);}}) as never;
 },failure=>reports.push({owner:failure.owner,needed:failure.needed}),declares);
 const result=await run().then(value=>({value,failure:undefined as unknown}),failure=>({value:undefined,failure}));
 return{...result,observed,reports};
};

test("the broken project is a real refusal of the aggregate table, named and stable",()=>{
 const broken=brokenProject();
 expect(broken.assemble).toThrow("test.aggregate-refused");
 let message="";try{broken.assemble();}catch(error){message=String((error as Error).message)+JSON.stringify((error as {values?:unknown}).values??{});}
 expect(message).toContain("route-undeclared");
});

test("a plan for a target that does not need the broken project succeeds and seals every other owner",async()=>{
 const broken=brokenProject(),seen=await walk([{project:"other",target:"build"}],"repo",broken.assemble);
 expect(seen.failure).toBeUndefined();
 expect(seen.value!.owners).toEqual(["other"]);
 expect(seen.observed).toEqual(["other"]);
});

test("a project only some admitted owner depends on is not in the task closure: the read closes over the others and names it as withheld",async()=>{
 const broken=brokenProject(),seen=await walk([{project:"app",target:"bundle"}],"repo",broken.assemble,(owner,target)=>owner==="repo"?target==="test":true);
 expect(seen.failure).toBeUndefined();
 expect(seen.value!.owners).toEqual(["app","core","lib"]);
 expect(seen.reports).toEqual([{owner:"repo",needed:false}]);
});

test("a `^` task of a target the broken project does not declare is not a task; one it declares is, and the plan fails precisely",async()=>{
 const broken=brokenProject(),without=await walk([{project:"app",target:"build"}],"repo",broken.assemble,(owner,target)=>owner==="repo"?target==="test":true);
 expect(without.failure).toBeUndefined();
 expect(without.reports).toEqual([{owner:"repo",needed:false}]);
 const withTarget=await walk([{project:"app",target:"build"}],"repo",broken.assemble,(owner,target)=>owner==="repo"?target==="build":true);
 expect(isOwnerFailureV1(withTarget.failure)).toBe(true);
 expect(withTarget.reports).toEqual([{owner:"repo",needed:true}]);
});

test("a plan that asks for a task of the broken project fails at the end with its named bilingual failure, after every other owner was observed",async()=>{
 const broken=brokenProject(),seen=await walk([{project:"repo",target:"test"},{project:"app",target:"bundle"}],"repo",broken.assemble);
 expect(isOwnerFailureV1(seen.failure)).toBe(true);
 const failure=seen.failure as {owners:{name:string;code:string}[];text:{en:string;de:string};stable:boolean};
 expect(failure.owners.map(owner=>owner.name)).toEqual(["repo"]);
 expect(failure.owners[0]!.code).toBe("invalid");
 expect(failure.stable).toBe(true);
 expect(failure.text.en).toContain("repo");
 expect(failure.text.de).toContain("repo");
 expect(failure.text.en).toContain("route-undeclared");
 expect(seen.observed).toContain("app");
 expect(seen.observed).toContain("lib");
 expect(seen.observed).toContain("core");
});

test("the service answers the failure of a needed project as an owner-scoped refusal, in both languages, never as an unexpected failure",async()=>{
 const broken=brokenProject(),seen=await walk([{project:"repo",target:"test"}],"repo",broken.assemble),logged:string[]=[];
 const owner=ownerFailureOfV1(seen.failure);
 expect(owner?.owners).toBe("repo");
 expect(owner?.message).toContain("test.aggregate-refused");
 const refusal=failureRefusalV1(seen.failure,{log:event=>logged.push(event),logPath:"service.log"}) as {code:string;key:string;fields:Record<string,string>;retry:string};
 expect(refusal).toMatchObject({code:"unserved",key:"host-graph-owner-failed",retry:"later"});
 expect(refusal.fields.owners).toBe("repo");
 expect(logged).toEqual(["graph-owner-failed"]);
 const wrapped=failureRefusalV1(Object.assign(Error("wrapper",{cause:seen.failure}),{}),{log:()=>{},logPath:"service.log"}) as {key:string};
 expect(wrapped.key).toBe("host-graph-owner-failed");
 const text=readFileSync(new URL("../../../../🏡️resident/🧬️schema/🔣️.json",import.meta.url),"utf8"),both=[...text.matchAll(/"host-graph-owner-failed": "([^"]+)"/gu)].map(row=>row[1]!);expect(both.length).toBe(2);
 const names=(value:string)=>[...value.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 expect(names(both[0]!)).toEqual(names(both[1]!));
 expect(names(both[0]!)).toEqual(["message","owners"]);
});
