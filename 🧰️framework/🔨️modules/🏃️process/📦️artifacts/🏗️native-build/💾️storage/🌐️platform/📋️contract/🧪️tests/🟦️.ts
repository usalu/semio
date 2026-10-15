import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import ts from "typescript";
import schema from "../../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import {readNativeImageMaintenanceRequestV1} from "../🟦️.ts";
import {checkResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

function maintenanceMethods(){const methods:ts.ArrowFunction[]=[];for(const path of["../../../🍎️image/🟦️.ts","../../../🧱️fixed/🟦️.ts"]){const file=ts.createSourceFile(path,readFileSync(new URL(path,import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);const visit=(node:ts.Node)=>{if(ts.isPropertyAssignment(node)&&node.name.getText(file)==="withExclusiveMaintenance"&&ts.isArrowFunction(node.initializer))methods.push(node.initializer);ts.forEachChild(node,visit);};visit(file);}return methods;}

test("closed maintenance results agree with independent Ajv and SQLite lifetime outcomes",()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.ImageMaintenanceResult),db=new Database(":memory:");
 try{for(const row of corpus.outcomes){const value:unknown=row.value,oracle=db.query("SELECT json_type(?1)='object' AND ((json_type(?1,'$.acquired')='false' AND (SELECT count(*) FROM json_each(?1))=1) OR (json_type(?1,'$.acquired')='true' AND (SELECT count(*) FROM json_each(?1))=2 AND EXISTS(SELECT 1 FROM json_each(?1) WHERE key='value'))) AS accepted").get(JSON.stringify(value)) as {accepted:number|null};expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(validate(value)).toBe(row.accepted);expect(validateJsonSchemaSubset(schema.definitions.ImageMaintenanceResult,value,schema).length===0).toBe(row.accepted);}}
 finally{db.close();}
 console.log("[DEBUG] Actual acquired-only maintenance results match Ajv and independent SQLite");
});

test("actual platform maintenance methods preserve original values and refuse cancellation across awaits",async()=>{
 const methods=maintenanceMethods(),db=new Database(":memory:");expect(methods.length).toBe(3);
 try{for(const method of methods){const opened=method.getText().includes("shared.release()");for(const row of corpus.callbacks){const abort=new AbortController(),failure=Error("original maintenance interruption"),value=Object.freeze({owner:Symbol("original")}),control={signal:abort.signal,remainingMilliseconds:()=>1000,publish:async()=>{},yieldContinuation:async()=>{}};let checks=0,calls=0,releases=0,sharedReleases=0;
  const recheck=async()=>{checks++;await Promise.resolve();if(checks===1&&row.interrupt==="precheck"||checks===(opened?3:2)&&row.interrupt==="postcheck")abort.abort(failure);},exclusive={release:()=>{releases++;}},bindings={control,recheck,port:Object.freeze({owner:Symbol("port")}),acquire:async()=>exclusive,lease:async()=>exclusive,options:{leaseDirectory:"unused"},resource:"owner",acquireResourceLease:async()=>exclusive,readNativeImageMaintenanceRequestV1,checkResourceLeaseControl,shared:{release:()=>{sharedReleases++;}},closed:false};
  const source=ts.transpileModule("const maintenance="+method.getText()+";",{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText,run=new Function(...Object.keys(bindings),source+";return maintenance;")(...Object.values(bindings)) as (request:unknown,body:(port:unknown)=>Promise<unknown>)=>Promise<{acquired:boolean;value?:unknown}>;
  let caught=false,actual:unknown,result:{acquired:boolean;value?:unknown}|undefined;try{result=await run({version:1,wait:true},async port=>{calls++;expect(port).toBe(bindings.port);await Promise.resolve();if(row.interrupt==="body")abort.abort(failure);if(row.interrupt==="failure")throw failure;if(row.interrupt==="undefined")throw undefined;return value;});}catch(error){caught=true;actual=error;}
  const oracle=db.query("SELECT ?1='none' AS accepted").get(row.interrupt) as {accepted:number};expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(caught).toBe(!row.accepted);if(row.accepted){expect(result?.acquired).toBe(true);expect(result?.value).toBe(value);expect(checks).toBe(opened?3:2);}else expect(actual).toBe(row.interrupt==="undefined"?undefined:failure);expect(calls).toBe(row.interrupt==="precheck"?0:1);expect(releases).toBe(opened&&row.interrupt!=="precheck"?1:0);expect(sharedReleases).toBe(opened&&row.interrupt!=="precheck"?1:0);
 }}}finally{db.close();}
 console.log("[DEBUG] Actual TypeScript-transpiled platform method bodies preserve callback identity, cancellation and original exclusive release");
});

test("actual opened maintenance methods retain exclusion through final backing recheck",()=>{
 const methods=maintenanceMethods();expect(methods.length).toBe(3);let opened=0,provisioned=0;
 for(const method of methods){const text=method.getText(),body=text.indexOf("await body(port)"),after=text.slice(body);expect(body).toBeGreaterThanOrEqual(0);expect(after).toContain("await recheck()");expect(after).toContain("checkResourceLeaseControl(control)");if(text.includes("shared.release()")){opened++;expect(text).toContain("finally{closed=true;exclusive.release();}");expect(text).toContain("return{acquired:false}");}else provisioned++;}
 expect([opened,provisioned]).toEqual([2,1]);
 console.log("[DEBUG] TypeScript AST binds two released opened custodians and one retained provisioned owner to post-body checks");
});
