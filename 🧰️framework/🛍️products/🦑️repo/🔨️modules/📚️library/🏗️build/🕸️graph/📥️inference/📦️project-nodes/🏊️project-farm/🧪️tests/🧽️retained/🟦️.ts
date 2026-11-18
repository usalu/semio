/** 🧽️ Proves a farm child keeps nothing from the owners it finished beyond a budget derived from its reservation: a child that held 113 MB of managed memory after its first Rust owner and 290 MB after its third (no cache of the bounded registry above 4 MB: the interned rows, verified blocks and images of the inference) drops them after a job once the memory it holds passes the budget, and the budget is a share of the reservation, never below a floor.
 * @see ../../👷️service/🟨️.mjs */
import{expect,test}from"bun:test";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1}from"../../🏊️pool/🟨️.mjs";
import{retainedBudgetBytesV1}from"../../👷️service/🟨️.mjs";

const child=fileURLToPath(new URL("./👷️child.mjs",import.meta.url)),service=pathToFileURL(fileURLToPath(new URL("../../👷️service/🟨️.mjs",import.meta.url))).href,observation=pathToFileURL(fileURLToPath(new URL("../../../../../../👁️observation/🟨️.mjs",import.meta.url))).href;
const run=(budget:number)=>{const result=Bun.spawnSync([process.execPath,child,service,observation,"60000","6",String(budget)],{stdout:"pipe",stderr:"pipe"});expect(result.exitCode,result.stderr.toString()).toBe(0);return JSON.parse(result.stdout.toString().trim().split("\n").pop()!) as{owner:number;trimmed:boolean;interned:number;retainedMegabytes:number}[];};

test("the budget is the declared share of the reservation, never below the floor, unbounded without a reservation",()=>{
 const policy=GRAPH_FARM_POLICY_V1 as unknown as{retainedBasisPoints:number;retainedMinimumMegabytes:number};
 expect(retainedBudgetBytesV1(1536)).toBe(Math.max(policy.retainedMinimumMegabytes,Math.floor(1536*policy.retainedBasisPoints/10000))*1048576);
 expect(retainedBudgetBytesV1(10)).toBe(policy.retainedMinimumMegabytes*1048576);
 expect(retainedBudgetBytesV1(undefined)).toBe(Infinity);expect(retainedBudgetBytesV1(0)).toBe(Infinity);
});

test("without a budget the child keeps the rows of every owner, with a budget it keeps none beyond it",()=>{
 const free=run(Infinity),bounded=run(8*1048576);
 expect(free.every(row=>!row.trimmed)).toBe(true);
 expect(free[2]!.interned).toBeGreaterThan(free[0]!.interned*2);
 expect(Math.max(...free.map(row=>row.retainedMegabytes))).toBeGreaterThan(Math.max(...bounded.map(row=>row.retainedMegabytes))+40);
 expect(bounded.every(row=>row.trimmed&&row.interned===0)).toBe(true);
 expect(bounded.every(row=>row.retainedMegabytes<=24),JSON.stringify(bounded)).toBe(true);
},120000);
