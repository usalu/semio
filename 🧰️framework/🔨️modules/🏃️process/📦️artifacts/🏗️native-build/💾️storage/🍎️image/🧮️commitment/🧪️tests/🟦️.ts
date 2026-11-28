/** 🧮️ Tests for the host-wide commitment budget: reserve, ceiling, admission and the layout invariant against an independent oracle and hand-checked rows, and the measurement of real receipts and ledgers. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{appendNativeImageShardEventV1}from"../../🧩️shard/🟦️.ts";
import{NATIVE_IMAGE_COMMITMENT_V1,measureNativeImageCommitmentV1,nativeImageCommitmentAdmitV1,nativeImageCommitmentCeilingV1,nativeImageCommitmentDeclarableBytesV1,nativeImageCommitmentLayoutFitsV1,nativeImageCommitmentRequestCostBytesV1,nativeImageCommitmentRefusedTextV1,nativeImageCommitmentReserveBytesV1,nativeImageCommitmentRowCostV1,nativeImageCommitmentSumV1}from"../🟦️.ts";
const GiB=1073741824,MiB=1048576;
const oracle=(...args:string[])=>{const python=testPythonV1();if(python===null)throw Error("Python required by the commitment oracle");const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8"});if(result.status!==0)throw result.error??Error(result.stderr);return JSON.parse(result.stdout.trim());};
const nativeImageCommitmentRowsV1=(rows:readonly{capacityBytes:number;allocatedBytes:number}[])=>rows.reduce((sum,row)=>sum+nativeImageCommitmentRowCostV1(row),0);
const answers=oracle("answers",join(import.meta.dir,"🔣️.json"));
test("the corpus is schema-valid",()=>{expect(validateJsonSchemaSubset(schema.definitions.Corpus,corpus,schema)).toEqual([]);expect(NATIVE_IMAGE_COMMITMENT_V1.reservePercent).toBe(8);});
test("the reserve and the ceiling equal the oracle and the hand-checked rows",()=>{
 const ours=corpus.ceiling.map(row=>({reserveBytes:nativeImageCommitmentReserveBytesV1(row.hostCapacityBytes),ceilingBytes:nativeImageCommitmentCeilingV1(row)}));
 expect(ours).toEqual(answers.ceiling);expect(ours).toEqual(corpus.ceiling.map(row=>({reserveBytes:row.reserveBytes,ceilingBytes:row.ceilingBytes})));
 expect(nativeImageCommitmentReserveBytesV1(228*GiB)).toBe(Math.ceil(228*GiB*0.08/GiB)*GiB);expect(nativeImageCommitmentReserveBytesV1(100*GiB)).toBe(16*GiB);
});
test("a host that falls below its reserve promises nothing more, and one with room above it promises the overcommitted free bytes on top of what is written",()=>{
 const base={hostCapacityBytes:228*GiB,allocatedBytes:80*GiB};
 expect(nativeImageCommitmentCeilingV1({...base,hostFreeBytes:14*GiB})).toBe(80*GiB);
 expect(nativeImageCommitmentCeilingV1({...base,hostFreeBytes:0})).toBe(80*GiB);
 const wide=nativeImageCommitmentCeilingV1({...base,hostFreeBytes:41*GiB});expect(wide).toBeGreaterThan(80*GiB);expect(wide).toBeLessThanOrEqual(228*GiB-nativeImageCommitmentReserveBytesV1(228*GiB));
 expect(nativeImageCommitmentCeilingV1({...base,hostFreeBytes:228*GiB})).toBe(Math.floor((228*GiB-nativeImageCommitmentReserveBytesV1(228*GiB))/GiB)*GiB);
});
test("the admission equals the oracle: a request inside the ceiling is admitted, one beyond it names the shortfall reclaiming must cover",()=>{
 const ours=corpus.admission.map(row=>nativeImageCommitmentAdmitV1(row));
 expect(ours).toEqual(answers.admission);expect(ours).toEqual(corpus.admission.map(row=>({admitted:row.admitted,roomBytes:row.roomBytes,shortfallBytes:row.shortfallBytes})));
 const tight=nativeImageCommitmentAdmitV1({ceilingBytes:100*GiB,committedBytes:90*GiB,requestBytes:16*GiB});expect(tight).toEqual({admitted:false,roomBytes:10*GiB,shortfallBytes:6*GiB});
 expect(nativeImageCommitmentAdmitV1({ceilingBytes:100*GiB,committedBytes:90*GiB,requestBytes:10*GiB}).admitted).toBe(true);
});
test("every image of every platform stays at or below the cap it can return blocks from: 32 GiB on macOS, so a 36 or 96 GiB container is no layout",()=>{
 const ours=corpus.layout.map(row=>nativeImageCommitmentLayoutFitsV1(row.platform,row.capacityBytes));
 expect(ours).toEqual(answers.layout);expect(ours).toEqual(corpus.layout.map(row=>row.fits));
 expect(nativeImageCommitmentLayoutFitsV1("darwin",32*GiB)).toBe(true);expect(nativeImageCommitmentLayoutFitsV1("darwin",36*GiB)).toBe(false);expect(nativeImageCommitmentLayoutFitsV1("darwin",96*GiB)).toBe(false);expect(nativeImageCommitmentLayoutFitsV1("freebsd",GiB)).toBe(false);
});
test("an image promises what it holds plus a bounded growth reserve, never more than it declares, so nearly empty sparse images leave room for the one that must grow",()=>{
 const ours=corpus.sum.map(row=>nativeImageCommitmentRowsV1(row.rows));
 expect(ours).toEqual(answers.sum);expect(ours).toEqual(corpus.sum.map(row=>row.committedBytes));
 const sparse=[{capacityBytes:26*GiB,allocatedBytes:GiB},{capacityBytes:30*GiB,allocatedBytes:2*GiB},{capacityBytes:30*GiB,allocatedBytes:2*GiB},{capacityBytes:16*GiB,allocatedBytes:16*GiB}];
 const sum=nativeImageCommitmentSumV1({hostCapacityBytes:228*GiB,hostFreeBytes:36*GiB},sparse.map((row,index)=>({id:"i"+index,...row})));
 expect(sum.declaredBytes).toBe(102*GiB);expect(sum.committedBytes).toBe((5+6+6+16)*GiB);expect(sum.roomBytes).toBeGreaterThanOrEqual(16*GiB);
 expect(nativeImageCommitmentRowCostV1({capacityBytes:GiB,allocatedBytes:0})).toBe(GiB);
});
test("the sum counts every row by what it promises and by what it declares, and leaves the room under the ceiling",()=>{
 const sum=nativeImageCommitmentSumV1({hostCapacityBytes:228*GiB,hostFreeBytes:41*GiB},[{id:"r",capacityBytes:32*GiB,allocatedBytes:20*GiB},{id:"r/p1",capacityBytes:16*GiB,allocatedBytes:GiB},{id:"d",capacityBytes:32*GiB,allocatedBytes:19*GiB}]);
 expect(sum.declaredBytes).toBe(80*GiB);expect(sum.committedBytes).toBe((24+5+23)*GiB);expect(sum.allocatedBytes).toBe(40*GiB);expect(sum.ceilingBytes).toBe(nativeImageCommitmentCeilingV1({hostCapacityBytes:228*GiB,hostFreeBytes:41*GiB,allocatedBytes:40*GiB}));expect(sum.roomBytes).toBe(Math.max(0,sum.ceilingBytes-52*GiB));
});
test("the measurement reads the receipts of the home images and the ledger of the shards below a cache base",()=>{
 const scratch=realpathSync(mkdtempSync(join(testScratchRootV1("image-tests"),"commitment-")));
 try{
  const home=join(scratch,"r");mkdirSync(home);writeFileSync(join(home,"image.json"),JSON.stringify({version:1,maximumPhysicalBytes:32*GiB}));writeFileSync(join(home,"fixed.dmg"),"x");
  appendNativeImageShardEventV1(home,{type:"born",id:"pabc",key:"pabc",root:0,path:"u/l/pabc",capacityBytes:16*GiB,atMs:1});
  appendNativeImageShardEventV1(home,{type:"born",id:"pold",key:"pold",root:0,path:"u/l/pold",capacityBytes:8*GiB,atMs:2});
  appendNativeImageShardEventV1(home,{type:"retired",id:"pold",atMs:3});
  const other=join(scratch,"d");mkdirSync(other);writeFileSync(join(other,"image.json"),JSON.stringify({version:1,maximumPhysicalBytes:32*GiB}));
  const junk=join(scratch,"c");mkdirSync(junk);writeFileSync(join(junk,"image.json"),"not json");
  const row=measureNativeImageCommitmentV1(scratch);
  expect(row.rows.map(item=>[item.id,item.capacityBytes])).toEqual([["d",32*GiB],["r",32*GiB],["r/pabc",16*GiB]]);expect(row.declaredBytes).toBe(80*GiB);expect(row.committedBytes).toBeGreaterThanOrEqual(12*GiB);expect(row.committedBytes).toBeLessThan(12*GiB+MiB);
  expect(row.hostCapacityBytes).toBeGreaterThan(row.hostFreeBytes);expect(row.ceilingBytes).toBe(nativeImageCommitmentCeilingV1(row));expect(row.roomBytes).toBe(Math.max(0,row.ceilingBytes-row.committedBytes));
 }finally{rmSync(scratch,{recursive:true,force:true});}
});
test("the refusal names what is asked, what is declared and the ceiling in both languages",()=>{
 const values={need:"16.0 GiB",committed:"150.0 GiB",ceiling:"156.0 GiB"};
 expect(nativeImageCommitmentRefusedTextV1("en",values)).toContain("16.0 GiB");expect(nativeImageCommitmentRefusedTextV1("de",values)).toContain("Obergrenze von 156.0 GiB");
});
test("a birth promises its growth reserve and a short room bounds its declaration, equal to the oracle: an oversized declaration no longer starves the room",()=>{
 expect(corpus.cost.map(row=>nativeImageCommitmentRequestCostBytesV1(row.declaredBytes))).toEqual(answers.cost);expect(answers.cost).toEqual(corpus.cost.map(row=>row.costBytes));
 expect(corpus.declarable.map(row=>nativeImageCommitmentDeclarableBytesV1(row.roomBytes)??null)).toEqual(answers.declarable);expect(answers.declarable).toEqual(corpus.declarable.map(row=>row.declarableBytes));
 const row=nativeImageCommitmentAdmitV1({ceilingBytes:122*GiB,committedBytes:119*GiB,requestBytes:nativeImageCommitmentRequestCostBytesV1(16*GiB)});
 expect(row.admitted).toBe(false);expect(nativeImageCommitmentAdmitV1({ceilingBytes:124*GiB,committedBytes:119*GiB,requestBytes:nativeImageCommitmentRequestCostBytesV1(16*GiB)}).admitted).toBe(true);
});
