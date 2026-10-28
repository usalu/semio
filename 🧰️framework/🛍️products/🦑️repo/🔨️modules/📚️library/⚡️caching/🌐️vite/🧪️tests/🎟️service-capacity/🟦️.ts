/** 🎟️ Law: a Vite command service (dev listener) holds the bounded `vite-dev-service` class for its life, equal to the independent Python derivation, so two listeners leave at least half of a host of eight or more cores to other compiles; the listener's own source asks for that class and never for the counted share. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {viteCommandServiceCapacityRequestV1} from "../../🎟️capacity/🟦️.ts";

const units=new URL("../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/",import.meta.url),oracle=fileURLToPath(new URL("🧪️tests/🐍️oracle/🐍️.py",units)),policy=fileURLToPath(new URL("🧬️schema/🔣️.json",units));
const cores=[1,2,3,4,6,8,10,12,16,32,64];

test("the service class equals the independent Python derivation and two listeners leave half of an eight-core or larger host",()=>{
 const python=testPythonV1();if(!python)throw Error("Independent Python oracle required");
 const result=spawnSync(python,["-I",oracle,policy],{input:JSON.stringify(cores.map(count=>({fixed:"vite-dev-service",cores:count}))),encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 const model=JSON.parse(result.stdout) as {kind:string;maximumParallelism:number;minimumParallelism:number}[];
 cores.forEach((count,index)=>{
  const actual=viteCommandServiceCapacityRequestV1(count) as {kind:string;maximumParallelism:number;minimumParallelism:number};
  expect<unknown>(actual,String(count)).toEqual(model[index]);
  expect(actual.kind).toBe("available");expect(actual.minimumParallelism).toBeGreaterThanOrEqual(1);
  if(count>=8)expect(2*actual.maximumParallelism,String(count)).toBeLessThanOrEqual(Math.floor(count/2));
 });
});

test("the listener asks for the service class, never for the counted share of the host",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),start=source.indexOf("export async function serveViteCommandV1("),end=source.indexOf("\n}\n",start);
 expect(start).toBeGreaterThan(-1);
 const body=source.slice(start,end);
 expect(body).toContain("capacity:viteCommandServiceCapacityRequestV1()");
 expect(body).not.toContain("countedCompilerCapacityRequestV1");
});
