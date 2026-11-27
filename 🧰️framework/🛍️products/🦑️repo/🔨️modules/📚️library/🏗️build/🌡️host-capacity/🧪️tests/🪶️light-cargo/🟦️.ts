/** 🪶️ Contract: a Cargo invocation that only produces a lockfile (`update`, `generate-lockfile`) asks the machine-wide capacity gate for nothing, so neither the 512 MB cargo ask nor the 1536 MB orchestration admission ever queues it behind compiles that fill the host; every compiling or fetching subcommand keeps its ask, and an independent Python reading of the same arguments agrees. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {requirePinnedTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {buildCapacityAskV1,lightCargoResolutionV1,withBuildCapacityV1} from "../../🟦️.ts";

const python=requirePinnedTestToolV1("python"),script=`
import json,sys
lock={"update","generate-lockfile"}
for args in json.load(sys.stdin):
    tail=args[args.index("--")+1:] if "--" in args else []
    word=next((a for a in tail if not a.startswith("-")),"")
    print(json.dumps(args[0]=="cargo" and "--" in args and word in lock))
`;
const vectors=[
 ["cargo","--store=shared","--configuration=prototype","--","update","--offline","--workspace","--manifest-path","/w/Cargo.toml"],
 ["cargo","--","generate-lockfile","--manifest-path","/w/Cargo.toml"],
 ["cargo","--store=shared","--","--locked","update"],
 ["cargo","--store=shared","--","build","--release"],
 ["cargo","--store=shared","--","test","-p","update"],
 ["cargo","--store=shared","--","metadata","--format-version","1"],
 ["cargo","--store=shared","--","fetch","--locked"],
 ["cargo","update"],
 ["run","p:update","--","update"],
];

test("a lockfile-only cargo invocation asks for no capacity, every other cargo or build invocation keeps its ask, and Python agrees",()=>{
 const run=spawnSync(python,["-I","-c",script],{input:JSON.stringify(vectors),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=run.stdout.trim().split("\n").map(line=>JSON.parse(line) as boolean);
 expect(vectors.map(args=>lightCargoResolutionV1(args))).toEqual(expected);
 expect(expected).toEqual([true,true,true,false,false,false,false,false,false]);
 for(const [index,args] of vectors.entries())expect(buildCapacityAskV1(args,{})===undefined,String(index)).toBe(expected[index]);
 expect(buildCapacityAskV1(vectors[3]!,{})?.operation).toBe("cargo");
 expect(buildCapacityAskV1(vectors[6]!,{})?.operation).toBe("cargo");
});

test("a lockfile-only cargo invocation passes the capacity door untouched: no grant, no cargo-orchestration reservation, no disk admission, the body runs with the original arguments",async()=>{
 for(const args of [vectors[0]!,vectors[1]!]){
  const written:string[]=[],granted=await withBuildCapacityV1(args,{},async value=>value,{write:line=>written.push(line),disk:{partition:()=>{throw new Error("[DEBUG] disk admission must not run");}}});
  expect(granted.handle).toBeUndefined();
  expect(granted.args).toEqual(args);
  expect(written).toEqual([]);
 }
});
