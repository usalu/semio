import{join}from"node:path";
const[mode,library,file]=process.argv.slice(2),observation=await import(join(library,"🏗️build/👁️observation/🟨️.mjs")),inference=await import(join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
if(mode==="record"){
 const{observations}=await observation.captureObservedSourceInputsV1(()=>observation.observedSourceAnalysisV1(inference.prepareSourceAnalysesV1().module,file),undefined,false);
 process.stdout.write(JSON.stringify(observations.filter(row=>row.kind==="analysis")));
}else{
 const rows=JSON.parse(await Bun.stdin.text()),verdict=async()=>(await Promise.all(rows.map(row=>observation.observationsHold([row],1,new Set(),new Set())))).every(Boolean);
 const before=await verdict();inference.prepareSourceAnalysesV1();
 process.stdout.write(JSON.stringify({rows:rows.length,before,after:await verdict()}));
}
