/** 🌈️ Handwritten shading functions, mesh octets and tiling or shading patterns. */
import type { PdfShading,PdfShadingKind,PdfPattern,PdfPatternKind } from "../../../🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,pdfBoolean,type Binary64,type SqliteRow } from "../../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../../🧩️cos/🟦️.ts";
import { writePdfColorSpace,readPdfColorSpace,writePdfFunction,readPdfFunction } from "../../🌈️color/🟦️.ts";
import { writePdfOperations,readPdfOperations } from "../../🖋️content/🟦️.ts";
import { artifactSqliteInteger,artifactSqliteText } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

function cells(value:readonly Binary64[]|null|undefined,width:number):(Binary64|null)[]{if(value==null)return Array(width).fill(null);if(value.length!==width)throw new Error("PDF tuple has a different native width");return [...value];}
function tuple(reader:PdfReader,table:string,row:SqliteRow,start:number,width:number):Binary64[]|null{const present=!reader.isNull(table,row,start);const result:Binary64[]=[];for(let column=start;column<start+width;column++){if(reader.isNull(table,row,column)===present)throw new Error("PDF optional tuple must be entirely present or absent");if(present)result.push(reader.real(table,row,column));}return present?result:null;}
function integer(value:number|null|undefined):bigint|null{return value==null?null:pdfInteger(value);}
function optionalNumber(row:SqliteRow,column:number):number|null{return row.values[column]===null?null:pdfNumber(row,column);}
async function writeSequence(out:PdfProjection,table:string,key:bigint,values:readonly Binary64[]):Promise<void>{out.checkRowsAdditional(values.length);for(const[ordinal,value]of values.entries())await out.insert(table,[key,BigInt(ordinal),value]);}
async function readSequence(reader:PdfReader,table:string,key:bigint):Promise<Binary64[]>{const values:Binary64[]=[];for(const child of await reader.children(table,1,2,key)){const row=await reader.take(table,child.rowid,4);values.push(reader.real(table,row,3));}return values;}
const identity:[Binary64,Binary64,Binary64,Binary64,Binary64,Binary64]=[{bits:0x3ff0000000000000n},{bits:0n},{bits:0n},{bits:0x3ff0000000000000n},{bits:0n},{bits:0n}];

