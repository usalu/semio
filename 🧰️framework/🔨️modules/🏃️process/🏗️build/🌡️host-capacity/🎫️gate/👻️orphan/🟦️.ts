/** 👻️ Tells when the client process that holds a grant has lost the reader of its standard output: the pipe broke (`repo show | grep | head` whose consumer ended) so writing to it fails with `EPIPE`, or the stream was destroyed or ended. A reparented process (`nohup`, `setsid`, a CI wrapper) is not orphaned and keeps its grant, and a standard output that is a file or the null device never breaks. */
export type HostOrphanWatchV1=Readonly<{lost:()=>boolean;stop:()=>void}>;

/** 🔭️ Builds the orphan watch from its facts, so the decision is testable without killing a process: `outputClosed` says whether the standard output can no longer be written. */
export function createHostOrphanWatchV1(facts:Readonly<{outputClosed:()=>boolean;stop:()=>void}>):HostOrphanWatchV1{
 return Object.freeze({lost:facts.outputClosed,stop:facts.stop});
}

/** 🧭️ The orphan watch of this very process: it notes a broken pipe on its standard output (`EPIPE`) and reads the stream state, and its stop is a termination signal to itself (the cleanup path then gives the grant back like any ended run). */
export function currentHostOrphanWatchV1():HostOrphanWatchV1{
 let broken=false;
 process.stdout.on("error",(error:NodeJS.ErrnoException)=>{if(error.code==="EPIPE")broken=true;});
 return createHostOrphanWatchV1({outputClosed:()=>broken||process.stdout.destroyed===true||process.stdout.writableEnded===true,stop:()=>{process.kill(process.pid,"SIGTERM");}});
}
