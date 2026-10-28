/** 🪟️ Exact acknowledged ordinary targets lend guarded original reads until paid source retirement. */
import type { OwnedUiInstance, OwnedUiInstanceSurface } from '@semio-tech/ui-contract/retained/instance';
import type { OwnedUiReadSubscription,OwnedUiNodeReadSnapshot } from '@semio-tech/ui-contract/retained/read-lease';
import type { ShardInstanceLifecycleLease } from '@semio-tech/framework/actor/shard-client/activation';
import type { UiIndexGrant } from '@semio-tech/ui-contract/retained/host';
import limits from './lease-limits.json';

export const OWNED_UI_SURFACE_LEASE_LIMITS=Object.freeze(limits);
export type OwnedUiSurfaceReadSource=Pick<OwnedUiInstanceSurface,'view'|'subscribeView'|'subscribeNode'|'acknowledgeRead'|'unsubscribeNode'|'retryNotification'>;
export interface OwnedUiSurfaceLease{readonly source:OwnedUiSurfaceReadSource;readonly signal:AbortSignal;current():boolean;close():Promise<void>}
type Scope={readonly state:{readonly disposing:boolean};readonly closingInstances:ReadonlySet<number>;readonly actorIdByInstance:ReadonlyMap<number,string>;readonly lifecycleByInstance:ReadonlyMap<number,ShardInstanceLifecycleLease>;readonly uiOwnerByInstance:ReadonlyMap<number,OwnedUiInstance>;readonly uiSurfaceByInstance:ReadonlyMap<number,ReadonlyMap<string,OwnedUiInstanceSurface>>;readonly uiReadsByInstance:Map<number,Set<Promise<unknown>>>};
type Installed={readonly instance:number;readonly surfaceId:string;readonly actor:string;readonly lifetime:ShardInstanceLifecycleLease;readonly owner:OwnedUiInstance;readonly surface:OwnedUiInstanceSurface;readonly view:OwnedUiInstanceSurface['view'];readonly stamp:object};
type Certificate={readonly instance:number;readonly surfaceId:string;readonly stamp:object};
type Read={raw:OwnedUiReadSubscription|null;readonly notify:()=>void;readonly public:OwnedUiReadSubscription;failed:boolean};
const error=(code:string)=>new Error(`plugin-ui.surface-lease-${code}`);

