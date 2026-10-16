import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import ts from "typescript";
import schema from "../../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import {readNativeImageMaintenanceRequestV1} from "../🟦️.ts";
import {checkResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";
import {findWorkspaceRoot,type ScriptInvocation} from "../../../../../../🧭️routing/🟦️.ts";
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
  const recheck=async()=>{checks++;await Promise.resolve();if(checks===1&&row.interrupt==="precheck"||checks===(opened?3:2)&&row.interrupt==="postcheck")abort.abort(failure);},exclusive={release:()=>{releases++;}},bindings={captureResourceLeaseWriterProofV1:()=>Object.freeze({fixtureOnly:true}),control,recheck,port:Object.freeze({owner:Symbol("port")}),acquire:async()=>exclusive,lease:async()=>exclusive,options:{leaseDirectory:"unused"},resource:"owner",acquireResourceLease:async()=>exclusive,readNativeImageMaintenanceRequestV1,checkResourceLeaseControl,shared:{release:()=>{sharedReleases++;}},closed:false};
  const source=ts.transpileModule("const maintenance="+method.getText()+";",{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText,run=new Function(...Object.keys(bindings),source+";return maintenance;")(...Object.values(bindings)) as (request:unknown,body:(port:unknown)=>Promise<unknown>)=>Promise<{acquired:boolean;value?:unknown}>;
  let caught=false,actual:unknown,result:{acquired:boolean;value?:unknown}|undefined;try{result=await run({version:1,wait:true},async port=>{calls++;expect(port).toBe(bindings.port);await Promise.resolve();if(row.interrupt==="body")abort.abort(failure);if(row.interrupt==="failure")throw failure;if(row.interrupt==="undefined")throw undefined;return value;});}catch(error){caught=true;actual=error;}
  const oracle=db.query("SELECT ?1='none' AS accepted").get(row.interrupt) as {accepted:number};expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(caught).toBe(!row.accepted);if(row.accepted){expect(result?.acquired).toBe(true);expect(result?.value).toBe(value);expect(checks).toBe(opened?3:2);}else expect(actual).toBe(row.interrupt==="undefined"?undefined:failure);expect(calls).toBe(row.interrupt==="precheck"?0:1);expect(releases).toBe(opened&&row.interrupt!=="precheck"?1:0);expect(sharedReleases).toBe(opened&&row.interrupt!=="precheck"?1:0);
 }}}finally{db.close();}
 console.log("[DEBUG] Actual TypeScript-transpiled platform method bodies preserve callback identity, cancellation and original exclusive release");
});

test("actual opened maintenance methods retain exclusion through final backing recheck",()=>{
 const methods=maintenanceMethods();expect(methods.length).toBe(3);let opened=0,provisioned=0;
 for(const method of methods){const text=method.getText(),body=text.indexOf("await body(port,"),after=text.slice(body);expect(body).toBeGreaterThanOrEqual(0);expect(after).toContain("await recheck()");expect(after).toContain("checkResourceLeaseControl(control)");if(text.includes("shared.release()")){opened++;expect(text).toContain("finally{closed=true;exclusive.release();}");expect(text).toContain("return{acquired:false}");}else provisioned++;}
 expect([opened,provisioned]).toEqual([2,1]);
 console.log("[DEBUG] TypeScript AST binds two released opened custodians and one retained provisioned owner to post-body checks");
});


test("maintenance disposition data agrees with independent validators and grants no writer proof",()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.ImageMaintenanceCustody),db=new Database(":memory:");try{for(const row of corpus.custody){const encoded=JSON.stringify(row.value),oracle=db.query("SELECT (json_extract(?1,'$.kind')='kernel-ex' AND json_type(?1,'$.proof')='object' AND (SELECT count(*) FROM json_each(?1))=2 AND (SELECT count(*) FROM json_each(?1,'$.proof'))=0) OR (json_extract(?1,'$.kind')='fresh-private' AND (SELECT count(*) FROM json_each(?1))=1) AS accepted").get(encoded)as{accepted:number|null};expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(Boolean(validate(row.value))).toBe(row.accepted);expect(validateJsonSchemaSubset(schema.definitions.ImageMaintenanceCustody,row.value,schema).length===0).toBe(row.accepted);}}finally{db.close();}console.log("[DEBUG] Closed maintenance dispositions agree with Ajv and SQLite; JSON supplies no private kernel authority");
});


