/** 🫥️ Contract: the wait of a selected read for the fleet counts as progress only what the fleet did, never what this reader does to itself: a record that is taken again and again (and found stale again and again) raised the sum of the seeded records at every poll, so the bound of 30 minutes without progress never came (g1-m01 waited 2530 s of a bound of 1800 s: `seeded` 59 to 2333). The sign is the number of machines that delivered plus the owners that arrived.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),wait=source.slice(source.indexOf("const waitHeavy=async()=>{"),source.indexOf("const largestOf=")),line=wait.split("\n").find(row=>row.includes("const sign=()=>"))!;

test("the progress sign of the heavy wait does not grow when the same record is seeded again",()=>{
 expect(line).toContain("Object.keys(fleetPort?.producers?.()??{}).length");
 expect(line).not.toContain("Object.values(fleetPort?.producers");
 expect(line).not.toContain("reduce(");
});

test("evaluated: re-seeding one machine's record 2000 times leaves the sign where it was; a second machine or an arrived owner moves it",()=>{
 const seeded:Record<string,number>={"g1-m02":1},taken=new Set<string>(),names=["a"];
 const sign=new Function("fleetPort","names","taken",`return ()=>${line.slice(line.indexOf("=>")+2).replace(/;$/u,"")}`)({producers:()=>seeded},names,taken) as ()=>number;
 const first=sign();
 for(let poll=0;poll<2000;poll++)seeded["g1-m02"]!++;
 expect(sign()).toBe(first);
 seeded["g1-m03"]=1;expect(sign()).toBe(first+1);
 taken.add("a");expect(sign()).toBe(first+2);
});
