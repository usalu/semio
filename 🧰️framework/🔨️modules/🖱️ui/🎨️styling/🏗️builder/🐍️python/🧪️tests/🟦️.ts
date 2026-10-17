/** 🧪️ Python: 6 tests covering neutral Python wheel intent and portable names agree with Ajv and SQLite; neutral actual Python wheel compiler and consumers retain genuine custody; neutral actual Python Source contract keeps old porta… */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{readStylingPythonRequestV1,stylingPythonEnvironmentV1}from"../📋️contract/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
/** 🧬️ Keeps Python producer intent finite and independently schema checked. */
test("neutral Python wheel intent and portable names agree with Ajv and SQLite",()=>{const ajv=new Ajv({strict:true}),validate=ajv.compile(schema.definitions.Request),wheel=ajv.compile(schema.definitions.WheelName),db=new Database(":memory:");try{db.exec("create table allowed(operation text);insert into allowed values('deps'),('build'),('test')");for(const row of fixture.valid){expect(row).toEqual(readStylingPythonRequestV1(row));expect(validate(row)).toBe(true);expect(validateJsonSchemaSubset(schema.definitions.Request,row)).toEqual([]);expect(db.query("select count(*) as count from allowed where operation=?").get(row.operation)).toEqual({count:1});}for(const row of fixture.invalid){expect(()=>readStylingPythonRequestV1(row)).toThrow();expect(validate(row)).toBe(false);expect(validateJsonSchemaSubset(schema.definitions.Request,row).length).toBeGreaterThan(0);}for(const row of fixture.wheels){expect(wheel(row.name)).toBe(row.accept);expect(validateJsonSchemaSubset(schema.definitions.WheelName,row.name).length===0).toBe(row.accept);}}finally{db.close();}});
/** 🐍️ Requires actual paid stores and post-CPU consumer placement at the public wheel producer. */
test("neutral actual Python wheel compiler and consumers retain genuine custody",()=>{const base=join(process.cwd(),"🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder"),text=readFileSync(join(base,"🐍️python/📜️script.ts"),"utf8")+readFileSync(join(base,"🐍️python/🟦️.ts"),"utf8"),source=ts.createSourceFile("python.ts",text,ts.ScriptTarget.Latest,true),names=new Set<string>();function visit(node:ts.Node):void{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))names.add(node.expression.text);ts.forEachChild(node,visit);}visit(source);expect(names.has("acquireCompilerCapacityLeaseV1")).toBe(false);expect(names.has("withCompilerCapacityConsumptionV1")).toBe(true);expect(names.has("readNativeStorageReservationPolicyV1")).toBe(true);expect(text).not.toContain("SEMIO_STYLING_PYTHON_BUILD_ROOT");expect(text).not.toContain("SEMIO_COMPILER_CAPACITY_HELD");expect(text).not.toContain("readProcessOwnerContextV1");expect(text).not.toContain("requestedParallelism");expect(names.has("stageRepositoryArtifacts")).toBe(true);const consumer=readFileSync(join(base,"🟦️.ts"),"utf8");expect(consumer).toContain("withStylingPythonWheelV1");expect(consumer).not.toContain('join(root, "dist/build")');});

/** 🧾️ Retains the earlier actual manifest, output and compiler-import vectors without fake Native orchestration. */
test("neutral actual Python Source contract keeps old portable wheel vectors",async()=>{const{testStylingPythonSourceContractV1}=await import("../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🐍️styling-python-outputs/🟦️.ts"),{parse}=await import("@iarna/toml"),text=readFileSync(join(process.cwd(),"🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/pyproject.toml"),"utf8");expect(Bun.TOML.parse(text)).toEqual(parse(text));expect(new Ajv({strict:true}).validate(schema.definitions.Manifest,Bun.TOML.parse(text))).toBe(true);await testStylingPythonSourceContractV1(process.cwd());},30000);

/** 🔍️ Requires the complete bounded environment observer before a readiness digest can be consumed. */
test("neutral Python environment readiness requires bounded complete file inventory",()=>{const text=readFileSync(join(process.cwd(),"🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/🟦️.ts"),"utf8");expect(text).toContain("observeStylingPythonEnvironmentV1");expect(text).toContain("environmentSha256");});

