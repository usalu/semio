import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {createHash,randomUUID} from "node:crypto";
import {mkdirSync,rmSync,writeFileSync,lstatSync} from "node:fs";
import {join,posix,win32} from "node:path";
import {nxCollectionAdmissionPlanV1,withNxCollectionAdmissionStoreV1} from "../🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {resourceLeasePathV1} from "@semio-tech/process/leases";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

test("neutral admission scope is independent of broad inference under actual SQLite exclusion",async()=>{
 const independent=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases){expect(independent(row)).toBe(true);const paths=row.platform==="windows"?win32:posix,plan=nxCollectionAdmissionPlanV1(row);expect(plan.directory).toBe(paths.join(row.storage,"admission"));expect(plan.resource).toBe("nx-collection-boundary:"+row.workspace);expect(plan.maximumSnapshotBytes).toBe(67108864);expect(createHash("sha256").update(plan.resource).digest("hex")).not.toBe(createHash("sha256").update("repository-graph-facts:"+row.workspace).digest("hex"));}
 for(const row of fixture.invalid)expect(()=>nxCollectionAdmissionPlanV1(row)).toThrow();
 await receiveScriptProcessInvocation(process.env,original=>withNxCollectionAdmissionStoreV1(process.cwd(),original,async directory=>{
  const root=join(directory,"qa-"+randomUUID().slice(0,8));mkdirSync(root);const plan=nxCollectionAdmissionPlanV1({version:1,platform:process.platform==="win32"?"windows":"posix",workspace:process.cwd(),storage:directory.slice(0,-"/admission".length)}),broadPath=resourceLeasePathV1(root,"repository-graph-facts:"+process.cwd()),admissionPath=resourceLeasePathV1(root,plan.resource),broad=new Database(broadPath),contender=new Database(broadPath),admission=new Database(admissionPath);
  try{broad.exec("CREATE TABLE actual(value INTEGER); INSERT INTO actual VALUES(7); BEGIN EXCLUSIVE");expect(()=>contender.query("SELECT value FROM actual").get()).toThrow();admission.exec("CREATE TABLE actual(value INTEGER); BEGIN EXCLUSIVE; INSERT INTO actual VALUES(11); COMMIT");expect(admission.query("SELECT value FROM actual").get()).toEqual({value:11});expect(lstatSync(root).dev).toBe(lstatSync(directory).dev);writeFileSync(join(root,"owned.json"),JSON.stringify({value:11}));}finally{broad.close();contender.close();admission.close();rmSync(root,{recursive:true,force:true});}
 }));
},30000);


test("actual collection publisher retains full checks outside the unrelated inference writer",async()=>{
 const {publishNxCollectionBoundary}=await import("../../../📜️script.ts");await receiveScriptProcessInvocation(process.env,async original=>{const started=performance.now();await publishNxCollectionBoundary(process.cwd(),original);});
},30000);
