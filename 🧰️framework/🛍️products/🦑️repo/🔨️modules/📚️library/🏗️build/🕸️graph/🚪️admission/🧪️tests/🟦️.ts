import {readBuildGraphV1} from "../../🟦️.ts";
import {buildGraphProjectInferenceOptionsV1} from "../../🧭️discovery/🟦️.ts";
import graphSchema from "../../../🧬️schema/🔣️.json" with {type:"json"};
import {selectBuildProjectsV1,selectBuildTasksV1,buildOptionsV1} from "./../../../🧭️selection/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import selectionSchema from "../../../🧬️schema/🚪️selection.json" with {type:"json"};
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../../../🧬️schema/🚪️admission.json" with {type:"json"};
import {buildInvocationGraphSelectionV1,buildGraphScopeKeyV1,buildSourceDemandV1,buildSourceContractGroupsV1,closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,readBuildGraphSelectionV1} from "../🟦️.ts";

test("catalog activation requires actual consumer tasks, direct input custody or intentional all-owner admission",async()=>{
 const registry=readBuildOwnerRegistryV1(fixture.registry),oracle=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");
 try{db.exec("CREATE TABLE consumer(root TEXT);CREATE TABLE task(owner TEXT,target TEXT);CREATE TABLE owner(name TEXT,root TEXT);INSERT INTO consumer VALUES ('registry');INSERT INTO owner VALUES ('app','app'),('registry','registry'),('provider','provider');");
 for(const row of fixture.cases){const rowInputs:readonly string[]=row.inputs,selection=readBuildGraphSelectionV1(row.selection),result=await closeBuildOwnerAdmissionV1(selection,registry,async(owner,targets,inputs)=>({project:{name:owner.name,targets:owner.name==="app"?{build:{dependsOn:row.dependsOn}}:owner.name==="registry"?Object.fromEntries(row.registryTargets.map(target=>[target,{}])):{}},files:[owner.manifest,...owner.name==="app"&&targets.length?["registry/api.ts"]:[],...owner.name==="provider"&&inputs.length?["provider/publication.ts"]:[]],dependencies:owner.name==="app"&&targets.length?["registry"]:[],dependencyInputs:Object.fromEntries(owner.name==="app"&&targets.length&&rowInputs.length?[["provider",[...rowInputs]]]:[])}));
 expect(oracle(result),JSON.stringify(oracle.errors)).toBe(true);expect(result.catalogProviders.map(provider=>provider.sourcePath),row.name).toEqual(row.expectedProviders);if(row.name==="imported consumer Source"){expect(result.owners).toContain("registry");expect(result.files).toContain("registry/api.ts");expect(result.targets.registry).toBeUndefined();}
 db.exec("DELETE FROM task");for(const[name,targets]of Object.entries(result.targets))for(const target of targets)db.query("INSERT INTO task VALUES (?,?)").run(name,target);
 const selected=db.query("SELECT DISTINCT consumer.root FROM consumer JOIN owner ON owner.root=consumer.root OR owner.root LIKE consumer.root||'/%' JOIN task ON task.owner=owner.name").all();expect(selection.kind==="all"||selected.length>0||rowInputs.includes("{workspaceRoot}/provider/publication.ts")).toBe(row.expectedProviders.length>0);
 }
 }finally{db.close();}
});

test("the selected source input walk bounds named references and literal leaves separately, so inferred input lists never exhaust the reference bound",()=>{
 const scope=(schema as any).$defs.sourceContractScope.const,literal=(index:number)=>"{workspaceRoot}/f/"+index+".ts",many=scope.maximumReferences+1;
 const project=(inputs:readonly unknown[],namedInputs:Record<string,readonly unknown[]>={})=>({name:"p",targets:{t:{inputs}},namedInputs,metadata:{sourceInputs:{contract:"c.json"}}});
 expect(scope.maximumLiterals).toBeGreaterThan(scope.maximumReferences);
 expect(buildSourceContractGroupsV1(project([...Array.from({length:many},(_,index)=>literal(index)),"group"],{group:["contract"]}),{},[])).toEqual(["contract"]);
 expect(()=>buildSourceContractGroupsV1(project(Array.from({length:many},(_,index)=>({input:"g"+index}))),Object.fromEntries(Array.from({length:many},(_,index)=>["g"+index,[]])),[])).toThrow(/closure required/u);
 expect(()=>buildSourceContractGroupsV1(project(Array.from({length:scope.maximumLiterals+1},(_,index)=>literal(index))),{},[])).toThrow(/literals required/u);
});

