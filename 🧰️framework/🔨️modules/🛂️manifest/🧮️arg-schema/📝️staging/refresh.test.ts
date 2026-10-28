import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import schema from './refresh.schema.json';
import stagingSchema from './🔣️.json';
import cases from './refresh-cases.json';
import {bindStagedFormTargets,type StagedFormRefreshTarget} from './refresh.ts';
import type {PluginViewState} from '../../👁️view-state-and-public-invocation/🟦️.ts';
const definition=(window:string,action:string)=>['left','right'].includes(window)&&action==='attachPropertySet'?{bodyKey:'framework.action.form'}:undefined;
const targets=cases.targets as StagedFormRefreshTarget[],view=cases.view as PluginViewState;
test('auxiliary refresh targets agree with the independent closed schema',()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(stagingSchema);const validate=ajv.compile(schema);for(const target of targets)expect(validate(target)).toBe(true);expect(validate({...targets[0],windowInstances:[]})).toBe(false);expect(validate({...targets[0],stagedRequest:{...targets[0]!.stagedRequest,sourceAuthority:true}})).toBe(false);
});
test('actual window projection is distinct from consumed form identity and preserves focus/roster',()=>{
 const before=structuredClone(view),bound=bindStagedFormTargets(view,targets,definition,['left','right','panel:inspection']);expect(bound).toHaveLength(2);
 for(let i=0;i<bound.length;i++){const row=bound[i]!,target=targets[i]!;expect(row.key).toBe(target.key);expect(row.viewState.windowId).toBe(target.contextWindowId);expect(row.viewState.activeWindowKindId).toBe(i?'model':'details');expect(row.viewState.activeUtilityId).toBe(i?'edit-right':'edit-left');expect(row.viewState.focusedWindowId).toBe('right');expect(row.viewState.windowInstances).toBe(view.windowInstances);expect(row.stagedRequest).toBe(target.stagedRequest);expect(Object.hasOwn(row.viewState,'stagedRequest')).toBe(false);expect(row.stagedRequest.token.generation).toBe(target.stagedRequest.token.generation);}
 expect(view).toEqual(before);
});
for(const row of cases.refusals)test('form refresh binding refuses '+row.id,()=>{
 const target=structuredClone(targets[0]!) as unknown as Record<string,unknown>;let cursor=target;for(const key of row.path.slice(0,-1))cursor=cursor[key] as Record<string,unknown>;cursor[row.path.at(-1)!]=row.value;
 expect(()=>bindStagedFormTargets(view,[target as unknown as StagedFormRefreshTarget],definition,['left','right'])).toThrow('staging.refresh');
});
test('duplicate real-window owners and occupied form surfaces refuse atomically',()=>{
 expect(()=>bindStagedFormTargets(view,[targets[0]!,{...targets[0]!,key:'another-form'}],definition,[])).toThrow('staging.refresh');expect(()=>bindStagedFormTargets(view,[targets[0]!,{...targets[1]!,key:targets[0]!.key}],definition,[])).toThrow('staging.refresh');expect(()=>bindStagedFormTargets(view,targets,definition,['form-right'])).toThrow('staging.refresh');expect(bindStagedFormTargets(view,[],definition,['left'])).toEqual([]);
});
test('duplicate actual roster identities and unresolved locale refuse',()=>{
 expect(()=>bindStagedFormTargets({...view,windowInstances:[...view.windowInstances!,view.windowInstances![0]!]},targets,definition,[])).toThrow('staging.refresh');expect(()=>bindStagedFormTargets({...view,locale:undefined},targets,definition,[])).toThrow('staging.refresh');
});
