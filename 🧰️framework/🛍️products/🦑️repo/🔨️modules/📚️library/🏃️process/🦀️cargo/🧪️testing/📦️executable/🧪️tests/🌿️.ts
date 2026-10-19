import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join,resolve,relative,isAbsolute} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

const text=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("cargo-executable.ts",text,ts.ScriptTarget.Latest,true);
const functions=(names:string[])=>ts.transpileModule(tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&names.includes(node.name?.text??"")).map(node=>node.getText(tree).replace(/^export\s+/,"")).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
const consumer=(value:Record<string,unknown>)=>Object.fromEntries(Object.entries(value).map(([key,value])=>[key,value===null?undefined:value]));

test("Cargo test runtime projection agrees with shared vectors and independent SQL and schema oracles",()=>{
 const definitions=schema.definitions,oracle=new Ajv({strict:true,allowUnionTypes:true}),validate=oracle.compile({...definitions.EnvironmentCase,definitions}),runtime=oracle.compile({...definitions.RuntimeEnvironment,definitions}),database=new Database(":memory:");
 const query=`with runtime(name) as (values ('SEMIO_TEST_CLI'),('SEMIO_TEST_ARTIFACT_DIR'),('SEMIO_KEEP_WORKSPACES'),('SEMIO_TYPE_BUDGET_MS')), matched as (select r.name,c.key,c.value,c.type,row_number() over(partition by r.name order by c.key=r.name desc,c.key) position,sum(c.key=r.name) over(partition by r.name) canonical from json_each(?1) c join runtime r on (case when ?2 then upper(c.key) else c.key end)=r.name) select * from matched`;
 try{for(const row of fixture.environment){
  expect(validate(row),JSON.stringify(validate.errors)).toBe(true);const mapped=database.query(query).all(JSON.stringify(row.consumer),row.windows?1:0)as{name:string,key:string,value:unknown,type:string,position:number,canonical:number}[];
  const ambiguous=database.query(`select name from (${query}) where canonical=0 group by name having count(distinct type||':'||quote(value))>1`).all(JSON.stringify(row.consumer),row.windows?1:0).length>0;
  const selected=Object.fromEntries(mapped.filter(value=>value.position===1&&value.type!=="null").map(value=>[value.name,value.value])),accepted=!ambiguous&&runtime(selected);expect(accepted,row.name).toBe(row.accepted);
  const project=new Function("schema","validateJsonSchemaSubset","process",functions(["cargoTestRuntimeEnvironmentV1"])+";return cargoTestRuntimeEnvironmentV1;")(schema,validateJsonSchemaSubset,{platform:row.windows?"win32":"linux"}),baseline=Object.freeze({...row.compiler}),provided=Object.freeze(consumer(row.consumer));
  if(!row.accepted){expect(()=>project(baseline,provided),row.name).toThrow("runtime environment");continue;}
  const protectedRows=database.query("select key,value from json_each(?1) where (case when ?2 then upper(key) else key end) not in ('SEMIO_TEST_CLI','SEMIO_TEST_ARTIFACT_DIR','SEMIO_KEEP_WORKSPACES','SEMIO_TYPE_BUDGET_MS')").all(JSON.stringify(row.compiler),row.windows?1:0)as{key:string,value:string}[];
  const independent={...Object.fromEntries(protectedRows.map(value=>[value.key,value.value])),...selected},actual=project(baseline,provided);expect(actual,row.name).toEqual(row.expected);expect(actual).toEqual(independent);expect(baseline).toEqual(row.compiler);
 }}finally{database.close();}
 console.log(`[DEBUG] actual Cargo runtime projection matched ${fixture.environment.length} SQL/Ajv cases, preserving compiler authority and Windows selector uniqueness`);
});

