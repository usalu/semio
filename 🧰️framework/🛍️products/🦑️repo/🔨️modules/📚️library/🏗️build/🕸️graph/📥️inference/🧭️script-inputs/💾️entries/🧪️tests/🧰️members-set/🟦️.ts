/** 🧰️ Contract: the walk members of a stored layer are compared with the members of the next loaded entry through one set held per member list, not through a set built from the whole list for every layer of every entry that is loaded (a take pass loaded the entries of every owner, 200 layers each, and built a set of the members of every layer again each time). */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createClosureStoreV1}from"../../🟨️.mjs";

test("loading many entries that name the same layers with members in another order builds one set per layer, not one per load",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-members-set-"));
 const NativeSet=globalThis.Set;let built=0;
 try{
  const directory=join(root,"ce"),store=createClosureStoreV1({directory}),pool=40,layersPer=30,walks=150,entries=60;
  const layer=(key:number)=>[{kind:"file",path:"/layer/"+key,fact:{size:key}}],fact=(key:number)=>({kind:"walk",path:"/walk/"+key});
  const members=(key:number,entry:number)=>{const base=Array.from({length:walks},(_all,at)=>"walk:"+key+":"+at+":"+"x".repeat(30));return[...base.slice(entry%7),...base.slice(0,entry%7)];};
  const pick=(entry:number)=>Array.from({length:layersPer},(_all,at)=>(entry*7+at)%pool);
  for(let entry=0;entry<entries;entry++){const keys=pick(entry);await store.save("entry-"+entry,{recipe:{fn:"fixture"},value:[entry],rows:[],layers:keys.map(layer),sidecar:{facts:keys.map(fact),members:keys.map(key=>members(key,entry))}});}
  const reader=createClosureStoreV1({directory,readOnly:true}),held:any[]=[];
  class CountingSet<T>extends NativeSet<T>{constructor(items?:Iterable<T>|null){super(items);if(items!==undefined&&items!==null&&(items as any).length>=100)built++;}}
  (globalThis as any).Set=CountingSet;
  for(let entry=0;entry<entries;entry++)held.push(await reader.load("entry-"+entry));
  (globalThis as any).Set=NativeSet;
  expect(held.every(item=>item?.sidecar?.members.length===layersPer)).toBe(true);
  expect(new NativeSet(held[3].sidecar.members[4]).size).toBe(walks);
  expect(built).toBeLessThanOrEqual(pool);
 }finally{(globalThis as any).Set=NativeSet;rmSync(root,{recursive:true,force:true});}
});
