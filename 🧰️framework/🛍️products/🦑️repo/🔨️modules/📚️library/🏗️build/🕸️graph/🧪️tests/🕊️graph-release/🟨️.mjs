import{join}from"node:path";
const[library,count]=process.argv.slice(2),{releaseBuildGraphV1,requireLiveBuildGraphV1}=await import(join(library,"🏗️build/🕸️graph/🟦️.ts"));
const payload=seed=>Array.from({length:4000},(_,index)=>"{workspaceRoot}/"+seed+"/"+index+"/"+Math.random().toString(36).repeat(8));
const synthetic=projects=>({version:1,projects:Object.fromEntries(Array.from({length:projects},(_,index)=>{const name="p"+index;return[name,{name,root:"r/"+name,targets:{build:{execution:{kind:"noop"},inputs:payload(index)}}}];})),dependencies:[],namedInputs:{}});
const heap=()=>{Bun.gc(true);return process.memoryUsage().heapUsed;};
const graph=synthetic(Number(count)),kept=graph.projects.p1;
await new Promise(resolve=>setTimeout(resolve,300));
const before=heap();
await releaseBuildGraphV1(graph);
await new Promise(resolve=>setTimeout(resolve,300));
const after=heap(),refused=[];
for(const read of[()=>graph.projects,()=>graph.dependencies,()=>requireLiveBuildGraphV1(graph)])try{read();refused.push(false);}catch(error){refused.push(String(error.message).includes("build.graph.released-read"));}
await releaseBuildGraphV1(graph);
process.stdout.write(JSON.stringify({before,after,refused,keys:Object.keys(graph),survivor:kept.targets.build.inputs.length}));
