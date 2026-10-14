import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {mkdir,readFile,rm,symlink,writeFile} from "node:fs/promises";
import {join,relative,resolve} from "node:path";

test("Bazel projects the portable task graph with SQL-verified dependency edges and forwarded arguments",async()=>{
 const {planBazelTasksV1}=await import("../📜️script.ts"),plan=planBazelTasksV1(fixture.graph as any,fixture.request),validate=new Ajv({strict:true}).compile(schema);expect(validate(plan),JSON.stringify(validate.errors)).toBe(true);expect(plan.selected).toEqual(fixture.expected.selected);
 const database=new Database(":memory:");try{database.exec("CREATE TABLE edges(source TEXT,target TEXT)");for(const[source,targets]of Object.entries(fixture.expected.dependencies))for(const target of targets)database.query("INSERT INTO edges VALUES (?1,?2)").run(source,target);for(const[id,task]of Object.entries(plan.tasks)){const oracle=database.query("SELECT target FROM edges WHERE source=?1 ORDER BY target").all(id).map(row=>(row as {target:string}).target);expect(task.dependencies).toEqual(oracle);expect(task.args).toEqual(fixture.expected.args[id as keyof typeof fixture.expected.args]);}}
 finally{database.close();}
});

test("Bazel rejects dependency cycles before publishing executable actions",async()=>{
 const {planBazelTasksV1}=await import("../📜️script.ts"),graph=structuredClone(fixture.graph);graph.projects.app.targets.prepare={execution:{kind:"noop"},dependsOn:["test"]} as any;expect(()=>planBazelTasksV1(graph as any,fixture.request)).toThrow("Build task dependency cycle");
});

test("explicit selections retain arguments while dependency project exclusions remain excluded",async()=>{
 const {planBazelTasksV1}=await import("../📜️script.ts"),graph=structuredClone(fixture.graph) as any;graph.projects.app.targets.build.dependsOn=[{target:"build",projects:["*","!app"]}];const plan=planBazelTasksV1(graph,{tasks:["app:build","library:build"],args:["--selected"]});expect(plan.tasks["library:build"]!.args).toEqual(["--selected"]);expect(plan.tasks["app:build"]!.dependencies).toEqual(["library:build"]);
});

test.skipIf(!process.env.SEMIO_TEST_ARTIFACT_DIR)("Bazel source identities follow changed symlinks and terminate directory cycles",async()=>{
 const {bazelSourceIdentityV1}=await import("../📜️script.ts"),directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"bz-source"),row=fixture.sourceIdentity,signal=new AbortController().signal,sha=(value:string)=>new Bun.CryptoHasher("sha256").update(value).digest("hex");await mkdir(directory,{recursive:true});const target=join(directory,row.target);await writeFile(target,row.contents[0]!);expect(await bazelSourceIdentityV1(target,signal)).toBe(sha("file:"+sha(row.contents[0]!)));
 if(process.platform==="win32")return;
 const link=join(directory,row.link);await rm(link,{force:true});await symlink(row.target,link);expect(await bazelSourceIdentityV1(link,signal)).toBe(sha("link:"+row.target+"\0file:"+sha(row.contents[0]!)));const first=await bazelSourceIdentityV1(link,signal);await writeFile(target,row.contents[1]!);expect(await bazelSourceIdentityV1(link,signal)).not.toBe(first);
 const tree=join(directory,row.directory);await mkdir(tree,{recursive:true});await writeFile(join(tree,"value.txt"),row.contents[0]!);await rm(join(tree,row.cycle),{force:true});await symlink(".",join(tree,row.cycle));const before=await bazelSourceIdentityV1(tree,signal);expect(await bazelSourceIdentityV1(tree,signal)).toBe(before);await writeFile(join(tree,"value.txt"),row.contents[1]!);expect(await bazelSourceIdentityV1(tree,signal)).not.toBe(before);
});

test("portable runtime output examples agree with an independent SQLite oracle",()=>{
 const database=new Database(":memory:");try{for(const row of fixture.runtime.sourceCases)expect((database.query("SELECT upper(?1) AS value").get(row.source) as {value:string}).value).toBe(row.output);}finally{database.close();}
});

