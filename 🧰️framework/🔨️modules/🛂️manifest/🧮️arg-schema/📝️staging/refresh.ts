/** 🪟️ Bind consumed auxiliary form identities to explicitly declared real-window contexts. */
import {windowViewContext,type PluginViewState} from '../../👁️view-state-and-public-invocation/🟦️.ts';
import type {StagedActionDefinition,StagedFormRequest} from './🟦️.ts';
export interface StagedFormRefreshTarget {
 readonly key:string;readonly contextWindowId:string;readonly bodyKey:string;readonly stagedRequest:StagedFormRequest;readonly hash?:string;
}
export interface BoundStagedFormRefreshTarget extends StagedFormRefreshTarget {readonly viewState:PluginViewState}
/** 🎯️ Validate every host-owned target before the caller submits any auxiliary surface mount. */
export function bindStagedFormTargets(view:PluginViewState,targets:readonly StagedFormRefreshTarget[],definition:(window:string,action:string)=>StagedActionDefinition|undefined,occupiedSurfaceKeys:Iterable<string>):readonly BoundStagedFormRefreshTarget[]{
 if(targets.length===0)return [];
 const fail=():never=>{throw new Error('staging.refresh');};
 const roster=view.windowInstances;if(!roster||view.locale!=='en'&&view.locale!=='de'||targets.length>roster.length)return fail();
 const windows=new Set<string>();for(const window of roster){if(!window.id||windows.has(window.id))return fail();windows.add(window.id);}
 const surfaces=new Set(occupiedSurfaceKeys),owners=new Set<string>(),bound:BoundStagedFormRefreshTarget[]=[];
 for(const target of targets){
  const token=target.stagedRequest.token,action=definition(target.contextWindowId,token.action);
  if(!target.key||surfaces.has(target.key)||!windows.has(target.contextWindowId)||owners.has(target.contextWindowId)||!target.bodyKey||!action||action.bodyKey!==target.bodyKey||token.window!==target.contextWindowId||!token.action||!token.revision||!/^(0|[1-9][0-9]{0,19})$/u.test(token.generation)||BigInt(token.generation)>(1n<<64n)-1n||target.stagedRequest.locale!==view.locale)return fail();
  const viewState=windowViewContext(view,target.contextWindowId);if(!viewState)return fail();
  surfaces.add(target.key);owners.add(target.contextWindowId);bound.push({...target,viewState});
 }
 return bound;
}
