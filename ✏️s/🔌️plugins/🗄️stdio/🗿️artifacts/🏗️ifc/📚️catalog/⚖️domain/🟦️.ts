import * as core from "./core.ts";
import {binary64Value} from "@semio-tech/framework/value/ieee754";
import type {IfcCatalog,IfcTypeExpression} from "../🟦️.ts";
import type {IfcValue} from "../../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type {IfcDomainControl,IfcValueChange,IfcDomainSteps} from "./core.ts";
export {ifcDomainBounds} from "./core.ts";
const values:core.IfcDomainValues<IfcValue>={view:value=>value,*numericValid(value){yield;return value.kind==="integer"?typeof value.value==="bigint"&&value.value>= -9223372036854775808n&&value.value<=9223372036854775807n:value.kind==="real"&&Number.isFinite(binary64Value(value.value));},*numbersEqual(left,right){yield;if(left.kind==="integer"&&right.kind==="integer")return left.value===right.value;if(left.kind==="real"&&right.kind==="real")return binary64Value(left.value)===binary64Value(right.value);const integer=left.kind==="integer"?left:right,real=left.kind==="real"?left:right;if(integer.kind!=="integer"||real.kind!=="real")return false;const number=binary64Value(real.value);return Number.isFinite(number)&&Number.isInteger(number)&&BigInt(number)===integer.value;},*numbersIdentical(left,right){yield;return left.kind==="integer"&&right.kind==="integer"?left.value===right.value:left.kind==="real"&&right.kind==="real"&&left.value.bits===right.value.bits;}};
/** 🧭️ Retain IFC graph validation over borrowed owned-model values. */
export function checkIfcValueDomainSteps(catalog:IfcCatalog,expression:IfcTypeExpression,value:IfcValue,control:core.IfcDomainControl={}):core.IfcDomainSteps{return core.checkIfcValueDomainSteps(catalog,expression,value,values,control);}
/** 🧮️ Retain exact IFC graph equality. */
export function ifcDomainEqualSteps(catalog:IfcCatalog,expression:IfcTypeExpression,left:IfcValue,right:IfcValue,control:core.IfcDomainControl={}):core.IfcDomainSteps<boolean>{return core.ifcDomainEqualSteps(catalog,expression,left,right,values,control);}
/** 🪢️ Retain IFC graph changed-value admission. */
export function checkIfcValueChangeSteps(catalog:IfcCatalog,expression:IfcTypeExpression,before:IfcValue,after:IfcValue,route:readonly number[],operation:core.IfcValueChange,control:core.IfcDomainControl={}):core.IfcDomainSteps{return core.checkIfcValueChangeSteps(catalog,expression,before,after,route,operation,values,control);}
/** 🛂️ Check owned IFC graph values through the borrowed domain interface. */
export function checkIfcValueDomain(catalog:IfcCatalog,expression:IfcTypeExpression,value:IfcValue,control:core.IfcDomainControl={}):void{core.checkIfcValueDomain(catalog,expression,value,values,control);}
/** ⚖️ Compare graph numbers and aggregates using EXPRESS value semantics. */
export function ifcDomainEqual(catalog:IfcCatalog,expression:IfcTypeExpression,left:IfcValue,right:IfcValue,control:core.IfcDomainControl={}):boolean{return core.ifcDomainEqual(catalog,expression,left,right,values,control);}
/** 🪡️ Admit changed graph values while retaining non-worsening aggregate repairs. */
export function checkIfcValueChange(catalog:IfcCatalog,expression:IfcTypeExpression,before:IfcValue,after:IfcValue,route:readonly number[],operation:core.IfcValueChange,control:core.IfcDomainControl={}):void{core.checkIfcValueChange(catalog,expression,before,after,route,operation,values,control);}
