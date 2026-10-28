import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './cases.json';
import schema from './cases.schema.json';
import {StagedFormSession,type StagedControlSource,type StagedResidentControl,type StagedFormIntent} from './🟦️.ts';
import {STAGED_FORM_EVENT_ACTION,type StagedFormBinding,type StagedFormStatus} from '@semio-tech/framework/manifest/staging';

function fixture(token=cases.token){
 const action={scope:'ifc',name:STAGED_FORM_EVENT_ACTION,version:1};
 const binding:StagedFormBinding={token,layout:'preserve',edits:[{source:'text',path:['count']}]};
 let view={surface:'form-left',revision:'publication-1'},status:StagedFormStatus={token,ready:true,diagnostics:[],diagnosticOffset:'0',diagnosticTotal:'0'};
 let control:StagedResidentControl<string>={node:'input-1',key:'count',trigger:'change',disabled:false,editable:true,input:'text',action,binding};
 const source:StagedControlSource<string,string>={view:()=>view,status:()=>status,read:(node,trigger)=>node===control.node&&trigger===control.trigger?control:undefined};
 const session=new StagedFormSession({token,locale:'en',controllerId:'ifc'},cases.args);
 const intent=(seq=1n,input:unknown='-'):StagedFormIntent<string,string>=>({...view,node:'input-1',nodeKey:'count',trigger:'change',action,args:{token:{...token,generation:'1'},path:['target'],value:'1'},input,seq});
 return {session,source,intent,control,binding,view,status,setView:(value:typeof view)=>view=value,setStatus:(value:StagedFormStatus)=>status=value,setControl:(value:typeof control)=>control=value};
}

test('language-neutral staged intent fixtures are closed and independently schema-valid',()=>{
 const ajv=new Ajv({strict:true});expect(ajv.validate(schema,cases)).toBe(true);expect(ajv.validate(schema,{...cases,authority:true})).toBe(false);
});

for(const row of cases.attacks)test('original staged intent refuses '+row.id,()=>{
 const f=fixture();expect(f.session.publish(f.source)).toBe(true);const before=f.session.request();expect(f.session.handle({...f.intent(),[row.field]:row.value} as StagedFormIntent<string,string>).kind).toBe('refused');expect(f.session.request()).toEqual(before);expect(f.session.canExecute()).toBe(true);
});

test('resident binding wins over forged event args and agrees with independent JSON Patch',async()=>{
 const f=fixture();f.session.publish(f.source);const before=structuredClone(cases.args),result=f.session.handle(f.intent(9007199254740993n,'9223372036854775807'));
 expect(result.kind).toBe('staged');const patch=await import('fast-json-patch');const expected=patch.applyPatch(structuredClone(cases.args),[{op:'replace',path:'/count',value:'9223372036854775807'}],true,false,true).newDocument;
 expect(f.session.request()?.args).toEqual(expected);expect(f.session.request()?.token.generation).toBe('9007199254740994');expect(cases.args).toEqual(before);expect(f.session.request()?.args.target).toBe('18446744073709551615');expect(f.session.canExecute()).toBe(false);expect(f.session.executionArgs()).toBeUndefined();
 expect(f.session.handle(f.intent(9007199254740993n,'replay')).kind).toBe('refused');expect(f.session.handle(f.intent(9007199254740992n,'older')).kind).toBe('refused');
});

test('all rapid partial drafts stay visible through one resident layout and only newest readiness executes',()=>{
 const f=fixture();f.session.publish(f.source);
 for(let i=0;i<cases.drafts.length;i++){expect(f.session.handle(f.intent(BigInt(i+1),cases.drafts[i])).kind).toBe('staged');expect(f.session.display('input-1','change')).toBe(cases.drafts[i]);expect(f.session.canExecute()).toBe(false);expect(f.session.request()?.token.generation).toBe(String(BigInt(cases.token.generation)+BigInt(i+1)));}
 expect(f.session.publish(f.source)).toBe(false);expect(f.session.display('input-1','change')).toBe(cases.drafts.at(-1));
 const current=f.session.request()!;f.setStatus({...f.status,token:current.token});f.setControl({...f.control,binding:{...f.binding,token:current.token}});f.setView({...f.view,revision:'publication-current'});
 expect(f.session.publish(f.source)).toBe(true);expect(f.session.canExecute()).toBe(true);expect(f.session.executionArgs()).toBe(current.args);
 f.setStatus(f.status);expect(f.session.publish(f.source)).toBe(false);expect(f.session.canExecute()).toBe(false);expect(f.session.display('input-1','change')).toBeUndefined();
});

