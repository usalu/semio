import{afterAll,expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{CACHE_SIZE_FILE_V1,cacheAreaBudgetBytesV1,cacheLedgerV1,cacheRootDirectoryV1}from"../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import{ledgerOverBudgetAreasV1}from"../🟦️.ts";
import fixture from"./🔣️.json";

const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const ledger=cacheLedgerV1(),sample=ledger.areas.find(area=>area.collect!==undefined&&cacheAreaBudgetBytesV1(ledger,area)!==null)!,budget=cacheAreaBudgetBytesV1(ledger,sample)!;
const workspaceWith=(content:string|undefined)=>{const root=mkdtempSync(join(tmpdir(),"ledger-pressure-"));roots.push(root);if(content!==undefined){const directory=cacheRootDirectoryV1(root,ledger,"repository");mkdirSync(directory,{recursive:true});writeFileSync(join(directory,CACHE_SIZE_FILE_V1),content);}return root;};

test("the recorded sizes decide: an area is over only above its budget, equal to the SQLite comparison, and an absent or torn ledger is never over",async()=>{
 const db=new Database(":memory:");
 try{
  for(const row of fixture.cases){
   const records=row.usedFactor===null?{}:{[sample.id]:{usedBytes:Math.round(budget*row.usedFactor),partial:false,atMs:1}},root=workspaceWith(JSON.stringify({version:1,records})),over=await ledgerOverBudgetAreasV1(root),model=row.usedFactor===null?0:(db.query("select ?1>?2 as over").get(Math.round(budget*row.usedFactor),budget)as{over:number}).over;
   expect(over.includes(sample.id),row.name).toBe(row.over);expect(Boolean(model),row.name).toBe(row.over);
  }
  expect(await ledgerOverBudgetAreasV1(workspaceWith(undefined))).toEqual([]);
  expect(await ledgerOverBudgetAreasV1(workspaceWith("{torn"))).toEqual([]);
 }finally{db.close();}
});
