/** 🌊️ Contract: what a read of many waves holds after wave N is what it held after wave 1 for each wave, not more: the row log of a finished wave keeps only the offsets, lengths and ids of its rows (the key table that deduplicates rows while they arrive is dropped when the wave is shaped), the carry of the build is named in the report of retained memory and grows by the same amount for every wave, the report names the row logs and the memos of the process, and the release of a wave leaves nothing in the collections that belong to a wave. A closed form of the open addressed key table (slots double while two slots stand for each row, 20 bytes for each) is the independent model of the bytes the report names.
 * @see ../../🗃️row-log/🟨️.mjs `seal`
 * @see ../../🟨️.mjs `carryCollectionsV1` */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {GENERATION_POLICY_V1} from "../../🟨️.mjs";
import {createRowLogV1,rowLogCollectionsV1,type RowLogListV1} from "../../🗃️row-log/🟨️.mjs";
import {collectSliceCollectionsV1,SLICE_REPORT_POLICY_V1} from "../../../../🧠️memory/📋️slice-report/🟨️.mjs";
import {registerRetainedCollectionsV1} from "../../../../🧠️memory/🧽️settle/🟨️.mjs";
import {graphStringInternerV1} from "../../../../🧠️memory/🟨️.mjs";
import {releaseWaveStateV1} from "../../../🕊️release/🟦️.ts";
import {createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const WAVES=8,OWNERS=48,SHARED=200,OWN=400;
const scratch=(name:string)=>mkdtempSync(join(realpathSync(tmpdir()),name));
const everything={...SLICE_REPORT_POLICY_V1,collectionsTop:100000,collectionsBytes:2**30};
const row=(wave:number,owner:number,at:number)=>({kind:"file",path:`/w/pkg/w${wave}/o${owner}/src/f${at}.ts`,fact:{size:at,mtimeMs:1.7e12+at,ino:wave*1e6+owner*1e3+at,sha256:String(wave*1e6+owner*1e3+at).padStart(64,"0")}});
const shared=(wave:number,at:number)=>({kind:"file",path:`/w/shared/w${wave}/f${at}.ts`,fact:{size:at,mtimeMs:1.7e12,ino:wave*1e6+at,sha256:String(at).padStart(64,"0")}});
const ownRows=(wave:number,owner:number)=>[...Array.from({length:SHARED},(_unused,at)=>shared(wave,at)),...Array.from({length:OWN},(_unused,at)=>row(wave,owner,at))];
const tableBytesOf=(rows:number)=>{let slots=1024;while(rows*2>slots)slots*=2;return slots*5*4;};
const retained=()=>{const rows=rowLogCollectionsV1();return{tables:rows[0]!.bytes,offsets:rows[1]!.bytes,rows:rows[0]!.entries,names:rows.map(item=>item.name)};};
const edit=async(dir:string,count:number)=>{await Bun.sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const waved=${at};\n`);};

const runWaves=(dir:string,seal:boolean)=>{
 const kept:RowLogListV1<any>[][]=[],logs:ReturnType<typeof createRowLogV1<any>>[]=[],series:{tables:number;offsets:number;rows:number}[]=[];
 for(let wave=0;wave<WAVES;wave++){
  const log=createRowLogV1<any>({path:join(dir,seal?"sealed":"open","rl",`w${wave}.rows`)});logs.push(log);
  const lists=Array.from({length:OWNERS},(_unused,owner)=>{const builder=log.builder();builder.add(ownRows(wave,owner));return builder.finish();});
  log.settle();
  const environment=log.builder();environment.add([shared(wave,0),{kind:"stamp",path:`inferrer:${wave}`,fact:"1"}]);lists.push(environment.finish());
  if(seal)log.seal();
  kept.push(lists);series.push(retained());
 }
 return{kept,logs,series};
};

test("a sealed row log keeps no key table: the bytes after wave N grow by the bytes of wave 1 for every wave, an unsealed one by several times as much, and the rows stay readable by id",()=>{
 const dir=scratch("wave-retention-rows-");
 try{
  const sealed=runWaves(dir,true);
  expect(retained().names[0]).toContain("0 open of "+WAVES);
  const sealedBytes=sealed.series.map(point=>point.tables+point.offsets),sealedFirst=sealedBytes[0]!;
  expect(sealed.series.every(point=>point.tables===0)).toBe(true);
  for(let wave=1;wave<WAVES;wave++)expect(Math.abs(sealedBytes[wave]!-sealedBytes[wave-1]!-sealedFirst)/sealedFirst,`growth of wave ${wave}`).toBeLessThan(0.05);
  const distinct=SHARED+OWNERS*OWN+1;
  expect(sealed.series[0]!.rows).toBe(distinct);
  const byPath=(a:{path:string},b:{path:string})=>a.path<b.path?-1:1;
  for(let wave=0;wave<WAVES;wave++)for(const owner of [0,17,OWNERS-1]){
   const list=sealed.kept[wave]![owner]!,expected=ownRows(wave,owner).map(item=>JSON.parse(JSON.stringify(item)));
   expect([...list.rows()].sort(byPath)).toEqual(expected.sort(byPath));
  }
  expect(()=>sealed.logs[0]!.id(shared(0,1))).toThrow("sealed");
  for(const log of sealed.logs)log.close();

  const open=runWaves(dir,false),openGrowth=(open.series[WAVES-1]!.tables+open.series[WAVES-1]!.offsets)/WAVES;
  expect(open.series[0]!.tables,"closed form of the key table of one wave").toBe(tableBytesOf(distinct));
  expect(openGrowth/sealedFirst,"unsealed bytes of a wave over sealed bytes of a wave").toBeGreaterThan(3);
  process.stdout.write(`[wave-retention] per wave: sealed ${sealedFirst} B, unsealed ${Math.round(openGrowth)} B (key table ${open.series[0]!.tables} B)\n`);
  for(const log of open.logs)log.close();
  expect(retained().rows).toBe(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the report names the row logs, the memos of the inputs and the observation, and the release of a wave empties the collections of the wave",async()=>{
 await registerRetainedCollectionsV1();
 const names=collectSliceCollectionsV1(everything).map(item=>item.name);
 for(const name of ["graph:rowLogs.keyTables","graph:rowLogs.offsets","inputs:inventoryMemo","inputs:verifiedClosures","observation:witnessStrings","observation:fileShaMemo","closure:verdictSeen","typescript:sharedSourceFiles"])expect(names.some(item=>item.startsWith(name)),name).toBe(true);
 graphStringInternerV1.share("a wave string that is long enough to be interned by the process");
 await releaseWaveStateV1();
 const after=new Map(collectSliceCollectionsV1(everything).map(item=>[item.name.replace(/ \(.*$/u,""),item]));
 for(const name of ["inputs:inventoryMemo","inputs:verifiedClosures","inputs:chunkCuts","closure:verdictSeen","typescript:sharedSourceFiles","memory:graphStringInterner"])expect(after.get(name)?.entries,name).toBe(0);
});

test("the carry of a build over many waves is named and grows by the same bytes for every wave, and the build forgets it when it ends",async()=>{
 const dir=scratch("wave-retention-carry-"),count=GENERATION_POLICY_V1.ownerWave*5+7,points:Record<string,{entries:number;estimatedBytes:number}>[]=[];
 try{
  await seed(dir,ringFiles(count+3));
  const world=createWorld(dir);
  await world.refresh();
  await edit(dir,count);
  const result=await world.refresh({wave:()=>{points.push(Object.fromEntries(collectSliceCollectionsV1(everything).filter(item=>item.name.startsWith("generation:")).map(item=>[item.name.replace(/ \(.*$/u,""),item])));}});
  expect(result.published).toBe(true);expect(result.computed).toBe(count);
  expect(points.length).toBe(5);
  const first=points[0]!["generation:carry.upserts"]!.estimatedBytes/points[0]!["generation:carry.upserts"]!.entries;
  for(let wave=0;wave<points.length;wave++){
   const point=points[wave]!;
   expect(point["generation:carry.upserts"]!.entries,`owners with rows after wave ${wave+1}`).toBe(GENERATION_POLICY_V1.ownerWave*(wave+1));
   expect(point["generation:carry.owners"]!.entries,"the owner table is the whole workspace from the first wave").toBe(count+3);
   const perOwner=point["generation:carry.upserts"]!.estimatedBytes/point["generation:carry.upserts"]!.entries;
   expect(Math.abs(perOwner-first)/first,`bytes of the rows of an owner after wave ${wave+1}`).toBeLessThan(0.02);
   if(wave>0)expect(point["generation:wave.frontier"]!.entries).toBe(points[1]!["generation:wave.frontier"]!.entries);
  }
  expect(collectSliceCollectionsV1(everything).some(item=>item.name.startsWith("generation:carry"))).toBe(false);
 }finally{rmSync(dir,{recursive:true,force:true});}
},120_000);
