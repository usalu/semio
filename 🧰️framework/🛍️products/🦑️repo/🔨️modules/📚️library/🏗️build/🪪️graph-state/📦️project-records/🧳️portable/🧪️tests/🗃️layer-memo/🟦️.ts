import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{closureKeyV1,hostOfRootV1,layerPackMemoV1,packBundleV1,portableRowsPolicyV1,unpackBundleV1,type ClosureBundleV1,type KeyMapV1,type ObservationRowV1}from"../../🟦️.ts";

const root="/Users/Shared/semio",host=hostOfRootV1(root),generation=".cache/b/d/m/j/0123456789abcdef0123";
const tool=(name:string)=>closureKeyV1({tool:name}),toolKeys=["a","b","c"].map(tool);
const core:readonly(readonly ObservationRowV1[])[]=Array.from({length:40},(_,layer)=>Object.freeze([
 {kind:"file",path:`${root}/core/${layer}.ts`,fact:{size:layer,mtimeMs:7,ctimeMs:8,ino:9,sha256:"f".repeat(64)}},
 {kind:"exists",path:`${root}/core/${layer}/node_modules`,fact:layer%2===0},
 {kind:"lstat",path:"/Users",fact:"directory"},
 {kind:"resolve",path:JSON.stringify([`${root}/core/${layer}.ts`,`./${(layer+1)%40}.ts`]),fact:`${root}/core/${(layer+1)%40}.ts`},
 {kind:"exists",path:`${root}/${generation}/node_modules/x${layer}`,fact:true},
 ...layer%10===0?[{kind:"closure",path:JSON.stringify([`${root}/core/${layer}.ts`,toolKeys[layer%3]!]),fact:`${root}/core`}]:[],
]));
const bundle=(owner:number,layers:readonly(readonly ObservationRowV1[])[]):Readonly<{recipeKey:string;bundle:ClosureBundleV1}>=>{
 const recipe={fn:"target-sources",root,owner};
 return{recipeKey:closureKeyV1(recipe),bundle:{version:1,recipe,value:[`${root}/own/${owner}.ts`],rows:[{kind:"file",path:`${root}/own/${owner}.ts`,fact:{size:owner}}],layers,sidecar:null} as unknown as ClosureBundleV1};
};
const owners=Array.from({length:9},(_,owner)=>bundle(owner,[[{kind:"exists",path:`${root}/own/${owner}`,fact:true}],...core.filter((_,at)=>(at+owner)%4!==0)]));
const pack=(input:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>,keys:KeyMapV1,memo?:ReturnType<typeof layerPackMemoV1>)=>{const packed=packBundleV1(input,host,keys,portableRowsPolicyV1,memo);expect(packed).toBeDefined();return{bytes:JSON.stringify(packed!.document),consulted:[...packed!.consulted].sort()};};
const neutralKeys=(suffix:string):KeyMapV1=>new Map(toolKeys.map(key=>[key,closureKeyV1({tool:key,suffix})]));

test("nine bundles over one shared core pack to the same documents and consulted keys through the process memo as without it, each layer converted once",()=>{
 const memo=layerPackMemoV1(host,1<<20),keys=neutralKeys("n");
 for(const input of owners)expect(pack(input,keys,memo)).toEqual(pack(input,keys));
 expect(memo.misses).toBe(40+9);
 expect(memo.hits).toBe(owners.reduce((sum,input)=>sum+[...input.bundle.layers].length,0)-memo.misses);
});

test("a remembered layer is converted again when the key map answers one of its closure keys otherwise, and the document follows the new answer",()=>{
 const memo=layerPackMemoV1(host,1<<20),first=neutralKeys("one"),second=new Map([...first].map(([key,value],at)=>[key,at===0?closureKeyV1({moved:key}):value]));
 for(const input of owners)pack(input,first,memo);
 const missesBefore=memo.misses;
 for(const input of owners){const through=pack(input,second,memo),fresh=pack(input,second);expect(through).toEqual(fresh);expect(fresh.consulted.some(([key,value])=>key===toolKeys[0]&&value===second.get(toolKeys[0]!))).toBe(true);}
 expect(memo.misses-missesBefore).toBe(core.filter((_,at)=>at%10===0&&at%3===0).length);
 const gone=new Map([...first].slice(1));
 for(const input of owners)expect(pack(input,gone,memo)).toEqual(pack(input,gone));
});

test("past its row limit the memo starts empty and packs the same bytes; another host or policy never uses it",()=>{
 const small=layerPackMemoV1(host,24),keys=neutralKeys("s");
 for(const input of owners)expect(pack(input,keys,small)).toEqual(pack(input,keys));
 expect(small.held).toBeLessThanOrEqual(24);
 const foreign=layerPackMemoV1(hostOfRootV1("/srv/other"),1<<20);
 for(const input of owners)expect(pack(input,keys,foreign)).toEqual(pack(input,keys));
 expect(foreign.hits+foreign.misses).toBe(0);
});

test("every packed layer is in the canonical order of the independent Python oracle",()=>{
 const memo=layerPackMemoV1(host,1<<20),keys=neutralKeys("o"),directory=mkdtempSync(join(tmpdir(),"layer-memo-"));
 try{
  for(const input of owners.slice(0,3)){
   const layers=unpackBundleV1(JSON.parse(pack(input,keys,memo).bytes)).layers as Iterable<readonly ObservationRowV1[]>;
   for(const layer of layers){
    const file=join(directory,"rows.json");writeFileSync(file,JSON.stringify({rows:[...layer].reverse()}));
    const run=spawnSync("python3",["-I",join(import.meta.dir,"../🔢️canonical-order/🐍️.py"),file],{encoding:"utf8"});
    expect(run.status,run.stderr).toBe(0);
    expect(JSON.parse(run.stdout)).toEqual([...layer]);
   }
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});