test("historical retirement request matches closed Ajv and SQLite images",async()=>{
 const {readNativeImageRetirementRequestV1}=await import("../🟦️.ts"),validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/ImageRetirementRequest"}),db=new Database(":memory:");
 try{for(const row of corpus.retirement){const encoded=JSON.stringify(row.value),value=row.value as unknown,oracle=db.query("SELECT json_extract(?1,'$.version')=1 AND json_type(?1,'$.wait') IN ('true','false') AND (SELECT count(*) FROM json_each(?1))=CASE WHEN json_type(?1,'$.expectedImage') IS NULL THEN 2 ELSE 3 END AND (json_type(?1,'$.expectedImage') IS NULL OR (json_type(?1,'$.expectedImage')='object' AND (SELECT count(*) FROM json_each(?1,'$.expectedImage'))=5 AND length(json_extract(?1,'$.expectedImage.receiptSha256'))=64 AND json_extract(?1,'$.expectedImage.receiptSha256') NOT GLOB '*[^a-f0-9]*' AND NOT EXISTS(SELECT 1 FROM json_each(?1,'$.expectedImage') WHERE key!='receiptSha256' AND (type!='integer' OR value<CASE WHEN key='size' THEN 1 ELSE 0 END OR value>9007199254740991)) AND json_type(?1,'$.expectedImage.dev')='integer' AND json_type(?1,'$.expectedImage.ino')='integer' AND json_type(?1,'$.expectedImage.size')='integer' AND json_type(?1,'$.expectedImage.allocatedBytes')='integer')) AS accepted").get(encoded) as {accepted:number|null};expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(validate(value)).toBe(row.accepted);if(row.accepted)expect(readNativeImageRetirementRequestV1(value).version).toBe(1);else expect(()=>readNativeImageRetirementRequestV1(value)).toThrow();}}
 finally{db.close();}
 const image={receiptSha256:"a".repeat(64),dev:1,ino:2,size:4096,allocatedBytes:4096};let reads=0;const changing={version:1,wait:false,expectedImage:{...image,get size(){return ++reads===1?4096:"changed";}}};expect(()=>readNativeImageRetirementRequestV1(changing)).toThrow("Closed original image retirement request required");
});

test("historical retirement binds queued exclusive acquisition to detached expected identity",async()=>{
 const {join}=await import("node:path"),fs=await import("node:fs"),crypto=await import("node:crypto"),root=findWorkspaceRoot(import.meta.dir),{withBuildTestInvocationV1}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/📥️invocation/🟦️.ts")),{scriptCompilerAdmissionControlV1}=await import("../../../../../../🏗️build/🎟️capacity/🟦️.ts"),leases=await import("../../../../../../🔒️leases/🟦️.ts"),{retireNativeImageStorageV1}=await import("../../../🍎️image/🟦️.ts");
 if(process.platform!=="darwin")return;
 await withBuildTestInvocationV1(async(original:ScriptInvocation)=>{const control=scriptCompilerAdmissionControlV1(original,"test:historical-retirement",0),scope=fs.mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"ri-")),directory=join(scope,"i"),leaseDirectory=join(scope,"l"),image=join(directory,"fixed.dmg"),receipt=join(directory,"image.json"),resource="native-fixed-image:"+directory,options={directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,maximumLeaseDatabaseBytes:16384,control};let blocker:Awaited<ReturnType<typeof leases.acquireResourceLease>>|undefined,work:Promise<boolean>|undefined;
 const create=(byte:number)=>{fs.mkdirSync(join(directory,"m"),{recursive:true});fs.writeFileSync(image,Buffer.alloc(4096,byte),{flag:"wx"});const row=fs.lstatSync(image);fs.writeFileSync(receipt,JSON.stringify({version:1,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:row.size,dev:row.dev,ino:row.ino,size:row.size})+"\n",{flag:"wx"});return{receiptSha256:crypto.createHash("sha256").update(fs.readFileSync(receipt)).digest("hex"),dev:row.dev,ino:row.ino,size:row.size,allocatedBytes:row.blocks*512};};
 try{const first=create(1),request={version:1 as const,wait:true,expectedImage:{...first}};blocker=await leases.acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",signal:control.signal,control,maximumDatabaseBytes:16384});let settled=false;work=retireNativeImageStorageV1(options,request);void work.then(()=>{settled=true;},()=>{settled=true;});await control.yieldContinuation();expect(settled).toBe(false);fs.renameSync(directory,join(scope,"old"));const replacement=create(2);Object.assign(request.expectedImage,replacement);blocker.release();blocker=undefined;await control.yieldContinuation();await expect(work).rejects.toThrow("Expected original image retirement identity changed");work=undefined;expect(fs.lstatSync(image).ino).toBe(replacement.ino);expect(fs.readFileSync(image)[0]).toBe(2);expect(fs.existsSync(join(scope,"old/fixed.dmg"))).toBe(true);
 for(const field of["dev","ino","size","allocatedBytes"] as const){await expect(retireNativeImageStorageV1(options,{version:1,wait:false,expectedImage:{...replacement,[field]:replacement[field]+1}})).rejects.toThrow("Expected original image retirement identity changed");expect(fs.readFileSync(image)[0]).toBe(2);}
 const exact={version:1 as const,wait:false,expectedImage:replacement};expect(await retireNativeImageStorageV1(options,exact)).toBe(true);expect(fs.existsSync(directory)).toBe(false);console.log("[DEBUG] Actual canonical SQLite imageEX queue refuses replacement and caller retargeting; exact unchanged detached fixture retires");
 }finally{blocker?.release();if(work)await Promise.allSettled([work]);fs.rmSync(scope,{recursive:true,force:true});}
 });
},30000);

import "./📥️persistent/🟦️.ts";

import "../../../🧹️cleanup/📥️receiver/🧪️tests/📥️release/🟦️.ts";

import "../../../🧱️fixed/🧪️tests/📥️receipt/🟦️.ts";

import "../../../🧱️fixed/🧪️tests/📥️paths/🟦️.ts";

import "./📥️parent/🟦️.ts";
