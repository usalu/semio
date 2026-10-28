import {validateIfc4x3Domains} from './admission.ts';
import {captureIfcGraphSteps,runIfcEditAsync} from '../🔺️diff/🟦️.ts';
import {parseExpressExpression} from './parser.ts';
import type {IfcSnapshot,IfcValue} from '../📸️snapshot/🟦️.ts';
import {ifc4x3Catalog,ifcIsSubtype} from '../📚️catalog/🟦️.ts';
import type {IfcTypeExpression} from '../📚️catalog/🟦️.ts';
import type {ExpressControl,ExpressDiagnostic,ExpressProgram,ExpressRule} from './ast.ts';
import {compileIfc4x3Express,retainedIfc4x3ExpressCompilation,type ExpressCompilation} from './compiler.ts';
import {captureExpressControlScope} from './control.ts';
import {ExpressEvaluator,ExpressEvaluationError,expressAggregate} from './evaluator.ts';
import type {ExpressValue,ExpressScope} from './evaluator.ts';
/** 🧾 Every executed rule retains its identity and three-valued result. */
export interface ExpressRuleResult {owner:string;rule:string;entityId?:bigint;result:'true'|'false'|'unknown'|'incomplete';kind:'where'|'unique'|'inverse'|'bound'|'global'}
/** 🧪 Semantic completeness describes execution coverage, not model conformance. */
export interface IfcExpressValidation {complete:boolean;valid:boolean;diagnostics:ExpressDiagnostic[];results:ExpressRuleResult[];work:number}
const unknown:ExpressValue={kind:'unknown'};
const flag=(value:ExpressValue)=>value.kind==='logical'?String(value.value):'unknown';
const aggregateItems=(value:ExpressValue)=>value.kind==='aggregate'?value.value as ExpressValue[]:[];
/** 🌐 Schema semantic context resolves entity, derived and inverse relationships lazily. */
export class IfcExpressContext {
 readonly entities=new Map<bigint,ExpressValue>();
 readonly #populations=new Map<string,ExpressValue[]>();
 #state:'new'|'initializing'|'ready'|'failed'='new';
 readonly derived=new Map<string,ExpressRule>();
 readonly pending=new Set<string>();
 readonly incoming=new Map<bigint,{entity:ExpressValue;attribute:string}[]>();
 readonly evaluator:ExpressEvaluator;
 constructor(readonly snapshot:IfcSnapshot,readonly program:ExpressProgram,control:ExpressControl={}){
  this.evaluator=new ExpressEvaluator(program,control,(value,name,qualifier,scope)=>this.attribute(value,name,qualifier,scope),(value,role)=>this.usedIn(value,role));
  for(const rule of program.derived)this.derived.set(`${rule.owner}.${rule.label}`,rule);
  }
 async initialize():Promise<void>{
  if(this.#state!=='new')throw new ExpressEvaluationError('context-state','Population context initialization is single-use');this.#state='initializing';
  try{const snapshot=this.snapshot;
  for(const entity of snapshot.entities){await this.evaluator.tick();const name=entity.name.toUpperCase();const types:string[]=[];let current:string|null=name;while(current){types.push(current);current=ifc4x3Catalog.entities[current]?.supertype??null;}const value:ExpressValue={kind:'entity',identity:entity.id,types,fields:new Map(),groups:new Map()};this.entities.set(entity.id,value);for(const type of types){await this.evaluator.tick();if(!this.#populations.has(type))this.#populations.set(type,[]);this.#populations.get(type)!.push(value);}}
  for(const [row,entity]of snapshot.entities.entries()){await this.evaluator.tick();const value=this.entities.get(entity.id)!,declaration=ifc4x3Catalog.entities[entity.name.toUpperCase()];for(let index=0;index<(declaration?.allAttributes.length??0);index++){const attribute=declaration!.allAttributes[index]!;const converted=await this.convert(await this.sourceAttribute(row,index),attribute.typeExpr);value.fields!.set(attribute.name.toUpperCase(),converted);const owner=attribute.declaredBy.toUpperCase();if(!value.groups!.has(owner))value.groups!.set(owner,new Map());value.groups!.get(owner)!.set(attribute.name.toUpperCase(),converted);}}
  const index=async(value:ExpressValue,entity:ExpressValue,attribute:string):Promise<void>=>{await this.evaluator.tick();if(value.kind==='entity'&&typeof value.identity==='bigint'){if(!this.incoming.has(value.identity))this.incoming.set(value.identity,[]);this.incoming.get(value.identity)!.push({entity,attribute});}else if(value.kind==='aggregate')for(const item of aggregateItems(value))await index(item,entity,attribute);};for(const entity of this.entities.values())for(const [attribute,value] of entity.fields!)await index(value,entity,attribute);
  await this.evaluator.tick();this.#state='ready';}catch(error){this.#state='failed';throw error;}
 }
 /** 🧭 Resolve the actual own or inherited source slot before semantic conversion. */
 async sourceAttribute(row:number,index:number):Promise<IfcValue>{
  await this.evaluator.tick();const entity=this.snapshot.entities[row],attribute=entity&&ifc4x3Catalog.entities[entity.name.toUpperCase()]?.allAttributes[index];
  if(!entity||!attribute)throw new ExpressEvaluationError('source-coordinate','Source attribute coordinate is unavailable');
  if(!entity.complex?.length)return entity.args[index]??{kind:'unset'};
  if(entity.complex.length>=32)throw new ExpressEvaluationError('complex-chain','Source component capacity exceeded');
  const owner=attribute.declaredBy.toUpperCase();let found=-1,matches=0;
  for(let part=0;part<=entity.complex.length;part++){await this.evaluator.tick();if((part===0?entity.name:entity.complex[part-1]!.name).toUpperCase()===owner){found=part;matches++;}}
  const ordinal=ifc4x3Catalog.entities[owner]?.attributes.findIndex(item=>item.name===attribute.name)??-1;
  if(matches!==1||ordinal<0)throw new ExpressEvaluationError('complex-owner','Source attribute owner is unavailable or ambiguous');
  return(found===0?entity.args:entity.complex[found-1]!.args)[ordinal]??{kind:'unset'};
 }
 typeNames(name:string,seen=new Set<string>()):string[]{name=name.toUpperCase();if(seen.has(name))return[];seen.add(name);const type=ifc4x3Catalog.types[name];return[name,...(type?.typeExpr?.kind==='named'?this.typeNames(type.typeExpr.name!,seen):[])];}
 async convert(value:IfcValue,type?:IfcTypeExpression,depth=0):Promise<ExpressValue>{
  await this.evaluator.tick();
  if(depth>(this.evaluator.control.maximumDepth??256))throw new ExpressEvaluationError('depth','Graph value conversion depth exceeded');
  let result:ExpressValue;
  switch(value.kind){case 'unset':case 'derived':return unknown;case 'integer':result={kind:'integer',value:value.value};break;case 'real':{const buffer=new ArrayBuffer(8),view=new DataView(buffer);view.setBigUint64(0,value.value.bits,false);result={kind:'real',value:view.getFloat64(0,false)};break;}case 'string':result={kind:'string',value:value.value};break;case 'binary':result={kind:'binary',value:value.value};break;case 'enum':result=['T','F','U'].includes(value.value.toUpperCase())?{kind:'logical',value:value.value.toUpperCase()==='T'?'true':value.value.toUpperCase()==='F'?'false':'unknown'}:{kind:'enum',value:value.value.toUpperCase()};break;case 'reference':return this.entities.get(value.value)??unknown;case 'typedValue':{result=await this.convert(value.value.items[0]??{kind:'unset'},{kind:'named',name:value.value.name.toUpperCase()},depth+1);return result;}case 'aggregate':{let expression=type;const visited=new Set<string>();while(expression?.kind==='named'&&ifc4x3Catalog.types[expression.name!]?.typeExpr&&!visited.has(expression.name!)){visited.add(expression.name!);expression=ifc4x3Catalog.types[expression.name!]!.typeExpr;}const converted:ExpressValue[]=[];for(const item of value.value)converted.push(await this.convert(item,expression?.item,depth+1));result=expressAggregate(converted,expression?.aggregate,expression?.aggregate==='ARRAY'?BigInt(expression.lower??1):1n);break;}}
  const names=type?.kind==='named'?this.typeNames(type.name!):[];return{...result,types:[...new Set([...names,...this.evaluator.primitiveTypes(result)])]};
 }
 scope(value:ExpressValue,parent?:ExpressScope):ExpressScope{const scope=new Map([['SELF',value]]);this.evaluator.inheritScope(scope,parent,value);return scope;}
 /** 👥 Project source-ordered subtype cohorts without lending the retained index array. */
 async population(name:string):Promise<ExpressValue>{
  await this.evaluator.tick();if(this.#state!=='ready')throw new ExpressEvaluationError('context-state','Population context initialization has not completed');const values:ExpressValue[]=[];
  for(const value of this.#populations.get(name.toUpperCase())??[]){await this.evaluator.tick();values.push(value);}
  return expressAggregate(values,'SET');
 }
 /** 🔗 Share original-controlled inverse identity matching with scalar cardinality. */
 async inverseValues(value:ExpressValue,inverse:ExpressProgram['inverse'][number]):Promise<ExpressValue[]>{
  await this.evaluator.tick();if(this.#state!=='ready')throw new ExpressEvaluationError('context-state','Population context initialization has not completed');const matching:ExpressValue[]=[],seen=new Set<ExpressValue>();
  for(const edge of typeof value.identity==='bigint'?this.incoming.get(value.identity)??[]:[]){await this.evaluator.tick();if(edge.attribute===inverse.attribute&&(edge.entity.types??[]).includes(inverse.type)&&!seen.has(edge.entity)){seen.add(edge.entity);matching.push(edge.entity);}}
  return matching;
 }
 async attribute(value:ExpressValue,name:string,qualifier?:string,scope?:ExpressScope):Promise<ExpressValue>{
  name=name.toUpperCase();const actual=typeof value.identity==='bigint'?this.entities.get(value.identity)??value:value;
  for(const owner of qualifier?[qualifier,...(actual.types??[])]:(actual.types??[])){const derived=this.derived.get(`${owner}.${name}`);if(derived){const key=`${String(actual.identity)}.${owner}.${name}`;if(this.pending.has(key))throw new ExpressEvaluationError('derived-cycle',`Cyclic derived attribute ${owner}.${name}`);this.pending.add(key);try{const result=await this.evaluator.evaluate(derived.expression,this.scope(actual===value?value:this.evaluator.typedView(value,actual.types??[],scope),scope));const type=ifc4x3Catalog.entities[owner]?.derived.find(attribute=>attribute.name.toUpperCase()===name)?.typeExpr;return type?.kind==='aggregate'&&result.kind==='aggregate'?{...result,aggregate:type.aggregate,lower:type.aggregate==='ARRAY'?BigInt(type.lower??1):1n}:result;}finally{this.pending.delete(key);}}}
  const inverse=this.program.inverse.find(rule=>rule.name===name&&(actual.types??[]).includes(rule.owner));if(inverse){const matching=await this.inverseValues(actual,inverse);return inverse.aggregate===false?matching[0]??unknown:expressAggregate(matching,'SET');}
  return this.evaluator.readValue(value.fields?.get(name)??unknown,scope,value);
 }
 references(value:ExpressValue,target:ExpressValue):boolean{return value.kind==='entity'?value.identity===target.identity:value.kind==='aggregate'?aggregateItems(value).some(item=>this.references(item,target)):false;}
 async usedIn(value:ExpressValue,role:string):Promise<ExpressValue>{const parts=role.toUpperCase().split('.'),owner=parts.at(-2),attribute=parts.at(-1);const results:ExpressValue[]=[];for(const edge of typeof value.identity==='bigint'?this.incoming.get(value.identity)??[]:[]){await this.evaluator.tick();if(owner&&!(edge.entity.types??[]).includes(owner))continue;if(!attribute||edge.attribute===attribute)results.push(edge.entity);}return expressAggregate(results,'BAG');}
}
/** ⚖ Execute the retained schema against a graph without claiming unsupported evaluation passed. */
export async function validateIfc4x3Express(snapshot:IfcSnapshot,control:ExpressControl={},program?:ExpressProgram):Promise<IfcExpressValidation>{
 const controlScope=captureExpressControlScope(control);control=controlScope.control;
 const cancelled=(report:IfcExpressValidation):IfcExpressValidation=>({...report,complete:false,valid:false,diagnostics:report.diagnostics.some(row=>row.code==='express.cancelled')?report.diagnostics:[...report.diagnostics,{code:'express.cancelled',message:'EXPRESS validation cancelled'}]});
 const outcome=async(report:IfcExpressValidation):Promise<IfcExpressValidation>=>{controlScope.rethrowCallerFailure();if(control.signal?.aborted)return cancelled(report);try{await controlScope.checkpoint({phase:'validation-result',completed:1,total:1});}catch(error){controlScope.rethrowCallerFailure();if(!control.signal?.aborted)throw error;return cancelled(report);}return report;};
 let compilation:ExpressCompilation;try{const retained=program?retainedIfc4x3ExpressCompilation(program):await compileIfc4x3Express(control);if(!retained)return outcome({complete:false,valid:false,diagnostics:[{code:'express.program.origin',message:'Full IFC validation requires the exact compiler-issued program'}],results:[],work:0});compilation=retained;}catch(error){controlScope.rethrowCallerFailure();return outcome({complete:false,valid:false,diagnostics:[{code:'express.compile.'+(error instanceof ExpressEvaluationError?error.code:'admission'),message:String(error)}],results:[],work:0});}
 const diagnostics:ExpressDiagnostic[]=[...compilation.diagnostics],results:ExpressRuleResult[]=[];let admitted:IfcSnapshot;try{admitted=await runIfcEditAsync(captureIfcGraphSteps(snapshot,'validateBase',{signal:control.signal,maxDepth:control.maximumDepth,maxValues:control.maximumWork}),{signal:control.signal,yieldEvery:control.batchSize,onProgress:progress=>control.progress?.({...progress,phase:'admit'})});}catch(error){controlScope.rethrowCallerFailure();return outcome({complete:false,valid:false,diagnostics:[{code:'express.admission',message:String(error)}],results,work:0});} snapshot=admitted;
 const normalizer=new ExpressEvaluator(undefined,{...control,progress:row=>control.progress?.({...row,phase:'normalize'})});try{
 for(const entity of snapshot.entities){await normalizer.tick();if(!ifc4x3Catalog.entities[entity.name.toUpperCase()])return outcome({complete:false,valid:false,diagnostics:[{code:'express.domain',message:`Unknown entity ${entity.name}`,entityId:entity.id}],results,work:0});if(!(entity.complex??[]).length)continue;if(entity.complex!.length>=876)return outcome({complete:false,valid:false,diagnostics:[{code:'express.complex',message:'Complex constituents exceed the complete schema declaration bound',entityId:entity.id}],results,work:0});const parts=[{name:entity.name,args:entity.args},...entity.complex!],names=new Set(parts.map(part=>part.name.toUpperCase()));const leaves:typeof parts=[];for(const part of parts){await normalizer.tick();if(!ifc4x3Catalog.entities[part.name.toUpperCase()])throw new ExpressEvaluationError('complex-domain','Unknown complex constituent '+part.name);let supertype=false;for(const other of parts){await normalizer.tick();if(other!==part&&ifcIsSubtype(other.name,part.name)){supertype=true;break;}}if(!supertype)leaves.push(part);}if(leaves.length!==1||names.size!==parts.length)return outcome({complete:false,valid:false,diagnostics:[{code:'express.complex',message:'Complex constituents must form one legal IFC subtype chain',entityId:entity.id}],results,work:0});const leaf=ifc4x3Catalog.entities[leaves[0]!.name.toUpperCase()]!;const chain:string[]=[];let current:string|null=leaf.name.toUpperCase();while(current){await normalizer.tick();chain.push(current);current=ifc4x3Catalog.entities[current]?.supertype??null;}if(chain.length!==names.size||chain.some(name=>!names.has(name))||leaf.abstractEntity)return outcome({complete:false,valid:false,diagnostics:[{code:'express.complex',message:'Complex constituents must include every inherited entity and a concrete leaf',entityId:entity.id}],results,work:0});const argumentsByOwner=new Map<string,Map<string,IfcValue>>();for(const part of parts){await normalizer.tick();const declaration=ifc4x3Catalog.entities[part.name.toUpperCase()]!,attributes=declaration.attributes;if(attributes.length!==part.args.length)return outcome({complete:false,valid:false,diagnostics:[{code:'express.complex',message:`Invalid direct arguments for ${part.name}`,entityId:entity.id}],results,work:0});argumentsByOwner.set(part.name.toUpperCase(),new Map(attributes.map((attribute,index)=>[attribute.name.toUpperCase(),part.args[index]!])));}for(const attribute of leaf.allAttributes){await normalizer.tick();if(attribute.derived&&argumentsByOwner.get(attribute.declaredBy.toUpperCase())?.get(attribute.name.toUpperCase())?.kind!=='derived')return outcome({complete:false,valid:false,diagnostics:[{code:'express.complex-derived',message:'Effective derived attribute requires its original derived marker',entityId:entity.id}],results,work:0});}entity.name=leaf.name.toUpperCase();entity.args=leaf.allAttributes.map(attribute=>attribute.derived?{kind:'derived'}:argumentsByOwner.get(attribute.declaredBy.toUpperCase())?.get(attribute.name.toUpperCase())??{kind:'unset'});entity.complex=[];}}catch(error){controlScope.rethrowCallerFailure();return outcome({complete:false,valid:false,diagnostics:[{code:'express.normalize.'+(error instanceof ExpressEvaluationError?error.code:'admission'),message:String(error)}],results,work:0});}
 let structural;try{structural=await validateIfc4x3Domains(snapshot,control);}catch(error){controlScope.rethrowCallerFailure();return outcome({complete:false,valid:false,diagnostics:[{code:'express.structure.'+(error instanceof ExpressEvaluationError?error.code:'admission'),message:String(error)}],results,work:0});}if(structural.length)return outcome({complete:false,valid:false,diagnostics:structural.map(row=>({code:'express.domain.'+row.code,message:row.message,entityId:row.entityId})),results,work:0});
 const context=new IfcExpressContext(snapshot,compilation.program,control),evaluator=context.evaluator;let complete=compilation.complete;
 const run=async(rule:ExpressRule,scope:ExpressScope,kind:ExpressRuleResult['kind'],entityId?:bigint)=>{try{const value=await evaluator.evaluate(rule.expression,scope),result=flag(value) as 'true'|'false'|'unknown';results.push({owner:rule.owner,rule:rule.label,kind,result,...(entityId===undefined?{}:{entityId})});if(result==='false')diagnostics.push({code:'express.rule',message:`${rule.owner}.${rule.label} is FALSE`,owner:rule.owner,rule:rule.label,entityId});}catch(error){controlScope.check();complete=false;results.push({owner:rule.owner,rule:rule.label,kind,result:'incomplete',entityId});diagnostics.push({code:`express.${error instanceof ExpressEvaluationError?error.code:'evaluation'}`,message:error instanceof Error?error.message:String(error),owner:rule.owner,rule:rule.label,entityId});if(error instanceof ExpressEvaluationError&&['cancelled','work-budget'].includes(error.code))throw error;}};
 const typeRules=async(value:ExpressValue,type:IfcTypeExpression,id:bigint,seen=new Set<string>()):Promise<void>=>{await evaluator.tick();if(value.kind==='unknown')return;if(type.kind==='aggregate'){for(const item of aggregateItems(value))if(type.item)await typeRules(item,type.item,id,new Set(seen));return;}let name=type.name!;if(ifc4x3Catalog.entities[name])return;if(value.types?.[0]&&ifc4x3Catalog.types[value.types[0]]&&ifc4x3Catalog.types[name]?.kind==='select')name=value.types[0];if(seen.has(name))return;seen.add(name);for(const rule of compilation.program.rules)if(rule.owner===name)await run(rule,context.scope(value),'where',id);const declared=ifc4x3Catalog.types[name];if(declared?.typeExpr)await typeRules(value,declared.typeExpr,id,seen);};
 try{
 await context.initialize();
 for(const entity of snapshot.entities){await evaluator.tick();const value=context.entities.get(entity.id)!,scope=context.scope(value);for(const rule of compilation.program.rules)if((value.types??[]).includes(rule.owner))await run(rule,scope,'where',entity.id);const declaration=ifc4x3Catalog.entities[entity.name.toUpperCase()];for(let index=0;index<(declaration?.allAttributes.length??0);index++){const attribute=declaration!.allAttributes[index]!;await typeRules(value.fields!.get(attribute.name.toUpperCase())??unknown,attribute.typeExpr,entity.id);}
  for(const derived of compilation.program.derived)if((value.types??[]).includes(derived.owner)){try{await context.attribute(value,derived.label);}catch(error){controlScope.check();complete=false;diagnostics.push({code:'express.derived-incomplete',owner:derived.owner,rule:derived.label,entityId:entity.id,message:String(error)});}}
 for(const bound of compilation.program.bounds)if((value.types??[]).includes(bound.owner)){try{const parts=bound.label.toUpperCase().split('.'),side=parts.pop()!,attribute=parts.shift()!;let aggregates=[await context.attribute(value,attribute)];for(const path of parts)if(path==='ITEM')aggregates=aggregates.flatMap(aggregateItems);const expected=await evaluator.evaluate(bound.expression,scope);for(const aggregate of aggregates){const actual=aggregate.kind==='aggregate'?{kind:'integer' as const,value:side==='UPPER'?(aggregate.lower??1n)+BigInt(aggregateItems(aggregate).length)-1n:aggregate.lower??1n}:unknown;const result=flag(await evaluator.binary('=',actual,expected)) as 'true'|'false'|'unknown';results.push({owner:bound.owner,rule:bound.label,kind:'bound',entityId:entity.id,result});if(result==='false')diagnostics.push({code:'express.bound',owner:bound.owner,rule:bound.label,entityId:entity.id,message:`Contextual bound ${bound.owner}.${bound.label}`});}}catch(error){controlScope.check();complete=false;diagnostics.push({code:'express.bound-incomplete',owner:bound.owner,rule:bound.label,entityId:entity.id,message:String(error)});}}
 for(const inverse of compilation.program.inverse)if((value.types??[]).includes(inverse.owner)){const size=BigInt((await context.inverseValues(value,inverse)).length);for(const [bound,operator] of [[inverse.lower??(inverse.aggregate===false?{kind:'integer',offset:0,text:'1'}:undefined),'>='],[inverse.upper??(inverse.aggregate===false?{kind:'integer',offset:0,text:'1'}:undefined),'<=']] as const){if(!bound||bound.kind==='unknown')continue;const limit=await evaluator.evaluate(bound,scope),comparison=await evaluator.binary(operator,{kind:'integer',value:size},limit);const result=flag(comparison) as 'true'|'false'|'unknown';results.push({owner:inverse.owner,rule:inverse.name,kind:'inverse',entityId:entity.id,result});if(result==='false')diagnostics.push({code:'express.inverse',message:`Inverse cardinality ${inverse.owner}.${inverse.name}`,owner:inverse.owner,rule:inverse.name,entityId:entity.id});}}
 }
 for(const rule of compilation.program.unique){const previous:{value:ExpressValue;id:bigint}[]=[];for(const [id,value] of context.entities){if(!(value.types??[]).includes(rule.owner))continue;try{const tuple=await evaluator.evaluate(rule.expression,context.scope(value));let result:'true'|'false'|'unknown'='true';for(const candidate of previous){const equal=flag(await evaluator.equal(tuple,candidate.value,true));if(equal==='true'){result='false';break;}if(equal==='unknown')result='unknown';}results.push({owner:rule.owner,rule:rule.label,kind:'unique',entityId:id,result});if(result==='false')diagnostics.push({code:'express.unique',message:`Duplicate ${rule.owner}.${rule.label}`,owner:rule.owner,rule:rule.label,entityId:id});previous.push({value:tuple,id});}catch(error){controlScope.check();complete=false;diagnostics.push({code:'express.unique-incomplete',owner:rule.owner,rule:rule.label,entityId:id,message:String(error)});}}}
 for(const global of compilation.program.globalRules){const scope=evaluator.proceduralScope();for(const population of global.populations)scope.set(population,await context.population(population));await evaluator.declare(global.locals,scope);await evaluator.statements(global.body,scope);for(const rule of global.rules)await run(rule,scope,'global');}
 }catch(error){controlScope.rethrowCallerFailure();complete=false;diagnostics.push({code:`express.${error instanceof ExpressEvaluationError?error.code:'context'}`,message:String(error)});}
 return outcome({complete,valid:complete&&!diagnostics.some(item=>['express.rule','express.unique','express.inverse','express.bound'].includes(item.code)),diagnostics,results,work:evaluator.work});
}