test("actual Cargo test runner retains current consumer selection and authoritative compiler custody",async()=>{
 const row=fixture.environment[0]!,workspace=resolve("fixture-workspace"),storeRoot=join(workspace,".cache/b/r/m"),product=join(storeRoot,"selected"),executable=join(storeRoot,"test-cli/selected"),invocation=Object.freeze({testOnly:true}),control=Object.freeze({signal:new AbortController().signal});
 for(const provided of [true,false]){
  let held=false,executions=0,rechecks=0;const commands:Record<string,any>[]=[];
  const ports={schema,validateJsonSchemaSubset,createHash,dirname,join,resolve,relative,isAbsolute,process:{platform:"linux",env:{...row.compiler,SEMIO_BUILD_MODE:"prototype"}},
   readBuildMode:(value:string)=>value,buildModePolicy:()=>({cargoProfile:"prototype"}),compilerAdmissionBudgetV1:()=>0,scriptCompilerAdmissionControlV1:(received:unknown)=>{expect(received).toBe(invocation);return control;},repositoryCargoNetworkV1:(received:unknown)=>{expect(received).toBe(invocation);return{offline:true};},
   scriptInvocationEnvironment:(received:unknown,env:Record<string,unknown>)=>{expect(received).toBe(invocation);return{...env,SEMIO_SCRIPT_PROCESS_INVOCATION:"port-current-original"};},
   withRepositoryCompilerStoreV1:async(_root:unknown,_mode:unknown,_control:unknown,_environment:unknown,body:Function)=>body({root:storeRoot,environment:{...row.compiler},sourceBoundary:{maxSourceBytes:1024},storage:{allocation:{bytes:1024},recheck:async()=>{rechecks++;}},buildDirectory:join(storeRoot,"build")},control),
   lstatSync:(path:string)=>({isDirectory:()=>path!==product&&!path.endsWith("Cargo.toml"),isFile:()=>path===product||path.endsWith("Cargo.toml"),isSymbolicLink:()=>false,nlink:1,size:8}),
   withRepositoryCargoInputsV1:async(options:any,body:Function)=>{expect(options.invocation).toBe(invocation);return body({environment:options.environment,selections:[{arguments:options.args}],control});},
   withCompilerWorkspaceWriteV1:async(_storage:unknown,_directory:unknown,_control:unknown,body:Function)=>body(),acquireCompilerCapacityLeaseV1:async()=>({parallelism:7,release:()=>{}}),admitCargoSelectionV1:async()=>{},acquireCargoBuildLeaseV1:async()=>({release:()=>{}}),requireNativeStoragePathV1:(_root:unknown,path:string)=>path,
   pinExecutableArtifact:async()=>executable,checkResourceLeaseControl:()=>{},withResourceLeases:async(options:any,body:Function)=>{expect(options.signal).toBe(control.signal);held=true;try{return await body();}finally{held=false;}},
   runOwnedCommand:async(command:string,_args:unknown,_cwd:unknown,stage:string,_budget:unknown,options:any)=>{commands.push({command,stage,env:options.env});expect(options.signal).toBe(control.signal);if(stage==="cargo:test-executable-build")options.onLine(JSON.stringify({reason:"compiler-artifact",target:{name:fixture.request.test,kind:["test"]},executable:product}),"stdout");else{expect(held).toBe(true);executions++;}},
  };
  const run=new Function(...Object.keys(ports),functions(["readCargoExecutableTestRequestV1","selectCargoTestExecutableV1","cargoTestRuntimeEnvironmentV1","runRepositoryCargoTestExecutableV1"])+";return runRepositoryCargoTestExecutableV1;")(...Object.values(ports));
  if(provided)await run(fixture.request,workspace,invocation,consumer(row.consumer));else await run(fixture.request,workspace,invocation);
  expect(executions).toBe(1);expect(rechecks).toBeGreaterThanOrEqual(2);expect(held).toBe(false);
  const build=commands.find(command=>command.stage==="cargo:test-executable-build")!.env,selected=commands.find(command=>command.stage==="cargo:test-executable")!.env;
  expect(build.SEMIO_TEST_CLI).toBe("stale");expect(selected.SEMIO_TEST_CLI).toBe(provided?"current":undefined);
  for(const key of Object.keys(row.compiler).filter(key=>!schema.definitions.RuntimeKeys.const.includes(key)&&key!=="SEMIO_SCRIPT_PROCESS_INVOCATION"))expect(selected[key],key).toBe((row.compiler as Record<string,string>)[key]);
  expect(selected.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe("port-current-original");expect(selected.UNDECLARED_CONSUMER).toBeUndefined();
 }
 console.log("[DEBUG] actual Cargo runner ports retained the selected test consumer under compiler/artifact custody and reapplied the received invocation identity");
});
