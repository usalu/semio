/** 📝️ Source-owned field descriptions and generation-bound ephemeral command edits. */
export * from './limits.ts';
export * from './refresh.ts';
export * from './page.ts';
export * from './handoff.ts';
export type StagedValue=null|boolean|number|string|StagedValue[]|{[key:string]:StagedValue};
export type StagedArgs=Readonly<Record<string,StagedValue>>;
export interface StagedFormToken {readonly action:string;readonly generation:string;readonly revision:string;readonly window:string}
export interface StagedFormLabel {readonly en:string;readonly de:string}
export type StagedFormPath=readonly string[];
interface Field {readonly path:StagedFormPath;readonly label:StagedFormLabel}
export interface StagedFormChoice {readonly id:string;readonly label:StagedFormLabel;readonly value:StagedValue}
export type StagedFormField=
 | Field&{readonly kind:'text';readonly required:boolean;readonly format:'text'|'decimal-i64'|'decimal-u64'|'binary64'|'guid';readonly pattern?:string;readonly description?:StagedFormLabel}
 | Field&{readonly kind:'choice';readonly required:boolean;readonly provenance:'declaration'|'source';readonly choices:readonly StagedFormChoice[];readonly resets?:readonly StagedFormPath[];readonly description?:StagedFormLabel}
 | Field&{readonly kind:'constant';readonly value:StagedValue}
 | StagedFormRecord
 | Field&{readonly kind:'array';readonly rows:readonly StagedFormField[];readonly add:readonly StagedFormChoice[]};