test("actual admission module exports closed monotonic source demand with independent SQLite priority",()=>{
 const valid=new Ajv({strict:true}).compile({$defs:schema.$defs,$ref:"#/$defs/sourceDemandExportCases"}),db=new Database(":memory:");expect(valid(fixture.sourceDemandExportCases),JSON.stringify(valid.errors)).toBe(true);
 try{db.exec("CREATE TABLE role(name TEXT PRIMARY KEY,priority INTEGER);INSERT INTO role VALUES('runtime',0),('compile',1)");for(const row of fixture.sourceDemandExportCases){const previous="previous"in row?row.previous:undefined,requested=row.requested,prior=typeof previous==="string"?previous:null,request=typeof requested==="string"?requested:null,oracle=db.query("SELECT name FROM role WHERE name IN(?1,?2) AND EXISTS(SELECT 1 FROM role WHERE name=?2) AND (?3=0 OR EXISTS(SELECT 1 FROM role WHERE name=?1)) ORDER BY priority DESC LIMIT 1").get(prior,request,Number("previous"in row))as{name:string}|null;expect(oracle?.name??null,row.name).toBe(row.expected);expect(oracle!==null,row.name).toBe(row.accepted);if(row.accepted)expect<unknown>(buildSourceDemandV1(previous,requested),row.name).toBe(row.expected);else expect(()=>buildSourceDemandV1(previous,requested)).toThrow("Closed source demand role required");}}finally{db.close();}
});

