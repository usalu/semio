import { devLocalHubDataDir } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync, lstatSync, mkdirSync, writeFileSync, existsSync, realpathSync, openSync, closeSync, readSync, fstatSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

export type TrustedCatalogByteClaimV1 = Readonly<{relativePath:string;sha256:string;byteLength:number}>;
export type ObservationClaimV1 = Readonly<{path:string;sha256:string;byteLength:number}>;
export type ActorProducerV1 = Readonly<{schema:"semio.os.closed-browser-actor-producer/v1";actor:TrustedCatalogByteClaimV1;component:TrustedCatalogByteClaimV1;descriptor:TrustedCatalogByteClaimV1;policyCanonical:string;policySha256:string;runtime:ObservationClaimV1;compiler:ObservationClaimV1;inputs:readonly Readonly<{logicalPath:string;path:string;sha256:string;byteLength:number}>[]}>;
export type PackageProducerV1 = Readonly<{pluginId:string;packageId:string;cargoPackage:string;component:TrustedCatalogByteClaimV1;descriptor:TrustedCatalogByteClaimV1;cargoInvocations:readonly ObservationClaimV1[];browserActor:ObservationClaimV1|null}>;
export type GenerationProducerV1 = Readonly<{schema:"semio.hub.trusted-catalog-producer/v1";profileId:string;generationId:string;bundle:TrustedCatalogByteClaimV1;packages:readonly PackageProducerV1[]}>;
export type TrustedCatalogPublicationProducerV1 = Readonly<{schema:"semio.hub.trusted-catalog-publication-producer/v1";dataRoot:string;profileId:string;generationId:string;publicationRevision:string;pointer:{path:string;sha256:string;byteLength:number};bundle:{path:string;sha256:string;byteLength:number};producer:{path:string;sha256:string;byteLength:number}}>;

function bytes(path:string):Buffer { const stat=lstatSync(path);if(!stat.isFile()||stat.isSymbolicLink()||stat.size>128*1024*1024)throw new Error("trusted producer input must be a bounded regular file");return readFileSync(path); }
export function trustedCatalogPhysicalClaimV1(path:string,check:()=>void=()=>{}):ObservationClaimV1 {
  check();const before=lstatSync(path);if(!before.isFile()||before.isSymbolicLink()||!Number.isSafeInteger(before.size))throw new Error("trusted producer byte owner refused");
  const file=openSync(path,"r"),hash=createHash("sha256"),chunk=Buffer.alloc(1024*1024);let length=0;
  try{const opened=fstatSync(file);if(opened.dev!==before.dev||opened.ino!==before.ino||opened.size!==before.size)throw new Error("trusted producer input changed at open");for(;;){check();const count=readSync(file,chunk,0,chunk.length,null);if(count===0)break;length+=count;if(length>before.size)throw new Error("trusted producer input grew");hash.update(chunk.subarray(0,count));}const after=fstatSync(file),linked=lstatSync(path);if(length!==before.size||[after,linked].some(stat=>stat.isSymbolicLink()||stat.dev!==before.dev||stat.ino!==before.ino||stat.size!==before.size||stat.mtimeMs!==before.mtimeMs||stat.ctimeMs!==before.ctimeMs))throw new Error("trusted producer input changed while observed");return {path,sha256:hash.digest("hex"),byteLength:length};}finally{closeSync(file);}
}
function parsedClaim(path:string,claim:ObservationClaimV1):any {const input=bytes(path);if(input.length!==claim.byteLength||createHash("sha256").update(input).digest("hex")!==claim.sha256)throw new Error("trusted producer parsed bytes changed");return JSON.parse(input.toString("utf8"));}
function inside(root:string,path:string):boolean {const child=relative(root,path);return child!==""&&!isAbsolute(child)&&child.split(/[\\/]/u)[0]!=="..";}
function owned(root:string,path:string):void {
  if(!isAbsolute(root)||!isAbsolute(path)||!inside(root,path))throw new Error("trusted producer physical owner escaped");
  for(let parent=dirname(path);;parent=dirname(parent)){const stat=lstatSync(parent);if(!stat.isDirectory()||stat.isSymbolicLink())throw new Error("trusted producer directory owner refused");if(parent===root)break;if(!inside(root,parent))throw new Error("trusted producer ancestor escaped");}
  const actual=realpathSync(existsSync(path)?path:dirname(path)),owner=realpathSync(root);if(actual!==owner&&!inside(owner,actual))throw new Error("trusted producer physical path escaped");
}
export function createTrustedCatalogProvenanceDirectoryV1(root:string,path:string):void {if(!inside(root,path))throw new Error("trusted producer directory escaped");const rootStat=lstatSync(root);if(!rootStat.isDirectory()||rootStat.isSymbolicLink())throw new Error("trusted producer root refused");let parent=root;for(const part of relative(root,path).split(/[\\/]/u)){parent=join(parent,part);if(!existsSync(parent))mkdirSync(parent,{mode:0o700});const stat=lstatSync(parent);if(!stat.isDirectory()||stat.isSymbolicLink())throw new Error("trusted producer directory owner refused");}owned(root,join(path,".ownership"));}
function matches(path:string,claim:{sha256:string;byteLength:number},root:string):void {owned(root,path);const observed=trustedCatalogPhysicalClaimV1(path);if(observed.sha256!==claim.sha256||observed.byteLength!==claim.byteLength)throw new Error("trusted producer bytes changed: "+path);}

