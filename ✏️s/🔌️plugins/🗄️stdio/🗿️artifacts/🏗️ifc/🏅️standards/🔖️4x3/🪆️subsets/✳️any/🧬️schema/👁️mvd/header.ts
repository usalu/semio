import {captureIfcEditOptions} from "../🔺️diff/control.ts";
/** 🪪️ Explicit released-view declarations use the existing reversible header command. */
import type {IfcValue} from "../📸️snapshot/🟦️.ts";
import type {IfcMutation} from "../../../../../4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
import {admitIfcDiffSteps,copyIfcOwnedSteps,runIfcEdit,runIfcEditAsync,IfcEditError,type IfcEditSteps,type IfcApplyOptions} from "../🔺️diff/🟦️.ts";
import type {Ifc4x3MvdId} from "./🟦️.ts";

export interface Ifc4x3MvdSelection {readonly view:Ifc4x3MvdId|null}
export interface Ifc4x3MvdDeclaration {readonly index:number;readonly views:readonly string[];readonly known:readonly Ifc4x3MvdId[];readonly malformed:boolean}
type HeaderMutation=Extract<IfcMutation,{mutation:"setFileDescription"}>;
const released=(value:unknown):value is Ifc4x3MvdId=>value==="ReferenceView"||value==="Alignment-basedReferenceView";
const whitespace=(text:string,index:number):boolean=>text[index]===" "||text.charCodeAt(index)>=9&&text.charCodeAt(index)<=13;
const refuse=(code:string,message:string):never=>{throw new IfcEditError(code,message);};

function* declaration(text:string,index:number):IfcEditSteps<Ifc4x3MvdDeclaration|undefined>{
 let start=0,end=text.length,work=0;
 const tick=()=>({phase:"admitInput" as const,completed:work,total:text.length});
 while(start<end&&whitespace(text,start)){start++;if(++work%256===0)yield tick();}
 if(text.slice(start,start+14)!=="ViewDefinition")return undefined;start+=14;
 if(start<end&&!whitespace(text,start)&&text[start]!=="[")return undefined;
 const malformed=():Ifc4x3MvdDeclaration=>({index,views:[],known:[],malformed:true});
 while(start<end&&whitespace(text,start)){start++;if(++work%256===0)yield tick();}
 while(end>start&&whitespace(text,end-1)){end--;if(++work%256===0)yield tick();}
 if(text[start]!=="["||text[end-1]!=="]")return malformed();
 const views:string[]=[],known:Ifc4x3MvdId[]=[];let from=++start;
 for(let cursor=start;cursor<end;cursor++){
  if(++work%256===0)yield tick();const char=text[cursor];if(char==="["||char==="]"&&cursor!==end-1)return malformed();
  if(char===","||cursor===end-1){let left=from,right=cursor;while(left<right&&whitespace(text,left)){left++;if(++work%256===0)yield tick();}while(right>left&&whitespace(text,right-1)){right--;if(++work%256===0)yield tick();}if(left===right)return malformed();const view=text.slice(left,right);views.push(view);if(released(view))known.push(view);from=cursor+1;}
 }
 return {index,views,known,malformed:false};
}
function* captureHeader(source:readonly IfcValue[],options:IfcApplyOptions):IfcEditSteps<IfcValue[]>{
 const values=yield* copyIfcOwnedSteps(source,options,true);
 if(!Array.isArray(values)||values.length!==2||values[0]?.kind!=="aggregate"||!Array.isArray(values[0].value)||!values[0].value.length||values[1]?.kind!=="string")return refuse("invalidMvdHeader","FILE_DESCRIPTION requires a nonempty description string list and implementation level");
 for(const item of values[0].value){if(item.kind!=="string")return refuse("invalidMvdHeader","description entries require strings");yield{phase:"admitInput",completed:0,total:values[0].value.length};}
 yield* admitIfcDiffSteps({fileDescription:values},options);return values;
}
function* read(values:IfcValue[]):IfcEditSteps<Ifc4x3MvdDeclaration[]>{
 const result:Ifc4x3MvdDeclaration[]=[];const list=(values[0] as Extract<IfcValue,{kind:"aggregate"}>).value;
 for(let index=0;index<list.length;index++){const item=yield* declaration((list[index] as Extract<IfcValue,{kind:"string"}>).value,index);if(item)result.push(item);yield{phase:"admitInput",completed:index+1,total:list.length};}return result;
}
function* readSteps(source:readonly IfcValue[],options:IfcApplyOptions):IfcEditSteps<Ifc4x3MvdDeclaration[]>{return yield* read(yield* captureHeader(source,options));}
function* selectSteps(source:readonly IfcValue[],input:Ifc4x3MvdSelection,options:IfcApplyOptions):IfcEditSteps<HeaderMutation>{
 const selection=yield* copyIfcOwnedSteps(input,options);if(selection===null||typeof selection!=="object"||!Object.hasOwn(selection,"view")||selection.view!==null&&!released(selection.view))return refuse("invalidMvdSelection","select an exact released view or explicitly clear it");
 for(const key in selection)if(Object.hasOwn(selection,key)){if(key!=="view")return refuse("invalidMvdSelection","selection carries an unknown field");yield{phase:"admitInput",completed:0,total:1};}
 const values=yield* captureHeader(source,options),declarations=yield* read(values);for(const item of declarations)if(item.malformed)return refuse("malformedMvdDeclaration",`repair description entry ${item.index} before selecting a released view`);
 const items:IfcValue[]=[],list=(values[0] as Extract<IfcValue,{kind:"aggregate"}>).value;let inserted=false,cursor=0;
 if(!declarations.length&&selection.view!==null){items.push({kind:"string",value:`ViewDefinition [${selection.view}]`});inserted=true;}
 for(let index=0;index<list.length;index++){if(declarations[cursor]?.index===index){cursor++;if(!inserted&&selection.view!==null){items.push({kind:"string",value:`ViewDefinition [${selection.view}]`});inserted=true;}}else items.push(list[index]!);yield{phase:"admitInput",completed:index+1,total:list.length};}
 if(!items.length)items.push({kind:"string",value:""});values[0]={kind:"aggregate",value:items};return{mutation:"setFileDescription",values};
}
/** 🔍️ Read declared names verbatim; known scope identifiers do not assert conformance. */
export function ifc4x3MvdDeclarations(values:readonly IfcValue[],options:IfcApplyOptions={}):Ifc4x3MvdDeclaration[]{options=captureIfcEditOptions(options);return runIfcEdit(readSteps(values,options),options);}
/** 🧵️ Read imported declarations with bounded cancellation checkpoints. */
export function ifc4x3MvdDeclarationsAsync(values:readonly IfcValue[],options:IfcApplyOptions={}):Promise<Ifc4x3MvdDeclaration[]>{options=captureIfcEditOptions(options);return runIfcEditAsync(readSteps(values,options),options);}
/** 📝️ Plan one owned header command while preserving unrelated metadata. */
export function ifc4x3MvdHeaderMutation(values:readonly IfcValue[],selection:Ifc4x3MvdSelection,options:IfcApplyOptions={}):HeaderMutation{options=captureIfcEditOptions(options);return runIfcEdit(selectSteps(values,selection,options),options);}
/** ⏳️ Cooperatively plan the same command for central history publication. */
export function ifc4x3MvdHeaderMutationAsync(values:readonly IfcValue[],selection:Ifc4x3MvdSelection,options:IfcApplyOptions={}):Promise<HeaderMutation>{options=captureIfcEditOptions(options);return runIfcEditAsync(selectSteps(values,selection,options),options);}
