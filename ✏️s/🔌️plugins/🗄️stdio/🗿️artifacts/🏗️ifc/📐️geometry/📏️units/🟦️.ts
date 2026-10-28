import type {GeometryEntity,GeometryGraph} from "../📖️graph/🟦️.ts";
export type LengthUnitResult={status:"pending"}|{status:"resolved";project:string;unit:string;metresPerUnit:number}|{status:"invalid"|"limited";entity:string;reason:string}|{status:"cancelled"};
export interface LengthUnitLimits{entities:number;references:number;depth:number}
export const DEFAULT_LENGTH_UNIT_LIMITS:Readonly<LengthUnitLimits>=Object.freeze({entities:200_000,references:200_000,depth:64});
export type LengthUnitControl="continue"|"yield"|"cancel";
const powers:Readonly<Record<string,number>>=Object.freeze({EXA:18,PETA:15,TERA:12,GIGA:9,MEGA:6,KILO:3,HECTO:2,DECA:1,DECI:-1,CENTI:-2,MILLI:-3,MICRO:-6,NANO:-9,PICO:-12,FEMTO:-15,ATTO:-18});
const u64=(value:bigint):boolean=>typeof value==="bigint"&&value>0n&&value<=18_446_744_073_709_551_615n;
/** 📏️ Resolve one project against its captured immutable source; retain the original snapshot lease until bounded close finishes. */
export class LengthUnitWork{
 private readonly index=new Map<bigint,number>();private readonly visited=new Set<bigint>();private readonly assignedUnits=new Set<bigint>();
 private cursor=0;private assignment=0n;private assigned=0;private unit=0n;private selected=0n;private scale=1;private phase:"index"|"project"|"assignment"|"units"|"resolve"="index";
 private sourceIdentity:object|undefined;private sourceLength:number|undefined;private result:LengthUnitResult={status:"pending"};private closed=false;
 work=0;
 constructor(readonly project:bigint,readonly sourceVersion:bigint,readonly limits:Readonly<LengthUnitLimits>=DEFAULT_LENGTH_UNIT_LIMITS){this.limits=Object.freeze({entities:limits.entities,references:limits.references,depth:limits.depth});if(!u64(project)||typeof sourceVersion!=="bigint"||sourceVersion<0n||sourceVersion>18_446_744_073_709_551_615n||Object.values(this.limits).some(value=>!Number.isSafeInteger(value)||value<1))this.fail(typeof project==="bigint"?project:0n,"invalid-length-unit-request");}
 private fail(entity:bigint,reason:string,status:"invalid"|"limited"="invalid"):void{this.result={status,entity:(typeof entity==="bigint"&&entity>=0n&&entity<=18_446_744_073_709_551_615n?entity:0n).toString(),reason};}
 private entity<V>(graph:GeometryGraph<V>,id:bigint):GeometryEntity<V>|undefined{const ordinal=this.index.get(id);return ordinal===undefined?undefined:graph.entity(ordinal);}
 advance<V>(graph:GeometryGraph<V>,sourceVersion:bigint,maximumWork:number,control:(work:number)=>LengthUnitControl=()=>"continue"):LengthUnitResult{
  if(this.closed)return {status:"cancelled"};
  if((this.sourceIdentity!==undefined&&this.sourceIdentity!==graph.sourceIdentity())||sourceVersion!==this.sourceVersion||(this.sourceLength!==undefined&&this.sourceLength!==graph.length)){this.fail(this.project,"source-version-changed");return this.result;}
  if(this.result.status!=="pending")return this.result;
  if(!Number.isSafeInteger(maximumWork)||maximumWork<0){this.fail(this.project,"invalid-work-grant");return this.result;}
  if(maximumWork===0){if(control(this.work)==="cancel")this.result={status:"cancelled"};return this.result;}
  if(!Number.isSafeInteger(graph.length)||graph.length<0||graph.length>this.limits.entities){this.fail(this.project,"entity-limit","limited");return this.result;}
  this.sourceIdentity=graph.sourceIdentity();this.sourceLength=graph.length;
  for(let spent=0;spent<maximumWork&&this.result.status==="pending";spent++){
   const decision=control(this.work);if(decision==="cancel"){this.result={status:"cancelled"};break;}if(decision==="yield")break;
   this.work++;this.step(graph);
  }
  return this.result;
 }
 private step<V>(graph:GeometryGraph<V>):void{
  const values=graph.values;
  if(this.phase==="index"){
   if(this.cursor===graph.length){this.phase="project";return;}
   const entity=graph.entity(this.cursor);if(!entity||!u64(entity.id)){this.fail(this.project,"invalid-instance");return;}
   if(this.index.has(entity.id)){this.fail(entity.id,"duplicate-instance");return;}this.index.set(entity.id,this.cursor++);return;
  }
  if(this.phase==="project"){
   const entity=this.entity(graph,this.project);if(!entity||entity.name!=="IFCPROJECT"||entity.constituentCount!==1){this.fail(this.project,"missing-project");return;}
   const assignment=values.reference(entity.args[8]);if(assignment===undefined){this.fail(this.project,"missing-unit-assignment");return;}this.assignment=assignment;this.phase="assignment";return;
  }
  if(this.phase==="assignment"){
   const entity=this.entity(graph,this.assignment);if(!entity||entity.name!=="IFCUNITASSIGNMENT"||entity.constituentCount!==1){this.fail(this.assignment,"invalid-unit-assignment");return;}
   const units=values.aggregate(entity.args[0]);if(!units){this.fail(this.assignment,"invalid-unit-assignment");return;}if(units.length>this.limits.references){this.fail(this.assignment,"reference-limit","limited");return;}this.phase="units";return;
  }
  if(this.phase==="units"){
   const units=values.aggregate(this.entity(graph,this.assignment)?.args[0]);if(!units){this.fail(this.assignment,"source-version-changed");return;}
   if(this.assigned===units.length){if(this.selected===0n){this.fail(this.assignment,"missing-length-unit");return;}this.unit=this.selected;this.phase="resolve";return;}
   const id=values.reference(units[this.assigned++]);if(id===undefined){this.fail(this.assignment,"invalid-assigned-unit");return;}
   if(this.assignedUnits.has(id)){this.fail(id,"duplicate-assigned-unit");return;}this.assignedUnits.add(id);
   const entity=this.entity(graph,id);if(!entity){this.fail(id,"missing-assigned-unit");return;}if(entity.constituentCount!==1){this.fail(id,"unsupported-complex-unit");return;}
   if(entity.name==="IFCMONETARYUNIT")return;
   if(!["IFCSIUNIT","IFCCONVERSIONBASEDUNIT","IFCCONVERSIONBASEDUNITWITHOFFSET","IFCCONTEXTDEPENDENTUNIT","IFCDERIVEDUNIT"].includes(entity.name)){this.fail(id,"invalid-assigned-unit");return;}
   if(values.enumeration(entity.args[1])!=="LENGTHUNIT")return;
   if(this.selected!==0n){this.fail(id,"ambiguous-length-unit");return;}this.selected=id;return;
  }
  const id=this.unit,entity=this.entity(graph,id);if(!entity||entity.constituentCount!==1){this.fail(id,"invalid-length-unit");return;}
  if(this.visited.has(id)){this.fail(id,"cyclic-length-unit");return;}if(this.visited.size>=this.limits.depth){this.fail(id,"depth-limit","limited");return;}this.visited.add(id);
  if(values.enumeration(entity.args[1])!=="LENGTHUNIT"){this.fail(id,"invalid-length-unit-type");return;}
  if(entity.name==="IFCSIUNIT"){
   if(!values.derived(entity.args[0])){this.fail(id,"invalid-si-dimensions");return;}
   if(values.enumeration(entity.args[3])!=="METRE"){this.fail(id,"invalid-si-length-name");return;}
   const prefix=values.enumeration(entity.args[2]),power=values.unset(entity.args[2])?0:prefix===undefined?undefined:powers[prefix];if(power===undefined){this.fail(id,"invalid-si-prefix");return;}
   const scale=this.scale*10**power;if(!Number.isFinite(scale)||scale<=0){this.fail(id,"unrepresentable-length-scale");return;}this.result={status:"resolved",project:this.project.toString(),unit:this.selected.toString(),metresPerUnit:scale};return;
  }
  if(!["IFCCONVERSIONBASEDUNIT","IFCCONVERSIONBASEDUNITWITHOFFSET"].includes(entity.name)){this.fail(id,"unsupported-length-unit");return;}
  if(entity.name==="IFCCONVERSIONBASEDUNITWITHOFFSET"&&(graph.standard==="IFC2X3"||values.real(entity.args[4])!==0)){this.fail(id,"nonlinear-length-unit");return;}
  const dimensionsId=values.reference(entity.args[0]),dimensions=dimensionsId===undefined?undefined:this.entity(graph,dimensionsId);if(!dimensions||dimensions.constituentCount!==1||dimensions.name!=="IFCDIMENSIONALEXPONENTS"||dimensions.args.length!==7||[0,1,2,3,4,5,6].some(index=>values.integer(dimensions.args[index])!==(index===0?1n:0n))){this.fail(id,"invalid-length-dimensions");return;}
  const factorId=values.reference(entity.args[3]),factor=factorId===undefined?undefined:this.entity(graph,factorId);if(!factor||factor.constituentCount!==1||factor.name!=="IFCMEASUREWITHUNIT"){this.fail(id,"invalid-length-factor");return;}
  const typed=values.typed(factor.args[0]),names=graph.standard==="IFC4"?["IFCLENGTHMEASURE","IFCPOSITIVELENGTHMEASURE","IFCNONNEGATIVELENGTHMEASURE"]:["IFCLENGTHMEASURE","IFCPOSITIVELENGTHMEASURE"],value=typed?.items.length===1&&names.includes(typed.name)?values.real(typed.items[0]):undefined,next=values.reference(factor.args[1]);if(value===undefined||!Number.isFinite(value)||value<=0||next===undefined){this.fail(id,"invalid-length-factor");return;}
  const scale=this.scale*value;if(!Number.isFinite(scale)||scale<=0){this.fail(id,"unrepresentable-length-scale");return;}this.scale=scale;this.unit=next;
 }
 /** 🧹️ Release retained lookup entries under the original owner's close grant. */
 close(maximumWork:number,control:(work:number)=>LengthUnitControl=()=>"continue"):boolean{
  if(!Number.isSafeInteger(maximumWork)||maximumWork<0)return false;
  for(let spent=0;spent<maximumWork&&(this.index.size||this.visited.size||this.assignedUnits.size);spent++){if(control(this.work)==="yield")return false;this.work++;if(this.index.size)this.index.delete(this.index.keys().next().value!);else if(this.visited.size)this.visited.delete(this.visited.values().next().value!);else this.assignedUnits.delete(this.assignedUnits.values().next().value!);}
  if(this.index.size||this.visited.size||this.assignedUnits.size)return false;this.closed=true;this.sourceIdentity=undefined;this.sourceLength=undefined;this.result={status:"cancelled"};return true;
 }
}
