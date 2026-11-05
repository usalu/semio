/** 📝️ One captured staged action owns the real surface lease until execution or cancellation settles. */
import type {StagedArgs,StagedFormRequest} from "@semio-tech/framework/manifest/staging";
import {StagedFormSession,type StagedFormOwner} from "../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🛡️intents/🟦️.ts";
import {OwnedStagedControls,type StagedOwnedSurface} from "../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🛡️intents/owned.ts";
import type {OwnedUiSurfaceLease} from "../../🔌️PluginRuntime/🧺️handle-ui-intake/lease.ts";
/** 🎟️ Coordination borrows a surface port; production keeps its nominal acknowledged lease by default. */
export interface StagedActionLease {readonly source:StagedOwnedSurface;readonly signal:AbortSignal;current():boolean;close():Promise<void>}
export interface StagedActionSource<Lease extends StagedActionLease=OwnedUiSurfaceLease> {
 readonly owner:StagedFormOwner;
 readonly surface:string;
 current():boolean;
 revision():string|null;
 subscribe(listener:()=>void):()=>void;
 query(request:StagedFormRequest,signal:AbortSignal):Promise<Lease>;
 execute(args:StagedArgs,revision:string,signal:AbortSignal):Promise<void>;
}
export interface StagedActionPresentation {
 readonly locale:"en"|"de";
 readonly controls:OwnedStagedControls|null;
 readonly pending:boolean;
 readonly executing:boolean;
 readonly closing:boolean;
 readonly error:string|null;
 readonly ready:boolean;
}
class StagedActionRun<Lease extends StagedActionLease> {
 readonly session:StagedFormSession<number,number>;
 #lease:Lease|null=null;
 #controls:OwnedStagedControls|null=null;
 #query:AbortController|null=null;
 #unsubscribe:()=>void=()=>{};
 #unobserve:()=>void=()=>{};
 #epoch=0;
 #closed=false;
 #finished=false;
 #queries=0;
 #closing:Promise<void>|null=null;
 #closeResult:"executed"|"cancelled"|null=null;
 readonly #leases=new Set<Lease>();
 #executing=false;
 #operationAbort:AbortController|null=null;
 #operation:Promise<void>|null=null;
 #edited=false;
 #initial=true;
 #failure:string|null=null;
 constructor(readonly source:StagedActionSource<Lease>,args:StagedArgs,readonly publish:()=>void,readonly finish:(result:"executed"|"cancelled")=>void,readonly retired:()=>void){
  this.session=new StagedFormSession(source.owner,args);
  this.#unsubscribe=source.subscribe(()=>this.changed());
 }
 presentation():StagedActionPresentation{return {locale:this.source.owner.locale,controls:this.#controls,pending:this.#query!==null,executing:this.#executing,closing:this.#closing!==null,error:this.#failure,ready:!this.#executing&&this.current()&&this.#lease?.current()===true&&this.session.canExecute()};}
 current():boolean{const request=this.session.request();return !this.#closed&&this.source.current()&&request!==undefined&&this.source.revision()===request.token.revision;}
 changed():void{
  if(this.#closed||this.#executing)return;
  if(!this.source.current()){void this.close("cancelled").catch(()=>{});return;}
  const revision=this.source.revision(),request=this.session.request();
  if(revision!==null&&request){const revised=revision!==request.token.revision;if(revised){this.#initial=false;this.session.revise(revision);}if(revised||this.#query===null&&!this.#lease?.current())void this.refresh();}
  this.publish();
 }
 async refresh():Promise<void>{
  if(this.#closed||this.#executing)return;
  if(!this.source.current()){await this.close("cancelled").catch(()=>{});return;}
  const revision=this.source.revision(),prior=this.session.request();
  if(revision!==null&&prior&&revision!==prior.token.revision){this.#initial=false;this.session.revise(revision);}
  const request=this.session.request();if(!request||!this.current()){this.publish();return;}
  this.#query?.abort();const controller=new AbortController(),epoch=++this.#epoch;this.#query=controller;this.#failure=null;++this.#queries;
  try{
   this.publish();if(this.#closed||controller.signal.aborted)return;
   const lease=await this.source.query(request,controller.signal);this.#leases.add(lease);
   if(this.#closed||controller.signal.aborted||epoch!==this.#epoch||!this.current()||this.session.request()?.token!==request.token){await this.release(lease);return;}
   const previous=this.#lease,old=this.#controls;this.#unobserve();old?.detach();
   this.#lease=lease;
   const controls=new OwnedStagedControls(lease.source,this.source.surface,this.session,()=>{this.#edited=true;this.#initial=false;void this.refresh();});
   this.#controls=controls;
   this.#unobserve=controls.subscribe(()=>{this.publish();if(this.#initial&&!this.#edited&&this.presentation().ready){this.#initial=false;void this.execute().catch(()=>{});}});
   lease.signal.addEventListener("abort",()=>{if(this.#lease===lease&&!this.#closed&&!this.#executing){this.#initial=false;this.publish();void this.refresh();}},{once:true});
   if(previous)await this.release(previous);
  }catch(error){if(!controller.signal.aborted&&!this.#closed&&epoch===this.#epoch)this.#failure=error instanceof Error?error.message:String(error);}
  finally{--this.#queries;if(this.#query===controller){this.#query=null;this.publish();}this.retired();}
 }
 async execute():Promise<void>{
  const request=this.session.request(),args=this.session.executionArgs();
  if(!request||!args||!this.presentation().ready)return;
  let settle!:()=>void;this.#operation=new Promise(resolve=>{settle=resolve;});const controller=new AbortController();this.#operationAbort=controller;
  this.#executing=true;this.#query?.abort();this.#query=null;this.publish();
  try{
   if(controller.signal.aborted)throw controller.signal.reason??new Error("staging.cancelled");
   if(!this.current()||!this.#lease?.current())throw new Error("staging.owner");
   await this.source.execute(args,request.token.revision,controller.signal);
   this.#executing=false;await this.close("executed");
  }catch(error){this.#executing=false;this.#initial=false;if(controller.signal.aborted)await this.close("cancelled");else{this.#failure=error instanceof Error?error.message:String(error);this.changed();this.publish();}}
  finally{this.#operationAbort=null;this.#operation=null;settle();}
 }
 terminal():boolean{return this.#finished&&this.#queries===0&&this.#leases.size===0&&this.#closing===null;}
 async release(lease:Lease):Promise<void>{await lease.close();this.#leases.delete(lease);this.retired();}
 close(result:"executed"|"cancelled"):Promise<void>{
  if(this.#executing){this.#operationAbort?.abort();return this.#operation!.then(()=>this.close(result));}
  if(this.#closing)return this.#closing;if(this.#finished&&this.#leases.size===0)return Promise.resolve();
  this.#closed=true;this.#closeResult??=result;++this.#epoch;this.#query?.abort();this.#query=null;
  let closing!:Promise<void>;closing=Promise.resolve().then(async()=>{
   let failure:{error:unknown}|undefined;
   for(const release of this.#finished?[]:[()=>{this.#unsubscribe();this.#unsubscribe=()=>{};},()=>{this.#unobserve();this.#unobserve=()=>{};},()=>this.session.close(),()=>this.#controls?.detach()]){try{release();}catch(error){failure??={error};}}
   for(const lease of this.#leases){try{await this.release(lease);}catch(error){failure??={error};}}
   if(failure)throw failure.error;
   this.#lease=null;this.#controls=null;if(!this.#finished){this.#finished=true;this.finish(this.#closeResult!);}this.publish();
  }).catch(error=>{this.#closing=null;this.#failure=error instanceof Error?error.message:String(error);this.publish();throw error;}).finally(()=>{if(this.#closing===closing)this.#closing=null;this.retired();this.publish();});
  this.#closing=closing;this.publish();return closing;
 }
}
/** 🎛️ A shell-local dialog slot preserves the original dispatch continuation across owned form reads. */
export class StagedActionHost<Lease extends StagedActionLease=OwnedUiSurfaceLease> {
 #run:StagedActionRun<Lease>|null=null;
 readonly #runs=new Set<StagedActionRun<Lease>>();
 #opening:object|null=null;
 #view:StagedActionPresentation|null=null;
 readonly #listeners=new Set<()=>void>();
 readonly snapshot=()=>this.#view;
 readonly subscribe=(listener:()=>void)=>{this.#listeners.add(listener);return()=>{this.#listeners.delete(listener);};};
 readonly #publish=()=>{this.#view=this.#run?.presentation()??null;for(const listener of this.#listeners){try{listener();}catch{}}};
 async open(source:StagedActionSource<Lease>,args:StagedArgs):Promise<"executed"|"cancelled">{
  const opening=this.#opening={};await this.#run?.close("cancelled");
  if(this.#opening!==opening)return "cancelled";
  return new Promise(resolve=>{const run:StagedActionRun<Lease>=new StagedActionRun(source,args,this.#publish,result=>{if(this.#run===run)this.#run=null;resolve(result);},()=>{if(run.terminal())this.#runs.delete(run);});this.#runs.add(run);this.#run=run;this.#publish();void run.refresh();});
 }
 readonly execute=()=>{void this.#run?.execute().catch(()=>{});};
 readonly cancel=()=>{void this.close().catch(()=>{});};
 async close():Promise<void>{this.#opening=null;const results=await Promise.allSettled([...this.#runs].map(run=>run.close("cancelled")));const failure=results.find(result=>result.status==="rejected");if(failure?.status==="rejected")throw failure.reason;}
}
