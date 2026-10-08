import { devLocalHubDataDir } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, writeFileSync, existsSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import {observePhysicalFileV1,readPhysicalFileV1,type PhysicalFileClaimV1,type FileObservationControlV1} from "../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts";

export type TrustedCatalogByteClaimV1 = Readonly<{relativePath:string;sha256:string;byteLength:number}>;
export type ActorProducerV1 = Readonly<{schema:"semio.os.closed-browser-actor-producer/v1";actor:TrustedCatalogByteClaimV1;component:TrustedCatalogByteClaimV1;descriptor:TrustedCatalogByteClaimV1;policyCanonical:string;policySha256:string;runtime:PhysicalFileClaimV1;compiler:PhysicalFileClaimV1;inputs:readonly Readonly<{logicalPath:string;path:string;sha256:string;byteLength:number}>[]}>;
export type PackageProducerV1 = Readonly<{pluginId:string;packageId:string;cargoPackage:string;component:TrustedCatalogByteClaimV1;descriptor:TrustedCatalogByteClaimV1;cargoInvocations:readonly PhysicalFileClaimV1[];browserActor:PhysicalFileClaimV1|null}>;
export type GenerationProducerV1 = Readonly<{schema:"semio.hub.trusted-catalog-producer/v1";profileId:string;generationId:string;bundle:TrustedCatalogByteClaimV1;packages:readonly PackageProducerV1[]}>;
export type TrustedCatalogPublicationProducerV1 = Readonly<{schema:"semio.hub.trusted-catalog-publication-producer/v1";dataRoot:string;profileId:string;generationId:string;publicationRevision:string;pointer:{path:string;sha256:string;byteLength:number};bundle:{path:string;sha256:string;byteLength:number};producer:{path:string;sha256:string;byteLength:number}}>;

async function bytes(path:string,control:FileObservationControlV1):Promise<Uint8Array> {return(await readPhysicalFileV1(path,{maxBytes:Math.min(control.maxBytes,128*1024*1024),maxWork:control.maxWork,chunkBytes:control.chunkBytes,cancelled:()=>control.cancelled(),remainingMs:()=>control.remainingMs(),onProgress:step=>control.onProgress(step)})).bytes;}
async function parsedClaim(path:string,claim:PhysicalFileClaimV1,control:FileObservationControlV1):Promise<any> {const input=await bytes(path,control);try{if(input.length!==claim.byteLength||createHash("sha256").update(input).digest("hex")!==claim.sha256)throw new Error("trusted producer parsed bytes changed");return JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(input));}finally{input.fill(0);}}
function inside(root:string,path:string):boolean {const child=relative(root,path);return child!==""&&!isAbsolute(child)&&child.split(/[\\/]/u)[0]!=="..";}
function owned(root:string,path:string):void {
  if(!isAbsolute(root)||!isAbsolute(path)||!inside(root,path))throw new Error("trusted producer physical owner escaped");
  for(let parent=dirname(path);;parent=dirname(parent)){const stat=lstatSync(parent);if(!stat.isDirectory()||stat.isSymbolicLink())throw new Error("trusted producer directory owner refused");if(parent===root)break;if(!inside(root,parent))throw new Error("trusted producer ancestor escaped");}
  const actual=realpathSync(existsSync(path)?path:dirname(path)),owner=realpathSync(root);if(actual!==owner&&!inside(owner,actual))throw new Error("trusted producer physical path escaped");
}
export function createTrustedCatalogProvenanceDirectoryV1(root:string,path:string):void {if(!inside(root,path))throw new Error("trusted producer directory escaped");const rootStat=lstatSync(root);if(!rootStat.isDirectory()||rootStat.isSymbolicLink())throw new Error("trusted producer root refused");let parent=root;for(const part of relative(root,path).split(/[\\/]/u)){parent=join(parent,part);if(!existsSync(parent))mkdirSync(parent,{mode:0o700});const stat=lstatSync(parent);if(!stat.isDirectory()||stat.isSymbolicLink())throw new Error("trusted producer directory owner refused");}owned(root,join(path,".ownership"));}
async function matches(path:string,claim:{sha256:string;byteLength:number},root:string,control:FileObservationControlV1):Promise<void> {owned(root,path);const observed=await observePhysicalFileV1(path,control);if(observed.sha256!==claim.sha256||observed.byteLength!==claim.byteLength)throw new Error("trusted producer bytes changed: "+path);}