/** 🌈️ Project the exact shading variant and every optional or ordered coordinate. */
export async function writePdfShading(out:PdfProjection,value:PdfShading):Promise<bigint>{
  const color=await writePdfColorSpace(out,value.colorSpace);const extra=await writePdfDictionary(out,value.extra??[]);const variant=value.kind;
  if(!["functionBased","axial","radial","mesh"].includes(variant.kind))throw new Error("Unknown PDF shading kind");
  const key=await out.insert("pdf_shading",[value.id,color,variant.kind,value.background==null?0n:1n,...cells(value.bbox,4),value.antiAlias?1n:0n,extra]);
  if(value.background!=null)await writeSequence(out,"pdf_shading_background",key,value.background);
  switch(variant.kind){
    case "functionBased":await out.insert("pdf_function_shading",[...cells(variant.domain,4),...cells(variant.matrix,6),await writePdfFunction(out,variant.function)],key);break;
    case "axial":await out.insert("pdf_axial_shading",[...cells(variant.coords,4),...cells(variant.domain,2),await writePdfFunction(out,variant.function),variant.extend[0]?1n:0n,variant.extend[1]?1n:0n],key);break;
    case "radial":await out.insert("pdf_radial_shading",[...cells(variant.coords,6),...cells(variant.domain,2),await writePdfFunction(out,variant.function),variant.extend[0]?1n:0n,variant.extend[1]?1n:0n],key);break;
    case "mesh":await out.insert("pdf_mesh_shading",[pdfInteger(variant.shadingType),pdfInteger(variant.bitsPerCoordinate),pdfInteger(variant.bitsPerComponent),integer(variant.bitsPerFlag),integer(variant.verticesPerRow),variant.function==null?null:await writePdfFunction(out,variant.function),await out.bytes(variant.data)],key);await writeSequence(out,"pdf_mesh_decode",key,variant.decode);break;
  }
  return key;
}
/** 📥️ Restore native widths and reject partially populated optional tuples. */
export async function readPdfShading(reader:PdfReader,key:bigint):Promise<PdfShading>{
  const row=await reader.take("pdf_shading",key,11);let kind:PdfShadingKind;
  switch(artifactSqliteText(row,3)){
    case "functionBased":{const value=await reader.take("pdf_function_shading",key,12);kind={kind:"functionBased",domain:tuple(reader,"pdf_function_shading",value,1,4)as [Binary64,Binary64,Binary64,Binary64]|null,matrix:tuple(reader,"pdf_function_shading",value,5,6)as [Binary64,Binary64,Binary64,Binary64,Binary64,Binary64]|null,function:await readPdfFunction(reader,artifactSqliteInteger(value,11))};break;}
    case "axial":{const value=await reader.take("pdf_axial_shading",key,10);const real=(column:number)=>reader.real("pdf_axial_shading",value,column);kind={kind:"axial",coords:[real(1),real(2),real(3),real(4)],domain:tuple(reader,"pdf_axial_shading",value,5,2)as [Binary64,Binary64]|null,function:await readPdfFunction(reader,artifactSqliteInteger(value,7)),extend:[pdfBoolean(value,8),pdfBoolean(value,9)]};break;}
    case "radial":{const value=await reader.take("pdf_radial_shading",key,12);const real=(column:number)=>reader.real("pdf_radial_shading",value,column);kind={kind:"radial",coords:[real(1),real(2),real(3),real(4),real(5),real(6)],domain:tuple(reader,"pdf_radial_shading",value,7,2)as [Binary64,Binary64]|null,function:await readPdfFunction(reader,artifactSqliteInteger(value,9)),extend:[pdfBoolean(value,10),pdfBoolean(value,11)]};break;}
    case "mesh":{const value=await reader.take("pdf_mesh_shading",key,8);kind={kind:"mesh",shadingType:pdfNumber(value,1),bitsPerCoordinate:pdfNumber(value,2),bitsPerComponent:pdfNumber(value,3),bitsPerFlag:optionalNumber(value,4),verticesPerRow:optionalNumber(value,5),decode:await readSequence(reader,"pdf_mesh_decode",key),function:value.values[6]===null?null:await readPdfFunction(reader,artifactSqliteInteger(value,6)),data:await reader.bytes(value,7)};break;}
    default:throw new Error("Unknown PDF shading kind");
  }
  return {id:await reader.text(row,1),colorSpace:await readPdfColorSpace(reader,artifactSqliteInteger(row,2)),kind,background:pdfBoolean(row,4)?await readSequence(reader,"pdf_shading_background",key):null,bbox:tuple(reader,"pdf_shading",row,5,4)as [Binary64,Binary64,Binary64,Binary64]|null,antiAlias:pdfBoolean(row,9),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,10))};
}

/** 🧱️ Project full tiling content or named shading references and exact matrices. */
export async function writePdfPattern(out:PdfProjection,value:PdfPattern):Promise<bigint>{
  const extra=await writePdfDictionary(out,value.extra??[]);const variant=value.kind;if(variant.kind!=="tiling"&&variant.kind!=="shading")throw new Error("Unknown PDF pattern kind");
  const key=await out.insert("pdf_pattern",[value.id,variant.kind,...cells(value.matrix??identity,6),extra]);
  switch(variant.kind){case "tiling":await out.insert("pdf_tiling_pattern",[pdfInteger(variant.paintType),pdfInteger(variant.tilingType),...cells(variant.bbox,4),variant.xStep,variant.yStep,await writePdfOperations(out,variant.content)],key);break;case "shading":await out.insert("pdf_shading_pattern",[variant.shading,variant.extGState??null],key);break;}
  return key;
}
/** 📥️ Restore only the entity owned by the declared pattern variant. */
export async function readPdfPattern(reader:PdfReader,key:bigint):Promise<PdfPattern>{
  const row=await reader.take("pdf_pattern",key,10);let kind:PdfPatternKind;
  switch(artifactSqliteText(row,2)){case "tiling":{const value=await reader.take("pdf_tiling_pattern",key,10);const real=(column:number)=>reader.real("pdf_tiling_pattern",value,column);kind={kind:"tiling",paintType:pdfNumber(value,1),tilingType:pdfNumber(value,2),bbox:[real(3),real(4),real(5),real(6)],xStep:real(7),yStep:real(8),content:await readPdfOperations(reader,artifactSqliteInteger(value,9))};break;}case "shading":{const value=await reader.take("pdf_shading_pattern",key,3);kind={kind:"shading",shading:await reader.text(value,1),extGState:await reader.optionalText(value,2)};break;}default:throw new Error("Unknown PDF pattern kind");}
  const real=(column:number)=>reader.real("pdf_pattern",row,column);return {id:await reader.text(row,1),matrix:[real(3),real(4),real(5),real(6),real(7),real(8)],kind,extra:await readPdfDictionary(reader,artifactSqliteInteger(row,9))};
}
