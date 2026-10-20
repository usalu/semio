import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {readFileSync} from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {linuxNamespaceOffsetMsV1} from "../🟦️.ts";

const python=(code:string,...args:string[])=>Bun.spawnSync([testPythonV1()??"python3","-c",code,...args],{stdout:"pipe",stderr:"pipe"});
const hasPython=testPythonV1()!==null;

test("the start of process 1 becomes the namespace offset of the boot epoch, as an independent python parse agrees",()=>{
 expect(schema.definitions.Protocol.const.bootEpoch.linux.clockTicksPerSecond).toBe(fixture.ticksPerSecond);
 for(const row of fixture.init){
  expect(linuxNamespaceOffsetMsV1(row.stat),row.name).toBe(row.offsetMs);
  if(hasPython)expect(Number(python("import sys\nt=sys.argv[1]\nf=t[t.rindex(')')+2:].split()\nprint(round(int(f[19])*1000/int(sys.argv[2])))",row.stat,String(fixture.ticksPerSecond)).stdout.toString().trim()),row.name).toBe(row.offsetMs);
 }
 for(const text of fixture.malformed)expect(()=>linuxNamespaceOffsetMsV1(text),JSON.stringify(text)).toThrow("Exact init start tick required");
});

test("the clock reads the namespace file only on Linux and the declared file is the init process",()=>{
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8");
 expect(source).toContain('platform==="linux"?hostEpochMs+linuxNamespaceOffsetMsV1(readFileSync(epoch.linux.namespaceFile');
 expect(schema.definitions.Protocol.const.bootEpoch.linux.namespaceFile).toBe("/proc/1/stat");
});

test.skipIf(process.platform!=="linux")("on a real Linux host init started no later than this process and the offset is not negative",()=>{
 const offset=linuxNamespaceOffsetMsV1(readFileSync("/proc/1/stat","utf8")),own=linuxNamespaceOffsetMsV1(readFileSync("/proc/self/stat","utf8"));
 expect(offset).toBeGreaterThanOrEqual(0);expect(own).toBeGreaterThanOrEqual(offset);
});
