import { createHash } from "node:crypto";
import { isAbsolute, join, win32, resolve } from "node:path";
import { parseDocumentOpenIntentV1, parseDocumentExecutionTargetLeaseFieldsV1, type DocumentOpenIntentV1, type DocumentExecutionTargetLeaseFieldsV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "../../../📇️directory/🧬️schema/🟦️.ts";
import { blake3Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";

export type ProtectedActorByteClaimV1 = Readonly<{sha256:string;byteLength:number}>;
export type ProtectedActorResponseInputV1 = Readonly<{asset:string;url:string;method:string;status:number;authenticated:boolean;intent:unknown;body:Uint8Array}>;
export type ProtectedActorResponseV1 = Readonly<{asset:string;url:string;method:"POST";status:200;authenticated:true;intent:DocumentOpenIntentV1;bytes:ProtectedActorByteClaimV1}>;
export type ProtectedActorSelectionV1 = Readonly<{generationId:string;pluginId:string;packageId:string;component:ProtectedActorByteClaimV1;descriptor:ProtectedActorByteClaimV1;actor:Extract<DocumentExecutionTargetLeaseFieldsV1["browserActor"],{kind:"closed-browser-actor"}>}>;
const assets=["manifest","component","descriptor","browser-actor"] as const;
const claim=(bytes:Uint8Array):ProtectedActorByteClaimV1=>({sha256:createHash("sha256").update(bytes).digest("hex"),byteLength:bytes.byteLength});
const same=(a:ProtectedActorByteClaimV1,b:ProtectedActorByteClaimV1)=>a.sha256===b.sha256&&a.byteLength===b.byteLength;
const fail=():never=>{throw Error("protected actor execution: selected authenticated byte join refused");};

/** 🔗️ Joins one server-selected private document target to its current same-package original producer. */
export function joinProtectedActorResponsesV1(selected:ProtectedActorSelectionV1,inputs:readonly ProtectedActorResponseInputV1[],hubOrigin:string){
  if(inputs.length!==assets.length||assets.some(asset=>inputs.filter(input=>input.asset===asset).length!==1))fail();
  const responses:ProtectedActorResponseV1[]=inputs.map(input=>{
    const url=new URL(input.url),intent=parseDocumentOpenIntentV1(input.intent),path=/^\/spaces\/([^/]+)\/documents\/([^/]+)\/execution-target\/(manifest|component|descriptor|browser-actor)$/u.exec(url.pathname);
    if(input.method!=="POST"||input.status!==200||input.authenticated!==true||url.origin!==new URL(hubOrigin).origin||url.search||url.hash||url.username||url.password||!path||path[3]!==input.asset||decodeURIComponent(path[1]!)!==intent.scope.spaceId||decodeURIComponent(path[2]!)!==intent.scope.documentId||!(input.body instanceof Uint8Array)||!input.body.byteLength)fail();
    return{asset:input.asset,url:url.href,method:"POST",status:200,authenticated:true,intent,bytes:claim(input.body)};
  });
  if(responses.some(response=>JSON.stringify(response.intent)!==JSON.stringify(responses[0]!.intent)))fail();
  const body=(asset:string)=>inputs.find(input=>input.asset===asset)!.body;
  const fields=parseDocumentExecutionTargetLeaseFieldsV1(JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(body("manifest"))));
  if(fields.catalog.generationId!==selected.generationId||fields.package.pluginId!==selected.pluginId||fields.package.packageId!==selected.packageId||JSON.stringify(fields.scope)!==JSON.stringify(responses[0]!.intent.scope)||!same(fields.component,selected.component)||!same(fields.descriptor,selected.descriptor)||fields.browserActor.kind!=="closed-browser-actor"||!same(fields.browserActor,selected.actor))fail();
  const actor=fields.browserActor;
  if(actor.sourceComponentSha256!==selected.actor.sourceComponentSha256||actor.sourceDescriptorByteSha256!==selected.actor.sourceDescriptorByteSha256||actor.policySha256!==selected.actor.policySha256||actor.codegenPolicy!==selected.actor.codegenPolicy||actor.schema!==selected.actor.schema||JSON.stringify(actor.importInterfaces)!==JSON.stringify(selected.actor.importInterfaces)||!same(claim(body("component")),selected.component)||!same(claim(body("descriptor")),selected.descriptor)||!same(claim(body("browser-actor")),selected.actor)||blake3Hex(body("component"))!==fields.component.blake3)fail();
  return{fields,responses,actor:body("browser-actor"),descriptor:body("descriptor")};
}


export interface ProtectedActorResponsePortV1 {
  url():string;
  status():number;
  headers():Record<string,string>;
  body():Promise<Uint8Array>;
  finished():Promise<Error|null>;
  request():{method():string;headers():Record<string,string>;allHeaders():Promise<Record<string,string>>;postDataJSON():unknown;sizes():Promise<{responseBodySize:number}>};
}
export interface ProtectedActorNetworkPortV1 {
  on(event:"response",observe:(response:ProtectedActorResponsePortV1)=>void):unknown;
  off(event:"response",observe:(response:ProtectedActorResponsePortV1)=>void):unknown;
}

/** 📥️ Observes bounded real protected responses without retaining capabilities or choosing a package. */
export class ProtectedActorResponseCaptureV1 {
  private readonly pending=new Set<Promise<void>>();
  private readonly inputs:(ProtectedActorResponseInputV1&{release:()=>void})[]=[];
  private readonly releases=new Set<()=>void>();
  private readonly bodies=new Set<Uint8Array>();
  private readonly failures:unknown[]=[];
  private reserved=0;
  private count=0;
  private seen=0;
  private readonly refuse=(error:unknown):void=>{if(!this.failures.length)this.failures.push(error);this.network.off("response",this.observe);};
  private closed=false;
  private activeIntent:string|null=null;
  private readonly manifests=new Map<string,Promise<void>>();
  private readonly observe=(response:ProtectedActorResponsePortV1):void=>{
    const url=new URL(response.url());
    if(url.origin!==new URL(this.hubUrl).origin||!/^\/spaces\/[^/]+\/documents\/[^/]+\/execution-target\/(manifest|component|descriptor|browser-actor)$/u.test(url.pathname)||response.status()!==200)return;
    if(++this.seen>64){this.refuse(Error("protected actor observation frame capacity refused"));return;}
    let task:Promise<void>;
    task=this.capture(response).catch(this.refuse).finally(()=>this.pending.delete(task));this.pending.add(task);
    try{if(url.pathname.endsWith("/manifest"))this.manifests.set(JSON.stringify(parseDocumentOpenIntentV1(response.request().postDataJSON())),task);}catch(error){this.refuse(error);}
  };
  constructor(private readonly network:ProtectedActorNetworkPortV1,private readonly hubUrl:string,private readonly wanted?:Readonly<{selection:ProtectedActorSelectionV1;kindId:string}>){network.on("response",this.observe);}
  private retire():void{for(const input of this.inputs){input.release();this.bodies.delete(input.body);input.body.fill(0);}this.inputs.length=0;}
  private async capture(response:ProtectedActorResponsePortV1):Promise<void>{
    const asset=new URL(response.url()).pathname.split("/").at(-1)!,request=response.request(),intent=parseDocumentOpenIntentV1(request.postDataJSON()),key=JSON.stringify(intent);
    if(this.wanted&&asset!=="manifest"){await this.manifests.get(key);if(this.activeIntent!==key||this.closed)return;}
    const length=Number(response.headers()["content-length"]),maximum=asset==="component"?DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES:asset==="descriptor"?DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES:asset==="browser-actor"?DOCUMENT_BROWSER_ACTOR_MAX_BYTES:1024*1024;
    if(!Number.isSafeInteger(length)||length<1||length>maximum||++this.count>64||this.reserved+length>256*1024*1024)throw Error("protected actor observation capacity refused");
    this.reserved+=length;
    const release=()=>{if(this.releases.delete(release))this.reserved-=length;};this.releases.add(release);
    let retained=false,body:Uint8Array|undefined;
    try{
      if(response.headers()["content-encoding"]&&response.headers()["content-encoding"]!=="identity"||await response.finished()!==null)throw Error("protected actor encoded/unfinished response refused");
      if(this.closed)return;
      const observed=(await request.sizes()).responseBodySize;
      if(!Number.isSafeInteger(observed)||observed!==length||observed>maximum)throw Error("protected actor observed body capacity refused before allocation");
      if(this.closed||this.wanted&&asset!=="manifest"&&this.activeIntent!==key)return;
      body=await response.body();this.bodies.add(body);if(this.closed)return;if(body.byteLength!==length)throw Error("protected actor observed length changed");
      if(this.wanted&&asset==="manifest"){
        const fields=parseDocumentExecutionTargetLeaseFieldsV1(JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(body))),selected=this.wanted.selection;
        if(fields.package.pluginId!==selected.pluginId||fields.artifact.kind!==this.wanted.kindId)return;
        if(fields.catalog.generationId!==selected.generationId||fields.package.packageId!==selected.packageId||!same(fields.component,selected.component)||!same(fields.descriptor,selected.descriptor)||fields.browserActor.kind!=="closed-browser-actor"||!same(fields.browserActor,selected.actor))fail();
        if(this.activeIntent!==key){this.retire();this.activeIntent=key;}
      }
      if(this.closed||this.wanted&&this.activeIntent!==key)return;
      const previous=this.inputs.findIndex(input=>input.asset===asset&&JSON.stringify(input.intent)===key);
      if(previous>=0){const retired=this.inputs.splice(previous,1)[0]!;retired.release();this.bodies.delete(retired.body);retired.body.fill(0);}
      const authenticated=/^Bearer [^\s]+$/u.test((await request.allHeaders()).authorization??"");
      if(this.closed||this.wanted&&this.activeIntent!==key)return;
      this.inputs.push({asset,url:response.url(),method:request.method(),status:response.status(),authenticated,intent,body,release});retained=true;
    }finally{if(!retained){release();if(body){this.bodies.delete(body);body.fill(0);}}}
  }
  async take(selected:ProtectedActorSelectionV1,kindId:string,spaceId:string,control?:Readonly<{signal:AbortSignal;deadlineAtMs:number}>){
    const check=()=>{if(this.closed)throw Error("protected actor observation closed");if(control?.signal.aborted)throw Error("protected actor observation cancelled");if(control&&Date.now()>=control.deadlineAtMs)throw Error("protected actor observation deadline elapsed");};
    try{for(;;){
      check();const completed=Promise.all([...this.pending]);
      if(control){
        let timer:ReturnType<typeof setTimeout>|undefined,abort:()=>void=()=>{};
        try{await Promise.race([completed,new Promise<never>((_,reject)=>{abort=()=>reject(Error("protected actor observation cancelled"));control.signal.addEventListener("abort",abort,{once:true});timer=setTimeout(()=>reject(Error("protected actor observation deadline elapsed")),Math.max(0,control.deadlineAtMs-Date.now()));if(control.signal.aborted)abort();})]);}
        finally{if(timer!==undefined)clearTimeout(timer);control.signal.removeEventListener("abort",abort);}
      }else await completed;
      check();if(this.failures.length)throw new AggregateError(this.failures,"protected actor response observation failed");
      const candidates=this.inputs.filter(input=>input.asset==="manifest").map(input=>({input,fields:parseDocumentExecutionTargetLeaseFieldsV1(JSON.parse(new TextDecoder().decode(input.body)))})).filter(candidate=>candidate.fields.package.pluginId===selected.pluginId&&candidate.fields.artifact.kind===kindId&&candidate.fields.scope.spaceId===spaceId);
      for(const candidate of candidates.reverse()){
        const intent=JSON.stringify(candidate.input.intent),group=assets.map(asset=>this.inputs.findLast(input=>input.asset===asset&&JSON.stringify(input.intent)===intent));
        if(group.every(input=>input!==undefined)){const joined=joinProtectedActorResponsesV1(selected,group as ProtectedActorResponseInputV1[],this.hubUrl);check();return joined;}
      }
      if(!control)throw Error("protected actor execution has no complete actual private document response chain");
      await new Promise<void>((resolve,reject)=>{const aborted=()=>{clearTimeout(timer);control.signal.removeEventListener("abort",aborted);reject(Error("protected actor observation cancelled"));},timer=setTimeout(()=>{control.signal.removeEventListener("abort",aborted);resolve();},Math.min(250,control.deadlineAtMs-Date.now()));control.signal.addEventListener("abort",aborted,{once:true});if(control.signal.aborted)aborted();});
    }}catch(error){this.close();throw error;}
  }
  close():void{this.closed=true;this.network.off("response",this.observe);this.retire();for(const body of this.bodies)body.fill(0);this.bodies.clear();for(const release of this.releases)release();}
}