test.skipIf(process.env.SEMIO_BAZEL_INTEGRATION!=="1")("Bazel executes direct task dependencies and restores cached products after deletion",async()=>{
 const {runBazelTasksV1}=await import("../📜️script.ts"),{withScriptProcessEnvelope,createScriptProcessEnvelope,readScriptPolicy}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),root=resolve(import.meta.dir,"../../../../../../../../.."),directory=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!directory)throw Error("Bazel integration requires ticket artifacts");
 const project=join(directory,"bz-it"),source=join(project,"source.txt"),count=join(project,"count.txt"),product=join(project,"output.txt"),script=join(project,"📜️script.ts"),projectRoot=relative(root,project).replaceAll("\\","/");await mkdir(project,{recursive:true});
 const backend=resolve(import.meta.dir,"../📜️script.ts"),routing=resolve(import.meta.dir,"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 await writeFile(script,`const root=import.meta.dir;if(process.argv[2]==="service"){console.log("[DEBUG] fixture service ready");setInterval(()=>{},1000);}else if(process.argv[2]==="nested"){const {runBazelTasksV1}=await import(${JSON.stringify(backend)}),{receiveScriptProcessInvocation}=await import(${JSON.stringify(routing)});await receiveScriptProcessInvocation(process.env,async invocation=>runBazelTasksV1(process.env.SEMIO_WORKSPACE_ROOT,JSON.parse(await Bun.file(root+"/graph.json").text()),{tasks:["fixture:build"],parallel:2},invocation));}else{const source=await Bun.file(root+"/source.txt").text();let count=0;try{count=Number(await Bun.file(root+"/count.txt").text())}catch{}await Bun.write(root+"/count.txt",String(count+1));await Bun.write(root+"/argv.json",JSON.stringify(process.argv.slice(2)));await Bun.write(root+"/"+process.env.SEMIO_BUILD_MODE+".txt",source.toUpperCase());await Bun.write(root+"/output.txt",source.toUpperCase());console.log("[DEBUG] Bazel fixture executed "+source);}`);await writeFile(source,"first");await rm(count,{force:true});
 const graph={version:1,projects:{fixture:{name:"fixture",root:projectRoot,targets:{prepare:{execution:{kind:"noop"}},build:{execution:{kind:"command",command:"bun ./📜️script.ts",cwd:projectRoot},dependsOn:["prepare"],cache:true,inputs:[projectRoot+"/source.txt",projectRoot+"/📜️script.ts"],outputs:[projectRoot+"/output.txt"]}}}},dependencies:[]} as any;
 const {createNativeBuildCallerCapabilitiesV1}=await import("../../../📥️caller/🟦️.ts");
 await withScriptProcessEnvelope(createScriptProcessEnvelope(readScriptPolicy({version:1,owner:"bazel:integration",maximumElapsedMilliseconds:240000}),createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",process.env),Date.now()),async invocation=>{
  await runBazelTasksV1(root,graph,{tasks:["fixture:build"],parallel:2},invocation);expect(await readFile(product,"utf8")).toBe("FIRST");expect(await readFile(count,"utf8")).toBe("1");await rm(product);
  await runBazelTasksV1(root,graph,{tasks:["fixture:build"],parallel:2},invocation);expect(await readFile(product,"utf8")).toBe("FIRST");expect(await readFile(count,"utf8")).toBe("1");
  await writeFile(source,"second");await runBazelTasksV1(root,graph,{tasks:["fixture:build"],parallel:2},invocation);expect(await readFile(product,"utf8")).toBe("SECOND");expect(await readFile(count,"utf8")).toBe("2");
  graph.projects.fixture.targets.nested={execution:{kind:"command",command:"bun ./📜️script.ts nested",cwd:projectRoot},cache:false};await writeFile(join(project,"graph.json"),JSON.stringify(graph));await rm(product);await runBazelTasksV1(root,graph,{tasks:["fixture:nested"],parallel:2},invocation);expect(await readFile(product,"utf8")).toBe("SECOND");expect(await readFile(count,"utf8")).toBe("2");
  graph.projects.fixture.targets.uncached={...graph.projects.fixture.targets.build,cache:false};await runBazelTasksV1(root,graph,{tasks:["fixture:uncached"]},invocation);await runBazelTasksV1(root,graph,{tasks:["fixture:uncached"]},invocation);expect(await readFile(count,"utf8")).toBe("4");
  graph.projects.fixture.targets.serve={...graph.projects.fixture.targets.uncached,continuous:true};let started=false;await runBazelTasksV1(root,graph,{tasks:["fixture:serve"]},invocation,()=>{started=true;});expect(started).toBe(true);expect(await readFile(count,"utf8")).toBe("5");graph.projects.fixture.targets.service={execution:{kind:"command",command:"bun ./📜️script.ts service",cwd:projectRoot,readyWhen:"[DEBUG] fixture service ready"},continuous:true,parallelism:false,cache:false};graph.projects.fixture.targets.consumer={...graph.projects.fixture.targets.uncached,parallelism:false,dependsOn:["service"]};await runBazelTasksV1(root,graph,{tasks:["fixture:consumer"]},invocation);expect(await readFile(count,"utf8")).toBe("6");graph.projects.fixture.targets.defaults={execution:{kind:"command",command:"bun ./📜️script.ts",cwd:projectRoot,args:["authored"],env:{SEMIO_BUILD_MODE:"prototype"}},cache:false,outputs:[projectRoot+"/{options.env.SEMIO_BUILD_MODE}.txt",projectRoot+"/argv.json"]};await runBazelTasksV1(root,graph,{tasks:["fixture:defaults"],args:["caller"]},invocation);expect(JSON.parse(await readFile(join(project,"argv.json"),"utf8"))).toEqual(["authored","caller"]);expect(await readFile(join(project,"prototype.txt"),"utf8")).toBe("SECOND");console.log("[DEBUG] Bazel cache, source invalidation, nested orchestration, uncached tasks and continuous launch verified");
 });
},250000);
