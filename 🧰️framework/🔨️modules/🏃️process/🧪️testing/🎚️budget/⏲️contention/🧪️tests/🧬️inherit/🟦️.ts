import {expect,test} from "bun:test";
import {join} from "node:path";
import {CONTENTION_POLICY_V1,budgetStretchEnvironmentV1,inheritedStretchRatioV1,launchStretchBasisPointsV1,stretchEnvironmentV1} from "../../🟦️.ts";

const name=CONTENTION_POLICY_V1.inheritance!.environment,module=join(import.meta.dir,"..","..","🟦️.ts");

/** 🧪️ Asks a fresh process, started with `handed` as the stretch of its parent run, what it funds a one second budget with. */
function childV1(handed:string|undefined):{basisPoints:number;funded:number}{
 const env:Record<string,string>={...process.env as Record<string,string>};delete env[name];if(handed!==undefined)env[name]=handed;
 const run=Bun.spawnSync([process.execPath,"-e",`const m=await import(${JSON.stringify(module)});console.log(JSON.stringify({basisPoints:m.launchStretchBasisPointsV1(),funded:m.stretchedBudgetV1(1000).milliseconds}));`],{env,stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString().trim().split("\n").pop()!);
}

test("a handed-down stretch is read once and refused when malformed or out of the policy bounds",()=>{
 expect(inheritedStretchRatioV1({[name]:"25000"})).toBe(2.5);
 for(const bad of [undefined,"","abc","9999","-1","1.5"," 20000","20000 ","99999999",String(CONTENTION_POLICY_V1.stretch.maximumBasisPoints+1)])expect(inheritedStretchRatioV1({[name]:bad}),String(bad)).toBeUndefined();
 expect(stretchEnvironmentV1(25000)).toEqual({[name]:"25000"});expect(stretchEnvironmentV1(24999.6)).toEqual({[name]:"25000"});
 const own=budgetStretchEnvironmentV1();expect(own[name]).toBe(String(launchStretchBasisPointsV1()));
 expect(launchStretchBasisPointsV1()).toBeGreaterThanOrEqual(CONTENTION_POLICY_V1.stretch.minimumBasisPoints);expect(launchStretchBasisPointsV1()).toBeLessThanOrEqual(CONTENTION_POLICY_V1.stretch.maximumBasisPoints);
});

test("a nested process never funds a deadline with less stretch than the run that started it, whatever its own load",()=>{
 for(const handed of ["10000","20000","40000","80000"]){
  const row=childV1(handed);
  expect(row.basisPoints,handed).toBeGreaterThanOrEqual(Number(handed));
  expect(row.funded,handed).toBeGreaterThanOrEqual(Math.ceil(1000*Number(handed)/10000));
 }
 const alone=childV1(undefined);expect(alone.basisPoints).toBeGreaterThanOrEqual(CONTENTION_POLICY_V1.stretch.minimumBasisPoints);
 const garbage=childV1("garbage");expect(garbage.basisPoints).toBeGreaterThanOrEqual(CONTENTION_POLICY_V1.stretch.minimumBasisPoints);
},60000);