export interface StagedFormRecord extends Field {readonly kind:'record';readonly fields:readonly StagedFormField[];readonly create?:StagedValue;readonly optional?:boolean}
export interface StagedFormDiagnostic {readonly path:StagedFormPath;readonly code:string;readonly label:StagedFormLabel}
export interface StagedFormRequest {readonly token:StagedFormToken;readonly args:StagedArgs;readonly locale:'en'|'de';readonly page?:import('./page.ts').StagedFormPage}
export interface StagedFormResponse {readonly token:StagedFormToken;readonly fields:readonly StagedFormField[];readonly diagnostics:readonly StagedFormDiagnostic[];readonly ready:boolean}
export interface StagedActionDefinition {readonly bodyKey:string;readonly limits?:import('./limits.ts').StagedFormLimits}
export interface StagedFormStatus {readonly token:StagedFormToken;readonly ready:boolean;readonly diagnostics:readonly StagedFormDiagnostic[];readonly diagnosticOffset:string;readonly diagnosticTotal:string}
export const STAGED_FORM_BODY_KEY='framework.action.form';
export const STAGED_FORM_STATUS_EXTENSION='framework.stagedFormStatus';
export const STAGED_FORM_EVENT_ACTION='framework.stageActionForm';
/** 🚦️ Publish bounded status pages while retaining the original complete diagnostic address space. */
export function stagedFormStatus(response:StagedFormResponse,offset=0):StagedFormStatus{if(!stagedFormToken(response.token))throw new Error('staging.token');if(!Number.isSafeInteger(offset)||offset<0||offset>response.diagnostics.length)throw new Error('staging.diagnostic-page');return {token:response.token,ready:response.ready,diagnostics:response.diagnostics.slice(offset,offset+16),diagnosticOffset:String(offset),diagnosticTotal:String(response.diagnostics.length)};}
export type StagedFormEdit={readonly kind:'set'|'append';readonly path:StagedFormPath;readonly value:StagedValue}|{readonly kind:'remove';readonly path:StagedFormPath};
export interface StagedFormEvent {readonly token:StagedFormToken;readonly edits:readonly StagedFormEdit[]}
export type StagedFormBindingEdit={readonly source:'fixed';readonly edit:StagedFormEdit}|{readonly source:'text';readonly path:StagedFormPath}|{readonly source:'choice';readonly kind:'set'|'append';readonly path:StagedFormPath;readonly options:readonly {readonly key:string;readonly value:StagedValue}[]};
export interface StagedFormBinding {readonly token:StagedFormToken;readonly layout:'preserve'|'replace';readonly edits:readonly StagedFormBindingEdit[]}
export interface StagedFormControl {readonly signal:AbortSignal;readonly progress?:(completed:number)=>void}
export type StagedFormProducer=(request:StagedFormRequest,control:StagedFormControl)=>Promise<StagedFormResponse>;
const forbidden=new Set(['__proto__','prototype','constructor']);
/** 🎟️ Admit the closed exact token without invoking source accessors or coercing its u64 generation. */
export function stagedFormToken(value:unknown):value is StagedFormToken{
 if(!value||typeof value!=='object'||![Object.prototype,null].includes(Object.getPrototypeOf(value))||Reflect.ownKeys(value).length!==4)return false;
 const token:Record<string,string>={};
 for(const key of ['action','generation','revision','window']){const field=Object.getOwnPropertyDescriptor(value,key);if(!field||!field.enumerable||!('value' in field)||typeof field.value!=='string'||!field.value)return false;token[key]=field.value;}
 const generation=token.generation!;return /^(0|[1-9][0-9]{0,19})$/u.test(generation)&&(generation.length<20||generation<='18446744073709551615');
}
/** 🔑️ Compare all original request authorities without coercing their exact counters. */
export function sameStagedFormToken(a:StagedFormToken,b:StagedFormToken):boolean{return stagedFormToken(a)&&stagedFormToken(b)&&a.action===b.action&&a.generation===b.generation&&a.revision===b.revision&&a.window===b.window;}
function pathKey(path:StagedFormPath):string{return JSON.stringify(path);}
function fieldsAt(fields:readonly StagedFormField[],index=new Map<string,StagedFormField>()):Map<string,StagedFormField>{for(const field of fields){const key=pathKey(field.path);if(index.has(key))throw new Error('staging.duplicate-field');index.set(key,field);if(field.kind==='record')fieldsAt(field.fields,index);if(field.kind==='array')fieldsAt(field.rows,index);}return index;}
function equal(a:StagedValue,b:StagedValue):boolean{if(a===b)return true;if(!a||!b||typeof a!=='object'||typeof b!=='object'||Array.isArray(a)!==Array.isArray(b))return false;const ak=Object.keys(a),bk=Object.keys(b);return ak.length===bk.length&&ak.every(key=>Object.hasOwn(b,key)&&equal((a as Record<string,StagedValue>)[key]!, (b as Record<string,StagedValue>)[key]!));}
function validateText(field:Extract<StagedFormField,{kind:'text'}>,value:StagedValue):void{
 if(typeof value!=='string'||field.required&&value.length===0)throw new Error('staging.value');
 if(field.format==='decimal-i64'||field.format==='decimal-u64'||field.format==='binary64'){if(!/^(0|-?[1-9][0-9]*)$/u.test(value))throw new Error('staging.exact-integer');const integer=BigInt(value),signed=field.format==='decimal-i64';if(integer<(signed?-(1n<<63n):0n)||integer>(signed?(1n<<63n)-1n:(1n<<64n)-1n))throw new Error('staging.exact-integer');}
 if(field.format==='guid'&&!/^[0-3][0-9A-Za-z_$]{21}$/u.test(value))throw new Error('staging.guid');if(field.pattern&&!new RegExp(field.pattern,'u').test(value))throw new Error('staging.pattern');
}
function changed(root:StagedValue,path:StagedFormPath,edit:StagedFormEdit):StagedValue {
 const [key,...rest]=path;if(!key||forbidden.has(key))throw new Error('staging.path');if(!root||typeof root!=='object')throw new Error('staging.parent');
 if(Array.isArray(root)){if(!/^(0|[1-9][0-9]*)$/u.test(key))throw new Error('staging.index');const index=Number(key);if(!Number.isSafeInteger(index)||index>=root.length)throw new Error('staging.index');const next=[...root];if(rest.length)next[index]=changed(root[index]!,rest,edit);else if(edit.kind==='remove')next.splice(index,1);else if(edit.kind==='append'){if(!Array.isArray(root[index]))throw new Error('staging.array');next[index]=[...root[index],structuredClone(edit.value)];}else next[index]=structuredClone(edit.value);return next;}
 const next={...root};if(rest.length)next[key]=changed(root[key]!,rest,edit);else if(edit.kind==='remove')delete next[key];else if(edit.kind==='append'){const old=root[key];if(old!==undefined&&!Array.isArray(old))throw new Error('staging.array');next[key]=[...(old??[]) as StagedValue[],structuredClone(edit.value)];}else next[key]=structuredClone(edit.value);return next;
}
/** 🧷️ Apply an authored resident control binding after the host validates its original UI intent. */
export function applyStagedFormBinding(args:StagedArgs,status:StagedFormStatus,binding:StagedFormBinding,input?:StagedValue):StagedArgs{
 if(!sameStagedFormToken(status.token,binding.token))throw new Error('staging.stale');
 if(binding.edits.length<1||binding.edits.length>8)throw new Error('staging.binding');
 if(binding.layout==='preserve'&&(binding.edits.length!==1||binding.edits[0]!.source!=='text'))throw new Error('staging.layout');
 const inputs=binding.edits.filter(edit=>edit.source!=='fixed');if(inputs.length>1||inputs.length===0&&input!==undefined)throw new Error('staging.input');
 const edits:StagedFormEdit[]=[];
 for(const source of binding.edits){let edit:StagedFormEdit;
  if(source.source==='fixed')edit=source.edit;
  else if(source.source==='text'){if(typeof input!=='string')throw new Error('staging.input');edit={kind:'set',path:source.path,value:input};}
  else {if(typeof input!=='string'||source.options.length<1||source.options.length>32||source.options.some(option=>!option.key)||new Set(source.options.map(option=>option.key)).size!==source.options.length)throw new Error('staging.choice');const option=source.options.find(option=>option.key===input);if(!option)throw new Error('staging.choice');edit={kind:source.kind,path:source.path,value:option.value};}
  if(edit.path.length<1||edit.path.length>64||edit.path.some(part=>!part||forbidden.has(part)))throw new Error('staging.path');edits.push(edit);
 }
 let next:StagedValue=args as Record<string,StagedValue>;for(const edit of edits)next=changed(next,edit.path,edit);return next as Record<string,StagedValue>;
}
/** 🧭️ Read an addressed staged payload without manufacturing missing containers. */
export function stagedFormValue(args:StagedArgs,path:StagedFormPath):StagedValue|undefined{let value:StagedValue|undefined=args as Record<string,StagedValue>;for(const key of path){if(forbidden.has(key)||!value||typeof value!=='object'||!Object.hasOwn(value,key))return undefined;value=(value as Record<string,StagedValue>)[key];}return value;}
/** 🛂️ Identify incomplete text drafts before the current generation can execute. */
export function stagedFormInvalidFields(args:StagedArgs,response:StagedFormResponse):StagedFormPath[]{const invalid:StagedFormPath[]=[];const visit=(fields:readonly StagedFormField[])=>{for(const field of fields){const value=stagedFormValue(args,field.path);if(field.kind==='text'){try{if(value!==undefined||field.required)validateText(field,value??null);}catch{invalid.push(field.path);}}else if(field.kind==='choice'){if((value!==undefined||field.required)&&!field.choices.some(choice=>equal(choice.value,value??null)))invalid.push(field.path);}else if(field.kind==='record'){if(value===undefined){if(!field.optional)invalid.push(field.path);}else visit(field.fields);}else if(field.kind==='array')visit(field.rows);}};visit(response.fields);return invalid;}
/** 📩️ Construct one field edit under the response's exact generation and revision. */
export function stagedFormEvent(response:StagedFormResponse,path:StagedFormPath,value:StagedValue):StagedFormEvent{return {token:response.token,edits:[{kind:'set',path,value}]};}
/** ♻️ Validate every edit before returning one immutable ephemeral argument replacement. */
export function applyStagedFormEvent(args:StagedArgs,response:StagedFormResponse,event:StagedFormEvent):StagedArgs {
 if(!sameStagedFormToken(response.token,event.token))throw new Error('staging.stale');const index=fieldsAt(response.fields);let next:StagedValue=args as Record<string,StagedValue>;
 for(const edit of event.edits){const field=index.get(pathKey(edit.path));if(!field||field.kind==='constant')throw new Error('staging.readonly');
 if(edit.kind==='append'){if(field.kind!=='array'||!field.add.some(choice=>equal(choice.value,edit.value)))throw new Error('staging.choice');}
 else if(edit.kind==='set'){if(field.kind==='text'){if(typeof edit.value!=='string')throw new Error('staging.value');}else if(field.kind==='choice'){if(!field.choices.some(choice=>equal(choice.value,edit.value)))throw new Error('staging.choice');}else if(field.kind!=='record'||field.create===undefined||!equal(field.create,edit.value))throw new Error('staging.record');}
 else {const parent=index.get(pathKey(edit.path.slice(0,-1))),row=parent?.kind==='array'&&parent.rows.some(row=>pathKey(row.path)===pathKey(edit.path));if(!row&&(field.kind==='array'||field.kind==='record'&&!field.optional||'required' in field&&field.required))throw new Error('staging.required');}
 next=changed(next,edit.path,edit);if(edit.kind==='set'&&field.kind==='choice')for(const path of field.resets??[])next=changed(next,path,{kind:'remove',path});}
 return next as Record<string,StagedValue>;
}
/** ⏳️ Replace one source query while retaining cancellation and rejecting late original answers. */
export class StagedFormQuery {
 private controller:AbortController|undefined;
 private current:StagedFormToken|undefined;
 constructor(private readonly produce:StagedFormProducer){}
 cancel():void{this.controller?.abort();this.controller=undefined;this.current=undefined;}
 async request(request:StagedFormRequest,progress?:(completed:number)=>void):Promise<StagedFormResponse|undefined>{this.cancel();const field=Object.getOwnPropertyDescriptor(request,'token');if(!field||!('value' in field)||!stagedFormToken(field.value))throw new Error('staging.token');const controller=new AbortController(),token={...field.value};this.controller=controller;this.current=token;try{const response=await this.produce(request,{signal:controller.signal,progress});if(controller.signal.aborted||this.current!==token||!sameStagedFormToken(response.token,token))return undefined;return response;}catch(error){if(controller.signal.aborted)return undefined;throw error;}}
}




