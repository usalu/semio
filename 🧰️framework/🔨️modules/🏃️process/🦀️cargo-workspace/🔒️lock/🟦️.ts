import{createHash,randomBytes}from"node:crypto";
import{existsSync,mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import{runtimeParseTomlV1}from"../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
const stale=policy.staleMarkers.map(marker=>new RegExp(marker,"u")),missing=policy.missingCrateMarkers.map(marker=>new RegExp(marker,"u"));
const digest=(bytes:Uint8Array|string):string=>createHash("sha256").update(bytes).digest("hex");
const fileDigest=(path:string):string|null=>{try{return digest(readFileSync(path));}catch{return null;}};

export type CargoLockLanguageV1="en"|"de";
export type CargoLockOutcomeV1="current"|"refreshed";
/** 📜️ Carries one finished Cargo child of the lock refresh with its separated streams. */
export type CargoLockRunV1=Readonly<{status:number|null;stdout:string;stderr:string}>;
/** 🔌️ Supplies the only effects of the lock refresh: the caller's Cargo environment, its per-workspace try-lease, bounded waiting and reporting. */
export interface CargoLockPortV1{
 run(args:readonly string[]):Promise<CargoLockRunV1>;
 tryHold(resource:string):Promise<Readonly<{release():void}>|null>;
 pause(milliseconds:number):Promise<void>;
 report(line:string):void;
}
export type CargoLockRequestV1=Readonly<{workspaceRoot:string;manifest:string;receiptDirectory:string|null;language:CargoLockLanguageV1;offline:boolean;force:boolean}>;

/** 🚫️ Refuses a lock refresh that cannot converge and names the unresolvable crate in both languages. */
export class CargoLockRefusedV1 extends Error{
 readonly crate:string|null;
 readonly texts:Readonly<Record<CargoLockLanguageV1,string>>;
 constructor(language:CargoLockLanguageV1,crate:string|null,texts:Readonly<Record<CargoLockLanguageV1,string>>){super(texts[language]);this.name="CargoLockRefused";this.crate=crate;this.texts=texts;}
}

const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\{([A-Za-z]+)\}/gu,(_,name:string)=>String(values[name]??""));
const text=(key:keyof typeof messages,language:CargoLockLanguageV1,values:Readonly<Record<string,string|number>>):string=>fill(messages[key][language],values);

/** 🔐️ Tells whether a Cargo failure says its lockfile lags the manifests while `--locked` forbade the update. */
export function cargoLockStaleOutputV1(output:string):boolean{return stale.some(pattern=>pattern.test(output));}
/** 🔒️ Tells whether any failure of a Cargo child, in its message or its streams, is a stale lockfile. */
export function cargoLockStaleFailureV1(error:unknown):boolean{return cargoLockStaleOutputV1(error instanceof Error?error.message:String(error));}
/** 🔎️ Names the crate a failed resolution could not find, or null when the failure names none. */
export function cargoLockMissingCrateV1(output:string):string|null{for(const pattern of missing){const match=pattern.exec(output);if(match?.[1])return match[1];}return null;}

/** 🧾️ Locates the receipt of one selected manifest inside the receipt directory. */
export function cargoLockReceiptPathV1(directory:string,workspaceRoot:string,manifest:string):string{return join(directory,digest(JSON.stringify([resolve(workspaceRoot),resolve(manifest)]))+policy.receiptSuffix);}

/** ✅️ Holds only while every recorded manifest and the lockfile still have their recorded bytes. */
export function cargoLockReceiptCurrentV1(path:string):boolean{
 let value:unknown;try{value=JSON.parse(readFileSync(path,"utf8"));}catch{return false;}
 if(validateJsonSchemaSubset(schema.definitions.Receipt,value,schema).length)return false;
 const receipt=value as Readonly<{lock:Readonly<{path:string;sha256:string}>;manifests:readonly Readonly<{path:string;sha256:string}>[]}>;
 return[receipt.lock,...receipt.manifests].every(row=>fileDigest(row.path)===row.sha256);
}

/** 🧭️ Collects the manifests whose bytes decide a workspace lockfile: members, their path dependencies, every owning workspace root and the Cargo configuration. */
export function cargoLockManifestsV1(workspaceRoot:string,members:readonly string[]):string[]{
 const documents=new Map<string,any>();
 const read=(path:string):any=>{let value=documents.get(path);if(value===undefined){try{value=runtimeParseTomlV1(readFileSync(path,"utf8"));}catch{value=null;}documents.set(path,value);}return value;};
 const owner=(manifest:string):string|null=>{if(read(manifest)?.workspace!==undefined)return manifest;for(let directory=dirname(dirname(manifest));;directory=dirname(directory)){const candidate=join(directory,"Cargo.toml");if(existsSync(candidate)&&read(candidate)?.workspace!==undefined)return candidate;if(dirname(directory)===directory)return null;}};
 const entries=(table:any,base:string):string[]=>Object.values(table??{}).flatMap(entry=>entry&&typeof entry==="object"&&typeof(entry as any).path==="string"?[join(base,(entry as any).path,"Cargo.toml")]:[]);
 const paths=(document:any,base:string):string[]=>{
  const groups=[document,...Object.values(document?.target??{})] as any[];
  return[...groups.flatMap(group=>policy.dependencyTables.flatMap(kind=>entries(group?.[kind],base))),...entries(document?.workspace?.dependencies,base),...Object.values(document?.patch??{}).flatMap(registry=>entries(registry,base))];
 };
 const seen=new Set<string>(),queue=[...members];
 while(queue.length){
  const manifest=queue.pop()!;if(seen.has(manifest))continue;seen.add(manifest);if(!existsSync(manifest))continue;
  const root=owner(manifest);if(root)queue.push(root);queue.push(...paths(read(manifest),dirname(manifest)));
 }
 return[...seen,...policy.extraInputs.map(name=>join(workspaceRoot,name))].filter(path=>existsSync(path)).sort();
}

