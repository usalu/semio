/** 🪦️ Contract: a closure entry that a take found stale is judged once per round: the next look of the same round is a miss at once, without loading the stored entry again, and a round that starts later (or a store that holds another entry) judges it anew. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

test("a stale entry is loaded and judged once per round, a later round judges it again",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("stale-take-");roots.push(root);
 write(root,{"tools/a.mjs":'import "./b.mjs";\nexport const a=1;\n',"tools/b.mjs":"export const b=2;\n"});
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,join(root,".ce"),true);
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),entry=join(root,"tools/a.mjs");
 const handle=scriptInputs.closureHandleOfV1(await api.closureOf([entry],[],"test-discovery")),recipes=[[handle.key,handle.recipe]];
 await api.resolve(recipes);
 write(root,{"tools/b.mjs":'import "./c.mjs";\nexport const b=2;\n',"tools/c.mjs":"export const c=3;\n"});
 const later=new Date(Date.now()+5000);utimesSync(join(root,"tools/b.mjs"),later,later);
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,join(root,".ce"),false);scriptInputs.closureApiV1(scripts,root);
 const stale=()=>scriptInputs.closureEntryCountsV1().stale as number,start=stale();
 const take=()=>scriptInputs.runClosureTakeV1(()=>api.resolve(recipes));
 await scriptInputs.withClosurePassV1(async()=>{
  await expect(take()).rejects.toMatchObject({code:"BUILD_TAKE_MISS"});
  expect(stale()).toBe(start+1);
  await expect(take()).rejects.toMatchObject({code:"BUILD_TAKE_MISS"});
  await expect(take()).rejects.toMatchObject({code:"BUILD_TAKE_MISS"});
  expect(stale()).toBe(start+1);
 });
 await scriptInputs.withClosurePassV1(async()=>{
  await expect(take()).rejects.toMatchObject({code:"BUILD_TAKE_MISS"});
  expect(stale()).toBe(start+2);
 });
}));