/** 🧾️ Observes a generation-owned file at the actual producer boundary. */
export function trustedCatalogByteClaimV1(root:string,path:string):TrustedCatalogByteClaimV1 {if(!isAbsolute(root)||!isAbsolute(path)||!inside(root,path))throw new Error("trusted producer claim escaped generation");owned(root,path);const observed=trustedCatalogPhysicalClaimV1(path);return {relativePath:relative(root,path).split(/[\\/]/u).join("/"),sha256:observed.sha256,byteLength:observed.byteLength};}
/** 🧭️ Resolves the immutable record for the actual selected current pointer bytes. */
export function trustedCatalogProvenancePathV1(dataRoot:string,currentSha256:string):string {if(!isAbsolute(dataRoot)||!/^[a-f0-9]{64}$/u.test(currentSha256))throw new Error("trusted producer pointer identity refused");return join(dataRoot,"trusted-catalog","provenance",currentSha256+".json");}
/** 🔏️ Retains a completed native publication observation without changing its selected generation. */
export function writeTrustedCatalogPublicationProvenanceV1(dataRoot:string,producerPath:string):string {
  if(!isAbsolute(dataRoot)||!isAbsolute(producerPath))throw new Error("trusted producer root must be absolute");
  owned(dataRoot,join(dataRoot,"trusted-catalog","current.json"));
  const pointer=trustedCatalogPhysicalClaimV1(join(dataRoot,"trusted-catalog","current.json")),current=parsedClaim(pointer.path,pointer);
  if(!/^[a-f0-9]{64}$/u.test(current.generationId)||!/^([1-9][0-9]*)$/u.test(current.publicationRevision)||typeof current.profileId!=="string")throw new Error("trusted producer current identity refused");
  const generationRoot=join(dataRoot,"trusted-catalog","generations",current.generationId),observationRoot=join(dataRoot,"trusted-catalog","provenance","generations",current.generationId);
  if(!inside(observationRoot,producerPath))throw new Error("trusted producer observation escaped current generation");
  owned(dataRoot,producerPath);const producer=trustedCatalogPhysicalClaimV1(producerPath),generation=parsedClaim(producerPath,producer);
  if(generation.schema!=="semio.hub.trusted-catalog-producer/v1"||generation.generationId!==current.generationId||generation.profileId!==current.profileId)throw new Error("trusted producer generation identity refused");
  const bundle=trustedCatalogPhysicalClaimV1(join(generationRoot,"trusted-catalog.json"));
  if(bundle.sha256!==current.bundleSha256)throw new Error("trusted producer current bundle changed");
  const record:TrustedCatalogPublicationProducerV1={schema:"semio.hub.trusted-catalog-publication-producer/v1",dataRoot,profileId:current.profileId,generationId:current.generationId,publicationRevision:current.publicationRevision,pointer,bundle,producer};
  const path=trustedCatalogProvenancePathV1(dataRoot,pointer.sha256);createTrustedCatalogProvenanceDirectoryV1(dataRoot,dirname(path));
  try {writeFileSync(path,JSON.stringify(record)+"\n",{flag:"wx",mode:0o600});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;const retained=JSON.parse(bytes(path).toString("utf8"));if(JSON.stringify(retained)!==JSON.stringify(record))throw new Error("trusted producer immutable publication already names another observation");}
  readTrustedCatalogPublicationProvenanceV1(dataRoot);return path;
}
/** 📖️ Reads the existing production ledger and refuses missing, changed or foreign selected bytes. */
export function readTrustedCatalogPublicationProvenanceV1(dataRoot:string):{path:string;record:TrustedCatalogPublicationProducerV1;generationPath:string;generation:GenerationProducerV1} {
  const pointerPath=join(dataRoot,"trusted-catalog","current.json");owned(dataRoot,pointerPath);const pointer=trustedCatalogPhysicalClaimV1(pointerPath),current=parsedClaim(pointerPath,pointer);
  const path=trustedCatalogProvenancePathV1(dataRoot,pointer.sha256);owned(dataRoot,path);const record=JSON.parse(bytes(path).toString("utf8")) as TrustedCatalogPublicationProducerV1;
  if(record.schema!=="semio.hub.trusted-catalog-publication-producer/v1"||record.dataRoot!==dataRoot||record.profileId!==current.profileId||record.generationId!==current.generationId||record.publicationRevision!==current.publicationRevision||record.pointer.path!==pointerPath)throw new Error("trusted producer selection changed");
  const generationRoot=join(dataRoot,"trusted-catalog","generations",record.generationId),bundlePath=join(generationRoot,"trusted-catalog.json");
  if(record.bundle.path!==bundlePath||record.bundle.sha256!==current.bundleSha256||!inside(join(dataRoot,"trusted-catalog","provenance","generations",record.generationId),record.producer.path))throw new Error("trusted producer selected paths refused");
  for(const claim of [record.pointer,record.bundle,record.producer])matches(claim.path,claim,dataRoot);
  const generation=parsedClaim(record.producer.path,record.producer) as GenerationProducerV1;
  if(generation.schema!=="semio.hub.trusted-catalog-producer/v1"||generation.profileId!==record.profileId||generation.generationId!==record.generationId||!Array.isArray(generation.packages)||generation.packages.length===0)throw new Error("trusted producer generation envelope refused");
  const claims=[generation.bundle,...generation.packages.flatMap((row:any)=>[row.component,row.descriptor])];
  const observationRoot=join(dataRoot,"trusted-catalog","provenance","generations",record.generationId);
  for(const claim of generation.packages.flatMap((row:any)=>[...row.cargoInvocations,...(row.browserActor?[row.browserActor]:[])])){if(typeof claim?.path!=="string"||!inside(observationRoot,claim.path))throw new Error("trusted producer observation claim escaped");matches(claim.path,claim,dataRoot);}
  for(const claim of claims){if(typeof claim?.relativePath!=="string")throw new Error("trusted producer byte claim missing");const file=resolve(generationRoot,claim.relativePath);if(!inside(generationRoot,file))throw new Error("trusted producer byte claim escaped");matches(file,claim,dataRoot);}
  for(const claim of [record.producer,record.bundle,record.pointer])matches(claim.path,claim,dataRoot);
  return {path,record,generationPath:record.producer.path,generation};
}