const probe=(manifest:string):string[]=>["update","--workspace","--locked","--offline","--manifest-path",manifest];
const detail=(run:CargoLockRunV1):string=>{const line=(run.stderr+"\n"+run.stdout).split("\n").find(row=>row.startsWith("error"))??"";return line.slice(0,policy.detailCharacters);};

/** ♻️ Brings the lockfile of the workspace owning `manifest` up to date with every manifest it depends on, once per manifest set, under a per-workspace try-lease. */
export async function ensureCargoLockV1(request:CargoLockRequestV1,port:CargoLockPortV1):Promise<CargoLockOutcomeV1>{
 const manifest=resolve(request.manifest),receipt=request.receiptDirectory===null?null:cargoLockReceiptPathV1(request.receiptDirectory,request.workspaceRoot,manifest);
 const current=():boolean=>receipt!==null&&cargoLockReceiptCurrentV1(receipt);
 if(!request.force&&current())return"current";
 const located=await port.run(["locate-project","--workspace","--message-format","plain","--manifest-path",manifest]),owner=located.status===0&&located.stdout.trim()?located.stdout.trim().split("\n")[0]!:manifest,workspace=dirname(owner);
 const record=async():Promise<void>=>{
  if(receipt===null)return;
  try{
   const listing=await port.run(["metadata","--no-deps","--format-version=1","--offline","--manifest-path",manifest]);if(listing.status!==0)return;
   const members=(JSON.parse(listing.stdout) as{packages:{manifest_path:string}[]}).packages.map(row=>row.manifest_path),lock=join(workspace,"Cargo.lock"),lockDigest=fileDigest(lock);if(lockDigest===null)return;
   const manifests=cargoLockManifestsV1(request.workspaceRoot,[owner,...members]).map(path=>({path,sha256:fileDigest(path)})).filter((row):row is{path:string;sha256:string}=>row.sha256!==null);
   mkdirSync(dirname(receipt),{recursive:true});const temporary=receipt+"."+randomBytes(6).toString("hex");writeFileSync(temporary,JSON.stringify({version:1,lock:{path:lock,sha256:lockDigest},manifests}));renameSync(temporary,receipt);rmSync(temporary,{force:true});
  }catch{}
 };
 const refuse=(crate:string|null,run:CargoLockRunV1):CargoLockRefusedV1=>{
  const key=crate===null?"failed":request.offline?"unresolvableOffline":"unresolvable",values={workspace,crate:crate??"",detail:detail(run)};
  return new CargoLockRefusedV1(request.language,crate,{en:text(key,"en",values),de:text(key,"de",values)});
 };
 const refresh=async():Promise<CargoLockOutcomeV1>=>{
  if(!request.force&&current())return"current";
  const first=await port.run(probe(manifest));if(first.status===0){await record();return"current";}
  port.report(text("refreshing",request.language,{workspace}));
  let update=await port.run(["update","--workspace","--offline","--manifest-path",manifest]);
  if(update.status!==0){
   const crate=cargoLockMissingCrateV1(update.stderr+"\n"+update.stdout);if(crate===null||request.offline)throw refuse(crate,update);
   port.report(text("online",request.language,{workspace,crate}));
   update=await port.run(["update","--workspace","--manifest-path",manifest]);if(update.status!==0)throw refuse(cargoLockMissingCrateV1(update.stderr+"\n"+update.stdout)??crate,update);
  }
  const verified=await port.run(probe(manifest));if(verified.status!==0)throw refuse(cargoLockMissingCrateV1(verified.stderr+"\n"+verified.stdout),verified);
  await record();port.report(text("refreshed",request.language,{workspace}));return"refreshed";
 };
 const resource=policy.resourcePrefix+workspace,started=Date.now();let reported=started;
 for(;;){
  const hold=await port.tryHold(resource);
  if(hold){try{return await refresh();}finally{hold.release();}}
  if(!request.force&&current())return"current";
  const now=Date.now();if(now-reported>=policy.waitingReportMilliseconds){reported=now;port.report(text("waiting",request.language,{workspace,seconds:Math.round((now-started)/1000)}));}
  await port.pause(policy.pollMilliseconds);
 }
}
