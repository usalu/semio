import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../🟨️.mjs";
import{createClosureStoreV1}from"../💾️entries/🟨️.mjs";
import{bundleKeysV1,hostOfRootV1,packBundleV1,unpackBundleV1}from"../../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";

/** 🔢️ The bundle of a closure entry is a function of what the recipe reads: two machines that derived the same entry in another order of rows export the same bytes, so the fleet cache sees no divergence for them; its sidecar names the derived facts of its layers and nothing else (a command walk is a `closure-graph` row of the entry, no layer). */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`closure-canon-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const recipe={fn:"native-command"},image=JSON.stringify({name:"alpha",closureRecipes:{[closureRecipeKeyV1(recipe)]:recipe}});
const question=(name:string)=>JSON.stringify({derive:1,workspaceRoot:"⟦semio-root⟧",recipe:{fn:"entries",name}});
const names=["a","🧱️b","c","😀️d","e","f","g","h"];
const rowsOf=(root:string,order:readonly string[])=>order.map(name=>({kind:"exists",path:`${root}/${name}/📋️project.json`,fact:true}));
const forward=names,backward=[...names].reverse();

async function derive(label:string,order:readonly string[]){
 const root=temp(label);mkdirSync(join(root,"ce"),{recursive:true});
 const own=question("own"),store=createClosureStoreV1({directory:join(root,"ce")});
 const facts=[null,{row:{kind:"derived",path:own,fact:{files:2}},views:{}}];
 await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:[`{workspaceRoot}/x.ts`],rows:rowsOf(root,order.filter(name=>name==="a"||name==="c")),layers:[rowsOf(root,order),[{kind:"exists",path:`${root}/own/📜️script.ts`,fact:true}]],sidecar:{facts}});
 bindClosureStoreV1(root,join(root,"ce"),true);
 const bundles=await closureTravelV1.export(root,image),host=hostOfRootV1(root),keys=bundleKeysV1(bundles,host,"neutral");
 const packed=bundles.map(entry=>packBundleV1(entry,host,keys)!.document);
 return{raw:JSON.stringify(packed),bundle:unpackBundleV1(packed[0]!),packed};
}

test("two machines that derived one entry in another order export byte-identical bundles: rows and layers in canonical order, a sidecar of facts only",async()=>{
 const a=await derive("a",forward),b=await derive("b",backward);
 expect(a.raw).toBe(b.raw);
 const sidecar=(a.bundle as unknown as {sidecar:Record<string,unknown>}).sidecar;
 expect(Object.keys(sidecar)).toEqual(["facts"]);
});

test("the exported bundle equals an independent Python reading of the canonical form (rows and layers sorted as UTF-16 code units, a sidecar of facts only)",async()=>{
 const a=await derive("o",backward);
 const file=join(temp("oracle"),"bundle.json"),b=a.bundle as unknown as {rows:unknown[];layers:Iterable<unknown[]>;sidecar:unknown};
 writeFileSync(file,JSON.stringify({rows:b.rows,layers:[...b.layers],sidecar:b.sidecar}));
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🐍️oracle","🔢️canonical.py"),file],{encoding:"utf8"});
 expect(python.status).toBe(0);
 expect(JSON.parse(python.stdout)).toEqual({rowsSorted:true,layersSorted:[true,true],factsOnly:true});
});
