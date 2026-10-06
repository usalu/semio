/** 🔢️ Exact word interfaces for owned snapshot numeric text. */
import {binary64,binary32,binary64Value,binary32Value,parseBinary64,parseBinary32,type Binary64,type Binary32} from "../../../../../🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type {Binary64,Binary32} from "../../../../../🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
function nan64(bits:bigint):boolean{return(bits&0x7ff0000000000000n)===0x7ff0000000000000n&&(bits&0xfffffffffffffn)!==0n}
function nan32(bits:number):boolean{return(bits&0x7f800000)===0x7f800000&&(bits&0x7fffff)!==0}
function number(text:string):number{if(typeof text!=="string"||! /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(text))throw Error("invalid numeric payload literal");return Number(text)}
/** 📤️ Formats a binary64 identity without evaluating its NaN payload. */
export function formatBinary64Literal(value:Binary64):string{const bits=parseBinary64(value).bits;if(nan64(bits))return"nan64_"+bits.toString(16).padStart(16,"0");if(bits===0x7ff0000000000000n)return"inf";if(bits===0xfff0000000000000n)return"-inf";if(bits===0x8000000000000000n)return"-0";return binary64Value({bits}).toString()}
/** 📥️ Admits exact binary64 words and ordinary numeric payload text. */
export function parseBinary64Literal(text:string):Binary64{if(typeof text!=="string")throw Error("binary64 literal requires text");if(text.startsWith("nan64_")){if(!/^nan64_[0-9a-fA-F]{16}$/.test(text))throw Error("invalid binary64 NaN word");const bits=BigInt("0x"+text.slice(6));if(!nan64(bits))throw Error("binary64 NaN literal requires a NaN word");return{bits}}if(text==="nan")return{bits:0x7ff8000000000000n};if(text==="inf")return{bits:0x7ff0000000000000n};if(text==="-inf")return{bits:0xfff0000000000000n};return binary64(number(text))}
/** 📤️ Formats a complete binary32 identity before any numeric widening. */
export function formatBinary32Literal(value:Binary32):string{const bits=parseBinary32(value).bits;if(nan32(bits))return"nan32_"+bits.toString(16).padStart(8,"0");if(bits===0x7f800000)return"inf";if(bits===0xff800000)return"-inf";if(bits===0x80000000)return"-0";return binary32Value({bits}).toString()}
/** 📥️ Admits exact binary32 words and native-width numeric payload text. */
export function parseBinary32Literal(text:string):Binary32{if(typeof text!=="string")throw Error("binary32 literal requires text");if(text.startsWith("nan32_")){if(!/^nan32_[0-9a-fA-F]{8}$/.test(text))throw Error("invalid binary32 NaN word");const bits=parseInt(text.slice(6),16);if(!nan32(bits))throw Error("binary32 NaN literal requires a NaN word");return{bits}}if(text==="nan")return{bits:0x7fc00000};if(text==="inf")return{bits:0x7f800000};if(text==="-inf")return{bits:0xff800000};return binary32(number(text))}
