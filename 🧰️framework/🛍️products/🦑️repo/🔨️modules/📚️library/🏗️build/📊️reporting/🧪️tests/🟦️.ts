import {acquireResourceLease} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {readFileSync,rmSync,readdirSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {repositoryBuildReportNameV1,withRepositoryBuildReportV1,type RepositoryBuildReportV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json" with {type:"json"};

test("build report names resolve only inside the finite diagnostics artifact owner",()=>{const database=new Database(":memory:"),validate=new Ajv({strict:false}).compile(schema);try{for(const row of fixture.reportPaths){const oracle=database.query("SELECT instr(?, '/')=0 AND instr(?, char(92))=0 AND ? LIKE '%.json' AND substr(?,1,1)<>'.' AS valid").get(row.requested,row.requested,row.requested,row.requested) as {valid:number};expect(Boolean(oracle.valid)).toBe(row.valid);expect(validate(row.requested)).toBe(row.valid);if(row.valid)expect(repositoryBuildReportNameV1(row.requested)).toBe(row.requested);else expect(()=>repositoryBuildReportNameV1(row.requested)).toThrow();}console.log("[DEBUG] Independent SQLite and Ajv reject foreign build report destinations");}finally{database.close();}});


test("build reports publish actual bounded complete bytes under original diagnostics custody",async()=>receiveScriptProcessInvocation(process.env,async original=>{const name="law-"+randomBytes(4).toString("hex")+".json";await withRepositoryBuildReportV1(process.cwd(),name,fixture.reportValue,original,async artifact=>{const path=artifact.path;try{expect(path).toBe(join(process.cwd(),".cache/b/o/m/reports",name));const bytes=readFileSync(path,"utf8"),database=new Database(":memory:");try{expect((database.query("SELECT json(?) AS value").get(bytes) as {value:string}).value).toBe(JSON.stringify(fixture.reportValue));}finally{database.close();}expect(readdirSync(dirname(path)).filter(file=>file.startsWith(name+"~"))).toEqual([]);console.log("[DEBUG] Actual bounded atomic graph report bytes agree with independent SQLite JSON oracle "+path);}finally{}});rmSync(join(process.cwd(),".cache/b/o/m/reports",name),{force:true});}));


test("build reports refuse values without JSON representation before storage admission",async()=>{await expect(withRepositoryBuildReportV1(process.cwd(),"undefined.json",undefined,undefined as never,async()=>{})).rejects.toThrow("JSON build report value required");await expect(withRepositoryBuildReportV1(process.cwd(),"bigint.json",1n,undefined as never,async()=>{})).rejects.toThrow();console.log("[DEBUG] Report encoding refuses absent and non-JSON values before any storage operation");});


test("build report callback holds exact immutable bytes until its reader settles",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const name="reader-"+randomBytes(4).toString("hex")+".json",control=scriptCompilerAdmissionControlV1(original,"report:retirement-proof",30000);let entered=false,retained:RepositoryBuildReportV1|undefined;
 await withRepositoryBuildReportV1(process.cwd(),name,fixture.reportValue,original,async artifact=>{entered=true;retained=artifact;await artifact.recheck();expect(readFileSync(artifact.path,"utf8")).toBe(JSON.stringify(fixture.reportValue,null,2)+"\n");let blocked=false;try{const exclusive=await acquireResourceLease({directory:join(process.cwd(),".cache/b/l/m/data"),resource:"repository-build-reports",mode:"exclusive",signal:control.signal,control,timeoutMs:0});exclusive.release();}catch(error){if(!String(error).includes("Resource lease timed out"))throw error;blocked=true;}expect(blocked).toBe(true);});
 expect(entered).toBe(true);await expect(retained!.recheck()).rejects.toThrow("Original report callback released");rmSync(join(process.cwd(),".cache/b/o/m/reports",name),{force:true});console.log("[DEBUG] Actual report consumer excludes retirement and its escaped original handle refuses after release");
}));

test("build report callback failures release original publication custody",async()=>receiveScriptProcessInvocation(process.env,async original=>{const name="failure-"+randomBytes(4).toString("hex")+".json",control=scriptCompilerAdmissionControlV1(original,"report:error-release",30000),failure=Error("Original report reader failure");try{await expect(withRepositoryBuildReportV1(process.cwd(),name,fixture.reportValue,original,async()=>{throw failure;})).rejects.toBe(failure);const successor=await acquireResourceLease({directory:join(process.cwd(),".cache/b/l/m/data"),resource:"repository-build-reports",mode:"exclusive",signal:control.signal,control,timeoutMs:0});successor.release();expect(readdirSync(join(process.cwd(),".cache/b/o/m/reports")).filter(file=>file.startsWith(name+"~"))).toEqual([]);console.log("[DEBUG] Actual report failure returns exact error and releases writer before successor admission");}finally{rmSync(join(process.cwd(),".cache/b/o/m/reports",name),{force:true});}}));
