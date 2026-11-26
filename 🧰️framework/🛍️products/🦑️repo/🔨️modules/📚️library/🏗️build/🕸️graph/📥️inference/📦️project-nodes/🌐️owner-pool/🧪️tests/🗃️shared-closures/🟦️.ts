/** 🗃️ Law: the pool children of one machine share one bounded closure store outside their staging directories, so the entries a child derived for the shared tool and provider groups are loaded, not walked again, by the child that starts after it (a pool child is recycled after almost every owner); an entry whose files moved is derived anew, and the store is bound by the pool policy.
 * @see ../../🔬️inference/🟨️.mjs `ownerPoolClosureStoreV1`
 * @see ../../🧬️schema/🔣️.json */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P}from"../../🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});

const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const scriptInputs=await import("../../../../🧭️script-inputs/🟨️.mjs") as any,driver=await import("../../🔬️inference/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

test("the shared pool store lies beside the staging directories under the machine's fast cache and is bounded by the pool policy",()=>{
 const shared=driver.ownerPoolClosureStoreV1("/srv/semio");
 expect(shared.directory.endsWith(join(".cache","f",P.closureStoreDirectory))).toBe(true);
 expect(shared.limits).toEqual({closureStoreMaximumEntries:P.closureStoreMaximumEntries,closureStoreMaximumLayers:P.closureStoreMaximumLayers,closureStoreMaximumValues:P.closureStoreMaximumValues});
});

test("a child that starts after another loads what it derived and derives only what moved",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("pool-closures-");roots.push(root);
 write(root,{"tools/a.mjs":'import "./b.mjs";\nexport const a = 1;\n',"tools/b.mjs":"export const b = 1;\n"});
 const store=join(root,".pool-closures"),limits={closureStoreMaximumEntries:64,closureStoreMaximumLayers:256,closureStoreMaximumValues:64};
 const child=async()=>{
  scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,store,true,limits);
  const scripts=inference.createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),handle=scriptInputs.closureHandleOfV1(await api.closureOf([join(root,"tools/a.mjs")],[],"test-discovery"));
  const before=scriptInputs.closureEntryCountsV1();
  await scriptInputs.withClosurePassV1(()=>api.resolve([[handle.key,handle.recipe]]));
  const after=scriptInputs.closureEntryCountsV1();
  return{derived:after.derived-before.derived,loaded:after.loaded-before.loaded};
 };
 expect(await child()).toEqual({derived:1,loaded:0});
 expect(await child()).toEqual({derived:0,loaded:1});
 const later=new Date(Date.now()+5000);writeFileSync(join(root,"tools/b.mjs"),'import "./c.mjs";\nexport const b = 2;\n');writeFileSync(join(root,"tools/c.mjs"),"export const c = 1;\n");utimesSync(join(root,"tools/b.mjs"),later,later);
 expect((await child()).derived).toBe(1);
 expect(await child()).toEqual({derived:0,loaded:1});
}));
