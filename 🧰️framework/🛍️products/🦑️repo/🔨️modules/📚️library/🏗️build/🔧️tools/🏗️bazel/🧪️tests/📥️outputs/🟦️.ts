/** 🔒 A cached Bazel task must produce every output it declares: the fixture is the contract, an independent Python implementation of the rule is the oracle. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import fixture from "./🔣️.json" with {type:"json"};
import {bazelMissingOutputRefusedV1} from "../../📜️script.ts";

const python=`
import json,sys
def refused(definition):
    semio=(definition.get("metadata") or {}).get("semio") or {}
    return definition.get("cache") is True and semio.get("optionalOutputs") is not True
print(json.dumps([refused(row["definition"]) for row in json.load(sys.stdin)["cases"]]))
`;

test("every fixture case is refused or admitted exactly as authored",()=>{
 for(const row of fixture.cases)expect(bazelMissingOutputRefusedV1(row.definition),row.name).toBe(row.refused);
});

test("an independent Python implementation of the rule agrees on every case",()=>{
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",python],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(fixture.cases.map(row=>row.refused));
});