export interface ProtectedActorBrowserPortV1 {
  evaluate<Result,Input>(execute:(input:Input)=>Promise<Result>,input:Input):Promise<Result>;
}
export type ProtectedActorChildExecutionV1=Readonly<{sourceDetached:true;describe:ProtectedActorByteClaimV1;transferCount:number;cancelled:true;capacityRestored:true;stages:readonly string[]}>;

/** 🧵️ Executes the observed protected bytes in the existing child on the same normal Dev server. */
export async function executeProtectedActorChildV1(page:ProtectedActorBrowserPortV1,childUrl:string,joined:Pick<ReturnType<typeof joinProtectedActorResponsesV1>,"actor"|"descriptor">,owner:Readonly<{root:string;baseUrl:string}>):Promise<ProtectedActorChildExecutionV1>{
  const origin=new URL(owner.baseUrl),expected=new URL(protectedActorChildUrlV1(owner.root,origin.href));
  if(!isAbsolute(owner.root)||!["http:","https:"].includes(origin.protocol)||origin.username||origin.password||new URL(childUrl).href!==expected.href||joined.actor.byteLength<1||joined.actor.byteLength>DOCUMENT_BROWSER_ACTOR_MAX_BYTES)throw Error("normal protected child actual owner URL refused");
  const result=await page.evaluate(async ({childUrl,actorBase64,actorSha256,actorByteLength})=>{
    if(new URL(childUrl).origin!==location.origin)throw Error("normal protected child must use the actual page origin");
    const api=await import(/* @vite-ignore */ childUrl),before=api.browserActorChildCapacity(),abort=new AbortController(),stages:string[]=[];
    const source=Uint8Array.from(atob(actorBase64),character=>character.charCodeAt(0)).buffer;
    let owner:any,guest:Uint8Array|undefined;
    try{
      owner=await api.reserveBrowserActorChild({actorId:"normal-protected-publication:"+crypto.randomUUID(),activationGeneration:1n,bundleSha256:actorSha256,bundleByteLength:actorByteLength},abort.signal);
      await owner.load(source,(progress:{stage:string})=>{if(stages.at(-1)!==progress.stage)stages.push(progress.stage);});
      if(source.byteLength!==0||!owner.progress().sourceDetached)throw Error("normal protected child retained source");
      guest=await owner.invoke(["describe","describe"],[]);if(!(guest instanceof Uint8Array))throw Error("normal protected describe shape");
      if(!["verified","imported","active"].every(stage=>stages.includes(stage)))throw Error("normal protected child did not report actual load/activation");
      const transferCount=owner.progress().resultTransfersDetached;if(transferCount!==1)throw Error("normal protected describe transfer");
      abort.abort();if(owner.progress().phase!=="closed")throw Error("normal protected child cancellation did not close");
      const after=api.browserActorChildCapacity();if(after.actors!==before.actors||after.bytes!==before.bytes)throw Error("normal protected child cancellation retained capacity");
      let refused=false;try{await owner.invoke(["describe","describe"],[]);}catch{refused=true;}if(!refused)throw Error("normal protected cancelled child invoked");
      return{guest:Array.from(guest),transferCount,stages};
    }finally{abort.abort();owner?.close();guest?.fill(0);if(source.byteLength)new Uint8Array(source).fill(0);}
  },{childUrl,actorBase64:Buffer.from(joined.actor).toString("base64"),actorSha256:claim(joined.actor).sha256,actorByteLength:joined.actor.byteLength});
  const guest=new Uint8Array(result.guest);
  try{
    const {verifyBrowserActorDescribeV1}=await import("../../../🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts");const {decodePackValue,encodePackValue}=await import("../../../../🟦️.ts");
    verifyBrowserActorDescribeV1(guest,joined.descriptor,{decode:decodePackValue,encode:encodePackValue});
    return{sourceDetached:true,describe:claim(guest),transferCount:result.transferCount,cancelled:true,capacityRestored:true,stages:result.stages};
  }finally{guest.fill(0);result.guest.fill(0);}
}


