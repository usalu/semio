import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import schema from './🔣️.json';
import cases from './cases.json';
import bindingCases from './binding-cases.json';
import limitCases from './limits-cases.json';
import requestCases from './request-cases.json';
import wireMapCases from './wire-map-cases.json';
import tokenCases from './token-cases.json';
import pageCases from './page-cases.json';
import bodyLimitsSchema from './body-limits.schema.json';
import bodyCompositionSchema from './body-composition.schema.json';
import bodyCompositionCases from './body-composition-cases.json';
import {stagedFormPage,applyStagedFormPageBinding,STAGED_FORM_PAGE_SIZES,STAGED_FORM_BODY_LIMITS,stagedFormBodyComposition} from './page.ts';
import {stagedFormCensusSteps,stagedFormCensusAsync,STAGED_FORM_LIMITS} from './🟦️.ts';
test('native body content and typed page slots agree with independent schema limits',()=>{const ajv=new Ajv({strict:true}),admit=ajv.compile(bodyCompositionSchema);expect(ajv.validate(bodyLimitsSchema,STAGED_FORM_BODY_LIMITS)).toBe(true);for(const row of bodyCompositionCases.cases){const input={content:row.content,pages:row.pages};expect(admit(input)).toBe(row.allowed);expect(stagedFormBodyComposition(row.content,row.pages)).toBe(row.allowed);}for(const content of [-1,1.5,Number.NaN,Number.MAX_SAFE_INTEGER+1])expect(stagedFormBodyComposition(content,0)).toBe(false);});

test('source-owned page destinations agree with independent closed schema and preserve command arguments',()=>{
 const validate=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Page'});
 expect(STAGED_FORM_PAGE_SIZES as Readonly<Record<string,number>>).toEqual(pageCases.pageSizes);
 for(const page of pageCases.valid){
  expect(validate(page)).toBe(true);expect(stagedFormPage(page)).toBe(true);
  const status={token:pageCases.token,ready:false,diagnostics:[],diagnosticOffset:'0',diagnosticTotal:'0'},args={template:'Pset_ActionRequest',target:'1'};
  const result=applyStagedFormPageBinding(status,pageCases.token,{token:pageCases.token,page});expect(result).toEqual(page);expect(args).toEqual({template:'Pset_ActionRequest',target:'1'});
 }
 for(const page of pageCases.invalid){expect(validate(page)).toBe(false);expect(stagedFormPage(page)).toBe(false);}
 const page=pageCases.valid[0]!;const status={token:pageCases.token,ready:true,diagnostics:[],diagnosticOffset:'0',diagnosticTotal:'0'};
 expect(()=>applyStagedFormPageBinding(status,{...pageCases.token,generation:'2'},{token:pageCases.token,page})).toThrow('staging.stale');
 let reads=0;expect(stagedFormPage(Object.defineProperty({},'fieldOffset',{enumerable:true,get(){reads++;return '0';}}))).toBe(false);expect(reads).toBe(0);
});

test('closed request receiving vectors agree with independent JSON Schema admission',()=>{
 const validate=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Request'});
 for(const value of requestCases.valid)expect(validate(value)).toBe(true);
 for(const value of requestCases.invalid)expect(validate(value)).toBe(false);
});
test('wire map neutral order agrees with independent system byte comparison',()=>{
 for(const row of wireMapCases.cases)expect(row.keys.every((key,index)=>index===0||Buffer.compare(Buffer.from(row.keys[index-1]!),Buffer.from(key))<0)).toBe(row.valid);
});