import{mkdtemp,mkdir,writeFile,rm,symlink}from"node:fs/promises";
import{fileURLToPath}from"node:url";
import{observeStylingPythonEnvironmentV1}from"../📋️inventory/🟦️.ts";
import{receiveScriptProcessInvocation,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1,scriptInvocationEnvironment}from"@semio-tech/process/routing/invocation/process";
import{scriptCompilerAdmissionControlV1}from"@semio-tech/process/compiler-capacity";
import{withCompilerCapacityConsumptionV1}from"@semio-tech/process/build/capacity/scope/consumption";
import{withResourceLeases,resourceLeasePathV1}from"@semio-tech/process/leases";
import{captureOwnedCommandResultV1}from"@semio-tech/process/owned-execution/capture";
import { buildCapacitySchema as capacitySchema } from "@semio-tech/process/build/capacity/schema";
/** 🔒️ Uses only real received controls, ordinary files, owned children and SQLite gates. */
test("actual Python environment inventory and post-CPU consumer retain complete ordinary evidence",()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:python-wheel-custody",30000),output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Actual ticket output required");const root=await mkdtemp(join(output,"p-")),environment=join(root,"environment"),namespace=root,python=process.platform==="win32"?"python":"python3",limits={maximumBytes:65536,maximumLines:128};
 const rejectExternal=async()=>{throw Error("Unknown external interpreter DATA");};
 try{
  await mkdir(join(environment,"lib"),{recursive:true});await writeFile(join(environment,"config.json"),'{}');await writeFile(join(environment,"lib","module.py"),'value = 7\n');
  const observed=await observeStylingPythonEnvironmentV1(environment,control,rejectExternal);expect(new Ajv({strict:true}).validate(schema.definitions.Inventory,observed)).toBe(true);
  const oracle=await captureOwnedCommandResultV1(python,["-c",`import os,sys,stat,hashlib,json,sqlite3
root=sys.argv[1];rows=[];files=0;entries=0;size=0
for directory,dirs,names in os.walk(root):
 relative=os.path.relpath(directory,root);relative='' if relative=='.' else relative.replace(os.sep,'/')
 rows.append([relative,'directory',os.lstat(directory).st_mode]);entries+=1
 for name in names:
  path=os.path.join(directory,name);data=open(path,'rb').read();files+=1;entries+=1;size+=len(data)
  rows.append([os.path.relpath(path,root).replace(os.sep,'/'),'file',os.lstat(path).st_mode,len(data),hashlib.sha256(data).hexdigest()])
rows.sort(key=lambda row:json.dumps(row,separators=(',',':'),ensure_ascii=False))
db=sqlite3.connect(':memory:');db.execute('create table inventory(files integer,entries integer,bytes integer)');db.execute('insert into inventory values(?,?,?)',(files,entries,size));counts=db.execute('select files,entries,bytes from inventory').fetchone();db.close()
print(json.dumps(dict(version=1,sha256=hashlib.sha256(json.dumps(rows,separators=(',',':'),ensure_ascii=False).encode()).hexdigest(),files=counts[0],entries=counts[1],bytes=counts[2])))`,environment],process.cwd(),{control,output:limits});expect(oracle.status,oracle.stderr).toBe(0);expect(oracle.signal).toBeNull();expect(observed).toEqual(JSON.parse(oracle.stdout));
  await writeFile(join(environment,"lib","module.py"),'value = 8\n');const changed=await observeStylingPythonEnvironmentV1(environment,control,rejectExternal);expect(changed.sha256).not.toBe(observed.sha256);expect(changed.files).toBe(observed.files);
  if(process.platform!=="win32"){await symlink("lib/module.py",join(environment,"module-link"));const linked=await observeStylingPythonEnvironmentV1(environment,control,rejectExternal);expect(linked.entries).toBe(changed.entries+1);await writeFile(join(root,"foreign.py"),"foreign");await symlink("../foreign.py",join(environment,"foreign-link"));await expect(observeStylingPythonEnvironmentV1(environment,control,rejectExternal)).rejects.toThrow("Unknown external interpreter DATA");}
  await expect(observeStylingPythonEnvironmentV1(environment,{...control},rejectExternal)).rejects.toThrow("Live Original resource control issuance required");
  const cpu=resourceLeasePathV1(namespace,capacitySchema.properties.resource.const),input=resourceLeasePathV1(namespace,"python:test:input"),artifact=resourceLeasePathV1(namespace,"python:test:output"),probe=async(path:string)=>{const row=await captureOwnedCommandResultV1(python,["-c","import sqlite3,sys\nd=sqlite3.connect(sys.argv[1],timeout=0)\ntry:\n d.execute('begin exclusive');print('free');d.rollback()\nexcept sqlite3.OperationalError as e:\n if 'locked' not in str(e):raise\n print('busy')\nfinally:d.close()",path],process.cwd(),{control,output:limits});expect(row.status,row.stderr).toBe(0);expect(row.signal).toBeNull();return row.stdout.trim()==="busy";};let actual={...fixture.lifetime,compileBusy:false,consumerFree:false,inputHeld:false,outputHeld:false,drained:false,freeAfter:false};
  await withResourceLeases({directory:namespace,control,signal:control.signal,resources:[{resource:"python:test:input",mode:"shared"},{resource:"python:test:output",mode:"shared"}]},()=>withCompilerCapacityConsumptionV1({request:{kind:"whole-host"},directory:namespace,control,environment:{}},async(_facts,cpuControl)=>{actual.compileBusy=await probe(cpu);const child=await captureOwnedCommandResultV1("node",["-e","const fs=require('node:fs');console.log(fs.readFileSync(process.argv[1],'utf8'))",join(environment,"lib/module.py")],root,{control:cpuControl,output:limits});expect(child.status,child.stderr).toBe(0);expect(child.stdout.trim()).toBe("value = 8");return ()=>withRequestedIssuedResourceControlV1(control,0,async actualControl=>{actual.consumerFree=!await probe(cpu);actual.inputHeld=await probe(input);actual.outputHeld=await probe(artifact);void withIssuedResourceOperationV1(actualControl,async()=>{await actualControl.yieldContinuation();actual.drained=true;});});}));actual.freeAfter=!await probe(cpu)&&!await probe(input)&&!await probe(artifact);expect(actual).toEqual(fixture.lifetime);expect(new Ajv({strict:true}).validate(schema.definitions.Lifetime,actual)).toBe(true);
  for(const kind of["consumer-error","consumer-undefined"]){const childPath=fileURLToPath(new URL("../../../../../🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/📜️script.ts",import.meta.url)),result=await captureOwnedCommandResultV1(process.execPath,[childPath,namespace,kind],process.cwd(),{control,env:scriptInvocationEnvironment(original,process.env),output:limits});expect(result.status).toBe(1);expect(result.signal).toBeNull();expect(JSON.parse(result.stdout)).toEqual({version:1,kind,exact:true,late:true,refused:true});expect(await probe(cpu)).toBe(false);}
 }finally{await rm(root,{recursive:true});}
}),30000);

/** 📥️ Captures closed own primitive requests and strips all inherited tool configuration at birth. */
test("neutral Python operation DATA cannot replay getters or ambient tool configuration",()=>{let reads=0;expect(()=>readStylingPythonRequestV1({version:1,get operation(){reads++;return "deps";},mode:"prototype"})).toThrow();expect(reads).toBe(0);const request={version:1,operation:"build",mode:"release"},captured=readStylingPythonRequestV1(request);request.operation="deps";expect(captured.operation).toBe("build");expect(stylingPythonEnvironmentV1({PATH:"/actual",UV_PROJECT_ENVIRONMENT:"/unowned",PYTHONPATH:"/foreign",PIP_INDEX_URL:"https://foreign.invalid",HATCH_ENV:"foreign",SEMIO_SCRIPT_PROCESS_INVOCATION:"foreign",NX_TASK_TARGET_PROJECT:"mutable"})).toEqual({PATH:"/actual"});});
