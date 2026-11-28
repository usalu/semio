import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {unitThreadsArgumentsV1,unitThreadsVerdictV1} from "../../🧵️threads/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "../../🧬️schema/🔣️.json";

const here=fileURLToPath(new URL(".",import.meta.url));
const python=(()=>{for(const name of ["python3","python"]){const run=spawnSync(name,["--version"]);if(run.status===0)return name;}return undefined;})();

test("the constrained thread count and its reason come from the policy and the decision follows peak and host memory",()=>{
 expect(schema.definitions.Policy.const.threads.constrained).toBe(1);
 const decided=corpus.cases.map(row=>unitThreadsVerdictV1({args:row.args,budgetMegabytes:corpus.budgetMegabytes,...("peakMegabytes" in row?{peakMegabytes:row.peakMegabytes}:{}),...("host" in row?{host:row.host as never}:{})}));
 expect(decided[0]).toBeUndefined();
 expect(decided[1]).toBeUndefined();
 expect(decided[2]).toEqual({threads:1,reason:"peak"});
 expect(decided[10]).toEqual({threads:1,reason:"peak"});
 expect(decided[11]).toBeUndefined();
 expect(unitThreadsArgumentsV1(decided[2])).toEqual(["-Z","threads=1"]);
 expect(unitThreadsArgumentsV1(undefined)).toEqual([]);
});

test("the decision of every corpus case equals the independent Python oracle",()=>{
 if(python===undefined)return;
 const run=spawnSync(python,[join(here,"🐍️.py"),join(here,"🔣️.json"),resolve(here,"../../🧬️schema/🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as Array<NonNullable<ReturnType<typeof unitThreadsVerdictV1>>|null>;
 const actual=corpus.cases.map(row=>unitThreadsVerdictV1({args:row.args,budgetMegabytes:corpus.budgetMegabytes,...("peakMegabytes" in row?{peakMegabytes:row.peakMegabytes}:{}),...("host" in row?{host:row.host as never}:{})})??null);
 expect(actual).toEqual(expected);
});
