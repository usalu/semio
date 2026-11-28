/** 🔓️ Contract: a `show` query under the explicit graph policy `current` only reads a sealed store, so it asks the machine-wide capacity gate for nothing and never queues behind a busy host; every other `show` and every other policy still asks, and an independent Python reading of the same argument vectors agrees. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {requirePinnedTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {buildCapacityAskV1,sealedCurrentReadV1,withBuildCapacityV1} from "../../🟦️.ts";

const python=requirePinnedTestToolV1("python"),script=`
import json,sys
for row in json.load(sys.stdin):
    args,env=row["args"],row["environment"];policy=env.get("SEMIO_BUILD_GRAPH_POLICY");explicit=policy is not None;stop=args.index("--") if "--" in args else len(args);i=0
    while i<stop:
        a=args[i]
        if a=="--graph-policy":policy=args[i+1] if i+1<stop else "";explicit=True;i+=1
        elif a.startswith("--graph-policy="):policy=a[15:];explicit=True
        i+=1
    print(json.dumps(args[0]=="show" and explicit and policy=="current"))
`;
const vectors=[
 {args:["show","project","@semio-tech/stdio-pdf","--json","--graph-policy=current"],environment:{}},
 {args:["show","project","p","--graph-policy","current"],environment:{}},
 {args:["show","projects"],environment:{SEMIO_BUILD_GRAPH_POLICY:"current"}},
 {args:["show","project","p","--graph-policy=fresh"],environment:{}},
 {args:["show","project","p"],environment:{}},
 {args:["show","project","p","--","--graph-policy=current"],environment:{}},
 {args:["show","project","p","--graph-policy=bogus"],environment:{}},
 {args:["graph","--graph-policy=current"],environment:{}},
 {args:["run","p:build","--graph-policy=current"],environment:{}},
];

test("a sealed current show asks for no capacity, any other invocation keeps its ask, and Python agrees",()=>{
 const run=spawnSync(python,["-I","-c",script],{input:JSON.stringify(vectors),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=run.stdout.trim().split("\n").map(line=>JSON.parse(line) as boolean);
 expect(vectors.map(({args,environment})=>sealedCurrentReadV1(args.slice(0,args.indexOf("--")<0?undefined:args.indexOf("--")),environment))).toEqual(expected);
 expect(expected).toEqual([true,true,true,false,false,false,false,false,false]);
 expect(buildCapacityAskV1(vectors[0]!.args,{})).toBeUndefined();
 expect(buildCapacityAskV1(vectors[1]!.args,{})).toBeUndefined();
 expect(buildCapacityAskV1(vectors[2]!.args,vectors[2]!.environment)).toBeUndefined();
 expect(buildCapacityAskV1(vectors[3]!.args,{})?.operation).toBe("show");
 expect(buildCapacityAskV1(vectors[4]!.args,{})?.operation).toBe("show");
 expect(buildCapacityAskV1(vectors[5]!.args,{})?.operation).toBe("show");
 expect(buildCapacityAskV1(vectors[7]!.args,{})?.operation).toBe("graph");
});

test("the flag form of a sealed current show passes the capacity door untouched: no grant, no disk admission, the body runs with the original arguments",async()=>{
 for(const args of [["show","project","p","--graph-policy=current"],["show","projects","--graph-policy","current"],["show","project","p","--tool=nx","--graph-policy=current"]]){
  const written:string[]=[],granted=await withBuildCapacityV1(args,{},async value=>value,{write:line=>written.push(line),disk:{partition:()=>{throw new Error("disk admission must not run");}}});
  expect(granted.handle).toBeUndefined();
  expect(granted.args).toEqual(args);
  expect(written).toEqual([]);
 }
});
