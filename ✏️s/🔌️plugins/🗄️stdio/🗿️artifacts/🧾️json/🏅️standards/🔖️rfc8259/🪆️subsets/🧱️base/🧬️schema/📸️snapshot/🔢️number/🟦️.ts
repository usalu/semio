/** 🔢️ Borrowed numeric meaning leaves the owned primitive lexeme unchanged. */
import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
/** 🧮️ RFC syntax and its optional finite binary64 query representation. */
export interface JsonNumberMeaning{readonly valid:boolean;readonly numeric:number|null}
/** 🔬️ Classify an authored number string with bounded cancellation and no copying. */
export async function jsonNumberMeaning(lexeme:string,control?:NativeDecodeControl):Promise<JsonNumberMeaning>{
 const work=lexeme.length>65536?(control??new NativeDecodeControl(0,()=>true)):undefined;
 const classify=async():Promise<JsonNumberMeaning>=>{
 let cursor=0,advanced=0;
 const checkpoint=async():Promise<void>=>{if(work)await work.advance(cursor-advanced);advanced=cursor;};
 const digits=async():Promise<boolean>=>{const start=cursor;while(cursor<lexeme.length&&lexeme.charCodeAt(cursor)>=48&&lexeme.charCodeAt(cursor)<=57){cursor++;if(cursor%65536===0)await checkpoint();}return cursor>start;};
 if(lexeme[cursor]==="-")cursor++;
 if(lexeme[cursor]==="0")cursor++;
 else if(lexeme.charCodeAt(cursor)>=49&&lexeme.charCodeAt(cursor)<=57){if(!await digits())return {valid:false,numeric:null};}else return {valid:false,numeric:null};
 if(lexeme[cursor]==="."){cursor++;if(!await digits())return {valid:false,numeric:null};}
 if(lexeme[cursor]==="e"||lexeme[cursor]==="E"){cursor++;if(lexeme[cursor]==="+"||lexeme[cursor]==="-")cursor++;if(!await digits())return {valid:false,numeric:null};}
 if(cursor!==lexeme.length)return {valid:false,numeric:null};
 await checkpoint();const value=Number(lexeme);return {valid:true,numeric:Number.isFinite(value)?value===0?0:value:null};
 };
 return work?work.scopedStage(async()=>{await work.beginStage(lexeme.length);return classify();}):classify();
}