test('staged census retains original cancellation during long text before wire publication',async()=>{
 const abort=new AbortController();let completed=0;
 await expect(stagedFormCensusAsync({text:'x'.repeat(8192)},{signal:abort.signal,progress:value=>{completed=value;abort.abort();}})).rejects.toThrow('io.cancelled');expect(completed).toBe(1024);
});
test('finite staged reservation census agrees with neutral values and independent JSON UTF8 bounds',()=>{
 const drain=(value:unknown,limits=STAGED_FORM_LIMITS)=>{const steps=stagedFormCensusSteps(value,limits);for(;;){const step=steps.next();if(step.done)return step.value;}};
 for(const row of limitCases.cases){const actual=drain(row.value);expect(actual).toEqual({utf8Bytes:row.reservation,nodes:row.nodes,depth:row.depth});expect(actual.utf8Bytes).toBeGreaterThanOrEqual(new TextEncoder().encode(JSON.stringify(row.value)).length);}
 for(const row of limitCases.refusals)expect(()=>drain(row.value,{...STAGED_FORM_LIMITS,[row.axis]:row.limit})).toThrow('staging.resource-limit');
 let getter=0;expect(()=>drain(Object.defineProperty({},'a',{enumerable:true,get(){getter++;return 1;}}))).toThrow('staging.source-invalid');expect(getter).toBe(0);
 for(const value of [NaN,Infinity,()=>1,Symbol('x'),'\uD800'])expect(()=>drain(value)).toThrow('staging.source-invalid');
 const shared={a:'same'};expect(drain([shared,shared]).nodes).toBe(5);const cycle:unknown[]=[];cycle.push(cycle);expect(()=>drain(cycle)).toThrow('staging.source-invalid');
});
import {applyStagedFormEvent,stagedFormInvalidFields,stagedFormEvent,StagedFormQuery,type StagedFormResponse,type StagedFormRequest} from './🟦️.ts';
const label={en:'Exact value',de:'Exakter Wert'},token=cases.token;
test('token admission keeps exact canonical u64 generations through schema binding and query',async()=>{
 const {stagedFormToken}=await import('./🟦️.ts');const validate=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Token'});
 let calls=0;const query=new StagedFormQuery(async request=>{calls++;return {token:request.token,fields:[],diagnostics:[],ready:false};});
 for(const generation of tokenCases.valid){const candidate={...token,generation};expect(validate(candidate)).toBe(true);expect(stagedFormToken(candidate)).toBe(true);expect((await query.request({token:candidate,args:{},locale:'en'}))?.token).toEqual(candidate);}
 const before=calls;
 for(const generation of tokenCases.invalid){const candidate={...token,generation};expect(validate(candidate)).toBe(false);expect(stagedFormToken(candidate)).toBe(false);expect(()=>applyStagedFormBinding({x:''},{token:candidate,ready:false,diagnostics:[],diagnosticOffset:'0',diagnosticTotal:'0'},{token:candidate,layout:'preserve',edits:[{source:'text',path:['x']}]},'1')).toThrow();await expect(query.request({token:candidate,args:{},locale:'en'})).rejects.toThrow('staging.token');}
 expect(calls).toBe(before);let invoked=0;expect(stagedFormToken(Object.defineProperty({...token},'generation',{get(){invoked++;return '1';},enumerable:true}))).toBe(false);expect(invoked).toBe(0);query.cancel();
});
test('query delegates original receiving without invoking argument getters first',async()=>{
 let invoked=0,received:StagedFormRequest|undefined;const source=Object.defineProperty({token,locale:'en'},'args',{enumerable:true,get(){invoked++;throw new Error('getter invoked');}}) as StagedFormRequest;
 const query=new StagedFormQuery(async request=>{received=request;return {token:request.token,ready:false,fields:[],diagnostics:[]};});
 expect((await query.request(source))?.ready).toBe(false);expect(received).toBe(source);expect(invoked).toBe(0);query.cancel();
});
const response:StagedFormResponse={token,ready:true,diagnostics:[],fields:[{kind:'text',path:cases.cases[0]!.path,label,required:true,format:'binary64'},{kind:'text',path:['count'],label,required:true,format:'decimal-i64'},{kind:'constant',path:['entries','0','kind'],label,value:'A'},{kind:'choice',path:['choice'],label,required:true,provenance:'source',choices:[{id:'label',label,value:{kind:'typedValue',value:{name:'IfcLabel',items:[{kind:'string',value:'Red'}]}}}]},{kind:'choice',path:['template'],label,required:true,provenance:'declaration',choices:[{id:'B',label,value:'B'}],resets:[['entries'],['identities'],['performance']]}]};
test('staging schemas admit exact neutral requests, fields and events and reject open payloads',()=>{const ajv=new Ajv({strict:true,allErrors:true});const validate=ajv.compile(schema);expect(validate({token,args:cases.args,locale:'en'})).toBe(true);expect(validate(response)).toBe(true);expect(validate(stagedFormEvent(response,cases.cases[0]!.path,cases.cases[0]!.value))).toBe(true);expect(validate({...response,admission:true})).toBe(false);expect(validate({...response,fields:[{...response.fields[0],path:['__proto__']}]})).toBe(false);});
for(const row of cases.cases)test('staging preserves exact siblings '+row.id,()=>{const before=structuredClone(cases.args),next=applyStagedFormEvent(cases.args,response,stagedFormEvent(response,row.path,row.value));expect(next).not.toBe(cases.args);expect(cases.args).toEqual(before);expect(next.target).toBe(cases.args.target);expect(next.identities).toEqual(cases.args.identities);});
for(const stale of cases.stale)test('stale event rejects '+JSON.stringify(stale),()=>expect(()=>applyStagedFormEvent(cases.args,response,{token:stale,edits:[{kind:'set',path:['count'],value:'1'}]})).toThrow('staging.stale'));
test('readonly discriminants and arbitrary choices refuse while exact text drafts remain incomplete',()=>{for(const [path,value] of [[['entries','0','kind'],'B'],[['choice'],'Red']] as const)expect(()=>applyStagedFormEvent(cases.args,response,stagedFormEvent(response,[...path],value))).toThrow();for(const value of ['-','01','9223372036854775808']){const draft=applyStagedFormEvent(cases.args,response,stagedFormEvent(response,['count'],value));expect(draft.count).toBe(value);expect(stagedFormInvalidFields(draft,response)).toContainEqual(['count']);}expect(cases.args.entries[0]!.kind).toBe('A');});
test('typed source choice stages its full payload and declaration change resets dependent arguments',()=>{const choice=response.fields[3];if(choice?.kind!=='choice')throw new Error('fixture');const selected=applyStagedFormEvent(cases.args,response,stagedFormEvent(response,['choice'],choice.choices[0]!.value));expect(selected.choice).toEqual(choice.choices[0]!.value);const reset=applyStagedFormEvent({...selected,performance:{phase:'old'}},response,stagedFormEvent(response,['template'],'B'));expect(reset.template).toBe('B');expect(Object.hasOwn(reset,'entries')).toBe(false);expect(Object.hasOwn(reset,'identities')).toBe(false);expect(Object.hasOwn(reset,'performance')).toBe(false);});
test('source query cancels its original request and refuses obsolete responses',async()=>{const pending:{request:StagedFormRequest;signal:AbortSignal;resolve:(value:StagedFormResponse)=>void}[]=[];const query=new StagedFormQuery((request,control)=>new Promise(resolve=>pending.push({request,signal:control.signal,resolve})));const first=query.request({token,args:cases.args,locale:'en'});const nextToken={...token,generation:'8'};const second=query.request({token:nextToken,args:cases.args,locale:'de'});expect(pending[0]!.signal.aborted).toBe(true);pending[0]!.resolve(response);expect(await first).toBeUndefined();pending[1]!.resolve({...response,token:nextToken});expect((await second)?.token).toEqual(nextToken);query.cancel();});

