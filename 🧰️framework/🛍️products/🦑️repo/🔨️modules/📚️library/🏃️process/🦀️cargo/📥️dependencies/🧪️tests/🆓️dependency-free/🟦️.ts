import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {repositoryCargoDependencyFreeV1} from "../../🟦️.ts";
import fixture from "./🔣️.json";

test("only metadata with --no-deps skips the dependency fetch, as the python oracle decides",()=>{
 const oracle=spawnSync("python3",["-I","-c","import json,sys\nfor c in json.load(sys.stdin)['cases']:\n a=c['args']\n i=next((k for k,x in enumerate(a) if not x.startswith(('+','-'))),None)\n print('1' if i is not None and a[i]=='metadata' and '--no-deps' in a else '0')"],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(oracle.status).toBe(0);
 const expected=oracle.stdout.trim().split("\n");
 fixture.cases.forEach((row,at)=>{expect(repositoryCargoDependencyFreeV1(row.args)).toBe(row.free);expect(expected[at]).toBe(row.free?"1":"0");});
});
