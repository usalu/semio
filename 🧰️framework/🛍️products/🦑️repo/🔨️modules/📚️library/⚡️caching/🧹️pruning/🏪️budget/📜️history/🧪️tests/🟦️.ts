import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync,existsSync,mkdirSync,mkdtempSync,writeFileSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {receiveScriptProcessInvocation,withIssuedResourceOperationV1,scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandResultV1,captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {openNativeImageStorageV1,retireNativeImageStorageV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts";
import {acquireNativeStorageV1,recoverNativeStorageReservationV1,nativeStorageRetirementPortV1,type NativeStoragePortV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {resourceLeasePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {openRepositoryCachePartitionV1,repositoryLeaseDirectoryV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {withCompilerWriterCustodyV1} from "../../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {repositoryCompilerLayoutV1} from "../../../../💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts";
import capacitySchema from "../../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧬️schema/🔣️.json";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {retireInactiveCargoStoreV1} from "../../⚙️execution/🟦️.ts";
import * as history from "../🟦️.ts";
import fixture from "./📋️custody.json";
import {detachedCompactLaw} from "./📦️compact/🟦️.ts";
import schema from "../🧬️schema/📋️custody.json";

test("closed historical custody corpus distinguishes real exact scope from copied or broadened requests",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE cases(variant TEXT PRIMARY KEY,accepted INTEGER)");for(const row of fixture.cases)db.query("INSERT INTO cases VALUES(?,?)").run(row.variant,Number(row.accepted));expect(db.query("SELECT variant FROM cases WHERE accepted != (variant = 'exact')").all()).toEqual([]);expect(db.query("SELECT COUNT(*) AS n FROM cases").get()).toEqual({n:10});}finally{db.close();}
 expect(typeof (history as Record<string,unknown>).requireHistoricalCargoMaintenanceV1).toBe("function");
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8");expect(source).toContain("withRequestedIssuedResourceControlV1");expect(source).toContain("WeakMap");
 const authored={version:1,mode:"prototype",preview:true},selected=history.readHistoricalCargoMaintenanceV1(authored);authored.preview=false;expect(selected.preview).toBe(true);expect(Object.isFrozen(selected)).toBe(true);
 let accesses=0;expect(()=>history.readHistoricalCargoMaintenanceV1({version:1,mode:"prototype",get preview(){accesses++;return true;}})).toThrow("plain");expect(accesses).toBe(0);expect(()=>history.readHistoricalCargoMaintenanceV1(new Proxy({version:1,mode:"prototype",preview:true},{}))).toThrow("plain");
 console.log("[DEBUG] Historical custody corpus agrees with independent SQLite exact-boundary discrimination and Ajv closed admission");
});


test("actual authenticated image scope rejects copies and drains accepted work before its owner returns",async()=>{
 await receiveScriptProcessInvocation(process.env,async invocation=>{
  const root=resolve(import.meta.dir,"../../../../../../../../../.."),mode=process.env.SEMIO_BUILD_MODE,control=scriptCompilerAdmissionControlV1(invocation,"test:historical-custody",120000),foreign=scriptCompilerAdmissionControlV1(invocation,"test:foreign-historical-control",120000),events:string[]=[];if(mode!=="prototype"&&mode!=="release")throw Error("Exact historical custody test mode required");let leaked:Parameters<typeof history.requireHistoricalCargoMaintenanceV1>[0]|undefined;
  await history.withHistoricalCargoImageMaintenanceV1(root,invocation,control,{version:1,mode,preview:true},async(image,buildDirectory,targetDirectory,heldControl)=>{
   const exact={workspace:root,invocation,control:heldControl,buildDirectory,relatedDirectories:[targetDirectory],leaseDirectory:join(root,".cache/b/l/m/data"),preview:true,wait:false};leaked=exact;
   const db=new Database(":memory:");try{db.exec("CREATE TABLE results(variant TEXT,accepted INTEGER)");for(const row of fixture.cases){const selected={...exact};switch(row.variant){case"lease":selected.leaseDirectory=join(root,"other");break;case"workspace":selected.workspace=join(root,"other");break;case"build":selected.buildDirectory=join(buildDirectory,"other");break;case"target":selected.relatedDirectories=[join(targetDirectory,"other")];break;case"invocation-copy":selected.invocation={...invocation};break;case"control-copy":selected.control={...heldControl};break;case"foreign-control":selected.control=foreign;break;case"preview":selected.preview=false;break;case"queued":selected.wait=true;break;}let accepted=true,actualRoot:string|undefined;try{actualRoot=history.requireHistoricalCargoMaintenanceV1(selected).root;}catch{accepted=false;}expect(accepted).toBe(row.accepted);if(accepted)expect(actualRoot).toBe(image.root);db.query("INSERT INTO results VALUES(?,?)").run(row.variant,Number(accepted));}expect(db.query("SELECT variant FROM results WHERE accepted != (variant = 'exact')").all()).toEqual([]);}finally{db.close();}
   const unowned=join(root,".unissued-cargo-retirement"),artifactDirectory=join(unowned,"artifacts");await expect(retireInactiveCargoStoreV1({...exact,control:foreign,buildDirectory:unowned,artifactDirectory,leaseDirectory:join(root,".cache/b/l/m/data"),policy:{version:1,retainedBytes:0,targetAvailableBytes:0,maximumDeletionBytes:1}})).rejects.toThrow("authenticated historical Cargo custody");expect(existsSync(artifactDirectory)).toBe(false);
   void withIssuedResourceOperationV1(heldControl,async()=>{await new Promise<void>(accept=>setImmediate(accept));await image.recheck();expect(history.requireHistoricalCargoMaintenanceV1(exact)).toBe(image);events.push("child-settled");});events.push("callback-returning");
  });events.push("owner-returned");expect(events).toEqual(["callback-returning","child-settled","owner-returned"]);expect(leaked).toBeDefined();expect(()=>history.requireHistoricalCargoMaintenanceV1(leaked!)).toThrow("authenticated historical Cargo custody");
  const artifact=join(root,".unissued-historical-custody");expect(existsSync(artifact)).toBe(false);const failed=await captureOwnedCommandResultV1(process.execPath,[join(import.meta.dir,"📜️script.ts")],root,{control:foreign,env:scriptInvocationEnvironment(invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}});expect(failed.status).toBe(1);expect(failed.stdout).toContain("Actual detached lower historical retirement accepted before callback return");expect(failed.stderr).toContain("Complete Cargo retirement policy required");expect(failed.stderr).not.toContain("silently lost");expect(existsSync(artifact)).toBe(false);await history.withHistoricalCargoImageMaintenanceV1(root,invocation,control,{version:1,mode,preview:true},async image=>{await image.recheck();});
  await detachedCompactLaw(invocation,root);console.log("[DEBUG] Actual RC6013/charge/imageEX custody matches ten SQLite cases, refuses unowned roots before effects, drains the same issued control and surfaces a genuine detached lower policy failure before effects; EX can be reacquired and retired control refuses reuse");
 });
},120000);
