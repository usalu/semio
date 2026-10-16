import{startTelemetrySamplerV1}from"../../../../📦️artifacts/🏗️native-build/📈️telemetry/⏱️sampling/🟦️.ts";
import stageFixture from"../../../../📦️artifacts/🏗️native-build/🧩️component-pass/🎟️stage/🧪️tests/🧫️fixtures/🔣️.json";
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{compilerCapacitySubdivisionV1,withCompilerCapacityScopeV1}from"../../🪆️scope/🟦️.ts";
import schema from"../../🪆️scope/🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
/** 🪆️ Admits closed language-neutral subdivisions without inventing inherited machine credits. */
test("neutral compiler capacity subdivisions preserve the finite parent grant",()=>{const oracle=new Ajv({strict:true});for(const row of fixture.subdivisions){const request={parent:row.parent,requested:row.requested};if(row.expected===null)expect(()=>compilerCapacitySubdivisionV1(row.parent,row.requested??undefined)).toThrow();else{expect(oracle.validate(schema.definitions.Request,request)).toBe(true);expect(compilerCapacitySubdivisionV1(row.parent,row.requested??undefined)).toBe(row.expected);}}});

import{mkdirSync,mkdtempSync,rmSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{availableParallelism}from"node:os";
import{resourceLeasePathV1}from"../../../../🔒️leases/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../🎛️owned-execution/📥️capture/🟦️.ts";
import{receiveScriptProcessInvocation,withIssuedResourceOperationV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../🟦️.ts";
import capacitySchema from"../../🧬️schema/🔣️.json";
/** 🔒️ Exercises a genuine parent EX and bounded same-process credits through actual detached canonical work. */
test("actual compiler capacity scope retains real SQLite ownership and fair bounded children",()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:compiler-scope",30000),output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Actual ticket output required");mkdirSync(output,{recursive:true});const directory=mkdtempSync(join(output,"scope-")),gate=resourceLeasePathV1(directory,capacitySchema.properties.resource.const),python=process.platform==="win32"?"python":"python3",limits={maximumBytes:65536,maximumLines:128};
 const probe=async()=>{const result=await captureOwnedCommandResultV1(python,["-c","import sqlite3,sys\nd=sqlite3.connect(sys.argv[1],timeout=0)\ntry:\n d.execute('begin exclusive');print('free');d.rollback()\nexcept sqlite3.OperationalError as e:\n if 'locked' not in str(e):raise\n print('busy')\nfinally:d.close()",gate],process.cwd(),{control,output:limits});expect(result.status,result.stderr).toBe(0);return result.stdout.trim();};
 let active=0,peak=0,detached=false;const order:number[]=[],db=new Database(":memory:");db.exec("create table actual_capacity_event(id integer primary key,delta integer not null)");
 try{await withCompilerCapacityScopeV1({directory,control,environment:{}},async(facts,actual)=>{
  expect(facts.parallelism).toBe(availableParallelism());expect(await probe()).toBe("busy");let reads=0;for(const bad of[{...actual}]){expect(()=>withCompilerCapacityScopeV1({control:bad,get directory():string{reads++;throw Error("Unread namespace");}},async()=>{})).toThrow();}expect(reads).toBe(0);
  const count=Math.min(2,facts.parallelism);await withCompilerCapacityScopeV1({directory,control:actual,requestedParallelism:count,environment:{SEMIO_COMPILER_CAPACITY_HELD:"true"}},async(_,parent)=>{
   const jobs=Array.from({length:4},(_,index)=>withCompilerCapacityScopeV1({directory,control:parent,requestedParallelism:1},async(child,phase)=>{order.push(index);active+=child.parallelism;db.query("insert into actual_capacity_event(delta) values(?)").run(child.parallelism);peak=Math.max(peak,active);await phase.yieldContinuation();await phase.yieldContinuation();active-=child.parallelism;db.query("insert into actual_capacity_event(delta) values(?)").run(-child.parallelism);}));await Promise.all(jobs);expect(peak).toBeLessThanOrEqual(count);const observed=db.query("select max(running) as peak from (select sum(delta) over (order by id) as running from actual_capacity_event)").get();if(!observed||typeof observed!=="object"||!("peak"in observed))throw Error("Independent actual capacity oracle absent");expect(observed.peak).toBe(peak);
  });
  await withCompilerCapacityScopeV1({directory,control:actual,requestedParallelism:1},async(_,parent)=>{let ready:()=>void=()=>{throw Error("Actual enrolled entry required");},unblock:()=>void=()=>{throw Error("Actual enrolled drain required");},second=false;const started=new Promise<void>(done=>{ready=done;}),gate=new Promise<void>(done=>{unblock=done;});
   const first=withCompilerCapacityScopeV1({directory,control:parent},async(_,phase)=>{void withIssuedResourceOperationV1(phase,async()=>{ready();await gate;});});await started;
   const next=withCompilerCapacityScopeV1({directory,control:parent},async()=>{second=true;});await parent.yieldContinuation();expect(second).toBe(false);unblock();await Promise.all([first,next]);expect(second).toBe(true);
  });
  void withCompilerCapacityScopeV1({directory,control:actual,requestedParallelism:1},async(_,phase)=>{void withIssuedResourceOperationV1(phase,async()=>{await phase.yieldContinuation();expect(await probe()).toBe("busy");detached=true;});});
 });expect(detached).toBe(true);expect(active).toBe(0);expect(order).toEqual([0,1,2,3]);expect(await probe()).toBe("free");
 let sampleActive=0,samplePeak=0,sampleCalls=0,terminal=false;let sampleStarted:()=>void=()=>{};const began=new Promise<void>(resolve=>{sampleStarted=resolve;});const sampler=startTelemetrySamplerV1(control,async actual=>{sampleCalls++;sampleActive++;samplePeak=Math.max(samplePeak,sampleActive);sampleStarted();try{const child=await captureOwnedCommandResultV1(process.execPath,["-e","setTimeout(()=>console.log('joined-sample'),30)"],process.cwd(),{control:actual,output:limits});expect(child.status,child.stderr).toBe(0);expect(child.signal).toBeNull();expect(child.stdout.trim()).toBe("joined-sample");terminal=true;}finally{sampleActive--;}},1);await began;await sampler.stop();expect(sampleCalls).toBe(1);expect(samplePeak).toBe(1);expect(sampleActive).toBe(0);expect(terminal).toBe(true);
 for(const kind of["error","undefined","cancel",...stageFixture.inputFailures,"sampler-error","sampler-undefined","sampler-cancel","consumer-error","consumer-undefined"]){expect(new Ajv({strict:true}).validate(schema.definitions.ChildCase,kind)).toBe(true);const result=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),directory,kind],process.cwd(),{control,output:limits});expect(result.status,result.stderr).toBe(1);expect(result.signal).toBeNull();const observation:unknown=JSON.parse(result.stdout);expect(observation).toEqual({version:1,kind,exact:true,late:true,refused:true});expect(await probe()).toBe("free");}
 const finalRoot=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),directory,"root-drain"],process.cwd(),{control,output:limits});expect(finalRoot.status,finalRoot.stderr).toBe(0);expect(JSON.parse(finalRoot.stdout)).toEqual({version:1,kind:"root-drain",late:true});expect(await probe()).toBe("free");
 console.log("[DEBUG] Scoped compiler capacity "+JSON.stringify({peak,active,order,detached}));}finally{db.close();rmSync(directory,{recursive:true,force:true});}
}),30000);

