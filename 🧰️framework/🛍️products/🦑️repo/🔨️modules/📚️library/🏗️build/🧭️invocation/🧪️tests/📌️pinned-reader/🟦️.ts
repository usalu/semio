/** 📌️ Contract: a nested child process starts without a Graph reader, so a reusing invocation load opens one itself and reaches the pinned snapshot check instead of refusing for a missing reader. Independent oracle: the same pin through the bare primitive refuses for the missing reader. A refusal inside an issued scope also fails that scope, so each refusal is asserted around its own scope. */
import{expect,test}from"bun:test";
import{createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{loadBuildGraphV1}from"../../../🕸️graph/🟦️.ts";
import{loadBuildInvocationGraphV1}from"../../🕸️graph/🟦️.ts";
import{withGraphTestInvocationV1}from"../../../🧪️tests/🕸️graph/🧰️support/🟦️.ts";

const pin={path:"/nonexistent/semio-pinned-reader-law/snapshot.json",digest:"0".repeat(64)};

const withPin=async(body:()=>Promise<void>)=>{
 const previous={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};
 process.env.SEMIO_BUILD_GRAPH_PATH=pin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pin.digest;
 try{await body();}finally{if(previous.path===undefined)delete process.env.SEMIO_BUILD_GRAPH_PATH;else process.env.SEMIO_BUILD_GRAPH_PATH=previous.path;if(previous.digest===undefined)delete process.env.SEMIO_BUILD_GRAPH_DIGEST;else process.env.SEMIO_BUILD_GRAPH_DIGEST=previous.digest;}
};

test("the bare primitive refuses a reusing load that holds no Graph reader",()=>withPin(async()=>{
 await expect(withGraphTestInvocationV1(async original=>{await loadBuildGraphV1(process.cwd(),original,createIssuedResourceControlV1(original,"graph:pinned-reader-bare",0),{selection:{kind:"all"},reuse:true});})).rejects.toThrow(/Required original cache (consumer|reader) unavailable/u);
}),60000);

test("a reusing invocation load opens its own Graph reader and reaches the pinned snapshot identity check",()=>withPin(async()=>{
 await expect(withGraphTestInvocationV1(async original=>{await loadBuildInvocationGraphV1(process.cwd(),["show","projects"],original,original,createIssuedResourceControlV1(original,"graph:pinned-reader-opened",0),true);})).rejects.toThrow("Original retained Graph snapshot identity required");
}),60000);

test("a neutral child is its own root and never inherits the pinned graph of its parent",()=>withPin(async()=>{
 await withGraphTestInvocationV1(async original=>{
  const{projectOwnedBuildChildEnvironmentV1}=await import("../../../📥️caller/📤️child/🟦️.ts"),environment=await projectOwnedBuildChildEnvironmentV1(process.cwd(),["show","projects"],original,createIssuedResourceControlV1(original,"graph:pinned-reader-child",0));
  expect(process.env.SEMIO_BUILD_GRAPH_PATH).toBe(pin.path);
  for(const name of["SEMIO_BUILD_GRAPH_PATH","SEMIO_BUILD_GRAPH_DIGEST","SEMIO_BUILD_REUSE_GRAPH"])expect(environment[name]).toBeUndefined();
 });
}),60000);
