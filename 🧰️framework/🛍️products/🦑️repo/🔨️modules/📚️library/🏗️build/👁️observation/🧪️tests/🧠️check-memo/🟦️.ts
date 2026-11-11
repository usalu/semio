/** 🧠️ Contract: the verdicts of one validation scope are remembered by the content of a row in a table of numbers and by the row object only weakly, so a cold inference that validates millions of rows of shared closure entries in one long-lived scope retains no row object, no fact text and no path of them. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {captureObservedSourceInputsV1,observationLayerHoldsAsyncV1,observeInferenceSnapshotV1,observedFs} from "../../🟨️.mjs";

const roots:string[]=[],FILES=400;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const sources=async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"observation-check-memo-"));roots.push(root);mkdirSync(join(root,"s"));
 for(let index=0;index<FILES;index++)writeFileSync(join(root,"s",`${index}.txt`),`source ${index}`);
 await Bun.sleep(2100);
 const captured=await captureObservedSourceInputsV1(()=>{for(let index=0;index<FILES;index++)observedFs.readFileSync(join(root,"s",`${index}.txt`),"utf8");},undefined,false);
 return{root,rows:captured.observations.filter(row=>row.kind==="file")};
};

const validateClones=async(rows:readonly object[])=>{
 const layers=rows.map(row=>[structuredClone(row)]),refs=layers.map(layer=>new WeakRef(layer[0] as object));
 for(const layer of layers)expect(await observationLayerHoldsAsyncV1(layer,async()=>{})).toBe(true);
 return refs;
};

test("validated rows are not kept alive by the scope that validated them",async()=>{
 const{rows}=await sources();expect(rows.length).toBe(FILES);
 await observeInferenceSnapshotV1(async()=>{
  const refs=await validateClones(rows);
  await Bun.sleep(50);Bun.gc(true);await Bun.sleep(50);Bun.gc(true);
  expect(refs.filter(ref=>ref.deref()!==undefined).length).toBe(0);
 });
},30000);

test("an equal row of another object shares the verdict and an edited file refutes it in the next scope",async()=>{
 const{root,rows}=await sources(),again=rows.map(row=>[structuredClone(row)]);
 await observeInferenceSnapshotV1(async()=>{
  for(const layer of rows.map(row=>[row]))expect(await observationLayerHoldsAsyncV1(layer,async()=>{})).toBe(true);
  for(const layer of again)expect(await observationLayerHoldsAsyncV1(layer,async()=>{})).toBe(true);
 });
 writeFileSync(join(root,"s","0.txt"),"edited source 0, longer than before");
 const edited=rows.map(row=>[structuredClone(row)]);
 await observeInferenceSnapshotV1(async()=>{
  const verdicts=[];for(const layer of edited)verdicts.push(await observationLayerHoldsAsyncV1(layer,async()=>{}));
  expect(verdicts.filter(holds=>!holds).length).toBe(1);
  expect(verdicts[0]).toBe(false);
 });
},30000);
