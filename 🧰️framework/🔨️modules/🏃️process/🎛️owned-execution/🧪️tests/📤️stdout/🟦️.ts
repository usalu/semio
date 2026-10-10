import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/📤️stdout/🔣️.json";
import domain from "../../🧬️schema/📤️stdout.json";

test("owned child stdout destination preserves machine output with independent Node stdio",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit artifact storage required");mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"owned-stdout-")),helper=join(root,"📜️script.ts"),api=fileURLToPath(new URL("../../🟦️.ts",import.meta.url));
 const body='process.stdout.write("stdout Ω\\n");process.stderr.write("stderr 😀\\n");';
 writeFileSync(helper,'import {runOwnedCommand} from '+JSON.stringify(api)+';await runOwnedCommand(process.execPath,["--eval",'+JSON.stringify(body)+'],process.cwd(),"stdout-routing",'+fixture.policy.maximumElapsedMilliseconds+',{stdout:process.argv[2]});');
 const admit=new Ajv({strict:true}).compile(domain),lines=(value:Uint8Array)=>Buffer.from(value).toString("utf8").trim().split("\n").filter(Boolean).sort();
 for(const row of fixture.cases){
  expect(admit(row.mode)).toBe(row.accepted);const actual=Bun.spawnSync([process.execPath,helper,row.mode],{cwd:root,env:process.env,stdout:"pipe",stderr:"pipe"});
  if(!row.accepted){expect(actual.exitCode).not.toBe(0);expect(lines(actual.stdout)).toEqual([]);continue;}
  const oracle=spawnSync("node",["--eval",'const fs=require("node:fs");'+(row.mode==="ignore"?'': 'fs.writeSync('+ (row.mode==="stderr"?2:1)+',"stdout Ω\\n");')+'fs.writeSync(2,"stderr 😀\\n");'],{cwd:root});
  expect(actual.exitCode,actual.stderr.toString()).toBe(0);expect(oracle.status,oracle.stderr.toString()).toBe(0);expect(lines(actual.stdout)).toEqual(row.stdout!);expect(lines(actual.stderr)).toEqual([...row.stderr!].sort());expect(lines(actual.stdout)).toEqual(lines(oracle.stdout));expect(lines(actual.stderr)).toEqual(lines(oracle.stderr));
 }
 console.error("[DEBUG] Original owned child stdout routing agrees with closed domain Ajv and independent Node UTF8 stdio; foreign destination refused before child output");
});