test("explicit exec owner admission matches the actual planner and independent SQLite registry closure",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"graph-exec-admission-test",maximumElapsedMilliseconds:30000},{},Date.now()),async original=>{
 const corpus=fixture.execOwnerAdmission,registry=readBuildOwnerRegistryV1(corpus.registry),valid=new Ajv({strict:true}).compile({$defs:schema.$defs,$ref:"#/$defs/execOwnerAdmissionCases"}),selectionValid=new Ajv({strict:true}).compile(selectionSchema),db=new Database(":memory:");expect(valid(corpus),JSON.stringify(valid.errors)).toBe(true);expect(graphSchema.properties.admission.properties.selection.oneOf).toEqual(selectionSchema.oneOf);
 try{db.exec("CREATE TABLE owner(name TEXT PRIMARY KEY,root TEXT);CREATE TABLE edge(source TEXT,target TEXT);INSERT INTO edge VALUES('app','lib')");for(const owner of registry.owners)db.query("INSERT INTO owner VALUES(?,?)").run(owner.name,owner.root);for(const row of corpus.cases){const selection=buildInvocationGraphSelectionV1(row.args);expect<unknown>(selection,row.name).toEqual(row.selection);expect(selectionValid(selection),JSON.stringify(selectionValid.errors)).toBe(true);const project=(owner:typeof registry.owners[number])=>({name:owner.name,root:owner.root,targets:{},tags:owner.name==="app"?["chosen"]:[]}),graph={version:1 as const,projects:Object.fromEntries(registry.owners.map(owner=>[owner.name,project(owner)])),dependencies:[]};const options=(names:string[])=>buildOptionsV1(row.args,names).flatMap(value=>value.split(",").filter(Boolean));expect(selectBuildProjectsV1(graph,options(["projects","p"]),options(["exclude"])),row.name).toEqual(row.projects);
 const planned=await selectBuildTasksV1("/workspace",graph,row.args,original);expect(planned.tasks,row.name).toEqual(row.projects.map(name=>name+":repo-exec"));const boundary=row.args.indexOf("--"),literal=row.args.slice(boundary+1);for(const name of row.projects){const target=graph.projects[name]!.targets as Record<string,any>;expect(target["repo-exec"].execution,row.name).toEqual({kind:"command",program:literal[0],args:literal.slice(1),cwd:graph.projects[name]!.root,forwardAllArgs:true});}
 if(selection.kind==="owners"){const chosen=db.query("SELECT name FROM owner o WHERE EXISTS(SELECT 1 FROM json_each(?1) p WHERE CASE WHEN substr(p.value,1,5)='name:' THEN o.name=substr(p.value,6) WHEN substr(p.value,1,10)='directory:' THEN o.root=substr(p.value,11) ELSE o.name=p.value OR o.root=p.value END) AND NOT EXISTS(SELECT 1 FROM json_each(?2) p WHERE CASE WHEN substr(p.value,1,5)='name:' THEN o.name=substr(p.value,6) WHEN substr(p.value,1,10)='directory:' THEN o.root=substr(p.value,11) ELSE o.name=p.value OR o.root=p.value END) ORDER BY name").all(JSON.stringify(selection.projects),JSON.stringify(selection.exclude))as{name:string}[];expect(chosen.map(row=>row.name),row.name).toEqual(row.projects);const closure=db.query("WITH RECURSIVE selected(name)AS(SELECT value FROM json_each(?1) UNION SELECT edge.target FROM edge JOIN selected ON edge.source=selected.name) SELECT name FROM selected ORDER BY name").all(JSON.stringify(chosen.map(row=>row.name)))as{name:string}[];expect(closure.map(row=>row.name),row.name).toEqual(row.owners);}
 const calls=new Map<string,number>(),admitted=await closeBuildOwnerAdmissionV1(selection,registry,async(owner,targets,inputs)=>{expect(targets).toEqual([]);expect(inputs).toEqual([]);calls.set(owner.name,(calls.get(owner.name)??0)+1);return{project:project(owner),files:[owner.manifest,...owner.name==="app"?["library/api.ts"]:[]],dependencies:owner.name==="app"?["lib"]:[]};});expect(admitted.owners,row.name).toEqual(row.owners);expect([...calls.keys()].sort(),row.name).toEqual(row.owners);expect([...calls.values()].every(value=>value===1),row.name).toBe(true);expect(admitted.targets).toEqual({});const serialized={version:1,projects:Object.fromEntries(registry.owners.map(owner=>[owner.name,project(owner)])),dependencies:[],admission:{selection,registry:"a".repeat(64),closure:"b".repeat(64)}};expect(new Ajv({strict:false}).compile(graphSchema)(serialized),row.name).toBe(true);expect(readBuildGraphV1(serialized).admission!.selection,row.name).toEqual(selection);const key=buildGraphScopeKeyV1(selection,registry);expect(key).toMatch(/^[0-9a-f]{64}$/u);if(selection.kind==="owners")expect(key).not.toBe(buildGraphScopeKeyV1({kind:"all"},registry));
 }}finally{db.close();}
}));

test("run-many with literal projects and targets admits their owner closure and the prerequisites of those targets, not the whole workspace",()=>{
 const owners={kind:"owners",projects:["app","lib"],exclude:["lib"],targets:["test","typecheck"]};
 expect<unknown>(buildInvocationGraphSelectionV1(["run-many","--targets=typecheck,test","--projects=app,lib","--exclude=lib"])).toEqual(owners);
 expect<unknown>(buildInvocationGraphSelectionV1(["run-many","-p","app,lib","--exclude=lib","-t","test","--target=typecheck","--parallel=1","--","--flag"])).toEqual(owners);
 const open=[["--targets=typecheck"],["--targets=typecheck","--projects=tag:scope"],["--targets=typecheck","--projects=app","--all"],["--targets=typecheck","--projects=*"],["--targets=type*","--projects=app"],["--projects=app"]];
 for(const rest of open)expect<unknown>(buildInvocationGraphSelectionV1(["run-many",...rest]),rest.join(" ")).toEqual({kind:"all"});
 expect<unknown>(buildInvocationGraphSelectionV1(["run-many","--targets=typecheck","--projects=app","--","--all"]).kind).toBe("owners");
 expect<unknown>(readBuildGraphSelectionV1(owners)).toEqual(owners);expect(()=>readBuildGraphSelectionV1({...owners,targets:["a*"]})).toThrow();
});


