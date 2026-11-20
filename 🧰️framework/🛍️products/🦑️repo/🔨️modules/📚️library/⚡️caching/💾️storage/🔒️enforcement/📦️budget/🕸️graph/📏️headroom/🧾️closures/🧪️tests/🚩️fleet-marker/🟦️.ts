/** 🚩️ Contract: the fleet marker the project record port writes beside a stored inference record (`np/<24 hex>.json.fleet`, the digest of the state it was published under) is a known closure leaf, so the closure proof of `repo reset` and of headroom recovery no longer refuses the store with "Unknown closure reference publication"; the marker is read, must be a bare digest, and contributes no reference. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import ts from "typescript";
import {physicalSourceOriginV1} from "../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {projectRecordMarkerPathV1} from "../../../../../../../../../🏗️build/🪪️graph-state/📦️project-records/🟦️.ts";
import {memberKindOfV1} from "../../🗂️members/🟨️.mjs";

const origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);
const source=(url:string)=>readFileSync(new URL(url,origin),"utf8");
const tree=ts.createSourceFile("closures.ts",source("../../🟦️.ts"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
const declaration=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="role")!;
const role=Function(ts.transpileModule(declaration.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return role")() as (name:string)=>string;
const record="inference/selections/"+"8d8f21d7".repeat(8)+"/np/72cd315a856dcac8c8e97b16.json";

test("the marker the writer names beside a record is a fleet marker leaf, in every namespace, and nothing near it is",()=>{
 expect(role(projectRecordMarkerPathV1(record))).toBe("fleet-marker");
 expect(role(projectRecordMarkerPathV1(record.replace("selections","contexts")))).toBe("fleet-marker");
 expect(role(projectRecordMarkerPathV1("inference/np/72cd315a856dcac8c8e97b16.json"))).toBe("fleet-marker");
 expect(role(record)).toBe("inference");
 for(const name of[record+".fleet.tmp",record+".fleet~1",record.replace(".json",".fleet"),record.replace("np/","xp/")+".fleet","inference/selections/"+"8d8f21d7".repeat(8)+"/np/short.json.fleet"])expect(()=>role(name),name).toThrow("Unknown closure reference publication");
});

test("the marker is not a member of a group index: its group stays provable and the marker is read instead",()=>{
 expect(memberKindOfV1(projectRecordMarkerPathV1(record))).toBeUndefined();
});

test("the reader admits a bare digest as a marker without references and refuses anything else",()=>{
 const worker=ts.createSourceFile("read.mjs",source("../../👷️closure-read-worker/🟨️.mjs"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const node=worker.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="fleetMarker")!;
 const admit=Function(ts.transpileModule(node.getText(worker),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return fleetMarker")() as (bytes:Uint8Array)=>null;
 const digest=createHash("sha256").update("state").digest("hex"),bytes=(text:string)=>new TextEncoder().encode(text);
 expect(admit(bytes(digest))).toBeNull();
 for(const text of[digest+":4505:1795025581994",digest+":4505:1795025581994:closures-1",digest+":closures-1"])expect(admit(bytes(text)),text).toBeNull();
 for(const text of["",digest+"\n",digest+":1",digest+":1:2:3",digest+":a:b",digest+":closures-2",digest+":1:2:closures-1:x",digest.slice(1),digest.toUpperCase(),"{\"version\":1}","SEMIO_SOURCE_CLOSURE_"+digest])expect(()=>admit(bytes(text)),text).toThrow("Bare fleet marker digest required");
 expect(()=>admit(new Uint8Array([0xff,0xfe]))).toThrow();
});
