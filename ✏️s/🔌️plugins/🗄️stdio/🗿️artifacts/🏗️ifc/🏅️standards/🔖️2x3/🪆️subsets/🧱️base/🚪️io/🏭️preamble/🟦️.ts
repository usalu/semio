import type {Ifc2x3EdmPreamble} from "../../🧬️schema/📸️snapshot/🟦️.ts";
import type {Part21Control} from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🟦️.ts";
const leading=/^[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000\ufeff]+/;
const OPEN="/******************************************************************************************",CLOSE="******************************************************************************************/";
const LABELS:readonly(readonly[keyof Ifc2x3EdmPreamble,string])[]=[["producer","STEP Physical File produced by:"],["module","Module:"],["creationDate","Creation date:"],["host","Host:"],["database","Database:"],["databaseVersion","Database version:"],["databaseCreationDate","Database creation date:"],["schema","Schema:"],["model","Model:"],["modelCreationDate","Model creation date:"],["headerModel","Header model:"],["headerModelCreationDate","Header model creation date:"],["user","EDMuser:"],["group","EDMgroup:"],["license","License ID and type:"],["options","EDMstepFileFactory options:"]];
/** 🏭️ Observe an actual parser comment and retain its complete known EDM metadata under the original exchange control. */
export function parseEdmPreambleComment(source:string,start:number,end:number,control:Part21Control):Ifc2x3EdmPreamble|undefined{
 if(!source.startsWith(OPEN,start))return undefined;
 const values:Partial<Ifc2x3EdmPreamble>={};let lineStart=start,first=true,closed=false;
 for(let at=start;at<=end;at++){if(at<end){control.tick();if(source[at]!=="\n")continue;}let stop=at;while(stop>lineStart&&source[stop-1]==="\r")stop--;const line=source.slice(lineStart,stop);lineStart=at+1;if(first){first=false;if(line!==OPEN)return undefined;continue;}if(line===CLOSE){closed=true;continue;}for(const[key,label]of LABELS){const prefix="* "+label;if(!line.startsWith(prefix))continue;if(values[key]!==undefined)throw Error("ifc2x3: duplicate EDM metadata field");values[key]=line.slice(prefix.length).replace(leading,"");break;}}
 if(values.producer===undefined)return undefined;
 if(!closed||LABELS.some(([key])=>values[key]===undefined))throw Error("ifc2x3: incomplete EDM metadata");
 const result=values as Ifc2x3EdmPreamble;validateEdmPreamble(result,control);return result;
}
/** 🧷️ Keep comment metadata reversible and prevent a field from injecting another STEP token. */
export function validateEdmPreamble(value:Ifc2x3EdmPreamble,control:Part21Control):void{for(const[key]of LABELS){const field=value[key];if(typeof field!=="string"||leading.test(field))throw Error("ifc2x3: invalid EDM metadata field");let previous="";for(const char of field){control.tick();if(char==="\r"||char==="\n"||previous==="*"&&char==="/")throw Error("ifc2x3: invalid EDM metadata field");previous=char;}}}
/** ✍️ Emit owned EDM fields in the native codec's label order without raw source shadow state. */
export function writeEdmPreamble(value:Ifc2x3EdmPreamble,control:Part21Control):string{validateEdmPreamble(value,control);let result=OPEN+"\n";for(const[key,label]of LABELS){control.tick();result+="* "+label.padEnd(31," ")+" "+value[key]+"\n";}return result+CLOSE+"\n";}