test("a task run that excludes its dependencies admits only its own owner, matching an independent SQLite closure",async()=>{
 const corpus=fixture.directTaskAdmission,registry=readBuildOwnerRegistryV1(corpus.registry),selectionValid=new Ajv({strict:true}).compile(selectionSchema),admissionValid=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE edge(source TEXT,target TEXT)");for(const[source,target]of corpus.edges)db.query("INSERT INTO edge VALUES(?,?)").run(source,target);
  for(const row of corpus.cases){
   const selection=buildInvocationGraphSelectionV1(row.args);expect<unknown>(selection,row.name).toEqual(row.selection);expect(selectionValid(selection),row.name+JSON.stringify(selectionValid.errors)).toBe(true);expect<unknown>(readBuildGraphSelectionV1(selection),row.name).toEqual(row.selection);
   const observed:string[]=[],admitted=await closeBuildOwnerAdmissionV1(selection,registry,async(owner,targets)=>{observed.push(owner.name);return{project:{name:owner.name,targets:{build:{dependsOn:owner.name==="app"?["^build"]:[]}}},files:[owner.manifest,...owner.name==="app"?["library/api.ts"]:[]],dependencies:owner.name==="app"?["lib"]:[]};});
   const direct=row.selection.excludeDependencies===true,closure=db.query("WITH RECURSIVE reach(name) AS (SELECT 'app' UNION SELECT edge.target FROM edge JOIN reach ON edge.source=reach.name WHERE ?1=0) SELECT name FROM reach ORDER BY name").all(direct?1:0).map(found=>(found as {name:string}).name);
   expect(admissionValid(admitted),row.name+JSON.stringify(admissionValid.errors)).toBe(true);expect(admitted.owners,row.name).toEqual(row.owners);expect(admitted.owners,row.name).toEqual(closure);expect(admitted.targets,row.name).toEqual(row.targets);if(direct)expect([...new Set(observed)],row.name).toEqual(["app"]);
  }
  expect(()=>readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:"app",target:"build"}],excludeDependencies:false})).toThrow();
  expect(buildGraphScopeKeyV1(readBuildGraphSelectionV1(corpus.cases[0]!.selection),registry)).not.toBe(buildGraphScopeKeyV1(readBuildGraphSelectionV1(corpus.cases[4]!.selection),registry));
 }finally{db.close();}
});

test("a task run that excludes its dependencies admits only its own owner, matching an independent SQLite closure",async()=>{
 const corpus=fixture.directTaskAdmission,registry=readBuildOwnerRegistryV1(corpus.registry),selectionValid=new Ajv({strict:true}).compile(selectionSchema),admissionValid=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE edge(source TEXT,target TEXT)");for(const[source,target]of corpus.edges)db.query("INSERT INTO edge VALUES(?,?)").run(source,target);
  for(const row of corpus.cases){
   const selection=buildInvocationGraphSelectionV1(row.args);expect<unknown>(selection,row.name).toEqual(row.selection);expect(selectionValid(selection),row.name+JSON.stringify(selectionValid.errors)).toBe(true);expect<unknown>(readBuildGraphSelectionV1(selection),row.name).toEqual(row.selection);
   const observed:string[]=[],admitted=await closeBuildOwnerAdmissionV1(selection,registry,async owner=>{observed.push(owner.name);return{project:{name:owner.name,targets:{build:{dependsOn:owner.name==="app"?["^build"]:[]}}},files:[owner.manifest,...owner.name==="app"?["library/api.ts"]:[]],dependencies:owner.name==="app"?["lib"]:[]};});
   const direct=(row.selection as {excludeDependencies?:true}).excludeDependencies===true,closure=db.query("WITH RECURSIVE reach(name) AS (SELECT 'app' UNION SELECT edge.target FROM edge JOIN reach ON edge.source=reach.name WHERE ?1=0) SELECT name FROM reach ORDER BY name").all(direct?1:0).map(found=>(found as {name:string}).name);
   expect(admissionValid(admitted),row.name+JSON.stringify(admissionValid.errors)).toBe(true);expect(admitted.owners,row.name).toEqual(row.owners);expect(admitted.owners,row.name).toEqual(closure);expect(admitted.targets,row.name).toEqual(row.targets);if(direct)expect([...new Set(observed)],row.name).toEqual(["app"]);
  }
  expect(()=>readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:"app",target:"build"}],excludeDependencies:false})).toThrow();
  expect(buildGraphScopeKeyV1(readBuildGraphSelectionV1(corpus.cases[0]!.selection),registry)).not.toBe(buildGraphScopeKeyV1(readBuildGraphSelectionV1(corpus.cases[4]!.selection),registry));
 }finally{db.close();}
});