/** 🎟️ Kept beside the authentic ACK installer; no scalar target or projected root can mint a certificate. */
export function createAcknowledgedUiSurfaceLeases(scope:Scope,grant:UiIndexGrant,yieldTurn:()=>Promise<void>,schedule:(run:()=>void)=>void){
 const installed=new Map<number,Map<string,Installed>>(),certificates=new WeakMap<object,Certificate>(),leases=new Map<number,Set<ReadLease>>(),revoked=new Set<number>(),pending=new Map<number,Map<string,object>>();
 const mint=Object.freeze({});
 const actorCurrent=(entry:Installed):boolean=>{
  if(revoked.has(entry.instance)||scope.state.disposing||scope.closingInstances.has(entry.instance)||scope.actorIdByInstance.get(entry.instance)!==entry.actor||scope.lifecycleByInstance.get(entry.instance)!==entry.lifetime||scope.uiOwnerByInstance.get(entry.instance)!==entry.owner||scope.uiSurfaceByInstance.get(entry.instance)?.get(entry.surfaceId)!==entry.surface)return false;
  try{entry.lifetime.activation.assertActive();return true;}catch{return false;}
 };
 const current=(entry:Installed):boolean=>{if(!actorCurrent(entry)||pending.get(entry.instance)?.has(entry.surfaceId))return false;const acknowledged=installed.get(entry.instance)?.get(entry.surfaceId);try{return !!acknowledged&&acknowledged.surface===entry.surface&&acknowledged.owner===entry.owner&&acknowledged.lifetime===entry.lifetime&&acknowledged.view===entry.surface.view;}catch{return false;}};
 class ReadLease implements OwnedUiSurfaceLease{
  readonly #entry:Installed;
  readonly #reads=new Map<OwnedUiReadSubscription,Read>();readonly #known=new WeakSet<object>();readonly #abort=new AbortController();#closed=false;#terminal=false;#closing:Promise<void>|undefined;
  readonly source:OwnedUiSurfaceReadSource;
  constructor(authority:object,entry:Installed){
   if(authority!==mint)throw error('authority');this.#entry=entry;
   const read=(id:number|null,notify:()=>void):OwnedUiReadSubscription=>{
    if(!this.current())throw error('stale');if(this.#reads.size>=OWNED_UI_SURFACE_LEASE_LIMITS.reads)throw error('read-capacity');
    const slot:Read={raw:null,notify,public:Object.freeze({get snapshot(){if(!lease.current()||!slot.raw)return null;const snapshot=slot.raw.snapshot;return snapshot?.version===entry.surface.view.revision?snapshot:null;}}),failed:false};const lease=this;this.#reads.set(slot.public,slot);this.#known.add(slot.public);
    const changed=()=>{if(this.#closed)return;try{notify();slot.failed=false;}catch(failure){slot.failed=true;throw failure;}};
    try{slot.raw=id===null?entry.surface.subscribeView(changed):entry.surface.subscribeNode(id,changed);}catch(failure){if(!slot.raw)this.#reads.delete(slot.public);throw failure;}
    if(!this.current()){void this.close().catch(()=>{});throw error('stale');}return slot.public;
   };
   const tracked=(subscription:OwnedUiReadSubscription):Read=>{const slot=this.#reads.get(subscription);if(!slot?.raw)throw error('foreign-read');return slot;};
   this.source=Object.freeze({get view(){if(!lease.current())throw error('stale');return entry.surface.view;},subscribeView:(notify:()=>void)=>read(null,notify),subscribeNode:(id:number,notify:()=>void)=>read(id,notify),acknowledgeRead:(subscription:OwnedUiReadSubscription,snapshot:OwnedUiNodeReadSnapshot)=>{if(!this.#reads.has(subscription)&&this.#known.has(subscription))return;entry.surface.acknowledgeRead(tracked(subscription).raw!,snapshot);},unsubscribeNode:(subscription:OwnedUiReadSubscription)=>{if(!this.#reads.has(subscription)&&this.#known.has(subscription))return;const slot=tracked(subscription);entry.surface.unsubscribeNode(slot.raw!);this.#reads.delete(subscription);},retryNotification:(subscription:OwnedUiReadSubscription)=>{if(!this.current())return false;const slot=tracked(subscription);if(slot.failed){slot.notify();slot.failed=false;return true;}return entry.surface.retryNotification(slot.raw!);}});const lease=this;Object.freeze(this);
  }
  get signal():AbortSignal{return this.#abort.signal;}
  current():boolean{return !this.#closed&&current(this.#entry);}
  matchesSurface(surfaceId:string):boolean{return this.#entry.surfaceId===surfaceId;}
  publication(unavailable=false):void{if(this.#closed||!unavailable&&!this.current())return;for(const slot of [...this.#reads.values()]){if(this.#closed||!unavailable&&!this.current()||!this.#reads.has(slot.public))continue;try{slot.notify();slot.failed=false;}catch{slot.failed=true;}}}
  close():Promise<void>{
   if(this.#closing)return this.#closing;if(this.#terminal)return Promise.resolve();
   let resolve!:()=>void,reject!:(failure:unknown)=>void;const promise=new Promise<void>((accept,refuse)=>{resolve=accept;reject=refuse;});this.#closing=promise;this.#closed=true;this.#abort.abort();
   const reads=scope.uiReadsByInstance.get(this.#entry.instance)??new Set<Promise<unknown>>();reads.add(promise);scope.uiReadsByInstance.set(this.#entry.instance,reads);
   void Promise.resolve().then(async()=>{
    let first:unknown;for(const [key,slot]of this.#reads){try{if(slot.raw)this.#entry.surface.unsubscribeNode(slot.raw);this.#reads.delete(key);}catch(failure){first??=failure;}}
    if(this.#reads.size)throw first??error('close-pending');
    for(let turn=0;this.#entry.owner.maintenancePending;turn++){if(turn>=OWNED_UI_SURFACE_LEASE_LIMITS.maintenanceTurns)throw error('close-budget');const step=this.#entry.owner.advanceMaintenance(grant);if(step.kind==='blocked'||step.kind==='rejected')throw error(`close-${step.kind}`);if(turn%OWNED_UI_SURFACE_LEASE_LIMITS.yieldEvery===OWNED_UI_SURFACE_LEASE_LIMITS.yieldEvery-1)await yieldTurn();}
    leases.get(this.#entry.instance)?.delete(this);this.#terminal=true;
   }).then(()=>{reads.delete(promise);resolve();},failure=>{reads.delete(promise);this.#closing=undefined;reject(failure);});return promise;
  }
 }
 Object.freeze(ReadLease.prototype);
 const beginPatch=(instance:number,surfaceId:string,intake:object):void=>{const entries=pending.get(instance)??new Map<string,object>();entries.set(surfaceId,intake);pending.set(instance,entries);schedule(()=>{for(const lease of [...(leases.get(instance)??[])])if(lease.matchesSurface(surfaceId))lease.publication(true);});};
 const install=(instance:number,surfaceId:string,surface:OwnedUiInstanceSurface,intake:object):boolean=>{
  if(pending.get(instance)?.get(surfaceId)!==intake)return false;
  const actor=scope.actorIdByInstance.get(instance),lifetime=scope.lifecycleByInstance.get(instance),owner=scope.uiOwnerByInstance.get(instance);if(!actor||!lifetime||!owner)return false;
  const entry:Installed={instance,surfaceId,actor,lifetime,owner,surface,view:surface.view,stamp:Object.freeze({})};if(!actorCurrent(entry))return false;const entries=installed.get(instance)??new Map<string,Installed>();entries.set(surfaceId,entry);installed.set(instance,entries);
  if(!actorCurrent(entry)||entry.view!==surface.view){entries.delete(surfaceId);return false;}pending.get(instance)!.delete(surfaceId);if(!current(entry)){entries.delete(surfaceId);return false;}schedule(()=>{for(const lease of [...(leases.get(instance)??[])])if(lease.matchesSurface(surfaceId))lease.publication();});return true;
 };
 const issueTarget=(instance:number,surfaceId:string,target:object,view:OwnedUiInstanceSurface['view']):boolean=>{const entry=installed.get(instance)?.get(surfaceId);if(!entry||entry.view!==view||!current(entry))return false;certificates.set(target,{instance,surfaceId,stamp:entry.stamp});if(installed.get(instance)?.get(surfaceId)!==entry||entry.surface.view!==view||!current(entry)){certificates.delete(target);return false;}return true;};
 const acquireOwnedUiSurface=(instance:number,target:unknown):OwnedUiSurfaceLease|undefined=>{
  if(!target||typeof target!=='object')return;const certificate=certificates.get(target);if(!certificate||certificate.instance!==instance)return;const entry=installed.get(instance)?.get(certificate.surfaceId);if(!entry||entry.stamp!==certificate.stamp||!current(entry))return;const owners=leases.get(instance)??new Set<ReadLease>();if(owners.size>=OWNED_UI_SURFACE_LEASE_LIMITS.leases)throw error('capacity');const lease=new ReadLease(mint,entry);owners.add(lease);leases.set(instance,owners);if(!lease.current()||installed.get(instance)?.get(certificate.surfaceId)?.stamp!==certificate.stamp){void lease.close().catch(()=>{});return;}return lease;
 };
 const beginClose=(instance:number):void=>{revoked.add(instance);for(const lease of [...(leases.get(instance)??[])])void lease.close().catch(()=>{});};
 const closeInstance=async(instance:number):Promise<void>=>{revoked.add(instance);const results=await Promise.allSettled([...(leases.get(instance)??[])].map(lease=>lease.close()));const failure=results.find((result):result is PromiseRejectedResult=>result.status==='rejected');if(failure)throw failure.reason;if((leases.get(instance)?.size??0)!==0)throw error('retirement-pending');leases.delete(instance);installed.delete(instance);pending.delete(instance);revoked.delete(instance);};
 return {beginPatch,install,issueTarget,acquireOwnedUiSurface,beginClose,closeInstance};
}
