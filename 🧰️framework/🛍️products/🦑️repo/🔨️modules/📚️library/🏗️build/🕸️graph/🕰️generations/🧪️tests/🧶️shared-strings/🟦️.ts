import{expect,test}from"bun:test";
import{createFrontierV1,createGenerationStoreV1}from"../../🟨️.mjs";
import{graphStringInternerV1}from"../../../../🧠️memory/🟨️.mjs";

/** 🗂️ A filesystem port over a map: a missing file reads as undefined, a content addressed write that finds its file does nothing. */
function memoryPort(){
 const files=new Map<string,Uint8Array>();
 return{
  async read(path:string){return files.get(path);},
  async write(path:string,bytes:Uint8Array,{ifAbsent=false}:{ifAbsent?:boolean}={}){if(ifAbsent&&files.has(path))return false;files.set(path,bytes);return true;},
  async names(directory:string){return[...files.keys()].filter(path=>path.startsWith(directory+"/")).map(path=>path.slice(directory.length+1));},
  async list(directory:string){return[...files.keys()].filter(path=>path.startsWith(directory+"/")).map(name=>({name:name.slice(directory.length+1),modifiedMs:0}));},
  async remove(path:string){files.delete(path);},
 };
}

test("stored fragments read back equal and share equal long strings",async()=>{
 const long="{workspaceRoot}/🧰️framework/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",owners=Array.from({length:6},(_,index)=>({name:"p"+index,root:"p/"+index,kind:"project" as const,node:{name:"p"+index,root:"p/"+index,targets:{build:{execution:{kind:"command",command:"echo "+index},inputs:[long,long+"?"+index]}}},edges:[],targets:["build"],rows:[{kind:"file",path:"/w/p/"+index+"/a.ts",fact:{size:index}}]}));
 const engine=createGenerationStoreV1({root:"/store",fs:memoryPort(),holds:async()=>true,resources:{facts:"facts",builder:"builder"}});
 const frontier=createFrontierV1({directories:[],members:[],capturedAt:0}),identity="a".repeat(64);
 const result=await engine.refresh({identity,implementation:"d".repeat(64),policy:"fresh",membership:async()=>({digest:"b".repeat(64),frontier,changes:[]}),compute:async()=>({owners,env:{digest:"c".repeat(64),rows:[]},global:{version:1,externalNodes:{},edges:[],unavailable:[]},replaceTags:[]})});
 const before=graphStringInternerV1.stats().hits,assembled=await engine.assemble(result.current,{});
 expect(Object.keys(assembled.projects)).toEqual(owners.map(owner=>owner.name));
 for(const owner of owners)expect(assembled.projects[owner.name]).toEqual(owner.node);
 expect(graphStringInternerV1.stats().hits).toBeGreaterThan(before);
});