/** 🌐️ Projects the actual fixed child owner through the existing Dev Vite filesystem route. */
export function protectedActorChildUrlV1(root:string,baseUrl:string):string{
  if(!isAbsolute(root)&&!win32.isAbsolute(root))throw Error("normal protected child needs an absolute physical owner");
  const path=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts").replaceAll("\\","/").replace(/^\/+/,"");
  return new URL("/@fs/"+path,baseUrl).href;
}


/** 🔐️ Requires the existing live normal Dev owner to hold this exact Hub port and selected data root. */
export async function observeNormalDevHubPublicationOwnerV1(root:string,dataRoot:string,hubUrl:string){
  const api=await import("../../🚀️local-hub/🏃️execution/🟦️.ts"),claims=await import("../../../../../../../🌎️hub/🏗️bootstrap/🧾️provenance/🟦️.ts"),url=hubUrl.replace(/\/+$/u,""),port=api.parseHubPort(url),paths=api.devHubLeasePathsV1(api.devHubLeaseRootV1(root),port,dataRoot),leases=paths.map(path=>api.liveDevHubLeaseV1(path));
  if(leases.some(lease=>lease===null||lease.port!==port||resolve(lease.dataDir)!==resolve(dataRoot)||lease.hubUrl!==url)||JSON.stringify(leases[0])!==JSON.stringify(leases[1]))throw Error("normal protected sweep requires the existing current Dev Hub owner/data-root lease");
  return{hubUrl:url,dataRoot:resolve(dataRoot),pid:leases[0]!.pid,acquiredAt:leases[0]!.acquiredAt,leases:paths.map(path=>claims.trustedCatalogPhysicalClaimV1(path))};
}