/** 🧾️ Observes a generation-owned file at the actual producer boundary. */
export async function trustedCatalogByteClaimV1(root:string,path:string,control:FileObservationControlV1):Promise<TrustedCatalogByteClaimV1> {if(!isAbsolute(root)||!isAbsolute(path)||!inside(root,path))throw new Error("trusted producer claim escaped generation");owned(root,path);const observed=await observePhysicalFileV1(path,control);return {relativePath:relative(root,path).split(/[\\/]/u).join("/"),sha256:observed.sha256,byteLength:observed.byteLength};}
/** 🧭️ Resolves the immutable record for the actual selected current pointer bytes. */
export function trustedCatalogProvenancePathV1(dataRoot:string,currentSha256:string):string {if(!isAbsolute(dataRoot)||!/^[a-f0-9]{64}$/u.test(currentSha256))throw new Error("trusted producer pointer identity refused");return join(dataRoot,"trusted-catalog","provenance",currentSha256+".json");}
/** 🔏️ Retains a completed native publication observation without changing its selected generation. */
export async function writeTrustedCatalogPublicationProvenanceV1(dataRoot:string,producerPath:string,control:FileObservationControlV1):Promise<string> {
  if(!isAbsolute(dataRoot)||!isAbsolute(producerPath))throw new Error("trusted producer root must be absolute");
  owned(dataRoot,join(dataRoot,"trusted-catalog","current.json"));
  const pointer=await observePhysicalFileV1(join(dataRoot,"trusted-catalog","current.json"),control),current=await parsedClaim(pointer.path,pointer,control);
  if(!/^[a-f0-9]{64}$/u.test(current.generationId)||!/^([1-9][0-9]*)$/u.test(current.publicationRevision)||typeof current.profileId!=="string")throw new Error("trusted producer current identity refused");
  const generationRoot=join(dataRoot,"trusted-catalog","generations",current.generationId),observationRoot=join(dataRoot,"trusted-catalog","provenance","generations",current.generationId);
  if(!inside(observationRoot,producerPath))throw new Error("trusted producer observation escaped current generation");
  owned(dataRoot,producerPath);const producer=await observePhysicalFileV1(producerPath,control),generation=await parsedClaim(producerPath,producer,control);
  if(generation.schema!=="semio.hub.trusted-catalog-producer/v1"||generation.generationId!==current.generationId||generation.profileId!==current.profileId)throw new Error("trusted producer generation identity refused");
  const bundle=await observePhysicalFileV1(join(generationRoot,"trusted-catalog.json"),control);
  if(bundle.sha256!==current.bundleSha256)throw new Error("trusted producer current bundle changed");
  const record:TrustedCatalogPublicationProducerV1={schema:"semio.hub.trusted-catalog-publication-producer/v1",dataRoot,profileId:current.profileId,generationId:current.generationId,publicationRevision:current.publicationRevision,pointer,bundle,producer};
  const path=trustedCatalogProvenancePathV1(dataRoot,pointer.sha256);createTrustedCatalogProvenanceDirectoryV1(dataRoot,dirname(path));
  try {writeFileSync(path,JSON.stringify(record)+"\n",{flag:"wx",mode:0o600});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;const retained=await parsedClaim(path,await observePhysicalFileV1(path,control),control);if(JSON.stringify(retained)!==JSON.stringify(record))throw new Error("trusted producer immutable publication already names another observation");}
  await readTrustedCatalogPublicationProvenanceV1(dataRoot,control);return path;
}
/** 📖️ Reads the existing production ledger and refuses missing, changed or foreign selected bytes. */
export async function readTrustedCatalogPublicationProvenanceV1(dataRoot:string,control:FileObservationControlV1):Promise<{path:string;record:TrustedCatalogPublicationProducerV1;generationPath:string;generation:GenerationProducerV1}> {
  const pointerPath=join(dataRoot,"trusted-catalog","current.json");owned(dataRoot,pointerPath);const pointer=await observePhysicalFileV1(pointerPath,control),current=await parsedClaim(pointerPath,pointer,control);
  const path=trustedCatalogProvenancePathV1(dataRoot,pointer.sha256);owned(dataRoot,path);const record=await parsedClaim(path,await observePhysicalFileV1(path,control),control) as TrustedCatalogPublicationProducerV1;
  if(record.schema!=="semio.hub.trusted-catalog-publication-producer/v1"||record.dataRoot!==dataRoot||record.profileId!==current.profileId||record.generationId!==current.generationId||record.publicationRevision!==current.publicationRevision||record.pointer.path!==pointerPath)throw new Error("trusted producer selection changed");
  const generationRoot=join(dataRoot,"trusted-catalog","generations",record.generationId),bundlePath=join(generationRoot,"trusted-catalog.json");
  if(record.bundle.path!==bundlePath||record.bundle.sha256!==current.bundleSha256||!inside(join(dataRoot,"trusted-catalog","provenance","generations",record.generationId),record.producer.path))throw new Error("trusted producer selected paths refused");
  for(const claim of [record.pointer,record.bundle,record.producer])await matches(claim.path,claim,dataRoot,control);
  const generation=await parsedClaim(record.producer.path,record.producer,control) as GenerationProducerV1;
  if(generation.schema!=="semio.hub.trusted-catalog-producer/v1"||generation.profileId!==record.profileId||generation.generationId!==record.generationId||!Array.isArray(generation.packages)||generation.packages.length===0)throw new Error("trusted producer generation envelope refused");
  const claims=[generation.bundle,...generation.packages.flatMap((row:any)=>[row.component,row.descriptor])];
  const observationRoot=join(dataRoot,"trusted-catalog","provenance","generations",record.generationId);
  for(const claim of generation.packages.flatMap((row:any)=>[...row.cargoInvocations,...(row.browserActor?[row.browserActor]:[])])){if(typeof claim?.path!=="string"||!inside(observationRoot,claim.path))throw new Error("trusted producer observation claim escaped");await matches(claim.path,claim,dataRoot,control);}
  for(const claim of claims){if(typeof claim?.relativePath!=="string")throw new Error("trusted producer byte claim missing");const file=resolve(generationRoot,claim.relativePath);if(!inside(generationRoot,file))throw new Error("trusted producer byte claim escaped");await matches(file,claim,dataRoot,control);}
  for(const claim of [record.producer,record.bundle,record.pointer])await matches(claim.path,claim,dataRoot,control);
  return {path,record,generationPath:record.producer.path,generation};
}

