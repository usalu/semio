import {expect,test} from 'bun:test';
import Ajv2020 from 'ajv/dist/2020.js';
import {createAcknowledgedUiSurfaceLeases,OWNED_UI_SURFACE_LEASE_LIMITS} from './lease.ts';
import cases from './lease-cases.json';
import schema from './lease.schema.json';
import limitSchema from './lease-limits.schema.json';
import type {OwnedUiInstance,OwnedUiInstanceSurface} from '@semio-tech/ui-contract/retained/instance';
import type {OwnedUiReadSubscription,OwnedUiNodeReadSnapshot} from '@semio-tech/ui-contract/retained/read-lease';
import type {ShardInstanceLifecycleLease} from '@semio-tech/framework/actor/shard-client/activation';

function fixture(){
 let view={revision:1,root:1,hash:'native:1'},blocked=false,unsubscribeFails=false,maintenance=0,active=true,onSubscribe:(()=>void)|undefined;const raw=new Map<OwnedUiReadSubscription,()=>void>(),scheduled:(()=>void)[]=[];
 const source={get view(){return view;},subscribeView:(notify:()=>void)=>source.subscribeNode(0,notify),subscribeNode:(_id:number,notify:()=>void)=>{const read={get snapshot(){return {version:view.revision,record:{}} as OwnedUiNodeReadSnapshot;}};raw.set(read,notify);onSubscribe?.();return read;},acknowledgeRead:(_read:OwnedUiReadSubscription,_snapshot:OwnedUiNodeReadSnapshot)=>{},unsubscribeNode:(read:OwnedUiReadSubscription)=>{if(unsubscribeFails)throw new Error('unsubscribe-refused');raw.delete(read);maintenance+=2;},retryNotification:(read:OwnedUiReadSubscription)=>{raw.get(read)?.();return true;}};
 const owner={get maintenancePending(){return maintenance>0;},advanceMaintenance(){if(blocked)return {kind:'blocked' as const,phase:'fixture',items:0,bytes:0};maintenance--;return {kind:'pending' as const,phase:'fixture',items:1,bytes:64};}} as unknown as OwnedUiInstance;
 const lifetime={activation:{assertActive(){if(!active)throw new Error('revoked');}}} as unknown as ShardInstanceLifecycleLease;
 const scope={state:{disposing:false},closingInstances:new Set<number>(),actorIdByInstance:new Map([[7,'actor']]),lifecycleByInstance:new Map([[7,lifetime]]),uiOwnerByInstance:new Map([[7,owner]]),uiSurfaceByInstance:new Map([[7,new Map([['7:window',source as OwnedUiInstanceSurface]])]]),uiReadsByInstance:new Map<number,Set<Promise<unknown>>>()};
 const registry=createAcknowledgedUiSurfaceLeases(scope,{maxItems:1,maxBytes:4096},async()=>{},run=>scheduled.push(run));
 const publish=()=>{const intake={};registry.beginPatch(7,'7:window',intake);expect(registry.install(7,'7:window',source as OwnedUiInstanceSurface,intake)).toBe(true);};
 const target=()=>{const target={key:'window',hash:view.hash};expect(registry.issueTarget(7,'7:window',target,view)).toBe(true);return target;};
 return {scope,registry,source,owner,raw,scheduled,publish,target,setView(){view={revision:view.revision+1,root:1,hash:`native:${view.revision+1}`};},block(value:boolean){blocked=value;},failUnsubscribe(value:boolean){unsubscribeFails=value;},onSubscribe(value:()=>void){onSubscribe=value;},revoke(){active=false;},drain(){while(scheduled.length)scheduled.shift()!();}};
}
test('ordinary target metadata follows the closed ACK lifetime matrix',async()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(cases)).toBe(true);expect(OWNED_UI_SURFACE_LEASE_LIMITS.leases).toBe(cases.limits.leases);expect(OWNED_UI_SURFACE_LEASE_LIMITS.reads).toBe(cases.limits.reads);
 expect(new Ajv2020({strict:true}).compile(limitSchema)(OWNED_UI_SURFACE_LEASE_LIMITS)).toBe(true);
 for(const row of cases.cases){const f=fixture();const first={key:'window',hash:'native:1'};let target:object=first,instance=7,registry=f.registry,lease:ReturnType<typeof registry.acquireOwnedUiSurface>;
  if(row.state==='before-ack'){expect(f.registry.issueTarget(7,'7:window',first,f.source.view)).toBe(false);}else{f.publish();target=f.target();lease=registry.acquireOwnedUiSurface(7,target);expect(lease).toBeDefined();}
  switch(row.state){case 'pending-next-ack':f.registry.beginPatch(7,'7:window',{});break;case 'next-ack-old-target':f.setView();f.publish();break;case 'copied-target':target={...target};break;case 'foreign-instance':instance=8;break;case 'foreign-handle':registry=fixture().registry;break;case 'closing':f.scope.closingInstances.add(7);break;case 'remounted':f.scope.lifecycleByInstance.set(7,{activation:{assertActive(){}}} as ShardInstanceLifecycleLease);break;}
  const acquired=registry.acquireOwnedUiSurface(instance,target);expect(!!acquired).toBe(row.acquire);expect(lease?.current()??false).toBe(row.current);await acquired?.close();await lease?.close();await f.registry.closeInstance(7);
 }
});
test('forged targets and copied scalar hashes never invoke getters or obtain original reads',async()=>{
 const f=fixture();f.publish();const target=f.target();let getters=0;const forged=Object.defineProperties({}, {key:{get(){getters++;throw new Error('getter');}},hash:{get(){getters++;throw new Error('getter');}}});expect(f.registry.acquireOwnedUiSurface(7,forged)).toBeUndefined();expect(f.registry.acquireOwnedUiSurface(7,{...target})).toBeUndefined();expect(getters).toBe(0);const wrongView={...f.source.view};expect(f.registry.issueTarget(7,'7:window',forged,wrongView)).toBe(false);await f.registry.closeInstance(7);
});
test('pre-ACK replacement and reentrant subscribe retire the actual read before a stale return',async()=>{
 const f=fixture();f.publish();const lease=f.registry.acquireOwnedUiSurface(7,f.target())!;const read=lease.source.subscribeNode(1,()=>{});expect(read.snapshot).not.toBeNull();const pending={};f.registry.beginPatch(7,'7:window',pending);expect(lease.current()).toBe(false);expect(read.snapshot).toBeNull();f.setView();expect(f.registry.install(7,'7:window',f.source as OwnedUiInstanceSurface,{})).toBe(false);expect(lease.current()).toBe(false);expect(f.registry.install(7,'7:window',f.source as OwnedUiInstanceSurface,pending)).toBe(true);expect(lease.current()).toBe(true);expect(read.snapshot?.version).toBe(2);
 f.onSubscribe(()=>f.scope.closingInstances.add(7));expect(()=>lease.source.subscribeNode(2,()=>{})).toThrow('stale');await lease.close();expect(f.raw.size).toBe(0);await f.registry.closeInstance(7);
});
test('unsubscribe and native maintenance failures retain the same lease close owner for retry',async()=>{
 const f=fixture();f.publish();const lease=f.registry.acquireOwnedUiSurface(7,f.target())!;const read=lease.source.subscribeNode(1,()=>{});f.failUnsubscribe(true);await expect(lease.close()).rejects.toThrow('unsubscribe-refused');expect(lease.current()).toBe(false);expect(f.raw.size).toBe(1);expect(read.snapshot).toBeNull();f.failUnsubscribe(false);f.block(true);await expect(lease.close()).rejects.toThrow('close-blocked');expect(f.raw.size).toBe(0);await expect(f.registry.closeInstance(7)).rejects.toThrow('close-blocked');f.block(false);await lease.close();lease.source.unsubscribeNode(read);await f.registry.closeInstance(7);expect(f.scope.uiReadsByInstance.get(7)?.size).toBe(0);
});
test('publication reentrancy uses a bounded resident read snapshot and close precedes source retirement',async()=>{
 const f=fixture();f.publish();f.drain();const lease=f.registry.acquireOwnedUiSurface(7,f.target())!;let calls=0;let read:OwnedUiReadSubscription;read=lease.source.subscribeNode(1,()=>{calls++;lease.source.unsubscribeNode(read);read=lease.source.subscribeNode(1,()=>{});});f.publish();f.drain();expect(calls).toBe(1);let reentered:Promise<void>|undefined;lease.signal.addEventListener('abort',()=>{expect(lease.current()).toBe(false);reentered=lease.close();});const closing=lease.close();expect(reentered).toBe(closing);await closing;expect(f.raw.size).toBe(0);await f.registry.closeInstance(7);
});
test('read and lease caps refuse new authority while preserving existing sources',async()=>{
 const f=fixture();f.publish();const target=f.target(),lease=f.registry.acquireOwnedUiSurface(7,target)!;for(let n=0;n<OWNED_UI_SURFACE_LEASE_LIMITS.reads;n++)lease.source.subscribeNode(n,()=>{});expect(()=>lease.source.subscribeNode(99,()=>{})).toThrow('read-capacity');const leases=[lease];for(let n=1;n<OWNED_UI_SURFACE_LEASE_LIMITS.leases;n++)leases.push(f.registry.acquireOwnedUiSurface(7,target)!);expect(()=>f.registry.acquireOwnedUiSurface(7,target)).toThrow('capacity');f.registry.beginClose(7);expect(leases.every(lease=>!lease.current()&&lease.signal.aborted)).toBe(true);await f.registry.closeInstance(7);expect(f.raw.size).toBe(0);
});
test('lease reflection cannot expose or mint the original facade owner',async()=>{
 const f=fixture();f.publish();const lease=f.registry.acquireOwnedUiSurface(7,f.target())!;expect(Object.getOwnPropertyNames(lease)).toEqual(['source']);expect(Object.isFrozen(lease)).toBe(true);expect(Object.isFrozen(Object.getPrototypeOf(lease))).toBe(true);let getters=0;const forged=Object.defineProperty({},'surface',{get(){getters++;throw new Error('getter');}});expect(()=>Reflect.construct(Object.getPrototypeOf(lease).constructor,[{},forged])).toThrow('authority');expect(getters).toBe(0);await lease.close();await f.registry.closeInstance(7);
});
