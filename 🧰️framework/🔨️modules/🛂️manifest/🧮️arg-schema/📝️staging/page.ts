/** 📄️ Exact resident page destinations replace layout without editing command arguments. */
import {sameStagedFormToken,type StagedFormPath,type StagedFormToken,type StagedFormStatus} from './🟦️.ts';
import bodyLimits from './body-limits.json';
export const STAGED_FORM_PAGE_ACTION='framework.stageActionFormPage';
export const STAGED_FORM_PAGE_SIZES=Object.freeze({controls:16,choices:32,diagnostics:16});
export const STAGED_FORM_BODY_LIMITS=Object.freeze(bodyLimits);
/** 🧮️ Content and typed page navigation spend independent source-owned body slots. */
export function stagedFormBodyComposition(content:number,pages:number):boolean{return Number.isSafeInteger(content)&&Number.isSafeInteger(pages)&&content>=0&&pages>=0&&content<=STAGED_FORM_BODY_LIMITS.content&&pages<=STAGED_FORM_BODY_LIMITS.pages&&content+pages<=STAGED_FORM_BODY_LIMITS.controls;}
export interface StagedFormPage {readonly fieldOffset:string;readonly diagnosticOffset:string;readonly choice?:{readonly path:StagedFormPath;readonly offset:string}}
export interface StagedFormPageBinding {readonly token:StagedFormToken;readonly page:StagedFormPage}
const data=(value:unknown,required:readonly string[],optional:readonly string[]=[]):Record<string,unknown>|undefined=>{
 if(!value||typeof value!=='object'||![Object.prototype,null].includes(Object.getPrototypeOf(value)))return;
 const keys=Reflect.ownKeys(value);if(keys.length<required.length||keys.length>required.length+optional.length||keys.some(key=>typeof key!=='string'||!required.includes(key)&&!optional.includes(key)))return;
 const result:Record<string,unknown>={};for(const key of keys){const field=Object.getOwnPropertyDescriptor(value,key);if(!field||!field.enumerable||!('value'in field))return;Object.defineProperty(result,key,{value:field.value,enumerable:true});}return required.every(key=>Object.hasOwn(result,key))?result:undefined;
};
const offset=(value:unknown):value is string=>typeof value==='string'&&value.length<=20&&/^(0|[1-9][0-9]*)$/.test(value)&&(value.length<20||value<='18446744073709551615');
/** 🔐️ Admit only closed plain data and canonical full source addresses without invoking accessors. */
export function stagedFormPage(value:unknown):value is StagedFormPage{
 const page=data(value,['fieldOffset','diagnosticOffset'],['choice']);if(!page||!offset(page.fieldOffset)||!offset(page.diagnosticOffset))return false;
 if(!Object.hasOwn(page,'choice'))return true;
 const choice=data(page.choice,['path','offset']);if(!choice||!offset(choice.offset)||!Array.isArray(choice.path)||Object.getPrototypeOf(choice.path)!==Array.prototype||choice.path.length<1||choice.path.length>64||Reflect.ownKeys(choice.path).length!==choice.path.length+1)return false;
 for(let index=0;index<choice.path.length;index++){const field=Object.getOwnPropertyDescriptor(choice.path,String(index));if(!field||!field.enumerable||!('value'in field)||typeof field.value!=='string'||!field.value||['__proto__','prototype','constructor'].includes(field.value))return false;}return true;
}
/** 🎫️ The host first authenticates the exact current resident node, revision, trigger and fixed binding. */
export function applyStagedFormPageBinding(status:StagedFormStatus,current:StagedFormToken,binding:StagedFormPageBinding):StagedFormPage{
 const admitted=data(binding,['token','page']);if(!admitted||!sameStagedFormToken(status.token,current)||!sameStagedFormToken(admitted.token as StagedFormToken,current))throw new Error('staging.stale');
 if(!stagedFormPage(admitted.page))throw new Error('staging.page');
 const page=admitted.page;return {fieldOffset:page.fieldOffset,diagnosticOffset:page.diagnosticOffset,...(page.choice?{choice:{path:[...page.choice.path],offset:page.choice.offset}}:{})};
}
