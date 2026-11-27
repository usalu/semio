import {resolve} from "node:path";
import {captureCheckoutSnapshotV1,checkoutSnapshotEnvironmentV1,forgetCheckoutSnapshotStepV1,pruneCheckoutSnapshotsV1,verifyCheckoutSnapshotV1} from "./🟦️.ts";

/** 🚪️ Captures, verifies or prunes checkout snapshots and prints one JSON document: `capture` (the build root and the environment that makes a build use it), `verify <root>` and `prune`. Progress goes to stderr; Ctrl-C cancels a capture.
 * @see ./🟦️.ts */
async function main(argv:readonly string[]):Promise<number>{
 const [command,...rest]=argv,option=(name:string):string|undefined=>rest.find(argument=>argument.startsWith(name+"="))?.slice(name.length+1);
 const workspace=resolve(option("--workspace")??process.env.SEMIO_WORKSPACE_ROOT??process.cwd()),controller=new AbortController(),onEvent=(text:string):void=>{process.stderr.write(text+"\n");};
 process.once("SIGINT",()=>controller.abort(Error("Checkout snapshot cancelled")));
 if(command==="capture"){
  const entries=option("--entries")?.split(",").filter(Boolean),step=option("--step"),owner=option("--owner")?Number(option("--owner")):undefined,snapshot=await captureCheckoutSnapshotV1({workspace,signal:controller.signal,onEvent,onProgress:event=>onEvent(event.text),...entries?{entries}:{},...step&&owner?{step,owner}:{}});
  const {rows:_rows,...facts}=snapshot;
  process.stdout.write(JSON.stringify({...facts,environment:checkoutSnapshotEnvironmentV1(snapshot)})+"\n");
  return 0;
 }
 if(command==="verify"&&rest[0]&&!rest[0].startsWith("--")){
  const result=await verifyCheckoutSnapshotV1(resolve(rest[0]));
  process.stdout.write(JSON.stringify(result)+"\n");
  return result.ok?0:1;
 }
 if(command==="forget"&&option("--step")){
  await forgetCheckoutSnapshotStepV1(workspace,option("--step")!);
  process.stdout.write(JSON.stringify({forgotten:option("--step")})+"\n");
  return 0;
 }
 if(command==="prune"){
  const keep=option("--keep")?.split(",").filter(Boolean),removed=await pruneCheckoutSnapshotsV1({workspace,onEvent,...keep?{keep}:{}});
  process.stdout.write(JSON.stringify({removed})+"\n");
  return 0;
 }
 process.stderr.write("usage: capture [--workspace=DIR] [--entries=a,b] [--step=NAME --owner=PID] | forget --step=NAME | verify ROOT | prune [--workspace=DIR] [--keep=ID,ID]\n");
 return 2;
}

try{process.exit(await main(process.argv.slice(2)));}catch(error){process.stderr.write((error instanceof Error?error.message:String(error))+"\n");process.exit(1);}