test('neutral nested exact edits agree with an independent JSON Patch implementation',async()=>{const library=await import('fast-json-patch');const oracle={apply(args:unknown,path:readonly string[],value:unknown){const pointer='/'+path.map(item=>item.replaceAll('~','~0').replaceAll('/','~1')).join('/');return library.applyPatch(structuredClone(args) as object,[{op:'add',path:pointer,value}],true,false,true).newDocument;}};for(const row of cases.cases){const next=applyStagedFormEvent(cases.args,response,stagedFormEvent(response,row.path,row.value));expect(oracle.apply(cases.args,row.path,row.value)).toEqual(next);}});
import {stagedFormStatus} from './🟦️.ts';
import {applyStagedFormBinding,type StagedFormBinding} from './🟦️.ts';
test('neutral resident binding outputs agree with the independent JSON Patch interface',async()=>{
 const library=await import('fast-json-patch');
 for(const row of bindingCases.cases){const binding={token,layout:'replace',edits:[row.edit]} as StagedFormBinding,input='input' in row?row.input:undefined,next=applyStagedFormBinding(cases.args,stagedFormStatus(response),binding,input),patch=row.kind==='remove'?{op:'remove' as const,path:'/'+row.path.join('/')}:{op:'add' as const,path:'/'+row.path.join('/'),value:row.value};expect(next).toEqual(library.applyPatch(structuredClone(cases.args),[patch],true,false,true).newDocument);}
 for(const row of bindingCases.invalid)expect(()=>applyStagedFormBinding(cases.args,stagedFormStatus(response),{token,layout:'replace',edits:[row.edit]} as StagedFormBinding,row.input)).toThrow();
});

