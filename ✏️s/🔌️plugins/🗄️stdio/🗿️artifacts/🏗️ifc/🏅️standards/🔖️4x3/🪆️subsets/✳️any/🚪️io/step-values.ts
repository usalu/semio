/** 🔢️ Own exact numeric primitives and canonical STEP scalar reference formatting. */
import type {IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {IfcStepError} from "./control.ts";
const numeric=new DataView(new ArrayBuffer(8));

/** 🧮️ Capture a finite IFC real in the framework's exact binary64 representation. */
export function ifcReal(value:number):IfcValue{if(!Number.isFinite(value))throw new IfcStepError("step.real","Finite real required");numeric.setFloat64(0,value);return{kind:"real",value:{bits:numeric.getBigUint64(0)}};}
/** 📐️ Read a numeric argument while refusing silent integer precision loss. */
export function ifcNumber(value:IfcValue|undefined):number{
 if(value?.kind==="real"){if(typeof value.value.bits!=="bigint"||value.value.bits<0n||value.value.bits>18446744073709551615n||(value.value.bits&0x7ff0000000000000n)===0x7ff0000000000000n)throw new IfcStepError("exchange.number","Finite binary64 argument required");numeric.setBigUint64(0,value.value.bits);return numeric.getFloat64(0);}
 if(value?.kind==="integer"&&Number.isSafeInteger(Number(value.value)))return Number(value.value);throw new IfcStepError("exchange.number","Finite numeric argument required");
}
/** 🧬️ Validate canonical binary text with unused high-bit count and zero padding. */
export function validIfcStepBinary(value:string):boolean{if(!/^[0-3][0-9A-F]*$/.test(value))return false;const unused=Number(value[0]);return value.length===1?unused===0:parseInt(value[1]!,16)<2**(4-unused);}
function name(value:string):string{if(!/^[A-Za-z_][A-Za-z_0-9]*$/.test(value))throw new IfcStepError("step.identifier","Invalid identifier");return value.toUpperCase();}
function text(value:string):string{if(!value.isWellFormed())throw new IfcStepError("step.string","Unpaired Unicode surrogate");let result="'";for(const char of value){const code=char.codePointAt(0)!;result+=char==="'"?"''":char==="\\"?"\\\\":code>=32&&code<=126?char:`\\X${code<=0xffff?"2":"4"}\\${code.toString(16).toUpperCase().padStart(code<=0xffff?4:8,"0")}\\X0\\`;}return result+"'";}

/** 🔤️ Format one borrowed value as a synchronous reference utility; document export uses bounded chunks. */
export function writeIfcStepValue(value:IfcValue,depth=0):string{
 if(depth>256)throw new IfcStepError("step.depth","Argument nesting limit exceeded");const args=(items:IfcValue[])=>`(${items.map(item=>writeIfcStepValue(item,depth+1)).join(",")})`;
 switch(value.kind){
 case"unset":return"$";case"derived":return"*";case"integer":if(value.value< -9223372036854775808n||value.value>9223372036854775807n)throw new IfcStepError("step.integer","Integer outside signed 64-bit range");return value.value.toString();
 case"reference":if(value.value<=0n||value.value>18446744073709551615n)throw new IfcStepError("step.id","Invalid reference id");return`#${value.value}`;
 case"string":return text(value.value);case"binary":if(!validIfcStepBinary(value.value))throw new IfcStepError("step.binary","Invalid canonical binary value");return`"${value.value}"`;
 case"enum":return`.${name(value.value)}.`;case"aggregate":return args(value.value);case"typedValue":return name(value.value.name)+args(value.value.items);
 case"real":{const number=ifcNumber(value);if(Object.is(number,-0))return"-0.";const string=number.toString().toUpperCase();return string.includes(".")?string:string.includes("E")?string.replace("E",".E"):string+".";}
 }
}
