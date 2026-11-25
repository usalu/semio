/** 🤝️ Contract: a row that a fresh complete verification found to hold is taken as holding by the in-round holds of the next pass for a bounded time, so a restarted pass probes the untouched rows once, not twice; the fresh verification never takes a carried verdict and refutes an edited row at once, after which the in-round holds probe it again. @see ../../🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {captureObservedSourceInputsV1,observationCarriedCountsV1,observationsHold,observationsHoldWithinRoundV1,observedFs} from "../../🟨️.mjs";

const roots:string[]=[],FILES=300;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const oracle=(rows:readonly object[],root:string):boolean[]=>{const file=join(root,"rows.json");writeFileSync(file,JSON.stringify(rows));const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),file]);return JSON.parse(run.stdout.toString());};

test("the in-round holds reuse the verdicts of a fresh verification, the fresh verification never does, and an edit refutes the row",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"observation-carried-"));roots.push(root);mkdirSync(join(root,"s"));
 for(let index=0;index<FILES;index++)writeFileSync(join(root,"s",`${index}.txt`),`source ${index}`);
 await Bun.sleep(2100);
 const captured=await captureObservedSourceInputsV1(()=>{for(let index=0;index<FILES;index++)observedFs.readFileSync(join(root,"s",`${index}.txt`),"utf8");},undefined,false),rows=captured.observations.filter(row=>row.kind==="file");
 expect(rows.length).toBe(FILES);expect(oracle(rows,root).every(Boolean)).toBe(true);
 const base=observationCarriedCountsV1();
 expect(await observationsHoldWithinRoundV1(rows,64)).toBe(true);
 const cold=observationCarriedCountsV1();expect(cold.probed-base.probed).toBe(FILES);expect(cold.reused-base.reused).toBe(0);
 expect(await observationsHold(rows,64)).toBe(true);
 const fresh=observationCarriedCountsV1();expect(fresh.probed-cold.probed).toBe(FILES);expect(fresh.reused-cold.reused).toBe(0);
 expect(await observationsHoldWithinRoundV1(rows,64)).toBe(true);
 const warm=observationCarriedCountsV1();expect(warm.reused-fresh.reused).toBe(FILES);expect(warm.probed-fresh.probed).toBe(0);
 writeFileSync(join(root,"s","7.txt"),"edited source 7, longer than before");
 expect(oracle(rows,root).filter(held=>!held).length).toBe(1);
 expect(await observationsHold(rows,64)).toBe(false);
 const refuted=observationCarriedCountsV1();expect(refuted.reused-warm.reused).toBe(0);
 expect(await observationsHoldWithinRoundV1(rows,64)).toBe(false);
 const after=observationCarriedCountsV1();expect(after.probed-refuted.probed).toBeGreaterThan(0);
},60000);