test('resident bindings accept partial text and resolve exact authored choices without event paths',()=>{
 const status=stagedFormStatus(response),text:StagedFormBinding={token,layout:'preserve',edits:[{source:'text',path:['count']}]};
 expect(applyStagedFormBinding(cases.args,status,text,'-').count).toBe('-');
 expect(()=>applyStagedFormBinding(cases.args,status,text,{path:['target'],value:'1'})).toThrow();
 const selected={kind:'typedValue',value:{name:'IFCINTEGER',items:[{kind:'integer',value:'9007199254740993'}]}};
 const choice:StagedFormBinding={token,layout:'replace',edits:[{source:'choice',kind:'set',path:['choice'],options:[{key:'0',value:selected}]},{source:'fixed',edit:{kind:'remove',path:['identities']}}]};
 const next=applyStagedFormBinding(cases.args,status,choice,'0');expect(next.choice).toEqual(selected);expect(Object.hasOwn(next,'identities')).toBe(false);expect(next.target).toBe(cases.args.target);
 expect(()=>applyStagedFormBinding(cases.args,status,{...choice,layout:'preserve'},'0')).toThrow('staging.layout');
 expect(()=>applyStagedFormBinding(cases.args,status,choice,'other')).toThrow();
 expect(()=>applyStagedFormBinding(cases.args,{...status,token:{...token,revision:'other'}},choice,'0')).toThrow('staging.stale');
 const schemaRoot={$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Binding'};const ajv=new Ajv({strict:true});expect(ajv.validate(schemaRoot,choice)).toBe(true);expect(ajv.validate(schemaRoot,{...choice,paths:[['target']]})).toBe(false);
});

test('status pages expose original token and complete diagnostic addressing',()=>{
 const response={token:{action:'a',window:'w',generation:'1',revision:'source'},fields:[],ready:false,diagnostics:Array.from({length:19},(_,i)=>({path:['target'],code:String(i),label:{en:'Invalid',de:'Ungültig'}}))} as const;
 const status=stagedFormStatus(response,16);
 expect(status).toEqual({token:response.token,ready:false,diagnostics:response.diagnostics.slice(16),diagnosticOffset:'16',diagnosticTotal:'19'});
 const statusSchema={$schema:schema.$schema,$defs:schema.$defs,$ref:'#/$defs/Status'};const ajv=new Ajv({strict:false});expect(ajv.validate(statusSchema,status)).toBe(true);
 expect(ajv.validate(statusSchema,{...status,fields:[]})).toBe(false);
 expect(()=>stagedFormStatus(response,-1)).toThrow();
});
