import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {Observable,fromEvent,takeUntil,finalize} from 'rxjs';
import cases from '../🧬️schema/🔺️diff/control-cases.json';
import schema from '../🧬️schema/🔺️diff/control.schema.json';
import * as diff from '../🧬️schema/🔺️diff/🟦️.ts';
import * as mutation from '../🧬️schema/🧬️mutations/🟦️.ts';
import {ifcTemplateIdentityKeys,ifcTemplateIdentityKeysAsync,ifcTemplateIdentityRequirementsAsync,captureIfcTemplateSteps,ifcTemplateDiffSteps} from '../🧬️schema/🏷️templates/✍️apply/🟦️.ts';
import {base,application,s} from './template-fixtures.ts';
import type {IfcApplyOptions,IfcEditSteps} from '../🧬️schema/🔺️diff/🟦️.ts';
const progress={phase:'compare' as const,completed:0,total:2};
function* steps():IfcEditSteps<number>{yield progress;yield {...progress,completed:1};return 7;}
const command=()=>({mutation:'setEntityArg' as const,id:1n,index:2,value:s('edited')});
type Outcome={failed:boolean;error?:unknown;value?:unknown};
async function outcome(run:()=>unknown):Promise<Outcome>{try{return{failed:false,value:await run()};}catch(error){return{failed:true,error};}}
function routes():[string,(options:IfcApplyOptions)=>unknown][]{
 return [
 ['runner-sync',o=>diff.runIfcEdit(steps(),o)],['runner-async',o=>diff.runIfcEditAsync(steps(),o)],
 ['diff-sync',o=>diff.applyIfcDiffTransaction(base(),{},o)],['diff-async',o=>diff.applyIfcDiffTransactionAsync(base(),{},o)],
 ['apply-diff-sync',o=>diff.applyIfcDiff(base(),{},o)],['apply-diff-async',o=>diff.applyIfcDiffAsync(base(),{},o)],
 ['inverse-diff-sync',o=>diff.inverseIfcDiff(base(),{},o)],['inverse-diff-async',o=>diff.inverseIfcDiffAsync(base(),{},o)],
 ['absorb-sync',o=>diff.absorbIfcDiff({}, {},o)],['absorb-async',o=>diff.absorbIfcDiffAsync({}, {},o)],
 ['compose-sync',o=>diff.composeIfcDiff(base(),{}, {},o)],['compose-async',o=>diff.composeIfcDiffAsync(base(),{}, {},o)],
 ['mutation-sync',o=>mutation.applyIfcMutation(base(),command(),o)],['mutation-async',o=>mutation.applyIfcMutationAsync(base(),command(),o)],
 ['mutation-diff-sync',o=>mutation.ifcMutationDiff(base(),command(),o)],['mutation-diff-async',o=>mutation.ifcMutationDiffAsync(base(),command(),o)],
 ['inverse-mutation-sync',o=>mutation.inverseIfcMutation(base(),command(),o)],['inverse-mutation-async',o=>mutation.inverseIfcMutationAsync(base(),command(),o)],
 ['keys-sync',o=>ifcTemplateIdentityKeys(base(),application(),o)],['keys-async',o=>ifcTemplateIdentityKeysAsync(base(),application(),o)],
 ['requirements-async',o=>ifcTemplateIdentityRequirementsAsync(base(),application(),o)],
 ['history-create-sync',o=>mutation.IfcEditHistory.createSync(base(),'persistedLocal',o)],['history-create-async',o=>mutation.IfcEditHistory.create(base(),'persistedLocal',o)],
 ...(['readSnapshotSync','readSnapshot','readEventsSync','readEvents'] as const).map(name=>[name,(o:IfcApplyOptions)=>{const h=mutation.IfcEditHistory.createSync(base());if(name.includes('Events'))h.execute(command(),0);return h[name](o);}] as [string,(o:IfcApplyOptions)=>unknown]),
 ...(['execute','executeAsync','undo','undoAsync','redo','redoAsync'] as const).map(name=>[name,(o:IfcApplyOptions)=>{const h=mutation.IfcEditHistory.createSync(base());if(name.startsWith('undo')||name.startsWith('redo'))h.execute(command(),0);if(name.startsWith('redo'))h.undo(1);return name==='execute'||name==='executeAsync'?h[name](command(),0,o):h[name](h.revision,o);}] as [string,(o:IfcApplyOptions)=>unknown])
 ];
}
test('closed language-neutral control law data is independently schema admitted',()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);});
for(const [name,run]of routes()){
 test(name+' preserves original signal and unbound observer',async()=>{
  const original=new AbortController(),replacement=new AbortController();let calls=0,redirected=0,receiver:unknown='unset';
  const options:IfcApplyOptions={signal:original.signal,yieldEvery:1,onProgress:function(this:unknown){receiver=this;calls++;if(this&&typeof this==='object'){Reflect.set(this,'signal',replacement.signal);Reflect.set(this,'onProgress',()=>redirected++);}options.signal=replacement.signal;options.onProgress=()=>redirected++;original.abort();}};
  const result=await outcome(()=>run(options));expect(result.failed).toBe(true);expect((result.error as diff.IfcEditError)?.code).toBe(cases.cancellation.code);expect(calls).toBe(1);expect(redirected).toBe(0);expect(receiver).toBeUndefined();
 });
 test(name+' reads every original control property once',async()=>{
  const counts=new Map<string,number>();let original=0,replacement=0;const options={} as IfcApplyOptions;
  for(const key of cases.properties)Object.defineProperty(options,key,{get(){const n=(counts.get(key)??0)+1;counts.set(key,n);return key==='onProgress'?(n===1?()=>original++:()=>replacement++):key==='yieldEvery'?4096:undefined;}});
  const result=await outcome(()=>run(options));expect(result.failed).toBe(false);expect(original).toBeGreaterThan(0);expect(replacement).toBe(0);expect(Object.fromEntries(counts)).toEqual(Object.fromEntries(cases.properties.map(k=>[k,1])));
 });
}
for(const mode of cases.modes){
 const drive=(g:IfcEditSteps<unknown>,o:IfcApplyOptions)=>mode==='sync'?diff.runIfcEdit(g,o):diff.runIfcEditAsync(g,o);
 for(const kind of cases.failures)test(mode+' exact observer failure '+kind+' survives finite cleanup',async()=>{
  const failure=kind==='undefined'?undefined:kind==='object'?{original:true}:kind==='editError'?new diff.IfcEditError('cancelled','caller'):new Error('caller'),cleanup=new Error('cleanup');let finished=false,count=0;
  function* g():IfcEditSteps<unknown>{try{yield progress;return 7;}finally{for(let i=0;i<cases.cleanup.yielded;i++)yield progress;finished=true;throw cleanup;}}
  const result=await outcome(()=>drive(g(),{yieldEvery:1,onProgress(){count++;throw failure;}}));expect(result.failed).toBe(true);expect(result.error).toBe(failure);expect(finished).toBe(true);expect(count).toBe(1);
 });
 test(mode+' cancellation wins over throwing yielding cleanup',async()=>{
  const original=new AbortController();let finished=false,count=0;
  function* g():IfcEditSteps<unknown>{try{yield progress;return 7;}finally{for(let i=0;i<3;i++)yield progress;finished=true;throw new Error('cleanup');}}
  const result=await outcome(()=>drive(g(),{signal:original.signal,yieldEvery:1,onProgress(){count++;original.abort();}}));expect((result.error as diff.IfcEditError)?.code).toBe('cancelled');expect(finished).toBe(true);expect(count).toBe(1);
 });
 test(mode+' generator-caught observer failure remains terminal',async()=>{
  const failure={first:true};const options:IfcApplyOptions={onProgress(){throw failure;}};function* g():IfcEditSteps<unknown>{try{options.onProgress!(progress);}catch{}yield progress;return 1;}
  const result=await outcome(()=>drive(g(),options));expect(result.failed).toBe(true);expect(result.error).toBe(failure);
 });
 test(mode+' final generator return cancellation cannot publish',async()=>{
  const original=new AbortController();function* g():IfcEditSteps<unknown>{yield progress;original.abort();return 7;}
  const result=await outcome(()=>drive(g(),{signal:original.signal}));expect((result.error as diff.IfcEditError)?.code).toBe('cancelled');
 });
}
test('cancelled history leaves revision, events and source unchanged',async()=>{const h=mutation.IfcEditHistory.createSync(base()),original=new AbortController(),options:IfcApplyOptions={signal:original.signal,onProgress(){options.signal=new AbortController().signal;original.abort();}};const result=await outcome(()=>h.executeAsync(command(),0,options));expect(result.failed).toBe(true);expect(h.revision).toBe(0);expect(h.events).toEqual([]);expect(h.snapshot).toEqual(base());});
test('caller limit replacement cannot erase original zero entity bound',async()=>{const options:IfcApplyOptions={maxEntities:0,onProgress(){options.maxEntities=1000;}};const result=await outcome(()=>mutation.applyIfcMutationAsync(base(),command(),options));expect((result.error as diff.IfcEditError)?.code).toBe('entityLimit');});
test('standalone factories capture controls before any next/model access',()=>{
 const factories:((options:IfcApplyOptions)=>IfcEditSteps<unknown>)[]=[
 (o:IfcApplyOptions)=>diff.copyIfcOwnedSteps(s('x'),o),(o:IfcApplyOptions)=>diff.captureIfcGraphSteps(base(),'validateBase',o),(o:IfcApplyOptions)=>diff.captureIfcDiffSteps({fileName:[s('x')]},o),(o:IfcApplyOptions)=>diff.validateIfcGraphSteps(base(),'validateBase',o),(o:IfcApplyOptions)=>diff.admitIfcDiffSteps({fileName:[s('x')]},o),(o:IfcApplyOptions)=>diff.validateIfcNativeShapeSteps(base(),'snapshot',o),(o:IfcApplyOptions)=>diff.prepareIfcDiffSteps(base(),function*(){return{};},o),(o:IfcApplyOptions)=>captureIfcTemplateSteps(application(),o),(o:IfcApplyOptions)=>ifcTemplateDiffSteps(base(),application(),o)];
 for(const factory of factories){const reads=new Map<string,number>();let maximum=0;const options={} as IfcApplyOptions;for(const key of cases.properties)Object.defineProperty(options,key,{get(){reads.set(key,(reads.get(key)??0)+1);return key==='maxValues'?maximum:undefined;}});const g=factory(options);expect(Object.fromEntries(reads)).toEqual(Object.fromEntries(cases.properties.map(k=>[k,1])));maximum=100000;try{diff.runIfcEdit(g);}catch{}expect(Object.fromEntries(reads)).toEqual(Object.fromEntries(cases.properties.map(k=>[k,1])));}
});
test('zero bounds remain limits and zero yieldEvery remains invalid only for async scheduling',async()=>{expect(()=>diff.applyIfcDiff(base(),{},{maxEntities:0})).toThrow('entityLimit');expect(()=>diff.applyIfcDiff(base(),{},{maxValues:0})).toThrow('valueLimit');expect(diff.runIfcEdit(steps(),{yieldEvery:0})).toBe(7);expect((await outcome(()=>diff.runIfcEditAsync(steps(),{yieldEvery:0}))).error).toMatchObject({code:'invalidLimit'});});
test('thirdparty cancellation stream agrees on admitted prefix and completed teardown',()=>{
 function oracle():{prefix:number[];closed:boolean}{const abort=new AbortController(),prefix:number[]=[];let closed=false;new Observable<number>(subscriber=>{for(let i=0;i<3&&!subscriber.closed;i++)subscriber.next(i);return()=>{closed=true;};}).pipe(takeUntil(fromEvent(abort.signal,'abort')),finalize(()=>{closed=true;})).subscribe(value=>{prefix.push(value);abort.abort();});return{prefix,closed};}
 const original=new AbortController(),prefix:number[]=[];let closed=false;function* g():IfcEditSteps<unknown>{try{for(let i=0;i<3;i++)yield{...progress,completed:i};}finally{yield progress;closed=true;}return undefined;}
 expect(()=>diff.runIfcEdit(g(),{signal:original.signal,onProgress(p){prefix.push(p.completed);original.abort();}})).toThrow('cancelled');expect({prefix,closed}).toEqual(oracle());
});
for(const [name,run]of routes())for(const kind of cases.failures)test(name+' preserves exact caller failure '+kind,async()=>{
 const failure=kind==='undefined'?undefined:kind==='object'?{caller:true}:kind==='editError'?new diff.IfcEditError('invalidLimit','caller'):new Error('caller');let calls=0;
 const result=await outcome(()=>run({onProgress(){calls++;throw failure;}}));expect(result.failed).toBe(true);expect(result.error).toBe(failure);expect(calls).toBe(1);
});
for(const kind of cases.failures)test('template semantic progress preserves exact mapped caller failure '+kind,async()=>{
 const failure=kind==='undefined'?undefined:kind==='object'?{caller:true}:kind==='editError'?new diff.IfcEditError('invalidDomain','caller'):new Error('caller'),h=mutation.IfcEditHistory.createSync(base());let seen=false;
 const result=await outcome(()=>h.executeAsync({mutation:'applyTemplate',application:application()},0,{onProgress(p){if(p.phase==='validateTemplate'){seen=true;throw failure;}}}));expect(seen).toBe(true);expect(result.failed).toBe(true);expect(result.error).toBe(failure);expect(h.revision).toBe(0);
});
test('template final semantic cancellation keeps original signal through publication',async()=>{const h=mutation.IfcEditHistory.createSync(base()),abort=new AbortController();let seen=false;const options:IfcApplyOptions={signal:abort.signal,onProgress(p){if(p.phase==='validateTemplate'){seen=true;options.signal=new AbortController().signal;abort.abort();}}};const result=await outcome(()=>h.executeAsync({mutation:'applyTemplate',application:application()},0,options));expect(seen).toBe(true);expect(result.failed).toBe(true);expect(h.revision).toBe(0);});
for(const mode of cases.modes)for(const key of cases.properties)test(mode+' getter failure '+key+' closes already entered finite generator',async()=>{
 const failure={getter:key};let finished=false;function* g():IfcEditSteps<unknown>{try{yield progress;}finally{yield progress;finished=true;}return undefined;}
 const generator=g();generator.next();const options=Object.defineProperty({},key,{get(){throw failure;}}),result=await outcome(()=>mode==='sync'?diff.runIfcEdit(generator,options):diff.runIfcEditAsync(generator,options));
 expect(result.failed).toBe(true);expect(result.error).toBe(failure);expect(finished).toBe(true);
});
test('captured ordinary scope cannot be redirected through observer receiver or function call property',()=>{let count=0;function observer(this:unknown){expect(this).toBeUndefined();count++;}Object.defineProperty(observer,'call',{get(){throw new Error('must not read call');}});expect(diff.runIfcEdit(steps(),{onProgress:observer})).toBe(7);expect(count).toBe(2);});
for(const row of cases.limits)test('captured '+row.name+' zero cannot be raised by progress',async()=>{const options:IfcApplyOptions={[row.name]:row.value,onProgress(){Object.assign(options,{maxEntities:100000,maxValues:100000,maxDepth:512});}};const result=await outcome(()=>mutation.applyIfcMutationAsync(base(),command(),options));expect(result.failed).toBe(true);expect((result.error as diff.IfcEditError)?.code).toBe(row.code);});
test('independent operations recapture the same mutable caller object',()=>{let first=0,second=0;const options:IfcApplyOptions={onProgress(){first++;}};expect(diff.runIfcEdit(steps(),options)).toBe(7);options.onProgress=()=>second++;expect(diff.runIfcEdit(steps(),options)).toBe(7);expect(first).toBe(2);expect(second).toBe(2);});
