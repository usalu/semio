/** 🧱️ Contract: a compacted source group names, besides its exact files, only the helper files whose code computes a closure token, and it obeys the scheduling table like every command closure: no file a scheduling domain declares output-neutral (custody of stores and ledgers) and not the module that composes the groups (its output is the inventory, which names the token). The selection equals an independent Python model, every kept helper exists, and the real function takes its helpers from the selection. Numbers over all owners: `🧰️scripts/R37/blast.py`. */
import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {join,relative,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {buildSchedulingDomainOfV1,buildSchedulingV1} from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import {compactionHelperPathsV1} from "../../🟨️.mjs";

const checkout=process.env.SEMIO_TEST_CHECKOUT??resolve(import.meta.dir,"../../../../../../../../..");
const inputs="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs";
const origin=pathToFileURL(join(checkout,inputs,"🟨️.mjs"));
const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
const helperList=(JSON.parse("["+/const COMPACTION_HELPERS_V1 = Object\.freeze\(\[(.*?)\]\);/su.exec(source)![1]!+"]") as string[]);
const kept=compactionHelperPathsV1(checkout,origin);

test("the selection equals the independent Python model of the table",()=>{
 const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py");
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({table:buildSchedulingV1(),helpers:helperList,directory:inputs}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 expect(kept).toEqual(JSON.parse(run.stdout.toString()));
});

test("no kept helper lies in a scheduling domain, the composing module is not a helper, every kept helper exists",()=>{
 for(const path of kept){expect(buildSchedulingDomainOfV1(path),path).toBeUndefined();expect(existsSync(join(checkout,path)),path).toBe(true);}
 expect(kept).not.toContain(inputs+"/🟨️.mjs");
 expect(helperList).not.toContain("./🟨️.mjs");
});

test("the files of the store custody domain that the list names are exactly the four dropped ones, the other helpers stay",()=>{
 const dropped=helperList.map(path=>relative(checkout,resolve(checkout,inputs,path)).split("\\").join("/")).filter(path=>!kept.includes(path)).sort();
 expect(dropped).toEqual([
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json",
  inputs+"/💾️storage/📒️ledger/🟨️.mjs",
  inputs+"/💾️storage/🟨️.mjs",
  inputs+"/💾️storage/🧬️schema/🔣️.json",
 ].sort());
 expect(kept.length).toBe(helperList.length-4);
 for(const must of ["🧮️digest/🟨️.mjs","🧮️digest/📥️read/🟨️.mjs","📋️data/📥️load/🟨️.mjs","🔒️lock/📥️read/🟨️.mjs","🔏️revision/🟨️.mjs"])expect(kept).toContain(inputs+"/"+must);
});

test("the real compaction takes its helpers from the selection, after the exact files",()=>{
 const body=/export function compactSourceInputs\(.*?\n\}\n/su.exec(source)![0];
 expect(body).toContain("const helpers = compactionHelperPathsV1(workspaceRoot);");
 expect(body).not.toContain("COMPACTION_HELPERS_V1");
});
