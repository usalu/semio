import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createClosureStoreV1}from"../🟨️.mjs";

/** 💾️ Loaded closure entries that name the same stored layers share the walk facts and walk members of a layer: the sidecar of a layer exists once however many entries name it (measured on the real store: 274 layers per entry, 3 766 entries held 4 GB of per-entry copies on a worker taking 180 owners). */
test("many loaded entries that share their layers hold the sidecar of a layer once, not once per entry",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-entries-share-"));
 try{
  const directory=join(root,"ce"),store=createClosureStoreV1({directory}),pool=300,layersPer=200,walks=150,entries=120;
  const layer=(key:number)=>[{kind:"file",path:"/layer/"+key,fact:{size:key}}],members=(key:number,entry=0)=>[...Array.from({length:walks},(_all,at)=>"walk:"+key+":"+at+":"+"x".repeat(40)),...entry%2===1?["extra:"+key]:[]],fact=(key:number)=>({kind:"walk",path:"/walk/"+key,fact:{members:key}});
  const pick=(entry:number)=>Array.from({length:layersPer},(_all,at)=>(entry*7+at)%pool);
  for(let entry=0;entry<entries;entry++){const keys=pick(entry);await store.save("entry-"+entry,{recipe:{fn:"fixture"},value:[entry],rows:[],layers:keys.map(layer),sidecar:{facts:keys.map(fact),members:keys.map(key=>members(key,entry))}});}
  const reader=createClosureStoreV1({directory,readOnly:true});
  Bun.gc(true);const before=process.memoryUsage().heapUsed,held:any[]=[];
  for(let entry=0;entry<entries;entry++)held.push(await reader.load("entry-"+entry));
  Bun.gc(true);const grown=(process.memoryUsage().heapUsed-before)/1048576;
  expect(held.every(item=>item?.sidecar?.members.length===layersPer&&item.digests.length===layersPer)).toBe(true);
  expect(held[2].sidecar.members[12]).toEqual(members(26,1));
  expect(held[1].sidecar.facts[0]).toEqual(fact(7));
  expect(held[2].sidecar.members[12]).toBe(held[3].sidecar.members[5]);
  expect(held[0].sidecar.members[26]).toBe(held[3].sidecar.members[5]);
  expect(new Set(held.flatMap(item=>item.sidecar.members)).size).toBeLessThanOrEqual(pool);
  expect(held[0].sidecar.facts[7]).toBe(held[1].sidecar.facts[0]);
  expect(grown).toBeLessThan(10);
 }finally{rmSync(root,{recursive:true,force:true});}
});