/** 🗄️ Resolves the existing standalone or development Hub storage owner. */
export function trustedCatalogDataRootV1(repoRoot:string,mode:"development"|"standalone",environment:Readonly<Record<string,string|undefined>>=process.env):string {return mode==="development"?devLocalHubDataDir(repoRoot,environment):resolve(environment.OS_HUB_DATA??join(repoRoot,".🧬semio","🌐hub"));}

/** 📦️ Records actual final custody while preserving the original completed compiler observation. */
export async function retainTrustedCargoInvocationV1(source:string,destination:string,stagedPaths:ReadonlyMap<string,string>,control:FileObservationControlV1):Promise<PhysicalFileClaimV1> {
  const observation=await parsedClaim(source,await observePhysicalFileV1(source,control),control);
  if(observation.version!==1||observation.status!==0||observation.cancelled!==false||!Array.isArray(observation.units)||observation.units.length===0)throw new Error("trusted Cargo producer did not complete");
  for(const [original,staged]of stagedPaths){const matches=observation.units.flatMap((unit:any)=>unit.artifacts.filter((row:any)=>row.path===original));if(matches.length!==1)throw new Error("trusted staged artifact lacks one original compiler witness");const claimed=await observePhysicalFileV1(staged,control);if(claimed.sha256!==matches[0].sha256)throw new Error("trusted staged artifact differs from compiler bytes");matches[0].stagedPath=staged;matches[0].stagedSha256=claimed.sha256;}
  mkdirSync(dirname(destination),{recursive:true,mode:0o700});writeFileSync(destination,JSON.stringify(observation)+"\n",{flag:"wx",mode:0o600});return observePhysicalFileV1(destination,control);
}
