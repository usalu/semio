/** 🚰️ Contract: a binding publication inside the producer loop pays no physical flush of its own (a Windows flush forces the disk and cost about two seconds, three per binding), the loop ends with one directory flush before the ready receipts are read, and a stale selected-retirement request is refused before the payload read. */
import{expect,test}from"bun:test";
import{mkdtempSync,mkdirSync,rmSync,readlinkSync}from"node:fs";
import{readFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{takeArtifactRelativeNativeProfileV1,withArtifactRelativeDirectoryV1,beginArtifactRelativeDeferredFlushV1,artifactRelativeFlushDeferredV1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";

const text=(relative:string)=>readFileSync(fileURLToPath(new URL(relative,import.meta.url)),"utf8");
const producer=text("../../🟦️.ts"),layer=text("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"),publication=text("../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts");

const publishCount=async(root:string,deferred:boolean,target:string)=>{
 const release=deferred?beginArtifactRelativeDeferredFlushV1():()=>{};takeArtifactRelativeNativeProfileV1();
 try{await withArtifactRelativeDirectoryV1(join(root,"p"),()=>{},async directory=>{const expected=await directory.captureBinding("node_modules");await directory.publishBinding(expected,{kind:"directory",path:target});});}finally{release();}
 return takeArtifactRelativeNativeProfileV1().fsyncSync?.[0]??0;
};

test("a binding publication flushes once on its own and not at all while the flush is deferred, and the link is the same",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79-flush-"));
 try{mkdirSync(join(root,"p"));mkdirSync(join(root,"a"));mkdirSync(join(root,"b"));
  expect(await publishCount(root,false,join(root,"a"))).toBe(1);expect(readlinkSync(join(root,"p","node_modules"))).toBe(join(root,"a"));
  expect(await publishCount(root,true,join(root,"b"))).toBe(0);expect(readlinkSync(join(root,"p","node_modules"))).toBe(join(root,"b"));
  expect(artifactRelativeFlushDeferredV1()).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the deferral nests, releases once and never leaks",()=>{
 const outer=beginArtifactRelativeDeferredFlushV1(),inner=beginArtifactRelativeDeferredFlushV1();expect(artifactRelativeFlushDeferredV1()).toBe(true);
 inner();inner();expect(artifactRelativeFlushDeferredV1()).toBe(true);outer();expect(artifactRelativeFlushDeferredV1()).toBe(false);
});

test("the Windows binding flush honours the deferral, the loop releases it in a finally and ends with one directory flush before the receipts",()=>{
 expect(layer).toContain('if(artifactRelativeFlushDeferredV1())return;if(!call(()=>kernel.symbols.FlushFileBuffers!(child.value)))throw failure("binding flush")');
 const begin=producer.indexOf("releaseDeferred=beginArtifactRelativeDeferredFlushV1();try{"),finish=producer.indexOf("}finally{releaseDeferred();}"),barrier=producer.indexOf("directory=>directory.sync()",finish),ready=producer.indexOf("await verifyReady();const finals");
 expect(begin).toBeGreaterThan(0);expect(finish).toBeGreaterThan(begin);expect(barrier).toBeGreaterThan(finish);expect(ready).toBeGreaterThan(barrier);
 expect(producer.slice(begin,finish)).toContain("publishJavascriptDependencyPreservingSourceDescriptorV1(binding");
});

test("a stale selected-retirement request is refused before the payload read",()=>{
 const call=publication.indexOf("requireRepositoryDependencySelectionCurrentV1(request,bindings,candidate);"),read=publication.indexOf("readJavascriptDependencySelectedPayloadV1(context,current,request.selection.prefix,members)");
 expect(call).toBeGreaterThan(0);expect(read).toBeGreaterThan(call);
 expect(publication).toContain('current.sha256!==selection.currentBindingsSha256)throw Error("Every current dependency scope must be qualified before inactivity")');
});
