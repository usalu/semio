/** 🧭️ Contract: Bazel source inputs follow the Nx transitive semantics, stay proportional to the inventory through per-directory filegroups, and refuse dangling task dependencies and unbounded caches with precise reports. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {minimatch} from "minimatch";
import {mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";
import actionsCorpus from "../📥️actions/🔣️.json";
import component from "../../🧬️schema/🔣️.json";

const api=await import("../../📜️script.ts"),python=testPythonV1(),node=Bun.which("node"),link=component.definitions.Actions.const.sources.link;
type Group={groups:readonly string[];files:readonly string[]};
const universe=[...corpus.cover.universe].sort(),order=(left:string,right:string)=>left<right?-1:left>right?1:0;
const expand=(groups:ReadonlyMap<string,Group>,name:string,depth=0):string[]=>{if(depth>64)throw Error("Bazel source group cycle: "+name);const row=groups.get(name);if(!row)throw Error("Bazel source group undefined: "+name);return[...row.files,...row.groups.flatMap(child=>expand(groups,child,depth+1))];};
const run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});

test("Bazel source fixtures are closed",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
});

test("Bazel source index reuses cached pattern matches without leaking between selections and agrees with the one-shot selector",()=>{
 const rows=actionsCorpus.sources.cases,index=api.bazelSourceIndexV1(actionsCorpus.sources.files),selected=(inputs:readonly unknown[])=>{const{marks,extra}=index.selectAll(inputs);return[...new Set([...index.files.filter((_path,position)=>marks[position]),...extra])].sort();};
 for(const row of [...rows,...[...rows].reverse(),...rows]){expect(selected(row.inputs),row.name).toEqual(api.bazelTaskSourcePathsV1(row.inputs,actionsCorpus.sources.files));expect(selected(row.inputs),row.name).toEqual(row.expected);}
});

test("Bazel directory covers expand to exactly the selection and agree with an independent Python computation of the maximal fully selected directories",()=>{
 const tree=api.bazelSourceCoverV1(universe),database=new Database(":memory:"),names=new Set<string>();
 try{
  for(const row of corpus.cover.cases){
   const chosen=new Uint8Array(universe.length);for(const path of row.selected)chosen[universe.indexOf(path)]=1;
   const cover=()=>tree.compose(tree.merge([tree.cover(chosen),{groups:[],files:row.extra}]),"i"),name=cover();expect(cover(),row.name).toBe(name);
   const top:Group=!name?{groups:[],files:[]}:name.startsWith("i_")?tree.groups.get(name)!:{groups:[name],files:[]};
   expect(top.groups.length,row.name).toBe(row.groups);expect([...top.files].sort(order),row.name).toEqual([...row.loose].sort(order));
   const files=name?expand(tree.groups,name).sort(order):[];expect(files,row.name).toEqual([...row.selected,...row.extra].sort(order));
   database.exec("DROP TABLE IF EXISTS member;CREATE TABLE member(parent TEXT,kind TEXT,child TEXT)");
   for(const[parent,group]of tree.groups){for(const child of group.groups)database.query("INSERT INTO member VALUES(?1,'group',?2)").run(parent,child);for(const file of group.files)database.query("INSERT INTO member VALUES(?1,'file',?2)").run(parent,file);}
   const oracle=name?(database.query("WITH RECURSIVE walk(name,depth) AS (SELECT ?1,0 UNION ALL SELECT m.child,w.depth+1 FROM member m JOIN walk w ON m.parent=w.name WHERE m.kind='group' AND w.depth<64) SELECT f.child AS file FROM walk w JOIN member f ON f.parent=w.name AND f.kind='file' ORDER BY f.child").all(name) as {file:string}[]).map(value=>value.file):[];
   expect([...oracle].sort(order),row.name).toEqual(files);
   if(python){
    const result=run("import json,sys\nd=json.loads(sys.stdin.read())\nU=d['universe']\nS=set(d['selected'])\ndirs={''}\nfor f in U:\n    p=f\n    while '/' in p:\n        p=p.rsplit('/',1)[0]\n        dirs.add(p)\ninside=lambda f,dr: dr=='' or f.startswith(dr+'/')\nparent=lambda dr: '' if '/' not in dr else dr.rsplit('/',1)[0]\nfull={dr for dr in dirs if any(inside(f,dr) for f in U) and all(f in S for f in U if inside(f,dr))}\nmaximal=[dr for dr in full if dr=='' or parent(dr) not in full]\ncovered={f for dr in maximal for f in U if inside(f,dr)}\nprint(json.dumps({'groups':sorted(sorted(f for f in U if inside(f,dr)) for dr in maximal),'loose':sorted([f for f in S if f not in covered]+d['extra'])}))",{universe,selected:row.selected,extra:row.extra});
    expect(result.exitCode,result.stderr.toString()).toBe(0);const independent=JSON.parse(result.stdout.toString());
    expect(top.groups.map(group=>expand(tree.groups,group).sort(order)).sort((left,right)=>order(left.join("\0"),right.join("\0"))),row.name).toEqual(independent.groups);expect([...top.files].sort(order),row.name).toEqual(independent.loose);
   }
  }
  for(const[name,group]of tree.groups){names.add(name);expect(name).toMatch(/^[dik]_[0-9a-f]{16}$/u);for(const child of group.groups)expect(tree.groups.has(child),child).toBe(true);expect(group.groups.length+group.files.length,name).toBeGreaterThan(0);}
  expect(names.size).toBe(tree.groups.size);
 }finally{database.close();}
},60_000);

test.skipIf(!python)("Bazel renders nested directory groups as labels before files and an independent Starlark parse reads back the same structure",()=>{
 const tree=api.bazelSourceCoverV1(universe),chosen=new Uint8Array(universe.length);for(const path of corpus.cover.cases[2]!.selected)chosen[universe.indexOf(path)]=1;
 const name=tree.compose(tree.cover(chosen),"i");expect(name).toBeDefined();
 const text=api.renderBazelBuildV1({actions:[],groups:tree.groups,bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:""}).replace(/^load\(.*$/mu,""),
  result=run("import ast,json,sys\nrows={}\nfor node in ast.parse(json.loads(sys.stdin.read())).body:\n    call=node.value\n    rows[[k.value.value for k in call.keywords if k.arg=='name'][0]]=[e.value for k in call.keywords if k.arg=='srcs' for e in k.value.elts]\nprint(json.dumps(rows))",text);
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 expect(JSON.parse(result.stdout.toString())).toEqual(Object.fromEntries([...tree.groups].map(([group,row])=>[group,[...row.groups.map(child=>":"+child),...row.files.map(path=>api.bazelSourceLabelV1(path))]])));
},60_000);

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,environment=(extra:Record<string,string>={})=>({...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{}),...extra});

test.skipIf(!binary||process.platform==="win32")("the real Bazel resolves nested directory groups to exactly the covered files",()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-cover-")),checkout=join(directory,"checkout"),workspace=join(directory,"workspace"),base=join(directory,"base"),install=join(directory,"install"),
  bazel=(args:string[])=>spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=60",...args],{cwd:workspace,encoding:"utf8",timeout:240000,env:environment()});
 try{
  for(const path of universe){mkdirSync(dirname(join(checkout,path)),{recursive:true});writeFileSync(join(checkout,path),path+"\n");}
  mkdirSync(workspace);writeFileSync(join(workspace,"MODULE.bazel"),'module(name="cover")\n');symlinkSync(checkout,join(workspace,link),"dir");
  const tree=api.bazelSourceCoverV1(universe),expected=new Map<string,string[]>();
  for(const row of corpus.cover.cases){const chosen=new Uint8Array(universe.length);for(const path of row.selected)chosen[universe.indexOf(path)]=1;const name=tree.compose(tree.cover(chosen),"i");if(name)expected.set(name,row.selected.map(path=>`//:${link}/${path}`).sort(order));}
  writeFileSync(join(workspace,"BUILD.bazel"),api.renderBazelBuildV1({actions:[],groups:tree.groups,bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:""}).replace(/^load\(.*$/mu,""));
  for(const[name,files]of expected){const query=bazel(["query","--lockfile_mode=off",`kind('source file', deps(//:${name}))`]);expect(query.status,query.stderr).toBe(0);expect(query.stdout.split("\n").filter(Boolean).sort(order)).toEqual(files);}
 }finally{spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:workspace,timeout:120000});rmSync(directory,{recursive:true,force:true});}
},600000);

test("Bazel source units carry only the exclusions that can overlap their include, so projects share them, and still select exactly what minimatch selects task-wide",()=>{
 const{files,cases,shared}=corpus.keys,index=api.bazelSourceIndexV1(files);
 for(const row of cases){
  const{keys,extra}=index.keys(row.inputs);expect(keys,row.name).toEqual(row.keys);expect(extra,row.name).toEqual(row.extra);
  const{marks,extra:again}=index.selectAll(row.inputs),actual=index.files.filter((_path,position)=>marks[position]);expect(again,row.name).toEqual(row.extra);
  if(row.oracle){const strings=row.inputs as string[],include=strings.filter(value=>!value.startsWith("!")),exclude=strings.filter(value=>value.startsWith("!")).map(value=>value.slice(1));expect(actual,row.name).toEqual(files.filter(path=>include.some(pattern=>minimatch(path,pattern))&&!exclude.some(pattern=>minimatch(path,pattern))).sort());}
 }
 for(const row of shared){const unit=(inputs:unknown[])=>JSON.stringify(index.keys(inputs).keys.find(key=>key.include===row.include));expect(unit(row.first),row.name).toBe(unit(row.second));expect(unit(row.first)).toBeDefined();}
});

const projectOf=(graph:{projects:Record<string,{root:string}>},pattern:string)=>Object.entries(graph.projects).find(([,project])=>pattern.startsWith(project.root+"/"))![0];

test("Bazel caret inputs reach the transitive dependency closure and agree with a SQLite recursive closure",()=>{
 const{graph,cases}=corpus.transitive,database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE edge(source TEXT,target TEXT)");for(const edge of graph.dependencies)database.query("INSERT INTO edge VALUES(?1,?2)").run(edge.source,edge.target);
  for(const row of cases){
   const task={project:row.project,definition:{inputs:row.inputs}} as any;
   if("error"in row){expect(()=>api.bazelTaskInputsV1(graph as any,task),row.id).toThrow(row.error);continue;}
   const actual=api.bazelTaskInputsV1(graph as any,task);expect(actual,row.id).toEqual(row.expected);
   const closure=(database.query("WITH RECURSIVE reach(name) AS (SELECT target FROM edge WHERE source=?1 UNION SELECT e.target FROM edge e JOIN reach r ON e.source=r.name) SELECT name FROM reach WHERE name!=?1 ORDER BY name").all(row.project) as {name:string}[]).map(value=>value.name);
   const owners=[...new Set((actual as string[]).map(pattern=>projectOf(graph as any,pattern)))].sort();
   expect(owners,row.id).toEqual([...closure,...(row.inputs as unknown[]).includes("default")?[row.project]:[]].sort());
  }
 }finally{database.close();}
},60_000);

const nxRoot=(()=>{try{return dirname(createRequire(import.meta.url).resolve("nx/package.json"));}catch{return undefined;}})(),nxProgram="const input=JSON.parse(require('fs').readFileSync(0,'utf8')),nx=input.nx,native=require(nx+'/dist/src/native'),{transformProjectGraphForRust}=require(nx+'/dist/src/native/transform-objects'),out=[];"+
 "for(const row of input.cases){const nodes={},dependencies={};for(const[name,project]of Object.entries(input.graph.projects)){nodes[name]={name,type:'lib',data:{root:project.root,targets:{t:{executor:'nx:noop',inputs:name===row.project?row.inputs:['default']}},...(project.namedInputs?{namedInputs:project.namedInputs}:{})}};dependencies[name]=[];}"+
 "for(const edge of input.graph.dependencies)dependencies[edge.source].push({source:edge.source,target:edge.target,type:'static'});"+
 "try{const planner=new native.HashPlanner({namedInputs:{}},native.transferProjectGraph(transformProjectGraphForRust({nodes,dependencies,externalNodes:{}}))),id=row.project+':t',graph={roots:[id],tasks:{[id]:{id,target:{project:row.project,target:'t'},outputs:[],overrides:{},parallelism:true,cache:true}},dependencies:{[id]:[]},continuousDependencies:{[id]:[]}};"+
 "const lines=planner.getPlans([id],graph)[id].filter(line=>{const cut=line.indexOf(':');return cut>0&&Object.hasOwn(nodes,line.slice(0,cut))&&!['ProjectConfiguration','TsConfig'].includes(line.slice(cut+1));});out.push({id:row.id,plan:lines.sort()});}catch(error){out.push({id:row.id,error:String(error.message)});}}"+
 "process.stdout.write(JSON.stringify(out));";

test.skipIf(!nxRoot||!node)("Bazel caret inputs name the same projects and patterns as Nx's own native hash planner",()=>{
 const{graph,cases}=corpus.transitive,result=spawnSync(node!,["-e",nxProgram],{input:JSON.stringify({nx:nxRoot,graph,cases}),encoding:"utf8",maxBuffer:1<<26});
 expect(result.status,result.stderr).toBe(0);
 const planned=new Map((JSON.parse(result.stdout) as {id:string;plan?:string[];error?:string}[]).map(row=>[row.id,row]));
 for(const row of cases){
  const reference=planned.get(row.id)!;
  if("error"in row){expect(reference.error,row.id).toMatch(/not defined/u);continue;}
  expect(reference.error,row.id).toBeUndefined();
  const actual=api.bazelTaskInputsV1(graph as any,{project:row.project,definition:{inputs:row.inputs}} as any) as string[];
  expect(actual.map(pattern=>projectOf(graph as any,pattern)+":"+pattern).sort(),row.id).toEqual(reference.plan!);
 }
});

test("Bazel refuses dangling task dependencies with the depending task, the declaration and the missing half, and lists them all at once",()=>{
 const{graph,expected,refusals,accepted}=corpus.dangling,database=new Database(":memory:");
 try{
  expect(api.bazelDanglingDependenciesV1(graph as any)).toEqual(expected as {task:string;dependency:string;missing:"target"|"project"}[]);
  database.exec("CREATE TABLE target(project TEXT,name TEXT);CREATE TABLE declared(task TEXT,dependency TEXT)");
  for(const[project,owner]of Object.entries(graph.projects)){for(const name of Object.keys(owner.targets))database.query("INSERT INTO target VALUES(?1,?2)").run(project,name);for(const[name,definition]of Object.entries(owner.targets))for(const row of [definition,...Object.values((definition as {configurations?:Record<string,unknown>}).configurations??{})])for(const entry of ((row as {dependsOn?:(string|{target:string})[]}).dependsOn??[])){const dependency=(typeof entry==="string"?entry:entry.target).replace(/^\^/u,"");if(dependency.includes(":"))database.query("INSERT INTO declared VALUES(?1,?2)").run(project+":"+name,dependency);}}
  const oracle=database.query("SELECT d.task AS task, d.dependency AS dependency, CASE WHEN NOT EXISTS(SELECT 1 FROM target t WHERE t.project=substr(d.dependency,1,instr(d.dependency,':')-1)) THEN 'project' ELSE 'target' END AS missing FROM declared d WHERE NOT EXISTS(SELECT 1 FROM target t WHERE t.project||':'||t.name=d.dependency) ORDER BY d.task,d.dependency").all();
  expect(oracle).toEqual(expected);
  for(const row of refusals){const candidate=structuredClone(graph) as any;candidate.projects.app.targets.build.dependsOn=row.dependsOn;expect(()=>api.planBazelTasksV1(candidate,{tasks:["app:build"]}),row.name).toThrow(row.message);}
  for(const dependsOn of accepted){const candidate=structuredClone(graph) as any;candidate.projects.app.targets.build.dependsOn=dependsOn;expect(()=>api.planBazelTasksV1(candidate,{tasks:["app:build"]}),JSON.stringify(dependsOn)).not.toThrow();}
 }finally{database.close();}
});

test("Bazel reports every unbounded cacheable declaration per owning project and agrees with an independent SQLite aggregation",()=>{
 const{graph,request,expected}=corpus.uncached,plan=api.planBazelTasksV1(graph as any,request),emitted=api.bazelActionsV1(graph as any,plan,false),report=api.bazelUncachedReportV1(plan,emitted),database=new Database(":memory:"),admit=new Ajv({strict:true}).addSchema(component,"component").compile({$ref:"component#/definitions/UncachedReport"});
 try{
  expect(report).toEqual(expected as typeof report);expect(admit(report),JSON.stringify(admit.errors)).toBe(true);
  database.exec("CREATE TABLE decision(project TEXT,target TEXT,cached INTEGER,reason TEXT)");
  for(const action of emitted.values()){const task=plan.tasks[action.id]!;database.query("INSERT INTO decision VALUES(?1,?2,?3,?4)").run(task.project,task.target,Number(action.cache.cache),action.cache.reason??null);}
  expect((database.query("SELECT COUNT(*) AS n FROM decision WHERE cached=1").get() as {n:number}).n).toBe(report.cached);
  expect(Object.fromEntries((database.query("SELECT reason,COUNT(*) AS n FROM decision WHERE cached=0 GROUP BY reason ORDER BY reason").all() as {reason:string;n:number}[]).map(row=>[row.reason,row.n]))).toEqual(report.uncached);
  expect((database.query("SELECT project,target FROM decision WHERE reason='unbounded' ORDER BY project,target").all() as {project:string;target:string}[]).map(row=>row.project+":"+row.target)).toEqual(report.unbounded.flatMap(owner=>owner.targets.map(row=>owner.project+":"+row.target)).sort());
  expect(admit({...report,uncached:{...report.uncached,because:1}})).toBe(false);expect(admit({...report,unbounded:[{project:"x",root:"x",targets:[]}]})).toBe(false);
  for(const action of emitted.values())expect(report.unbounded.some(owner=>owner.targets.some(row=>plan.tasks[action.id]!.project===owner.project&&row.target===plan.tasks[action.id]!.target)),action.id).toBe(action.cache.reason==="unbounded");
 }finally{database.close();}
},60_000);