test("the authored workspace registry has one identity per physical owner",async()=>{
 const registry=JSON.parse(await Bun.file(new URL("../../../../../../../../../📋️build-owners.json",import.meta.url)).text()),db=new Database(":memory:");
 try{db.exec("CREATE TABLE owner(name TEXT PRIMARY KEY,root TEXT UNIQUE,manifest TEXT UNIQUE)");for(const owner of registry.owners)db.query("INSERT INTO owner VALUES(?,?,?)").run(owner.name,owner.root,owner.manifest);expect(readBuildOwnerRegistryV1(registry).owners.length).toBe((db.query("SELECT COUNT(*) AS count FROM owner").get() as {count:number}).count);}finally{db.close();}
});

test("direct project inference retains native source observations and infers only admitted owners",async()=>{
 const corpus=fixture.directTaskAdmission,registry=readBuildOwnerRegistryV1(corpus.registry),db=new Database(":memory:");
 try{
  const rows=db.query("SELECT value AS manifest,substr(value,1,instr(value,'/')-1) AS root FROM json_each(?) ORDER BY manifest").all(JSON.stringify(corpus.nativeManifests)) as {manifest:string;root:string}[];
  for(const row of corpus.cases){
   const selection=buildInvocationGraphSelectionV1(row.args);
   const admitted=await closeBuildOwnerAdmissionV1(selection,registry,async owner=>({
    project:{name:owner.name,targets:{build:{dependsOn:owner.name==="app"?["^build"]:[]}}},
    files:[owner.manifest,...owner.name==="app"?corpus.nativeManifests:[]],
    dependencies:owner.name==="app"?["lib"]:[],
   }));
   const options=buildGraphProjectInferenceOptionsV1(selection,registry,admitted);
   expect(options.ownerAdmission,row.name).toBe(admitted);
   expect(admitted.files.filter(path=>path.endsWith("/Cargo.toml")),row.name).toEqual(rows.map(({manifest})=>manifest));
   const direct=selection.kind==="tasks"&&selection.excludeDependencies===true;
   const roots=db.query("SELECT json_extract(owners.value,'$.root') AS root FROM json_each(?) AS owners JOIN json_each(?) AS selected ON selected.value=json_extract(owners.value,'$.name') ORDER BY root").all(JSON.stringify(corpus.registry.owners),JSON.stringify(row.owners)) as {root:string}[];
   expect(options.ownerRoots&&[...options.ownerRoots].sort(),row.name).toEqual(direct?roots.map(({root})=>root):undefined);
   expect(rows.filter(({root})=>options.ownerRoots&&!options.ownerRoots.has(root)).map(({manifest})=>manifest),row.name).toEqual(direct?["library/Cargo.toml"]:[]);
  }
 }finally{db.close();}
});


test("owners reached only through file edges are admitted ownership-only and observed once an actual edge needs them",async()=>{
 const owner=(name:string)=>({name,root:name,manifest:name+"/📋️project.json",packages:[]}),registry=readBuildOwnerRegistryV1({version:1,owners:["app","hub","leaf","dep"].map(owner),catalogProviders:[]}),calls:string[]=[];
 const run=(dependencies:string[])=>closeBuildOwnerAdmissionV1({kind:"owners",projects:["app"],exclude:[]},registry,async(row)=>{calls.push(row.name);return{project:{name:row.name,targets:{}},files:[row.manifest,...row.name==="app"?["hub/a.ts","leaf/b.ts"]:row.name==="hub"?["leaf/c.ts"]:[]],dependencies:row.name==="app"?dependencies:[]};});
 const only=await run([]);
 expect(calls).toEqual(["app"]);expect(only.owners).toEqual(["app","hub","leaf"]);expect(only.files).toEqual(["app/📋️project.json","hub/a.ts","hub/📋️project.json","leaf/b.ts","leaf/📋️project.json"]);
 calls.length=0;const promoted=await run(["hub"]);
 expect(calls.sort()).toEqual(["app","hub"]);expect(promoted.owners).toEqual(["app","hub","leaf"]);expect(promoted.files).toContain("leaf/c.ts");expect(promoted.files).toContain("leaf/📋️project.json");
});
