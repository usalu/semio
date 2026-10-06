/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
import type {CurationSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {parseCurationArtifact} from "../../../🧬️schema/🟦️.ts";
import {encodeIeee754Cells,readBinary64,readBinary32,type Ieee754Column,type Ieee754Cell} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary64,parseBinary32,type Binary64,type Binary32} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {ArtifactSqliteProjection,ArtifactSqliteRowIndex,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteDocument,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {sqliteOperation,type SqliteDatabase,type SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
/** 🧱️ Exact native geometry identities at their authored scalar widths. */
export type CurationSqliteGeometry={kind:"box";width:Binary64;height:Binary64;depth:Binary64}|{kind:"frame";width:Binary64;height:Binary64;depth:Binary64;profile:Binary64}|{kind:"slab";width:Binary64;depth:Binary64;thickness:Binary64}|{kind:"mesh";positions:Binary32[];normals:Binary32[];indices:number[]}|{kind:"glb";url:string;extent:Binary64};
/** 🗂️ Complete persisted catalog, sourcing metadata and selection fields. */
export type CurationSqliteSnapshot=Omit<CurationSnapshot,"stockExtra">&{stockExtra:(Omit<CurationSnapshot["stockExtra"][number],"geometry">&{geometry:CurationSqliteGeometry})[]};
export const CURATION_SQLITE_SCHEMA:string=sql;
const BOX:readonly Ieee754Column[]=[{index:2,width:64},{index:3,width:64},{index:4,width:64}];
const FRAME:readonly Ieee754Column[]=[{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64}];
const GLB:readonly Ieee754Column[]=[{index:3,width:64}];
const MESH:readonly Ieee754Column[]=[{index:3,width:32}];
function record(value:unknown,keys:readonly string[]):Record<string,unknown>{if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Curation requires a native record");for(const key in value)if(Object.hasOwn(value,key)&&!keys.includes(key))throw Error("Curation record has an undeclared field");for(const key of keys)if(!Object.hasOwn(value,key))throw Error("Curation record lacks a required field");return value as Record<string,unknown>}
function text(value:unknown):string{if(typeof value!=="string")throw Error("Curation requires literal TEXT");return value}
function uint(value:unknown):bigint{if(typeof value!=="number"||!Number.isSafeInteger(value)||value<0||value>4294967295)throw Error("Curation requires uint32");return BigInt(value)}
function word64(value:unknown):Binary64{record(value,["bits"]);return parseBinary64(value)}
function word32(value:unknown):Binary32{record(value,["bits"]);return parseBinary32(value)}
function list(value:unknown):unknown[]{if(!Array.isArray(value))throw Error("Curation requires an ordered collection");return value}
function count(snapshot:CurationSqliteSnapshot):number{let rows=2+snapshot.curated.length;for(const extra of snapshot.stockExtra){rows+=3+extra.typologyPath.length;if(extra.geometry.kind==="mesh")rows+=extra.geometry.positions.length+extra.geometry.normals.length+extra.geometry.indices.length;if(!Number.isSafeInteger(rows))throw Error("Curation row count overflow")}return rows}
function alias(row:SqliteRow,columns:number):void{if(row.values.length!==columns||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid)throw Error("Curation requires complete positive aliased entities")}
function readUint(row:SqliteRow,index:number):number{const value=artifactSqliteInteger(row,index);if(value<0n||value>4294967295n)throw Error("Curation scalar exceeds uint32");return Number(value)}
/** 📤️ Projects every explicit recipe branch and ordered member under caller control. */
export async function curationSnapshotToSqliteDatabase(snapshot:CurationSqliteSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(sql,options);record(snapshot,["catalog","stockExtra","curated"]);list(snapshot.stockExtra);list(snapshot.curated);const catalog=parseCurationArtifact({catalog:snapshot.catalog,stockExtra:[],curated:[]}).catalog,total=count(snapshot);out.checkRowsAdditional(total);let completed=0;
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,total,false);
 const insert=async(table:string,cells:Parameters<ArtifactSqliteProjection["insert"]>[1],id?:bigint):Promise<bigint>=>{const key=await out.insert(table,cells,id);if(++completed%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total);return key};
 const geometry=async(table:string,cells:readonly Ieee754Cell[],columns:readonly Ieee754Column[]):Promise<bigint>=>insert(table,encodeIeee754Cells(cells,columns,options.maxColumns).slice(1));
 await insert("curation_document",[],1n);const target=catalog.target;await insert("curation_catalog",[1n,catalog.childId,target.artifactId,target.dialect.artifactKind,target.dialect.standard,target.dialect.subset]);
 for(let ordinal=0;ordinal<snapshot.stockExtra.length;ordinal++){
  const e=record(snapshot.stockExtra[ordinal],["id","name","moduleId","typologyPath","availability","geometry"]),id=await insert("curation_stock_extra",[1n,BigInt(ordinal),text(e.id),text(e.name),text(e.moduleId),uint(e.availability)]),path=list(e.typologyPath);for(let ordinal=0;ordinal<path.length;ordinal++)await insert("curation_typology_segment",[id,BigInt(ordinal),text(path[ordinal])]);
  const g=e.geometry as CurationSqliteGeometry,key=await insert("curation_geometry",[id,text(record(g,["kind",...(g.kind==="mesh"?["positions","normals","indices"]:g.kind==="glb"?["url","extent"]:g.kind==="slab"?["width","depth","thickness"]:g.kind==="frame"?["width","height","depth","profile"]:["width","height","depth"])]).kind)]);
  switch(g.kind){
   case"box":await geometry("curation_box",[1n,key,word64(g.width),word64(g.height),word64(g.depth)],BOX);break;
   case"frame":await geometry("curation_frame",[1n,key,word64(g.width),word64(g.height),word64(g.depth),word64(g.profile)],FRAME);break;
   case"slab":await geometry("curation_slab",[1n,key,word64(g.width),word64(g.depth),word64(g.thickness)],BOX);break;
   case"glb":await geometry("curation_glb",[1n,key,text(g.url),word64(g.extent)],GLB);break;
   case"mesh":{const mesh=await insert("curation_mesh",[key]);list(g.positions);list(g.normals);list(g.indices);for(let ordinal=0;ordinal<g.positions.length;ordinal++)await geometry("curation_mesh_position",[1n,mesh,BigInt(ordinal),word32(g.positions[ordinal])],MESH);for(let ordinal=0;ordinal<g.normals.length;ordinal++)await geometry("curation_mesh_normal",[1n,mesh,BigInt(ordinal),word32(g.normals[ordinal])],MESH);for(let ordinal=0;ordinal<g.indices.length;ordinal++)await insert("curation_mesh_index",[mesh,BigInt(ordinal),uint(g.indices[ordinal])]);break}
   default:throw Error("Curation recipe discriminant is undeclared");
  }
 }
 for(let ordinal=0;ordinal<snapshot.curated.length;ordinal++){const row=record(snapshot.curated[ordinal],["objectId","count"]);await insert("curation_curated",[1n,BigInt(ordinal),text(row.objectId),uint(row.count)])}
 if(completed!==total)throw Error("Curation projected workload differs from its known ownership");await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total,false);return out.finish();
}
/** 📥️ Reconstructs exact literal ownership and typed recipes independently of surrogate names. */
export async function curationSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<CurationSqliteSnapshot>{
 options=sqliteOperation(options);
 const tables=await artifactSqliteTables(database,sql,options),total=tables.reduce((n,rows)=>n+rows.length,0),names=["curation_document","curation_catalog","curation_stock_extra","curation_typology_segment","curation_geometry","curation_box","curation_frame","curation_slab","curation_mesh","curation_glb","curation_mesh_position","curation_mesh_normal","curation_mesh_index","curation_curated"],columns=[1,7,7,4,3,11,14,11,2,6,6,6,4,5];
 let completed=0;
 for(let at=0;at<tables.length;at++)for(const row of tables[at]!){alias(row,columns[at]!);if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total)}
 const index=await ArtifactSqliteRowIndex.create(database.tables,options),document=artifactSqliteDocument(tables[0]!);
 if(tables[1]!.length!==1)throw Error("Curation requires exactly one mandatory Kit catalog");
 const catalog=tables[1]![0]!;if(artifactSqliteInteger(catalog,1)!==1n)throw Error("Curation catalog has wrong document owner");
 const child=parseCurationArtifact({catalog:{childId:artifactSqliteText(catalog,2),target:{artifactId:artifactSqliteText(catalog,3),dialect:{artifactKind:artifactSqliteText(catalog,4),standard:artifactSqliteText(catalog,5),subset:artifactSqliteText(catalog,6)}}},stockExtra:[],curated:[]}).catalog;
 await index.take(document);await index.take(catalog);
 const one=async(table:number,parent:bigint):Promise<SqliteRow>=>{const rows=await index.grouped(names[table]!,parent,null);if(rows.length!==1)throw Error("Curation recipe requires one complete matching entity");const row=rows[0]!;await index.take(row);return row};
 const stockExtra:CurationSqliteSnapshot["stockExtra"]=[],curated:CurationSnapshot["curated"]=[];
 for(const row of await index.grouped(names[2]!,1n)){
  await index.take(row);const recipe=await one(4,row.rowid),kind=artifactSqliteText(recipe,2);let geometry:CurationSqliteGeometry;
  switch(kind){
   case"box":{const r=await one(5,recipe.rowid);geometry={kind,width:readBinary64(r,2,BOX),height:readBinary64(r,3,BOX),depth:readBinary64(r,4,BOX)};break}
   case"frame":{const r=await one(6,recipe.rowid);geometry={kind,width:readBinary64(r,2,FRAME),height:readBinary64(r,3,FRAME),depth:readBinary64(r,4,FRAME),profile:readBinary64(r,5,FRAME)};break}
   case"slab":{const r=await one(7,recipe.rowid);geometry={kind,width:readBinary64(r,2,BOX),depth:readBinary64(r,3,BOX),thickness:readBinary64(r,4,BOX)};break}
   case"glb":{const r=await one(9,recipe.rowid);geometry={kind,url:artifactSqliteText(r,2),extent:readBinary64(r,3,GLB)};break}
   case"mesh":{
    const r=await one(8,recipe.rowid),positions:Binary32[]=[],normals:Binary32[]=[],indices:number[]=[];
    for(const row of await index.grouped(names[10]!,r.rowid)){positions.push(readBinary32(row,3,MESH));await index.take(row)}
    for(const row of await index.grouped(names[11]!,r.rowid)){normals.push(readBinary32(row,3,MESH));await index.take(row)}
    for(const row of await index.grouped(names[12]!,r.rowid)){indices.push(readUint(row,3));await index.take(row)}
    geometry={kind,positions,normals,indices};break
   }
   default:throw Error("Curation recipe kind is undeclared");
  }
  const typologyPath:string[]=[];for(const entry of await index.grouped(names[3]!,row.rowid)){typologyPath.push(artifactSqliteText(entry,3));await index.take(entry)}
  stockExtra.push({id:artifactSqliteText(row,3),name:artifactSqliteText(row,4),moduleId:artifactSqliteText(row,5),typologyPath,availability:readUint(row,6),geometry});
 }
 for(const row of await index.grouped(names[13]!,1n)){curated.push({objectId:artifactSqliteText(row,3),count:readUint(row,4)});await index.take(row)}
 await index.finish();return{catalog:child,stockExtra,curated};
}
