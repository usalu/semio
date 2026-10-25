import fonts from"./🔣️.json"with{type:"json"};
import{preparePrintMaterialV1,withPreparedPrintMaterialV1,type PreparedPrintMaterialV1}from"../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import type{PrintExecutionV1}from"../../📦️artifacts/🎛️operation/🟦️.ts";
/** 🔤️ Describes authored Source and compiler-facing font filenames. */
export interface PrintFontDescriptor{readonly family:string;readonly directory:string;readonly filename:string;readonly texFilename:string}
/** 🪪️ Returns immutable authored font DATA without granting a reader. */
export function printFontDescriptors():readonly PrintFontDescriptor[]{return Object.freeze(fonts.map(font=>Object.freeze({...font})));}
/** 📤️ Explicitly prepares the selected mode's fixed font artifact under actual product and scratch custody. */
export function stagePrintFonts(execution:PrintExecutionV1):Promise<Readonly<{files:number;bytes:number}>>{return preparePrintMaterialV1(execution.operation,"fonts",execution.phaseControl);}
/** 📖️ Retains the actual prepared font reader through its consumer callback. */
export function withPrintFontsV1<T>(execution:PrintExecutionV1,body:(fonts:PreparedPrintMaterialV1)=>Promise<T>):Promise<T>{return withPreparedPrintMaterialV1(execution.operation,"fonts",execution.phaseControl,body);}
