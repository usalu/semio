import {captureIfcEditOptions} from "../🔺️diff/control.ts";
/** 🧬️ Sparse commands and revision-checked events share one bounded transaction authority. */
import type {IfcSnapshot,IfcValue,IfcEntity} from "../📸️snapshot/🟦️.ts";
import type {IfcMutation} from "../../../../../4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
import {applyIfcDiff,applyIfcDiffAsync,prepareIfcDiffSteps,captureIfcGraphSteps,validateIfcNativeShapeSteps,validateIfcGraphSteps,admitIfcDiffSteps,copyIfcOwnedSteps,equalIfcValueSteps,runIfcEdit,runIfcEditAsync,checkIfcCancellation,IfcEditError,type IfcDiff,type IfcEntityDiff,type IfcApplyOptions,type IfcEditSteps,type IfcDiffTransaction} from "../🔺️diff/🟦️.ts";
import {captureIfcTemplateSteps,ifcTemplateDiffSteps,validateIfcTemplateTransaction,type IfcTemplateApplication} from "../🏷️templates/✍️apply/🟦️.ts";
export type Ifc4x3Mutation={mutation:"applyTemplate";application:IfcTemplateApplication}|{mutation:"applyDiff";diff:IfcDiff}|IfcMutation|{mutation:"setEntityValue";id:bigint;index:number;path:readonly number[];value:IfcValue};
const refusal=(code:string,message:string):never=>{throw new IfcEditError(code,message);};
const position=(index:number,length:number,insertion=false):void=>{if(!Number.isSafeInteger(index)||index<0||index>length||(!insertion&&index===length))refusal("invalidIndex",`index ${index} is outside ${length} items`);};
function* findEntity(snapshot:IfcSnapshot,id:bigint):IfcEditSteps<{entity:IfcEntity;ordinal:number}>{for(let i=0;i<snapshot.entities.length;i++){if(snapshot.entities[i]!.id===id)return{entity:snapshot.entities[i]!,ordinal:i};yield{phase:"admitInput",completed:i+1,total:snapshot.entities.length};}return refusal("missingEntity",`entity #${id} does not exist`);}
function* replaceValue(root:IfcValue,path:readonly number[],replacement:IfcValue,options:IfcApplyOptions):IfcEditSteps<IfcValue>{
 if(path.length>(options.maxDepth??512))refusal("depthLimit","nested edit exceeds the value depth limit");const parents:{value:IfcValue;items:IfcValue[];index:number}[]=[];let value=root;
 for(let i=0;i<path.length;i++){const items=value.kind==="aggregate"?value.value:value.kind==="typedValue"?value.value.items:refusal("invalidPath","nested path traverses a scalar");position(path[i]!,items.length);parents.push({value,items,index:path[i]!});value=items[path[i]!]!;yield{phase:"admitInput",completed:i+1,total:path.length};}
 value=replacement;for(let i=parents.length-1;i>=0;i--){const parent=parents[i]!,items:IfcValue[]=[];for(let j=0;j<parent.items.length;j++){items.push(j===parent.index?value:parent.items[j]!);yield{phase:"admitInput",completed:j+1,total:parent.items.length};}value=parent.value.kind==="aggregate"?{kind:"aggregate",value:items}:{kind:"typedValue",value:{name:parent.value.kind==="typedValue"?parent.value.value.name:"",items}};}return value;
}
function* mutationPlan(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<IfcDiff>{
 const field=(id:bigint,diff:IfcEntityDiff):IfcDiff=>({entities:{modified:[{id,diff}]}});let diff:IfcDiff,before:unknown,after:unknown;const target="id"in mutation?yield* findEntity(snapshot,mutation.id):undefined;
 switch(mutation.mutation){
  case"applyTemplate":diff=yield* ifcTemplateDiffSteps(snapshot,mutation.application,options);break;
  case"applyDiff":diff=mutation.diff;break;
  case"setFileDescription":diff={fileDescription:mutation.values};before=snapshot.header.fileDescription;after=mutation.values;break;
  case"setFileName":diff={fileName:mutation.values};before=snapshot.header.fileName;after=mutation.values;break;
  case"setFileSchema":diff={fileSchema:mutation.values};before=snapshot.header.fileSchema;after=mutation.values;break;
  case"insertEntity":position(mutation.index,snapshot.entities.length,true);for(let i=0;i<snapshot.entities.length;i++){if(snapshot.entities[i]!.id===mutation.entity.id)refusal("duplicateTarget","inserted entity identity already exists");yield{phase:"admitInput",completed:i+1,total:snapshot.entities.length};}diff={entities:{added:[{index:mutation.index,entity:mutation.entity}]}};break;
  case"removeEntity":diff={entities:{removed:[{id:mutation.id,index:target!.ordinal}]}};break;
  case"setEntityName":diff=field(mutation.id,{name:mutation.name});before=target!.entity.name;after=mutation.name;break;
  case"setEntityComplex":diff=field(mutation.id,{complex:mutation.complex});before=target!.entity.complex;after=mutation.complex;break;
  case"setEntityArg":case"setEntityValue":{position(mutation.index,target!.entity.args.length);yield* admitIfcDiffSteps(field(mutation.id,{args:{modified:[{index:mutation.index,value:mutation.value}]}}),options);before=target!.entity.args[mutation.index]!;after=mutation.mutation==="setEntityValue"?yield* replaceValue(before as IfcValue,mutation.path,mutation.value,options):mutation.value;diff=field(mutation.id,{args:{modified:[{index:mutation.index,value:after as IfcValue}]}});break;}
  case"insertEntityArg":position(mutation.index,target!.entity.args.length,true);diff=field(mutation.id,{args:{added:[{index:mutation.index,value:mutation.value}]}});break;
  case"removeEntityArg":position(mutation.index,target!.entity.args.length);diff=field(mutation.id,{args:{removed:[mutation.index]}});break;
  default:return refusal("unknownMutation","unknown IFC mutation");
 }
 yield* admitIfcDiffSteps(diff,options);if(before!==undefined&&(yield* equalIfcValueSteps(before,after)))return{};return diff;
}
interface IfcCommandTransaction extends IfcDiffTransaction {mutation:Ifc4x3Mutation}
function proposal(mutation:Ifc4x3Mutation):IfcDiff{const field=(id:bigint,diff:IfcEntityDiff):IfcDiff=>({entities:{modified:[{id,diff}]}});switch(mutation.mutation){
 case"applyTemplate":return{};case"applyDiff":return mutation.diff;
 case"setFileDescription":return{fileDescription:mutation.values};case"setFileName":return{fileName:mutation.values};case"setFileSchema":return{fileSchema:mutation.values};
 case"insertEntity":return{entities:{added:[{index:mutation.index,entity:mutation.entity}]}};case"removeEntity":return{entities:{removed:[{id:mutation.id,index:0}]}};
 case"setEntityName":return field(mutation.id,{name:mutation.name});case"setEntityComplex":return field(mutation.id,{complex:mutation.complex});
 case"setEntityArg":case"setEntityValue":return field(mutation.id,{args:{modified:[{index:mutation.index,value:mutation.value}]}});
 case"insertEntityArg":return field(mutation.id,{args:{added:[{index:mutation.index,value:mutation.value}]}});case"removeEntityArg":return field(mutation.id,{args:{removed:[mutation.index]}});
 default:return refusal("unknownMutation","unknown IFC mutation");
}}
function* captureMutation(source:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<Ifc4x3Mutation>{const mutation=yield* copyIfcOwnedSteps(source,options,true);yield* validateIfcNativeShapeSteps(mutation,"mutation",options);if(mutation.mutation==="applyTemplate"){mutation.application=yield* captureIfcTemplateSteps(mutation.application,options);return mutation;}if(mutation.mutation==="setEntityValue"){if(!Array.isArray(mutation.path)||mutation.path.length>(options.maxDepth??512))refusal("invalidPath","nested paths require bounded index arrays");for(const index of mutation.path){if(!Number.isSafeInteger(index)||index<0)refusal("invalidIndex","path coordinates require nonnegative safe integers");yield{phase:"admitInput",completed:0,total:mutation.path.length};}}yield* admitIfcDiffSteps(proposal(mutation),options);return mutation;}
function* mutationDiffSteps(source:IfcSnapshot,input:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<IfcDiff>{const snapshot=yield* captureIfcGraphSteps(source,"validateBase",options),mutation=yield* captureMutation(input,options),diff=yield* mutationPlan(snapshot,mutation,options);return diff;}
/** 📐️ Plans an owned sparse diff directly from the command target. */
export function ifcMutationDiff(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):IfcDiff{options=captureIfcEditOptions(options);return runIfcEdit(mutationDiffSteps(snapshot,mutation,options),options);}
/** 📝️ Cooperatively plans and copies an admitted command. */
export function ifcMutationDiffAsync(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Promise<IfcDiff>{options=captureIfcEditOptions(options);return runIfcEditAsync(mutationDiffSteps(snapshot,mutation,options),options);}
function* commandSteps(snapshot:IfcSnapshot,source:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<IfcCommandTransaction>{const mutation=yield* captureMutation(source,options),transaction=yield* prepareIfcDiffSteps(snapshot,owned=>mutationPlan(owned,mutation,options),options);return{...transaction,mutation};}
/** ▶️ Executes one central graph transaction. */
export function applyIfcMutation(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):IfcSnapshot{options=captureIfcEditOptions(options);const transaction=runIfcEdit(commandSteps(snapshot,mutation,options),options);if(transaction.mutation.mutation==="applyTemplate")refusal("asyncRequired","Template authoring requires async semantic admission");return transaction.snapshot;}
/** 🧵️ Cooperatively executes one transaction without intermediate publication. */
export async function applyIfcMutationAsync(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Promise<IfcSnapshot>{options=captureIfcEditOptions(options);const transaction=await runIfcEditAsync(commandSteps(snapshot,mutation,options),options);if(transaction.mutation.mutation==="applyTemplate")await validateIfcTemplateTransaction(transaction,options);checkIfcCancellation(options);return transaction.snapshot;}
function* inverseMutationSteps(source:IfcSnapshot,input:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<Ifc4x3Mutation>{const snapshot=yield* captureIfcGraphSteps(source,"validateBase",options),mutation=yield* captureMutation(input,options);if(mutation.mutation==="applyTemplate"||mutation.mutation==="applyDiff"){const transaction=yield* prepareIfcDiffSteps(snapshot,owned=>mutationPlan(owned,mutation,options),options);return{mutation:"applyDiff",diff:transaction.inverse};}yield* mutationPlan(snapshot,mutation,options);const target="id"in mutation?yield* findEntity(snapshot,mutation.id):undefined;let result:Ifc4x3Mutation;
 switch(mutation.mutation){
  case"setFileDescription":result={mutation:mutation.mutation,values:snapshot.header.fileDescription};break;
  case"setFileName":result={mutation:mutation.mutation,values:snapshot.header.fileName};break;
  case"setFileSchema":result={mutation:mutation.mutation,values:snapshot.header.fileSchema};break;
  case"insertEntity":result={mutation:"removeEntity",id:mutation.entity.id};break;
  case"removeEntity":result={mutation:"insertEntity",index:target!.ordinal,entity:target!.entity};break;
  case"setEntityName":result={mutation:mutation.mutation,id:mutation.id,name:target!.entity.name};break;
  case"setEntityComplex":result={mutation:mutation.mutation,id:mutation.id,complex:target!.entity.complex!};break;
  case"setEntityArg":case"setEntityValue":case"removeEntityArg":result={mutation:mutation.mutation==="removeEntityArg"?"insertEntityArg":"setEntityArg",id:mutation.id,index:mutation.index,value:target!.entity.args[mutation.index]!};break;
  case"insertEntityArg":result={mutation:"removeEntityArg",id:mutation.id,index:mutation.index};break;
  default:return refusal("unknownMutation","unknown IFC mutation");
 }return result;
}
/** 🔁️ Restores the original target through an admitted inverse command. */
export function inverseIfcMutation(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Ifc4x3Mutation{options=captureIfcEditOptions(options);return runIfcEdit(inverseMutationSteps(snapshot,mutation,options),options);}
/** 🔙️ Cooperatively prepares an owned inverse command. */
export function inverseIfcMutationAsync(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Promise<Ifc4x3Mutation>{options=captureIfcEditOptions(options);return runIfcEditAsync(inverseMutationSteps(snapshot,mutation,options),options);}
export interface IfcEditEvent {revision:number;kind:"mutation"|"undo"|"redo";scope:"persistedLocal"|"persistedShared";diff:IfcDiff;inverse:IfcDiff;mutation?:Ifc4x3Mutation}
interface IfcHistoryEntry {diff:IfcDiff;inverse:IfcDiff}
interface Prepared {transaction:IfcDiffTransaction;event:IfcEditEvent;returned:IfcEditEvent;entry:IfcHistoryEntry}
/** ⏪️ Private owned state changes only after complete preparation and a final revision check. */
export class IfcEditHistory {
 #snapshot:IfcSnapshot;#events:IfcEditEvent[]=[];#undo:IfcHistoryEntry[]=[];#redo:IfcHistoryEntry[]=[];
 private constructor(snapshot:IfcSnapshot,readonly scope:IfcEditEvent["scope"]){this.#snapshot=snapshot;}
 static createSync(snapshot:IfcSnapshot,scope:IfcEditEvent["scope"]="persistedLocal",options:IfcApplyOptions={}):IfcEditHistory{options=captureIfcEditOptions(options);return new IfcEditHistory(applyIfcDiff(snapshot,{},options),scope);}
 static async create(snapshot:IfcSnapshot,scope:IfcEditEvent["scope"]="persistedLocal",options:IfcApplyOptions={}):Promise<IfcEditHistory>{options=captureIfcEditOptions(options);return new IfcEditHistory(await applyIfcDiffAsync(snapshot,{},options),scope);}
 get snapshot():IfcSnapshot{return this.readSnapshotSync();}
 get revision():number{return this.#events.length;}
 get events():readonly IfcEditEvent[]{return this.readEventsSync();}
 *#readSnapshot(options:IfcApplyOptions):IfcEditSteps<IfcSnapshot>{const snapshot=this.#snapshot;yield* validateIfcGraphSteps(snapshot,"validateBase",options);return yield* copyIfcOwnedSteps(snapshot,options);}
 readSnapshotSync(options:IfcApplyOptions={}):IfcSnapshot{options=captureIfcEditOptions(options);return runIfcEdit(this.#readSnapshot(options),options);}
 readSnapshot(options:IfcApplyOptions={}):Promise<IfcSnapshot>{options=captureIfcEditOptions(options);return runIfcEditAsync(this.#readSnapshot(options),options);}
 readEventsSync(options:IfcApplyOptions={}):IfcEditEvent[]{options=captureIfcEditOptions(options);return runIfcEdit(copyIfcOwnedSteps(this.#events,options),options);}
 readEvents(options:IfcApplyOptions={}):Promise<IfcEditEvent[]>{options=captureIfcEditOptions(options);return runIfcEditAsync(copyIfcOwnedSteps(this.#events,options),options);}
 #check(revision:number):void{if(revision!==this.revision)refusal("staleRevision",`expected revision ${this.revision}, received ${revision}`);}
 *#prepare(kind:IfcEditEvent["kind"],revision:number,options:IfcApplyOptions,mutation?:Ifc4x3Mutation):IfcEditSteps<Prepared|undefined>{
  this.#check(revision);const previous=kind==="undo"?this.#undo.at(-1):kind==="redo"?this.#redo.at(-1):undefined;if(kind!=="mutation"&&!previous)return undefined;const transaction=kind==="mutation"?yield* commandSteps(this.#snapshot,mutation!,options):yield* prepareIfcDiffSteps(this.#snapshot,function*(){return kind==="undo"?previous!.inverse:previous!.diff;},options);if(kind==="mutation"&&!Object.keys(transaction.diff).length)return undefined;
  const event:IfcEditEvent={revision:revision+1,kind,scope:this.scope,diff:transaction.diff,inverse:transaction.inverse,...(kind==="mutation"?{mutation:(transaction as IfcCommandTransaction).mutation}:{})};const returned=yield* copyIfcOwnedSteps(event,options);return{transaction,event,returned,entry:previous??{diff:transaction.diff,inverse:transaction.inverse}};
 }
 #publish(kind:IfcEditEvent["kind"],revision:number,prepared:Prepared|undefined,options:IfcApplyOptions):IfcEditEvent|undefined{
  checkIfcCancellation(options);this.#check(revision);if(!prepared)return undefined;this.#snapshot=prepared.transaction.snapshot;this.#events.push(prepared.event);if(kind==="mutation"){this.#undo.push(prepared.entry);this.#redo=[];}else if(kind==="undo"){this.#undo.pop();this.#redo.push(prepared.entry);}else{this.#redo.pop();this.#undo.push(prepared.entry);}return prepared.returned;
 }
 execute(mutation:Ifc4x3Mutation,revision:number,options:IfcApplyOptions={}):IfcEditEvent|undefined{options=captureIfcEditOptions(options);const prepared=runIfcEdit(this.#prepare("mutation",revision,options,mutation),options);if(prepared?.event.mutation?.mutation==="applyTemplate")refusal("asyncRequired","Template authoring requires async semantic admission");return this.#publish("mutation",revision,prepared,options);}
 async executeAsync(mutation:Ifc4x3Mutation,revision:number,options:IfcApplyOptions={}):Promise<IfcEditEvent|undefined>{options=captureIfcEditOptions(options);const prepared=await runIfcEditAsync(this.#prepare("mutation",revision,options,mutation),options);if(prepared?.event.mutation?.mutation==="applyTemplate")await validateIfcTemplateTransaction(prepared.transaction,options);return this.#publish("mutation",revision,prepared,options);}
 undo(revision:number,options:IfcApplyOptions={}):IfcEditEvent|undefined{options=captureIfcEditOptions(options);return this.#publish("undo",revision,runIfcEdit(this.#prepare("undo",revision,options),options),options);}
 async undoAsync(revision:number,options:IfcApplyOptions={}):Promise<IfcEditEvent|undefined>{options=captureIfcEditOptions(options);return this.#publish("undo",revision,await runIfcEditAsync(this.#prepare("undo",revision,options),options),options);}
 redo(revision:number,options:IfcApplyOptions={}):IfcEditEvent|undefined{options=captureIfcEditOptions(options);return this.#publish("redo",revision,runIfcEdit(this.#prepare("redo",revision,options),options),options);}
 async redoAsync(revision:number,options:IfcApplyOptions={}):Promise<IfcEditEvent|undefined>{options=captureIfcEditOptions(options);return this.#publish("redo",revision,await runIfcEditAsync(this.#prepare("redo",revision,options),options),options);}
}
