import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,symlinkSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {runtimeVerificationReportV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
test("runtime verifier reports remain inside their finite original diagnostics namespace",()=>{const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original test output required");mkdirSync(output,{recursive:true});const root=mkdtempSync(join(resolve(output),"rv-")),foreign=mkdtempSync(join(resolve(output),"rf-")),database=new Database(":memory:");try{symlinkSync(foreign,join(root,"link"),process.platform==="win32"?"junction":"dir");for(const law of fixture.reports){const request=law.path===null?undefined:resolve(root,law.path);let accepted=false;try{const report=runtimeVerificationReportV1(root,request);accepted=true;expect(report.startsWith(root+"/")||process.platform==="win32"&&report.startsWith(root+"\\")).toBe(true);}catch{}expect(accepted).toBe(law.accepted);const independent=database.query("SELECT CASE WHEN ? IS NULL OR (? NOT LIKE '../%' AND ? NOT LIKE 'link/%') THEN 1 ELSE 0 END AS accepted").get(law.path,law.path,law.path) as {accepted:number};expect(Boolean(independent.accepted)).toBe(law.accepted);}console.log("[DEBUG] Runtime verifier closed path corpus agrees with independent SQLite, including real external symlink refusal");}finally{database.close();rmSync(root,{recursive:true,force:true});rmSync(foreign,{recursive:true,force:true});}});

