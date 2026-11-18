/** 🧒️ One graph read of the stopped-wave laws: a process that infers a fixed list of synthetic owners one after the other through the real project record path of a private repository (the same dataset, quota ledger and record files a wave of the group server uses), tells every step on stdout, and can stop itself between two owners or inside one so that the parent can signal it. It runs under `bun test` only because the build test envelope needs the test runner; its single test is the read.
 * @see ../🟦️.ts */
import{test}from"bun:test";
import{createHash}from"node:crypto";
import{join,relative}from"node:path";
const B="../../../../../../",library=await import(new URL(B+"../🧪️testing/🧫️private-repository/🟦️.ts",import.meta.url).href),birth=await import(new URL(B+"../🧪️testing/🪪️native-birth/🟦️.ts",import.meta.url).href),routing=await import(new URL(B+"../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url).href);
const api=await import(new URL(B+"📥️inputs/💾️storage/🟨️.mjs",import.meta.url).href) as any,observation=await import(new URL(B+"👁️observation/🟨️.mjs",import.meta.url).href) as any,present=await import(new URL("../../../📋️present/🟨️.mjs",import.meta.url).href) as any;
type Config={source:string;owners:number;stopAfter:number;stallIn:number;session:string;shared:number;key:string};
const config=JSON.parse(process.env.R60_CHILD!) as Config,say=(line:string)=>process.stdout.write("R60 "+line+"\n");
const workspace=await library.privateRepositoryWorkspaceV1("graph");
birth.declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});
const identity=createHash("sha256").update(config.source).digest("hex");
let computes=0;
test("stopped-wave read",async()=>{
await workspace.invoke((original:any)=>api.withRepositoryBuildInferenceDatasetV1(workspace.root,original,routing.createIssuedResourceControlV1(original,"graph:stopped-wave",0),{version:1,identity,sourceRoot:relative(workspace.root,config.source).replaceAll("\\","/")},async(dataset:any)=>{
 say("ROOT "+dataset.root);
 const outer=join(dataset.root,"inference","emoji-project-json.json");
 await observation.observeInferenceSnapshotV1(()=>observation.observeInference({cachePath:outer,key:["outer",config.session]},async()=>{
  let finished=0;
  for(let index=0;index<config.owners;index++){
   const owner="owner-"+index,cachePath=join(dataset.root,"np",present.ownerRecordKeyV1(owner)+".json"),files=[join(config.source,"in"+index+".txt"),...index===config.owners-1?[join(config.source,"in"+config.shared+".txt")]:[]];
   await observation.observeProjectInferenceV1({cachePath,key:["r60",index,config.key],owner},async()=>{
    computes++;say("COMPUTE "+index);
    if(config.stallIn===index){say("STALLED "+index);await new Promise(()=>setInterval(()=>{},1000));}
    await new Promise(resolve=>setTimeout(resolve,60));
    return{owner:index,text:files.map(file=>observation.memoizedFileRecord("r60",file,(bytes:Buffer)=>bytes.toString()))};
   });
   finished++;say("DONE "+index);
   if(config.stopAfter===finished){say("READY "+finished);await new Promise(()=>setInterval(()=>{},1000));}
  }
  return{finished};
 }));
}),{owner:"stopped-wave-child",maximumElapsedMilliseconds:600000});
say("FINISHED "+computes);
process.exit(0);
},600000);
