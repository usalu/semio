/** 📨️ Required typed values are caller-authored under the original source and declaration. */
import type {StagedFormRequest,StagedFormResponse,StagedFormBinding,StagedFormControl,StagedFormField,StagedFormDiagnostic,StagedFormPageBinding,StagedValue} from '@semio-tech/framework/manifest/staging';
import {stagedFormToken,stagedFormPage,STAGED_FORM_BODY_LIMITS,STAGED_FORM_PAGE_SIZES} from '@semio-tech/framework/manifest/staging';
import {ifc4x3Catalog as catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {copyIfcOwnedSteps,captureIfcGraphSteps,runIfcEditAsync,type IfcEditSteps,type IfcApplyOptions} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts';
import {checkIfcValueDomainSteps} from '../📚️catalog/⚖️domain/🟦️.ts';
import {ifcSubtypeSteps} from '../📚️catalog/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcTypeExpression} from '../📚️catalog/🟦️.ts';
import type {IfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import {compileIfc4x3Express} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/compiler.ts';
import {IfcExpressContext} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/population.ts';
import type {ExpressValue} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/evaluator.ts';
const tick=()=>({phase:'admitInput'as const,completed:0,total:0}),label=(en:string,de:string)=>({en,de});
const collectionPageSize=Math.floor((STAGED_FORM_BODY_LIMITS.content-2)/2);
const choicePageSize=STAGED_FORM_PAGE_SIZES.choices-1;
const object=(value:StagedValue|undefined):Record<string,StagedValue>|undefined=>value!==null&&typeof value==='object'&&!Array.isArray(value)?value:undefined;
function fail(code:string):never{throw Error(code);}
function options(control:StagedFormControl):IfcApplyOptions{const signal=control.signal,progress=control.progress;let completed=0;return{signal,onProgress:()=>progress?.(++completed),yieldEvery:128};}
function decimal(value:StagedValue|undefined,positive:boolean):bigint{if(typeof value!=='string'||!(positive?/^[1-9][0-9]{0,19}$/u:/^(0|[1-9][0-9]{0,19})$/u).test(value))return fail('type.input-decimal');const result=BigInt(value);if(result>18446744073709551615n)return fail('type.input-range');return result;}
type InputKind='reference'|'string'|'integer'|'enum'|'real'|'number'|'binary'|'aggregate'|'select'|'set'|'array'|'list';
interface InputMember {name:string;kind:Exclude<InputKind,'aggregate'|'select'>;members?:readonly string[]}
interface InputDomain {kind:InputKind;members?:readonly string[];upper?:number;select?:readonly InputMember[]}
function* selectLeaves(name:string,ancestors:readonly string[]=[],seen=new Set<string>()):IfcEditSteps<InputMember[]>{
 if(ancestors.length>=64||ancestors.includes(name))fail('type.input-depth');const declaration=catalog.types[name];if(declaration?.kind!=='select'||!declaration.members.length)fail('type.input-control');const leaves:InputMember[]=[];
 for(const member of declaration.members){yield tick();if(catalog.types[member]?.kind==='select'){const nested=yield* selectLeaves(member,[...ancestors,name],seen);for(const leaf of nested){yield tick();leaves.push(leaf);}}else{if(seen.has(member))fail('type.input-member');seen.add(member);const domain=yield* inputKind({kind:'named',name:member},false);if(domain.kind==='aggregate'||domain.kind==='select')fail('type.input-control');leaves.push({name:member,kind:domain.kind,...(domain.members?{members:domain.members}:{})});}}
 return leaves;
}
function* inputKind(expression:IfcTypeExpression,container=true):IfcEditSteps<InputDomain>{
 let current=expression;for(let depth=0;depth<64;depth++){
  yield tick();if(current.kind==='aggregate'){
   if(!container&&current.aggregate==='ARRAY'&&Number.isSafeInteger(current.lower)&&Number.isSafeInteger(current.upper)&&current.upper!>=current.lower!&&Number.isSafeInteger(current.upper!-current.lower!+1)&&!current.optional&&current.item){const item=yield* inputKind(current.item,false);if(item.kind!=='real')fail('type.input-control');return{kind:'array'};}
   if(!container&&current.aggregate==='SET'&&current.unique&&Number.isInteger(current.lower)&&current.lower!>=0&&(current.upper===null||Number.isInteger(current.upper)&&current.upper!>=current.lower!)&&current.item){const item=yield* inputKind(current.item,false);if(item.kind!=='reference')fail('type.input-control');return{kind:'set'};}
   if(current.aggregate!=='LIST'||!Number.isInteger(current.lower)||!Number.isInteger(current.upper)||current.lower!<0||current.upper!<current.lower!||current.upper!>=STAGED_FORM_BODY_LIMITS.content||!current.item)fail('type.input-control');const item=yield* inputKind(current.item,false);if(!container){if(item.kind!=='integer')fail('type.input-control');return{kind:'list'};}if(item.kind!=='real')fail('type.input-control');return{kind:'aggregate',upper:current.upper!};
  }
  if(current.kind==='named'&&catalog.entities[current.name??''])return{kind:'reference'};
  if(current.kind==='named'&&['STRING','INTEGER','REAL','NUMBER','BINARY'].includes(current.name??''))return{kind:current.name!.toLowerCase()as InputKind};
  if(current.kind==='named'&&['BOOLEAN','LOGICAL'].includes(current.name??''))return{kind:'enum',members:current.name==='BOOLEAN'?['T','F']:['T','F','U']};
  const declaration=current.kind==='named'?catalog.types[current.name??'']:undefined;
  if(declaration?.kind==='select'){
   if(!container)fail('type.input-control');const select=yield* selectLeaves(current.name!);let references=0;for(const member of select){yield tick();references+=Number(member.kind==='reference');}if(references===select.length)return{kind:'reference'};if(references>1)fail('type.input-control');return{kind:'select',select};
  }
  if(declaration?.kind==='enum')return{kind:'enum',members:declaration.members};if(declaration?.kind!=='defined'||!declaration.typeExpr)fail('type.input-control');current=declaration.typeExpr;
 }
 return fail('type.input-depth');
}
interface Required extends InputDomain{index:number;name:string}

function integer(value:StagedValue|undefined):bigint{if(typeof value!=='string'||value.length>20||!/^(0|-?[1-9][0-9]*)$/u.test(value))return fail('type.input-integer');const result=BigInt(value);if(result< -9223372036854775808n||result>9223372036854775807n)return fail('type.input-range');return result;}
function real(value:StagedValue|undefined):IfcValue{const words=object(value);if(!words||Object.keys(words).length!==1||!Object.hasOwn(words,'bits'))fail('type.input-bits');const bits=decimal(words.bits,false);if((bits>>52n&2047n)===2047n)fail('type.input-nonfinite');return{kind:'real',value:{bits}};}
function aggregateItems(value:StagedValue|undefined):StagedValue[]{const aggregate=object(value);if(!aggregate||Object.keys(aggregate).length!==2||aggregate.kind!=='aggregate'||!Array.isArray(aggregate.value))fail('type.input-row');return aggregate.value;}
function payloadValue(value:StagedValue|undefined,member=false):Record<string,StagedValue>{const row=object(value);if(!row||Object.keys(row).length!==2||!Object.hasOwn(row,'value')||typeof row.kind!=='string'||!(member?['real']:['reference','string','integer','enum','real','binary','aggregate','typedValue']).includes(row.kind))fail('type.input-row');if(row.kind==='typedValue'){const typed=object(row.value);if(!typed||Object.keys(typed).length!==2||typeof typed.name!=='string'||!Array.isArray(typed.items)||typed.items.length>1)fail('type.input-row');for(const item of typed.items){if(object(item)?.kind==='aggregate')aggregateItems(item);else{if(!['string','integer','real','enum','binary'].includes(String(object(item)?.kind)))fail('type.input-row');payloadValue(item);}}}else if(row.kind==='aggregate'){if(!Array.isArray(row.value)||row.value.length>=STAGED_FORM_BODY_LIMITS.content)fail('type.input-row');for(const item of row.value)payloadValue(item,true);}else if(row.kind==='real'){const bits=object(row.value);if(!bits||Object.keys(bits).length!==1||typeof bits.bits!=='string')fail('type.input-row');}else if(typeof row.value!=='string')fail('type.input-row');return row;}
function* aggregate(value:StagedValue|undefined):IfcEditSteps<IfcValue>{if(!Array.isArray(value))fail('type.input-row');const items:IfcValue[]=[];for(const item of value){yield tick();items.push(real(payloadValue(item,true).value));}return{kind:'aggregate',value:items};}
function* binary(value:StagedValue|undefined):IfcEditSteps<IfcValue>{if(typeof value!=='string'||value.length===0)fail('type.input-binary');for(let offset=0;offset<value.length;offset+=64){yield tick();if(!/^[0-9A-F]+$/u.test(value.slice(offset,offset+64)))fail('type.input-binary');}const unused=value.charCodeAt(0)-48;if(unused<0||unused>3||value.length===1&&value!=='0'||value.length>1&&parseInt(value[1]!,16)>=2**(4-unused))fail('type.input-binary');return{kind:'binary',value};}
interface Prepared {source:IfcSnapshot;request:StagedFormRequest;position:number;name:string;map:(number|null)[];args:IfcValue[];missing:Required[];required:Required[];invalid:boolean}
function* prepare(source:IfcSnapshot,revision:string,input:StagedFormRequest,opts:IfcApplyOptions,draft:boolean):IfcEditSteps<Prepared>{
 const request=yield* copyIfcOwnedSteps(input,opts,false,{plainDataOnly:true,maxProperties:65536,maxContainerDepth:16});if(!stagedFormToken(request.token)||request.token.revision!==revision)fail('staging.stale');if(request.token.action!=='setSnapshotValue')fail('staging.action');if(request.page!==undefined&&(!stagedFormPage(request.page)||request.page.fieldOffset!=='0'||request.page.diagnosticOffset!=='0'))fail('type.input-page');
 const sourceCopy=yield* captureIfcGraphSteps(source,'validateBase',opts),path=request.args.path,value=object(request.args.value);if(typeof path!=='string'||!/^\/entities\/(0|[1-9][0-9]*)$/u.test(path)||!value||Object.keys(value).some(key=>!['$ifcType','$ifcInputs'].includes(key))||typeof value.$ifcType!=='string'||value.$ifcType.length>256||!/^[_A-Za-z][_A-Za-z0-9]*$/u.test(value.$ifcType))fail('type.input-request');
 const position=Number(path.slice(10)),entity=sourceCopy.entities[position];if(!Number.isSafeInteger(position)||!entity)fail('type.input-source');let count=0;for(const candidate of sourceCopy.entities){yield tick();if(candidate.id===entity.id)count++;}if(count!==1)fail('type.input-identity');
 const name=value.$ifcType.toUpperCase(),old=catalog.entities[entity.name.toUpperCase()],next=catalog.entities[name];if(!old||!next||old.attributes.length!==entity.args.length||old.attributes.length===0||next.attributes.length===0)fail('type.input-shape');const repair=name===entity.name.toUpperCase()&&old.attributes.length===next.attributes.length;if(repair?!entity.complex?.length&&(old.supertype!==null||old.allAttributes.length!==old.attributes.length):!entity.complex?.length||old.attributes.length===next.attributes.length)fail('type.input-shape');
 const rows=value.$ifcInputs===undefined?[]:value.$ifcInputs;if(!Array.isArray(rows)||rows.length>next.attributes.length)fail('type.input-rows');const supplied=new Map<number,Record<string,StagedValue>>();let previous=-1;
 for(const item of rows){yield tick();const row=object(item);if(!row||Object.keys(row).length!==2||!Object.hasOwn(row,'index')||!Object.hasOwn(row,'value'))fail('type.input-row');const ordinal=decimal(row.index,false);if(ordinal>=BigInt(next.attributes.length))fail('type.input-index');const index=Number(ordinal),value=payloadValue(row.value);if(index<=previous)fail('type.input-row');previous=index;supplied.set(index,value);}
 const args:IfcValue[]=[],map:(number|null)[]=[],missing:Prepared['missing']=[],required:Prepared['required']=[];let invalid=false;
 for(const [index,attribute]of next.attributes.entries()){
  yield tick();if(attribute.derived)fail('type.input-derived');let matched=-1;
  for(const [ordinal,before]of old.attributes.entries()){yield tick();const current=entity.args[ordinal]!;if(before.name===attribute.name&&JSON.stringify(before.typeExpr)===JSON.stringify(attribute.typeExpr)&&current.kind!=='derived'&&(attribute.optional||current.kind!=='unset')){matched=ordinal;break;}}
  const payload=supplied.get(index);if(matched>=0){if(payload)fail('type.input-preserved-slot');map.push(matched);args.push(entity.args[matched]!);continue;}
  map.push(null);if(attribute.optional){if(payload)fail('type.input-optional-slot');args.push({kind:'unset'});continue;}
  const domain=yield* inputKind(attribute.typeExpr),{kind}=domain,field={index,name:attribute.name,...domain};required.push(field);if(payload&&(kind==='number'?!['integer','real'].includes(String(payload.kind)):kind==='select'?payload.kind!=='typedValue'&&!(payload.kind==='reference'&&domain.select!.some(member=>member.kind==='reference')):payload.kind!==kind))fail('type.input-kind');
  if(!payload){missing.push(field);args.push({kind:'unset'});continue;}
  if(kind==='select'&&payload.kind==='typedValue'){
   const typed=object(payload.value)!,items=typed.items as StagedValue[],member=domain.select!.find(member=>member.name===typed.name),actual=object(items[0])?.kind;if(!member||member.kind==='reference'||items.length&&(member.kind==='number'?!['integer','real'].includes(String(actual)):actual!==(['set','array','list'].includes(member.kind)?'aggregate':member.kind)))fail('type.input-member');if(items.length===0){missing.push(field);args.push({kind:'unset'});continue;}
   if(member.kind==='set'||member.kind==='array'||member.kind==='list'){const members=aggregateItems(items[0]),values:IfcValue[]=[],mode=member.kind==='set'?'reference':member.kind==='list'?'integer':'real';let valid=true;for(const item of members){yield tick();const child=payloadValue(item);if(child.kind!==mode)fail('type.input-member');try{values.push(mode==='reference'?{kind:'reference',value:decimal(child.value,true)}:mode==='integer'?{kind:'integer',value:integer(child.value)}:real(child.value));}catch(error){if(!draft)throw error;valid=false;}}if(valid)args.push({kind:'typedValue',value:{name:member.name,items:[{kind:'aggregate',value:values}]}});else{invalid=true;missing.push(field);args.push({kind:'unset'});}continue;}
   try{const child=object(items[0])!,value=child.kind==='real'?real(child.value):child.kind==='integer'?{kind:'integer'as const,value:integer(child.value)}:child.kind==='binary'?yield* binary(child.value):{kind:child.kind as'string'|'enum',value:child.value as string};args.push({kind:'typedValue',value:{name:member.name,items:[value]}});}catch(error){if(!draft)throw error;invalid=true;missing.push(field);args.push({kind:'unset'});}continue;
  }
  try{if(kind==='enum'){let accepted=false;for(const member of domain.members!){yield tick();accepted ||= member===payload.value;}if(!accepted)fail('type.input-member');}args.push(payload.kind==='aggregate'?yield* aggregate(payload.value):payload.kind==='real'?real(payload.value):payload.kind==='binary'?yield* binary(payload.value):payload.kind==='reference'?{kind:'reference',value:decimal(payload.value,true)}:payload.kind==='integer'?{kind:'integer',value:integer(payload.value)}:{kind:payload.kind as'string'|'enum',value:payload.value as string});}catch(error){if(!draft)throw error;invalid=true;missing.push(field);args.push({kind:'unset'});}
 }
 if(repair&&required.length===0)fail('type.input-no-change');
 if(missing.length===0){for(const [index,attribute]of next.attributes.entries()){try{for(const _ of checkIfcValueDomainSteps(catalog,attribute.typeExpr,args[index]!,{optional:attribute.optional,reference:function*(id,expected){let count=0,accepted=false;for(const candidate of sourceCopy.entities){yield;if(candidate.id!==id)continue;count++;accepted ||= yield* ifcSubtypeSteps(catalog,candidate===entity?name:candidate.name,expected);for(const part of candidate.complex??[]){yield;accepted ||= yield* ifcSubtypeSteps(catalog,part.name,expected);}}return count===1&&accepted;}}))yield tick();}catch(error){if(!draft)throw error;invalid=true;}}}
 else if(!draft)fail('type.input-required');
 return{source:sourceCopy,request,position,name,map,args,missing,required,invalid};
}
async function localRules(prepared:Prepared,opts:IfcApplyOptions,draft:boolean):Promise<void>{
 if(prepared.missing.length||prepared.invalid||prepared.name!==prepared.source.entities[prepared.position]!.name.toUpperCase())return;
 const original=prepared.source.entities[prepared.position]!;let callerFailure:unknown,callerFailed=false;
 const control={signal:opts.signal,progress:()=>{try{opts.onProgress?.(tick());}catch(error){callerFailed=true;callerFailure=error;throw error;}}};
 try{
  const issued=await compileIfc4x3Express(control);if(!issued.complete)fail('type.input-rule-incomplete');
  prepared.source.entities[prepared.position]={...original,name:prepared.name,args:prepared.args};
  const context=new IfcExpressContext(prepared.source,issued.program,control);
  for(const [index,attribute]of catalog.entities[prepared.name]!.allAttributes.entries()){
   const current=await context.sourceAttribute(prepared.position,index);if(attribute.derived){if(current.kind!=='derived')fail('type.input-derived');continue;}
   for(const _ of checkIfcValueDomainSteps(catalog,attribute.typeExpr,current,{optional:attribute.optional,reference:function*(id,expected){let count=0,accepted=false;for(const candidate of prepared.source.entities){yield;if(candidate.id!==id)continue;count++;accepted ||= yield* ifcSubtypeSteps(catalog,candidate.name,expected);for(const part of candidate.complex??[]){yield;accepted ||= yield* ifcSubtypeSteps(catalog,part.name,expected);}}return count===1&&accepted;}}))await context.evaluator.tick();
  }
  await context.initialize();
  const entity=context.entities.get(original.id)!;
  const run=async(owner:string,self:ExpressValue)=>{for(const rule of issued.program.rules){await context.evaluator.tick();if(rule.owner!==owner)continue;const value=await context.evaluator.evaluate(rule.expression,context.scope(self));if(value.kind!=='logical'||value.value!=='true')fail(value.value==='false'?'type.input-rule-false':'type.input-rule-incomplete');}};
  const visit=async(value:ExpressValue,type:IfcTypeExpression,optional:boolean,depth=0):Promise<void>=>{
   await context.evaluator.tick();if(depth>=64)fail('type.input-depth');
   if(value.kind==='unknown'){if(!optional)fail('type.input-rule-incomplete');return;}
   if(type.kind==='aggregate'){if(value.kind!=='aggregate'||!type.item)fail('type.input-rule-incomplete');for(const item of value.value as ExpressValue[])await visit(item,type.item,false,depth+1);return;}
   let name=type.name;if(!name||catalog.entities[name])return;let declaration=catalog.types[name];
   if(declaration?.kind==='select'){name=value.types?.[0];if(!name)fail('type.input-rule-incomplete');declaration=catalog.types[name];}
   await run(name,value);if(declaration?.typeExpr)await visit(value,declaration.typeExpr,optional,depth+1);
  };
  for(const owner of entity.types??[])await run(owner,entity);
  for(const attribute of catalog.entities[prepared.name]!.allAttributes){await context.evaluator.tick();if(!attribute.derived)await visit(entity.fields!.get(attribute.name.toUpperCase())??{kind:'unknown'},attribute.typeExpr,attribute.optional);}
  await context.evaluator.tick();
 }catch(error){if(callerFailed)throw callerFailure;if(opts.signal?.aborted)throw error;if(!draft)throw error;prepared.invalid=true;}
 finally{prepared.source.entities[prepared.position]=original;}
}
/** 🧭️ All required typed fields retain their original target order and caller payloads. */
export async function typeInputForm(source:IfcSnapshot,revision:string,request:StagedFormRequest,control:StagedFormControl):Promise<{response:StagedFormResponse;bindings:StagedFormBinding[];pageBindings:StagedFormPageBinding[]}>{
 const opts=options(control),prepared=await runIfcEditAsync(prepare(source,revision,request,opts,true),opts);await localRules(prepared,opts,true);return runIfcEditAsync((function*():IfcEditSteps<{response:StagedFormResponse;bindings:StagedFormBinding[];pageBindings:StagedFormPageBinding[]}>{
 const fields:StagedFormField[]=[],diagnostics:StagedFormDiagnostic[]=[],response:StagedFormResponse={token:prepared.request.token,fields,diagnostics,ready:prepared.missing.length===0&&!prepared.invalid},bindings:StagedFormBinding[]=[],pageBindings:StagedFormPageBinding[]=[];
 if(prepared.required.length>16)fail('type.input-content');
 const marker=object(prepared.request.args.value)!,rows=marker.$ifcInputs as StagedValue[]|undefined,coordinates:{path:string[];offset:string}[]=[];
 const address=(index:number)=>['entities',String(prepared.position),'args',String(index)];
 for(const field of prepared.required){
  yield tick();if(field.members&&field.members.length>32)for(let offset=0;offset<field.members.length;offset+=31){yield tick();coordinates.push({path:address(field.index),offset:String(offset)});}
  if(field.kind==='select'){const paged=field.select!.length>STAGED_FORM_PAGE_SIZES.choices;if(paged)for(let offset=0;offset<field.select!.length;offset+=choicePageSize){yield tick();coordinates.push({path:address(field.index),offset:String(offset)});}const row=rows?.find(row=>object(row)?.index===String(field.index)),typed=object(object(object(row)?.value)?.value);if(field.select!.some(member=>member.name===typed?.name&&['set','array','list'].includes(member.kind))){const items=typed!.items as StagedValue[],count=items.length?aggregateItems(items[0]).length:0;for(let offset=0;offset<Math.max(count,1);offset+=collectionPageSize){yield tick();if(offset===0&&paged)continue;coordinates.push({path:address(field.index),offset:String(offset)});}}}
 }
 const requested=prepared.request.page?.choice;let active=0;
 if(requested){active=coordinates.findIndex(page=>page.offset===requested.offset&&JSON.stringify(page.path)===JSON.stringify(requested.path));if(active<0)fail('type.input-page');}
 const current=coordinates[active];for(const index of [active-1,active+1]){yield tick();const choice=coordinates[index];if(choice)pageBindings.push({token:prepared.request.token,page:{fieldOffset:'0',diagnosticOffset:'0',choice}});}
 for(const field of prepared.required){
  yield tick();const found=rows?.findIndex(row=>object(row)?.index===String(field.index))??-1,insertion=rows?.findIndex(row=>Number(object(row)?.index)>field.index)??-1,row=found<0?(insertion<0?(rows?.length??0):insertion):found,payloadPath=['value','$ifcInputs',String(row),'value'],payload=found<0?undefined:object(object(rows![found])?.value),kind=field.kind==='number'?payload?.kind:field.kind,path=[...payloadPath,'value',...(kind==='real'?['bits']:[])],draft={index:String(field.index),value:{kind:field.kind==='number'?'integer':field.kind,value:field.kind==='real'?{bits:''}:field.kind==='aggregate'?[]:''}},fixed:StagedFormBinding['edits']=found<0?[{source:'fixed',edit:{kind:'set',path:['value','$ifcInputs'],value:[...(rows??[]).slice(0,row),draft,...(rows??[]).slice(row)]}}]:[];
  if(field.kind==='select'){
   const typed=payload?.kind==='typedValue'?object(payload.value):undefined,items=typed?.items as StagedValue[]|undefined,selected=field.select!.find(member=>payload?.kind==='reference'?member.kind==='reference':member.name===typed?.name),choices=[],offset=current&&JSON.stringify(current.path)===JSON.stringify(address(field.index))?Number(current.offset):0,start=offset%choicePageSize===0&&offset<field.select!.length?offset:0,members=field.select!.length>STAGED_FORM_PAGE_SIZES.choices?field.select!.slice(start,start+choicePageSize):[...field.select!];
   if(selected&&!members.some(member=>member.name===selected.name))members.push(selected);
   for(const member of members){yield tick();const actual=object(items?.[0])?.kind,compatible=selected&&(['set','array','list'].includes(member.kind)?selected.kind===member.kind:member.kind==='number'?actual==='integer'||actual==='real':actual===member.kind);choices.push({id:member.name,label:label(member.name,member.name),value:member.kind==='reference'?{kind:'reference',value:payload?.kind==='reference'?payload.value:''}:{kind:'typedValue',value:{name:member.name,items:compatible?(items??[]):[]}}});}
   fields.push({kind:'choice',path:payloadPath,label:label(field.name+' member',field.name+' Auswahltyp'),required:true,provenance:'declaration',choices});bindings.push({token:prepared.request.token,layout:'replace',edits:[{source:'choice',kind:'set',path:found<0?['value','$ifcInputs']:payloadPath,options:choices.map(choice=>({key:choice.id,value:found<0?[...(rows??[]).slice(0,row),{index:String(field.index),value:choice.value},...(rows??[]).slice(row)]:choice.value}))}]});
   if(selected?.kind==='set'||selected?.kind==='array'||selected?.kind==='list'){
    const container=items!.length>0,path=[...payloadPath,'value','items',...(container?['0','value']:[])],members=container?aggregateItems(items![0]):[],requestedOffset=current&&JSON.stringify(current.path)===JSON.stringify(address(field.index))?Number(current.offset):0,offset=requestedOffset%collectionPageSize===0&&requestedOffset<members.length?requestedOffset:0,children:StagedFormField[]=[],integer=selected.kind==='list',real=selected.kind==='array',name=real?'ARRAY':integer?'LIST':'SET';
    for(let index=offset;index<Math.min(members.length,offset+collectionPageSize);index++){yield tick();const row=[...path,String(index)],value=[...row,'value',...(real?['bits']:[])];children.push({kind:'record',path:row,label:label(field.name+' '+String(index+1),field.name+' '+String(index+1)),fields:[{kind:'text',path:value,label:real?label('Binary64 word','Binary64-Wort'):integer?label('Integer','Ganzzahl'):label('Property definition reference','Referenz zur Eigenschaftsdefinition'),required:true,format:real?'binary64':integer?'decimal-i64':'decimal-u64'}]});bindings.push({token:prepared.request.token,layout:'preserve',edits:[{source:'text',path:value}]});}
    fields.push({kind:'array',path,label:container?label(field.name+' members',field.name+' Elemente'):label('Create '+name+' container',name+'-Container erstellen'),rows:children,add:[{id:container?(integer?'integer':real?'real':'reference'):'container',label:container?(real?label('Add real word','REAL-Wort hinzufügen'):integer?label('Add integer','Ganzzahl hinzufügen'):label('Add reference','Referenz hinzufügen')):label('Create '+name,name+' erstellen'),value:container?(real?{kind:'real',value:{bits:''}}:{kind:integer?'integer':'reference',value:''}):{kind:'aggregate',value:[]}}]});
   }else if(selected?.kind==='enum'){
    const childPath=[...payloadPath,'value','items','0'],choices=selected.members!.map(word=>({id:word,label:word==='T'?label('True','Wahr'):word==='F'?label('False','Falsch'):word==='U'?label('Unknown','Unbekannt'):label(word,word),value:{kind:'enum',value:word}}));
    fields.push({kind:'choice',path:childPath,label:label(field.name+' value',field.name+' Wert'),required:true,provenance:'declaration',choices});bindings.push({token:prepared.request.token,layout:'replace',edits:[{source:'choice',kind:items!.length?'set':'append',path:items!.length?childPath:[...payloadPath,'value','items'],options:choices.map(choice=>({key:choice.id,value:choice.value}))}]});
   }else if(selected?.kind==='number'){
    const child=items![0],itemPath=[...payloadPath,'value','items'],childPath=[...itemPath,'0'],choices=['integer','real'].map(mode=>({id:mode,label:mode==='integer'?label('Integer','Ganzzahl'):label('Binary64','Binary64'),value:object(child)?.kind===mode?child!:{kind:mode,value:mode==='real'?{bits:''}:''}}));
    fields.push({kind:'choice',path:childPath,label:label(field.name+' representation',field.name+' Darstellung'),required:true,provenance:'declaration',choices});bindings.push({token:prepared.request.token,layout:'replace',edits:[{source:'choice',kind:items!.length?'set':'append',path:items!.length?childPath:itemPath,options:choices.map(choice=>({key:choice.id,value:choice.value}))}]});
    if(child){const actual=object(child)!.kind,path=[...childPath,'value',...(actual==='real'?['bits']:[])];fields.push({kind:'text',path,label:label(field.name+' numeric value',field.name+' Zahlenwert'),required:true,format:actual==='real'?'binary64':'decimal-i64'});bindings.push({token:prepared.request.token,layout:'preserve',edits:[{source:'text',path}]});}
   }else if(selected){const reference=selected.kind==='reference',path=reference?[...payloadPath,'value']:[...payloadPath,'value','items','0','value',...(selected.kind==='real'?['bits']:[])],edits:StagedFormBinding['edits']=reference||items!.length?[]:[{source:'fixed',edit:{kind:'set',path:[...payloadPath,'value','items'],value:[{kind:selected.kind,value:selected.kind==='real'?{bits:''}:''}]}}];fields.push({kind:'text',path,label:label(field.name+' '+(reference?'reference':selected.kind==='integer'?'integer':selected.kind==='real'?'binary64 words':selected.kind==='binary'?'encoded BINARY':'text'),field.name+' '+(reference?'Referenz':selected.kind==='integer'?'Ganzzahl':selected.kind==='real'?'Binary64-Wort':selected.kind==='binary'?'BINARY-Kodierung':'Text')),required:true,format:reference?'decimal-u64':selected.kind==='integer'?'decimal-i64':selected.kind==='real'?'binary64':'text',...(selected.kind==='binary'?{pattern:'^[0-3][0-9A-F]*$'}:{})});bindings.push({token:prepared.request.token,layout:reference||items!.length?'preserve':'replace',edits:[...edits,{source:'text',path}]});}
   if(!response.ready)diagnostics.push({path:payloadPath,code:'type.input-required',label:label('Choose a declared member and supply its value','Deklarierten Auswahltyp und Wert angeben')});continue;
  }
  if(field.kind==='aggregate'){
   const members=payload?.value as StagedValue[]|undefined,lengths=Array.from({length:field.upper!+1},(_value,index)=>index);if(members&&members.length>field.upper!)lengths.push(members.length);
   const choices=[];for(const length of lengths){yield tick();const value:StagedValue[]=[];for(let index=0;index<length;index++){yield tick();value.push(members?.[index]??{kind:'real',value:{bits:''}});}choices.push({id:String(length),label:label(String(length)+' members',String(length)+' Elemente'),value:{kind:'aggregate',value}});}
   fields.push({kind:'choice',path:payloadPath,label:label(field.name+' length',field.name+' Länge'),required:true,provenance:'declaration',choices});bindings.push({token:prepared.request.token,layout:'replace',edits:[...fixed,{source:'choice',kind:'set',path:payloadPath,options:choices.map(choice=>({key:choice.id,value:choice.value}))}]});
   for(const [index]of(members??[]).entries()){yield tick();const path=[...payloadPath,'value',String(index),'value','bits'];fields.push({kind:'text',path,label:label(field.name+' '+String(index+1)+' binary64 words',field.name+' '+String(index+1)+' Binary64-Wort'),required:true,format:'binary64'});bindings.push({token:prepared.request.token,layout:'preserve',edits:[{source:'text',path}]});}
   if(!response.ready)diagnostics.push({path:payloadPath,code:'type.input-required',label:label('Supply the declared LIST','Deklarierte LIST angeben')});continue;
  }
  if(field.kind==='number'){
   const choices=['integer','real'].map(mode=>({id:mode,label:mode==='integer'?label('Integer','Ganzzahl'):label('Binary64','Binary64'),value:payload?.kind===mode?payload:{kind:mode,value:mode==='real'?{bits:''}:''}}));
   fields.push({kind:'choice',path:payloadPath,label:label(field.name+' representation',field.name+' Darstellung'),required:true,provenance:'declaration',choices});
   bindings.push({token:prepared.request.token,layout:'replace',edits:[...fixed,{source:'choice',kind:'set',path:payloadPath,options:choices.map(choice=>({key:choice.id,value:choice.value}))}]});
   if(!payload)continue;
  }
  if(field.members){
   const offset=current&&JSON.stringify(current.path)===JSON.stringify(address(field.index))?Number(current.offset):0,words=field.members.length>32?field.members.slice(offset,offset+31):[...field.members],selected=found<0?undefined:object(object(rows![found])?.value)?.value;
   if(typeof selected==='string'&&field.members.includes(selected)&&!words.includes(selected))words.push(selected);
   const choices=[];for(const word of words){yield tick();choices.push({id:word,label:word==='T'?label('True','Wahr'):word==='F'?label('False','Falsch'):word==='U'?label('Unknown','Unbekannt'):label(word,word),value:{kind:'enum',value:word}});}
   fields.push({kind:'choice',path:payloadPath,label:label(field.name,field.name),required:true,provenance:'declaration',choices});
   bindings.push({token:prepared.request.token,layout:'replace',edits:[...fixed,{source:'choice',kind:'set',path:payloadPath,options:choices.map(choice=>({key:choice.id,value:choice.value}))}]});
  }else{
   bindings.push({token:prepared.request.token,layout:found<0?'replace':'preserve',edits:[...fixed,{source:'text',path}]});
   fields.push({kind:'text',path,label:label(field.name+' '+(kind==='reference'?'reference':kind==='integer'?'integer':kind==='real'?'binary64 words':kind==='binary'?'encoded BINARY':'text'),field.kind==='reference'&&field.name==='Profile'?'Profilreferenz':field.name+' '+(kind==='reference'?'Referenz':kind==='integer'?'Ganzzahl':kind==='real'?'Binary64-Wort':kind==='binary'?'BINARY-Kodierung':'Text')),required:true,format:kind==='reference'?'decimal-u64':kind==='integer'?'decimal-i64':kind==='real'?'binary64':'text',...(kind==='binary'?{pattern:'^[0-3][0-9A-F]*$'}:{})});
  }
  if(!response.ready)diagnostics.push({path:field.members?payloadPath:path,code:'type.input-required',label:label('Supply the declared value','Deklarierten Wert angeben')});
 }
 const content=(fields:readonly StagedFormField[]):number=>fields.reduce((count,field)=>count+(field.kind==='array'?1+field.rows.reduce((count,row)=>count+1+(row.kind==='record'?content(row.fields):1),0):field.kind==='record'?content(field.fields):1),0);if(content(fields)>STAGED_FORM_BODY_LIMITS.content)fail('type.input-content');return{response,bindings,pageBindings};})(),opts);
}
/** 🔀️ Exact caller-authored values join the existing central sparse diff and inverse authority. */
export async function typeInputMutations(source:IfcSnapshot,revision:string,request:StagedFormRequest,control:StagedFormControl):Promise<IfcMutation[]>{
 const opts=options(control),prepared=await runIfcEditAsync(prepare(source,revision,request,opts,false),opts);await localRules(prepared,opts,false);return runIfcEditAsync((function*():IfcEditSteps<IfcMutation[]>{const entity=prepared.source.entities[prepared.position]!,result:IfcMutation[]=[];if(prepared.name!==entity.name.toUpperCase())result.push({mutation:'setEntityName',id:entity.id,name:prepared.name});for(let index=entity.args.length-1;index>=prepared.args.length;index--){yield tick();result.push({mutation:'removeEntityArg',id:entity.id,index});}for(const [index,value]of prepared.args.entries()){yield tick();if(prepared.map[index]===index||prepared.map[index]===null&&value.kind==='unset'&&entity.args[index]?.kind==='unset')continue;result.push(index<entity.args.length?{mutation:'setEntityArg',id:entity.id,index,value}:{mutation:'insertEntityArg',id:entity.id,index,value});}return result;})(),opts);
}