test('exact authored choice replaces layout and refuses old descendants until fresh publication',()=>{
 const f=fixture();const binding:StagedFormBinding={token:cases.token,layout:'replace',edits:[{source:'choice',kind:'set',path:['choice'],options:[cases.choice]}]};f.setControl({...f.control,input:'choice',binding});f.session.publish(f.source);
 expect(f.session.handle(f.intent(1n,'other')).kind).toBe('refused');expect(f.session.handle(f.intent(2n,cases.choice.key)).kind).toBe('staged');expect(f.session.request()?.args.choice).toEqual(cases.choice.value);expect(f.session.handle(f.intent(3n,cases.choice.key)).kind).toBe('refused');expect(f.session.display('input-1','change')).toBeUndefined();expect(f.session.canExecute()).toBe(false);
});

for(const change of ['disabled','readonly','input-kind','token','compound-preserve','sequence-zero','sequence-negative','sequence-overflow','resident-action','resident-key'] as const)test('resident admission refuses '+change,()=>{
 const f=fixture();let intent=f.intent();let control=f.control;
 if(change==='disabled')control={...control,disabled:true};if(change==='readonly')control={...control,editable:false};if(change==='input-kind')control={...control,input:'choice'};
 if(change==='token')control={...control,binding:{...control.binding,token:{...cases.token,window:'right'}}};
 if(change==='compound-preserve')control={...control,binding:{...control.binding,edits:[...control.binding.edits,{source:'fixed',edit:{kind:'remove',path:['target']}}]}};
 if(change==='sequence-zero')intent={...intent,seq:0n};if(change==='sequence-negative')intent={...intent,seq:-1n};if(change==='sequence-overflow')intent={...intent,seq:1n<<64n};if(change==='resident-action')control={...control,action:{...control.action,scope:'other'}};if(change==='resident-key')control={...control,key:'other'};
 f.setControl(control);f.session.publish(f.source);expect(f.session.handle(intent).kind).toBe('refused');expect(f.session.request()?.args).toEqual(cases.args);
});

test('fixed structural controls admit no raw input and cannot masquerade as text',()=>{
 const f=fixture(),binding:StagedFormBinding={token:cases.token,layout:'replace',edits:[{source:'fixed',edit:{kind:'append',path:['entries'],value:{kind:'nominal',value:'18446744073709551615'}}}]};f.setControl({...f.control,trigger:'activate',input:'none',binding});f.session.publish(f.source);
 expect(f.session.handle({...f.intent(),trigger:'activate',input:'injected'}).kind).toBe('refused');expect(f.session.handle({...f.intent(2n),trigger:'activate',input:null}).kind).toBe('staged');expect(f.session.request()?.args.entries).toEqual([{kind:'nominal',value:'18446744073709551615'}]);
});

test('reset, revision replacement, close and generation overflow cannot resurrect old authority',()=>{
 const reset=fixture();reset.session.publish(reset.source);expect(reset.session.reset({...cases.args,count:'reset'})).toBe(true);expect(reset.session.handle(reset.intent()).kind).toBe('refused');expect(reset.session.publish(reset.source)).toBe(false);expect(reset.session.request()?.args.count).toBe('reset');
 const revision=fixture();revision.session.publish(revision.source);expect(revision.session.revise('another-source')).toBe(true);expect(revision.session.handle(revision.intent()).kind).toBe('refused');expect(revision.session.publish(revision.source)).toBe(false);expect(revision.session.request()?.token.revision).toBe('another-source');
 const closed=fixture();closed.session.publish(closed.source);closed.session.close();closed.session.close();expect(closed.session.handle(closed.intent()).kind).toBe('refused');expect(closed.session.publish(closed.source)).toBe(false);expect(closed.session.reset(cases.args)).toBe(false);expect(closed.session.request()).toBeUndefined();expect(closed.session.canExecute()).toBe(false);
 const full=fixture({...cases.token,generation:'18446744073709551615'});full.session.publish(full.source);expect(full.session.handle(full.intent()).kind).toBe('refused');expect(full.session.reset(cases.args)).toBe(false);expect(full.session.request()?.token.generation).toBe('18446744073709551615');expect(full.session.canExecute()).toBe(true);
});

test('publication replacement retires prior record and two windows have separate draft owners',()=>{
 const left=fixture(),right=fixture({...cases.token,window:'detail-right'});left.session.publish(left.source);right.session.publish(right.source);left.session.handle(left.intent(1n,'left'));
 expect(right.session.request()?.args.count).toBe('0');expect(right.session.canExecute()).toBe(true);left.setView({...left.view,revision:'new-unverified-publication'});expect(left.session.handle(left.intent(2n,'stale')).kind).toBe('refused');expect(left.session.display('input-1','change')).toBeUndefined();expect(left.session.request()?.args.count).toBe('left');
});

test('owner scalars are captured before later caller replacement',()=>{
 const f=fixture(),owner={token:{...cases.token},locale:'en' as 'en'|'de',controllerId:'ifc'},session=new StagedFormSession(owner,cases.args);owner.locale='de';owner.controllerId='other';owner.token.window='other';expect(session.request()?.locale).toBe('en');expect(session.request()?.token.window).toBe('detail-left');session.publish(f.source);expect(session.handle(f.intent()).kind).toBe('staged');
});