/** 🗄️ Resolves the existing standalone or development Hub storage owner. */
export function trustedCatalogDataRootV1(repoRoot:string,mode:"development"|"standalone",environment:Readonly<Record<string,string|undefined>>=process.env):string {return mode==="development"?devLocalHubDataDir(repoRoot,environment):resolve(environment.OS_HUB_DATA??join(repoRoot,".🧬semio","🌐hub"));}

/** 📦️ Records actual final custody while preserving the original completed compiler observation. */
export function retainTrustedCargoInvocationV1(source:string,destination:string,stagedPaths:ReadonlyMap<string,string>,check:()=>void=()=>{}):ObservationClaimV1 {
  const observation=JSON.parse(bytes(source).toString("utf8"));
  if(observation.version!==1||observation.status!==0||observation.cancelled!==false||!Array.isArray(observation.units)||observation.units.length===0)throw new Error("trusted Cargo producer did not complete");
  for(const [original,staged]of stagedPaths){const matches=observation.units.flatMap((unit:any)=>unit.artifacts.filter((row:any)=>row.path===original));if(matches.length!==1)throw new Error("trusted staged artifact lacks one original compiler witness");const claimed=trustedCatalogPhysicalClaimV1(staged,check);if(claimed.sha256!==matches[0].sha256)throw new Error("trusted staged artifact differs from compiler bytes");matches[0].stagedPath=staged;matches[0].stagedSha256=claimed.sha256;}
  mkdirSync(dirname(destination),{recursive:true,mode:0o700});writeFileSync(destination,JSON.stringify(observation)+"\n",{flag:"wx",mode:0o600});return trustedCatalogPhysicalClaimV1(destination);
}
