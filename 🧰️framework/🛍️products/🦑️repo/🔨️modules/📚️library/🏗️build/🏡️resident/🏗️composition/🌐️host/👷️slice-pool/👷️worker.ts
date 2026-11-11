/** 👷 One slice load in its own process (`🧵️process`): the body of the host service's `loadSlice` (the graph of one request's selection under the service's own birth) so that its synchronous phases never block the service's event loop. */
import{parentPort,workerData}from"./🧵️process/🟨️.mjs";
import{heapStats}from"bun:jsc";
const MiB=1048576,sample=():void=>{const heap=heapStats();parentPort!.postMessage({type:"memory",heapMb:Math.round(heap.heapSize/MiB*10)/10,extraMb:Math.round(heap.extraMemorySize/MiB*10)/10,objectCount:heap.objectCount,objectTypes:Object.entries(heap.objectTypeCounts).sort((a,b)=>b[1]-a[1]).slice(0,8).map(([type,count])=>({type,count}))});};
sample();const beat=setInterval(sample,workerData.heartbeatMilliseconds??2000);beat.unref();
const routing=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const{loadBuildInvocationGraphV1}=await import("../../../../🧭️invocation/🕸️graph/🟦️.ts");
const{withRepositoryCompilerRuntimeEntriesV1}=await import("../../../../📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts");
const{publishBuildBootstrapSourcesV1}=await import("../../../../🚀️bootstrap/🟦️.ts");
const{buildBackendCompactGraphV1,loadBuildBackendV1,buildBackendDefaultIdV1}=await import("../../../../🔌️backends/🟦️.ts");
const{sealedGenerationPinV1}=await import("../../../../🕸️graph/🟦️.ts"),{onGenerationProgressV1}=await import("../../../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs");
onGenerationProgressV1((event:{phase:string;done:number;total:number;running?:unknown;decision?:unknown})=>parentPort!.postMessage({type:"progress",...event}));
try{
  await routing.receiveScriptProcessInvocation(process.env,async(original:any)=>{
    const control=routing.createIssuedResourceControlV1(original,"graph:slice-worker",0),tool=workerData.tool??buildBackendDefaultIdV1(),backend=await loadBuildBackendV1(tool);
    publishBuildBootstrapSourcesV1(workerData.root);
    const graph:any=await withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control,async()=>loadBuildInvocationGraphV1(workerData.root,workerData.arguments,original,original,control,false,undefined,workerData.graphPolicy,buildBackendCompactGraphV1(tool),await backend.graphExternalInputs?.()??[]));
    parentPort!.postMessage({type:"done",graph,generationPin:sealedGenerationPinV1(tool),...graph?.stats===undefined?{}:{stats:graph.stats}});
  });
}catch(error:any){parentPort!.postMessage({type:"error",message:String(error?.message??error),stack:String(error?.stack??""),...error?.name==="RepoError"&&typeof error.code==="string"&&typeof error.key==="string"?{refusal:{code:error.code,key:error.key,values:{...error.values}}}:{}});}