/** 📦️ Verifies the actual fixed remote leaf has one admitted parent, no recursive CPU claim, and no public compile API. */
test("neutral actual Bun compiler leaf preserves parent scoped capacity and received provenance",()=>{
 const root=process.cwd(),base=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle"),parent=readFileSync(join(base,"🟦️.ts"),"utf8"),worker=readFileSync(join(base,"🟦️bun/📜️script.ts"),"utf8"),ast=ts.createSourceFile("worker.ts",worker,ts.ScriptTarget.Latest,true);let recursive=0,publicCompiler=0;
 const walk=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&["acquireCompilerCapacityLeaseV1","withCompilerCapacityScopeV1","withWholeHostCompilerCapacity"].includes(node.expression.text))recursive++;if(ts.isFunctionDeclaration(node)&&node.name?.text==="compileBunBundle"&&node.modifiers?.some(value=>value.kind===ts.SyntaxKind.ExportKeyword))publicCompiler++;ts.forEachChild(node,walk);};walk(ast);
 expect(recursive).toBe(0);expect(publicCompiler).toBe(0);expect(worker.indexOf("await receiveScriptProcessInvocation")).toBeLessThan(worker.indexOf("for await(const chunk of process.stdin)"));expect(worker).toContain("workspace!==authority.repositoryRoot");expect(worker).toContain("authority.original.buildMode");expect(parent).toContain("facts.parallelism!==availableParallelism()");expect(parent).toContain("async(environment,control)=>");expect(parent).toContain("SEMIO_COMPILER_CAPACITY_HELD:undefined");expect(parent).toContain("compilerAdmissionBudgetV1(control)");
});
