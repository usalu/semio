import {chmod,lstat,mkdir,rm,writeFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {blake3Hex as checksum} from "../../../../../🔏️hash/🟦️.ts";
import {basename,dirname,join,relative,resolve,isAbsolute,sep} from "node:path";
import {FileObservationErrorV1,readPhysicalFileV1,type FileObservationControlV1} from "../../../../../📁️filesystem/🧾️observation/🟦️.ts";
import {concurrentEditOfV1,concurrentEditV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {mapBoundedV1,CURRENT_PHYSICAL_CONCURRENCY_V1} from "../../../../../📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../💾️storage/🌐️platform/📋️contract/🟦️.ts";
import type {CargoSourceDriftV1} from "../../📥️source/👁️drift/🟦️.ts";
import {cargoDepInfoChecksumsV1,type CargoDepInfoChecksumV1,type CargoInputObservationV1} from "../🟦️.ts";
import schema from "../../../📤️publication/📥️sources/🧬️schema/🔣️.json" with {type:"json"};

const compilerArtifactExtension=/\.(?:rlib|rmeta|so|dylib|dll|lib|wasm|exe)$/u;

function within(root:string,path:string):boolean{
  const distance=relative(root,path);
  return !!distance&&!isAbsolute(distance)&&distance!==".."&&!distance.startsWith(".."+sep);
}

async function readAnnouncedFile(path:string,physical:FileObservationControlV1):Promise<Uint8Array|undefined>{
  const stat=()=>lstat(path).catch(error=>{
    if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;
  });
  const before=await stat();
  if(!before||before.size>physical.maxBytes)return;
  try{return (await readPhysicalFileV1(path,physical)).bytes;}
  catch(error){
    if(concurrentEditOfV1(error)||error instanceof FileObservationErrorV1&&(error.code==="changed"||error.code==="byte-capacity"))return;
    if(error instanceof FileObservationErrorV1&&error.code==="io"&&!await stat())return;
    throw error;
  }
}

/** 📥️ Reads bounded dep-info bytes associated with Cargo's announced artifact paths. */
export async function* announcedCargoSourceChecksumsV1(filenames:readonly string[],roots:readonly string[],physical:FileObservationControlV1,control:ResourceLeaseControl,executable?:string):AsyncGenerator<CargoDepInfoChecksumV1>{
  checkResourceLeaseControl(control);
  const candidates=new Set<string>();
  for(const filename of filenames){
    if(!isAbsolute(filename))continue;
    const file=resolve(filename);
    if(!roots.some(root=>within(root,file))||file!==executable&&!compilerArtifactExtension.test(file))continue;
    const stem=file.replace(compilerArtifactExtension,"");
    candidates.add(stem+".d");
    candidates.add(join(dirname(stem),basename(stem).replace(/^lib/u,"")+".d"));
  }
  for(const candidate of candidates){
    const info=await readAnnouncedFile(candidate,physical);
    checkResourceLeaseControl(control);
    if(info===undefined)continue;
    for(const claim of cargoDepInfoChecksumsV1(new TextDecoder().decode(info))){
      checkResourceLeaseControl(control);
      if(isAbsolute(claim.path)&&Number.isSafeInteger(claim.length)&&claim.length<=physical.maxBytes)yield {...claim,path:resolve(claim.path)};
    }
  }
}

/** ⚖️ Holds the first pair of incompatible consumed-byte claims for one attempt's admitted paths. */
export class CargoSourceAttemptV1 {
  private readonly admitted:ReadonlyMap<string,string>;
  private readonly first=new Map<string,{unit:string;fingerprint:string}>();
  private disagreement?:CargoSourceDriftV1;

  constructor(admitted:ReadonlyMap<string,string>){this.admitted=admitted;}

  get conflict():CargoSourceDriftV1|undefined{return this.disagreement;}

  record(unit:string,claim:CargoDepInfoChecksumV1):void{
    const {path,length,blake3}=claim;
    if(this.disagreement||!this.admitted.has(path))return;
    const fingerprint=blake3+":"+length,prior=this.first.get(path);
    if(!prior){this.first.set(path,{unit,fingerprint});return;}
    if(prior.unit!==unit&&prior.fingerprint!==fingerprint){
      const [expected,observed]=[prior.fingerprint,fingerprint].sort();
      this.disagreement={path,expected:expected!,observed:observed!};
    }
  }
}

type SourceBlobV1=Readonly<{path:string;sha256:string;blake3:string;length:number;stored:Promise<boolean>}>;

export interface CargoSourceAdmissionWitnessV1 {
  has(path:string,sha256:string):boolean;
  expect(path:string):void;
  remember(path:string,bytes:Uint8Array,control:ResourceLeaseControl):Promise<void>;
}

/** 📎️ Private, bounded input bodies captured by Source admission and selected only by rustc's consumed-byte checksum. */
export class CargoSourceWitnessV1 implements CargoSourceAdmissionWitnessV1 {
  private readonly blobs=new Map<string,SourceBlobV1>();
  private readonly paths=new Map<string,Map<string,SourceBlobV1>>();
  private readonly consumed=new Map<string,Readonly<{sha256:string|null;length?:number}>>();
  private readonly selected=new Map<string,SourceBlobV1>();
  private readonly unchecked=new Set<string>();
  private readonly budget:number;
  private readonly workspace:string;
  private readonly directory:string;
  private readonly storage:NativeStorageReservationV1;
  private associationCount=0;
  private bytes=0;
  private directoryReady?:Promise<void>;
  private directoryOwned=false;

  constructor(workspace:string,directory:string,storage:NativeStorageReservationV1,control:ResourceLeaseControl,maximumBytes=storage.allocation.bytes){
    requireNativeStorageReservationV1(storage,control);
    requireNativeStoragePathV1(storage.backing.root,directory);
    this.workspace=workspace;
    this.directory=directory;
    this.storage=storage;
    if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1)throw Error("Finite compiler source witness extent required");
    this.budget=Math.min(maximumBytes,storage.allocation.bytes,schema.properties.maximumRetainedCompilationBytes.const);
  }

  get retainedBytes():number{return this.bytes;}

  expect(path:string):void{
    const distance=relative(this.workspace,path);
    if(distance&&!isAbsolute(distance)&&distance!==".."&&!distance.startsWith(".."+sep)&&resolve(path)===path&&!this.paths.has(path)&&this.paths.size<schema.properties.maximumEntries.const)this.paths.set(path,new Map());
  }

  has(path:string,sha256:string):boolean{return this.paths.get(path)?.has(sha256)??false;}
  covers(path:string):boolean{return this.paths.has(path);}

  async remember(path:string,bytes:Uint8Array,control:ResourceLeaseControl):Promise<void>{
    checkResourceLeaseControl(control);
    this.expect(path);
    const versions=this.paths.get(path);
    if(!versions||this.associationCount>=schema.properties.maximumEntries.const)return;
    const sha256=createHash("sha256").update(bytes).digest("hex");
    if(this.has(path,sha256))return;
    let blob=this.blobs.get(sha256);
    if(!blob){
      if(this.blobs.size>=schema.properties.maximumFiles.const||bytes.byteLength>this.budget-this.bytes)return;
      this.bytes+=bytes.byteLength;
      const target=requireNativeStoragePathV1(this.storage.backing.root,join(this.directory,sha256));
      blob=Object.freeze({path:target,sha256,blake3:checksum(bytes),length:bytes.byteLength,stored:this.store(target,bytes,control)});
      this.blobs.set(sha256,blob);
    }
    if(!await blob.stored){if(this.blobs.get(sha256)===blob)this.blobs.delete(sha256);return;}
    checkResourceLeaseControl(control);
    if(this.associationCount<schema.properties.maximumEntries.const&&!versions.has(sha256)){versions.set(sha256,blob);this.associationCount++;}
  }

  private async store(path:string,bytes:Uint8Array,control:ResourceLeaseControl):Promise<boolean>{
    requireNativeStorageReservationV1(this.storage,control);
    try{
      await (this.directoryReady??=mkdir(this.directory).then(()=>{this.directoryOwned=true;}));
      checkResourceLeaseControl(control);
      await writeFile(path,bytes,{flag:"wx",mode:0o600});
      await chmod(path,0o400);
      checkResourceLeaseControl(control);
      return true;
    }catch(error){
      if((error as NodeJS.ErrnoException).code!=="ENOSPC")throw error;
      if(this.directoryOwned)await rm(path,{force:true});
      else this.directoryReady=undefined;
      this.bytes-=bytes.byteLength;
      return false;
    }
  }

  canTransfer(path:string,expected:CargoDepInfoChecksumV1):boolean{
    return [...this.paths.get(path)?.values()??[]].some(blob=>blob.blake3===expected.blake3&&blob.length===expected.length);
  }

  /** 🪪️ Confirms an announced checksum belongs to the specific admitted source bytes. */
  matchesAdmitted(expected:CargoDepInfoChecksumV1,sha256:string):boolean{
    const blob=this.paths.get(expected.path)?.get(sha256);
    return blob!==undefined&&blob.blake3===expected.blake3&&blob.length===expected.length;
  }

  /** 📌️ Resolves only a source version already selected under compiler input custody. */
  selectedPath(path:string,sha256:string):string|undefined{
    const blob=this.selected.get(path);
    return !this.unchecked.has(path)&&blob?.sha256===sha256?blob.path:undefined;
  }

  /** 🧭️ Keeps a source under live custody if any unit lacks checksum evidence for it. */
  noteUncheckedInput(path:string,sha256:string|null):void{
    this.checkVersion(path,sha256);
    this.unchecked.add(path);
  }

  /** 📖️ Holds an admitted body before native Cargo can start reading it. */
  async captureAdmitted(path:string,sha256:string,physical:FileObservationControlV1,control:ResourceLeaseControl):Promise<void>{
    this.expect(path);
    if(!this.covers(path)||this.has(path,sha256))return;
    const bytes=await readAnnouncedFile(path,physical);
    if(!bytes)throw concurrentEditV1("vanished",path,"Admitted compiler source vanished before capture","build");
    try{
      if(createHash("sha256").update(bytes).digest("hex")!==sha256)throw concurrentEditV1("bytes",path,"Admitted compiler source changed before capture","build");
      await this.remember(path,bytes,control);
    }finally{bytes.fill(0);}
  }

  /** 🔬️ Retains exact consumed bodies from announced dep-info without pinning compiler outputs or selecting provenance. */
  async captureAnnouncedSources(filenames:readonly string[],roots:readonly string[],physical:FileObservationControlV1,control:ResourceLeaseControl):Promise<void>{
    for await (const expected of announcedCargoSourceChecksumsV1(filenames,roots,physical,control))await this.captureClaim(expected,physical,control);
  }

  /** 🧷️ Retains an exact source body or reports stable evidence that an unwitnessed source changed. */
  async captureClaim(expected:CargoDepInfoChecksumV1,physical:FileObservationControlV1,control:ResourceLeaseControl):Promise<CargoSourceDriftV1|undefined>{
    const {path,length,blake3}=expected;
    if(!this.covers(path)||this.canTransfer(path,expected)||!Number.isSafeInteger(length)||length<0||length>physical.maxBytes)return;
    const source=await readAnnouncedFile(path,physical);
    checkResourceLeaseControl(control);
    if(source===undefined)return;
    const observed=checksum(source);
    if(source.byteLength===length&&observed===blake3)await this.remember(path,source,control);
    else if(!this.canTransfer(path,expected))return {path,expected:blake3+":"+length,observed:observed+":"+source.byteLength};
  }

  async input(path:string,expected:CargoDepInfoChecksumV1,physical:FileObservationControlV1):Promise<Readonly<{row:CargoInputObservationV1;retained:boolean}>>{
    const versions=this.paths.get(path);
    const blob=[...versions?.values()??[]].find(row=>row.blake3===expected.blake3&&row.length===expected.length);
    let sha256:string;
    if(blob){
      sha256=blob.sha256;
    }else{
      const {bytes,claim}=await readPhysicalFileV1(path,physical);
      if(bytes.byteLength!==expected.length||checksum(bytes)!==expected.blake3)throw concurrentEditV1("bytes",path,"Compiler consumed bytes have no matching retained source witness","snapshot");
      sha256=claim.sha256;
    }
    this.checkVersion(path,sha256,expected.length);
    if(blob)this.selected.set(path,blob);
    return {row:{path,kind:"file",sha256},retained:blob!==undefined};
  }

  checkVersion(path:string,sha256:string|null,length?:number):void{
    const previous=this.consumed.get(path);
    if(previous&&(previous.sha256!==sha256||previous.length!==undefined&&length!==undefined&&previous.length!==length)){
      const claim=(sha256:string|null,length?:number)=>length===undefined?`unchecked SHA-256 ${sha256??"absent"}`:`checksum-backed SHA-256 ${sha256} (${length} bytes)`;
      throw concurrentEditV1("bytes",path,`Compiler consumed inconsistent source versions (${claim(previous.sha256,previous.length)} versus ${claim(sha256,length)})`,"snapshot");
    }
    if(!previous||previous.length===undefined&&length!==undefined)this.consumed.set(path,Object.freeze({sha256,length}));
  }

  async recheck(physical:()=>FileObservationControlV1):Promise<void>{
    await mapBoundedV1([...new Set(this.selected.values())],CURRENT_PHYSICAL_CONCURRENCY_V1,async blob=>{
      const info=await lstat(blob.path);
      if(!info.isFile()||info.isSymbolicLink()||info.nlink!==1||info.size!==blob.length)throw Error("Actual retained compiler source witness required");
      const {bytes,claim}=await readPhysicalFileV1(blob.path,physical());
      if(claim.sha256!==blob.sha256||bytes.byteLength!==blob.length||checksum(bytes)!==blob.blake3)throw Error("Retained compiler source witness changed before transfer");
    });
  }

  async close():Promise<void>{
    if(this.directoryOwned)await rm(this.directory,{recursive:true,force:true});
  }
}
