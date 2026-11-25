/** 🧊️ Probe: a `closure-graph` global row recorded from a fresh answer holds right after, stops holding when a file of its closure changes, and a fresh answer recorded after the edit holds again, in a process that binds the script cache and in one that binds none. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
const inference=await import("../../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const scriptInputs=await import("../../🟨️.mjs") as any,observation=await import("../../../../../👁️observation/🟨️.mjs") as any;
const roots:string[]=[],line=(name:string)=>["import","\"."+"/"+name+".mjs\";\n"].join(" ");
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,name:string,text:string,at?:Date)=>{mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);if(at)utimesSync(join(root,name),at,at);};
const segments={roots:["{workspaceRoot}/tools/a.mjs"],requiredSources:false,resources:[],selectors:[],stops:[],view:null,provider:null};
const row=(root:string,fact:unknown)=>({kind:"global",path:JSON.stringify(["closure-graph",root,segments]),fact:JSON.stringify(fact)});
const holds=(root:string,fact:unknown)=>scriptInputs.withClosurePassV1(()=>observation.observationsHold([row(root,fact)]));

test("a global closure-graph row follows the tree through edits of its closure",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("global-row-");roots.push(root);
 write(root,"tools/a.mjs",line("b"));write(root,"tools/b.mjs","export const b=1;\n");
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,join(root,".ce"),true);
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true});scriptInputs.bindClosureScriptsV1(root,scripts);
 const question={roots:[join(root,"tools/a.mjs")],requiredSources:false,resources:[],selectors:[],stops:[],view:undefined};
 const first=await scriptInputs.withClosurePassV1(()=>scriptInputs.closureGraphAnswerV1(scripts,root,question));
 expect(await holds(root,first)).toBe(true);
 write(root,"tools/c.mjs","export const c=1;\n");write(root,"tools/b.mjs",line("c")+"export const b=2;\n",new Date(Date.now()+5000));
 expect(await holds(root,first)).toBe(false);
 const second=await scriptInputs.withClosurePassV1(()=>scriptInputs.closureGraphAnswerV1(scripts,root,question));
 expect(second.digest).not.toBe(first.digest);
 expect(await holds(root,second)).toBe(true);
 expect(await holds(root,first)).toBe(false);
 scriptInputs.resetClosureEntriesV1();
 expect(await holds(root,second)).toBe(true);
 expect(await holds(root,first)).toBe(false);
}));
