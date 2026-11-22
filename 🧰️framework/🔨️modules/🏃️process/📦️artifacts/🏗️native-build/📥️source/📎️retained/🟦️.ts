import {createHash} from "node:crypto";
import {lstat,readdir} from "node:fs/promises";
import {isAbsolute,join,resolve} from "node:path";
import {FileObservationErrorV1,readPhysicalFileV1,type FileObservationControlV1} from "../../../../../📁️filesystem/🧾️observation/🟦️.ts";
import {concurrentEditV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../💾️storage/🟦️.ts";
import {CargoSourceAttemptV1,CargoSourceWitnessV1} from "../../🧾️receipt/📎️source-witness/🟦️.ts";
import {cargoDepInfoChecksumsV1,cargoProvenancePhysicalControlV1} from "../../🧾️receipt/🟦️.ts";
import type {AdmittedCargoSourceRosterV1} from "../🎟️admission/🟦️.ts";
import type {CargoAdmittedSourceV1,CargoSourceDriftV1} from "../👁️drift/🟦️.ts";
import schema from "../../../📤️publication/📥️sources/🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ Which admitted Source bytes an artifact provably consumed: the roster digest and every edit deferred while it compiled. */
export type CargoSourceProvenanceV1=Readonly<{version:1;files:number;sha256:string;proven:number;deferred:readonly Readonly<{path:string;admitted:string;observed:string}>[]}>;

/** 📖️ Holds every admitted body in the witness before a compiler can read it; shared by every Cargo compiler owner.
 * @see ../../🧾️receipt/📎️source-witness/🟦️.ts */
export async function captureAdmittedCargoSourcesV1(witness:CargoSourceWitnessV1,inputs:Iterable<CargoAdmittedSourceV1>,admitted:Map<string,string>,physical:FileObservationControlV1,control:ResourceLeaseControl):Promise<void>{
  for(const {path,digest} of inputs){
    const previous=admitted.get(path);
    if(previous===digest)continue;
    if(previous!==undefined)throw concurrentEditV1("bytes",path,"Admitted source digests disagree","build");
    admitted.set(path,digest);
    await witness.captureAdmitted(path,digest,physical,control);
    if(admitted.size%32===0)await control.yieldContinuation();
  }
}

/** ⏳️ Defers an edit to an admitted file whose exact body the witness holds, publishing it once per path; false means the edit must stop the compiler.
 * @see ../👁️drift/🟦️.ts */
export function deferCargoSourceDriftV1(witness:CargoSourceWitnessV1|undefined,deferred:Set<string>,mismatch:CargoSourceDriftV1,control:Pick<ResourceLeaseControl,"publish">,proof:"announced"|"pending"):boolean{
  if(!witness?.has(mismatch.path,mismatch.expected))return false;
  if(!deferred.has(mismatch.path)){
    deferred.add(mismatch.path);
    try{void Promise.resolve(control.publish({resource:"cargo:deferred-source-digest-mismatch:"+JSON.stringify({...mismatch,proof}),mode:"shared",elapsedMs:0})).catch(()=>{});}catch{}
  }
  return true;
}

const toleranceMs=2000;

async function* depInfoFiles(roots:readonly string[],control:ResourceLeaseControl):AsyncGenerator<Readonly<{path:string;mtimeMs:number}>>{
  const pending=[...roots];
  let entries=0;
  while(pending.length){
    const directory=pending.pop()!;
    checkResourceLeaseControl(control);
    const rows=await readdir(directory,{withFileTypes:true}).catch(()=>[]);
    for(const row of rows){
      if(++entries>schema.properties.maximumEntries.const)return;
      const path=join(directory,row.name);
      if(row.isDirectory())pending.push(path);
      else if(row.isFile()&&row.name.endsWith(".d")){
        const info=await lstat(path).catch(()=>undefined);
        if(info?.isFile())yield {path,mtimeMs:info.mtimeMs};
      }
    }
  }
}

/** 📎️ Compiles from the live tree but proves afterwards which admitted bytes it consumed: an edit after admission is deferred instead of
 * cancelling the compiler, and the artifact is accepted only when rustc's own consumed-byte checksums equal the admitted bodies.
 * A path edited while compiling and lacking such proof refuses with a concurrent edit, so the next attempt re-admits it.
 * @see ../../🧾️receipt/📎️source-witness/🟦️.ts */
export class RetainedCargoSourcesV1{
  readonly witness:CargoSourceWitnessV1;
  private readonly admitted:ReadonlyMap<string,string>;
  private readonly cargoWorkspace:string;
  private readonly deferred=new Map<string,CargoSourceDriftV1>();
  private readonly published=new Set<string>();

  private constructor(witness:CargoSourceWitnessV1,admitted:ReadonlyMap<string,string>,cargoWorkspace:string){
    this.witness=witness;
    this.admitted=admitted;
    this.cargoWorkspace=cargoWorkspace;
  }

  /** 🔒️ Captures the admitted roster's bodies into private storage; an edit between admission and capture refuses before compiling. */
  static async admit(roster:AdmittedCargoSourceRosterV1&Readonly<{cargoWorkspace?:string}>,options:Readonly<{workspace:string;directory:string;storage:NativeStorageReservationV1;control:ResourceLeaseControl;maxSourceBytes:number;maximumBytes?:number}>):Promise<RetainedCargoSourcesV1>{
    requireNativeStorageReservationV1(options.storage,options.control);
    const witness=new CargoSourceWitnessV1(options.workspace,options.directory,options.storage,options.control,options.maximumBytes??Math.floor(options.storage.allocation.bytes/4)),admitted=new Map<string,string>();
    try{
      const physical=cargoProvenancePhysicalControlV1({cancelled:()=>options.control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(options.control),onProgress:()=>checkResourceLeaseControl(options.control)});
      await captureAdmittedCargoSourcesV1(witness,roster.inputs,admitted,{...physical,maxBytes:options.maxSourceBytes},options.control);
    }catch(error){
      await witness.close();
      throw error;
    }
    return new RetainedCargoSourcesV1(witness,admitted,resolve(roster.cargoWorkspace??options.workspace));
  }

  /** 👁️ The drift callback: true keeps compiling because the admitted body is retained, false stops the compiler. */
  defer(mismatch:CargoSourceDriftV1,control:Pick<ResourceLeaseControl,"publish">):boolean{
    const held=deferCargoSourceDriftV1(this.witness,this.published,mismatch,control,"pending");
    if(held)this.deferred.set(mismatch.path,mismatch);
    return held;
  }

  /** 🔬️ Proves from the finished compiler's dep-info that every consumed admitted file equals its admitted body.
   * Records written before `sinceMs` belong to cached units and may only add proof; they never contradict the compile. */
  async prove(roots:readonly string[],sinceMs:number,control:ResourceLeaseControl):Promise<CargoSourceProvenanceV1>{
    const physical=cargoProvenancePhysicalControlV1({cancelled:()=>control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(control),onProgress:()=>checkResourceLeaseControl(control)});
    const attempt=new CargoSourceAttemptV1(this.admitted),matched=new Set<string>(),contradicted=new Map<string,string>();
    for await(const {path:file,mtimeMs} of depInfoFiles(roots,control)){
      let bytes:Uint8Array;
      try{bytes=(await readPhysicalFileV1(file,physical)).bytes;}
      catch(error){
        if(error instanceof FileObservationErrorV1&&error.code!=="cancelled"&&error.code!=="deadline")continue;
        throw error;
      }
      const recent=mtimeMs>=sinceMs-toleranceMs;
      for(const row of cargoDepInfoChecksumsV1(new TextDecoder().decode(bytes))){
        const claim={...row,path:isAbsolute(row.path)?resolve(row.path):resolve(this.cargoWorkspace,row.path)},sha256=this.admitted.get(claim.path);
        if(sha256===undefined||!this.witness.has(claim.path,sha256))continue;
        if(this.witness.matchesAdmitted(claim,sha256))matched.add(claim.path);
        else if(recent)contradicted.set(claim.path,claim.blake3+":"+claim.length);
        if(recent)attempt.record(file,claim);
      }
    }
    checkResourceLeaseControl(control);
    const conflict=attempt.conflict;
    if(conflict)throw concurrentEditV1("bytes",conflict.path,"Compiler consumed inconsistent Source versions: "+JSON.stringify(conflict),"build");
    for(const [path,observed] of contradicted)throw concurrentEditV1("bytes",path,"Compiler consumed Source that differs from admission: "+observed,"build");
    for(const [path,mismatch] of this.deferred)if(!matched.has(path))throw concurrentEditV1("bytes",path,"No consumed-byte checksum proves the admitted version of an edited Source: "+JSON.stringify(mismatch),"build");
    const digest=createHash("sha256");
    for(const [path,sha256] of [...this.admitted].sort(([left],[right])=>left<right?-1:left>right?1:0))digest.update(path+"\0"+sha256+"\n");
    return Object.freeze({version:1,files:this.admitted.size,sha256:digest.digest("hex"),proven:matched.size,deferred:Object.freeze([...this.deferred.values()].map(row=>Object.freeze({path:row.path,admitted:row.expected,observed:row.observed})))});
  }

  async close():Promise<void>{await this.witness.close();}
}
