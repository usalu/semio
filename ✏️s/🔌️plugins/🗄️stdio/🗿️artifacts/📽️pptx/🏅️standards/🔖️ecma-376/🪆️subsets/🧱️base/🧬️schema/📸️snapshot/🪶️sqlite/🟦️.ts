/// <reference path="./🗄️.sql.d.ts" />
/** 📕️ PPTX literal package and XML relations. */
import sql from "./🗄️.sql" with {type:"text"};
import type {PptxSnapshot} from "../🟦️.ts";
import {artifactSqliteTables,artifactSqliteDocument,artifactSqliteInteger,artifactSqliteText,artifactSqliteTextBytes,artifactSqliteValueBudget,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type{SqliteDatabase,SqliteRow}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import{projectXmlDocuments,reconstructXmlDocuments,measureXmlDocuments,type XmlSqliteTables}from"../../../../../../../../📰️xml/🟦️.ts";
import{measureOpcPackage,projectOpcPackage,reconstructOpcPackage,type OpcSqliteTables}from"../../../../../../../../🎒️zip/📦️opc/🪶️sqlite/🟦️.ts";
import{measurePresentation,projectPresentation,reconstructPresentation}from"./🧩️presentation/🟦️.ts";
export const PPTX_SQLITE_SCHEMA:string=sql;
const OPC:OpcSqliteTables={package:"pptx_package",part:"pptx_binary_part",defaultType:"pptx_default_content_type",overrideType:"pptx_override_content_type",relationshipOwner:"pptx_relationship_owner",relationship:"pptx_relationship"};
const XML:XmlSqliteTables={document:"pptx_xml_document",node:"pptx_xml_node",element:"pptx_xml_element",text:"pptx_xml_text",cdata:"pptx_xml_cdata",comment:"pptx_xml_comment",processingInstruction:"pptx_xml_processing_instruction",attribute:"pptx_xml_attribute",child:"pptx_xml_child",documentMisc:"pptx_xml_document_misc",declaration:"pptx_xml_declaration",doctype:"pptx_xml_doctype",entity:"pptx_xml_entity"};
function rowsLimit(count:number,options:ArtifactSqliteOptions):void{if(!Number.isSafeInteger(count)||count>(options.maxRows??1_000_000))throw new Error("PPTX row limit");}
function table(database:SqliteDatabase,name:string):readonly SqliteRow[]{const found=database.tables.find(table=>table.name===name);if(!found)throw new Error("PPTX declared table missing");return found.rows;}
/** 🏗️ Projects exactly the persisted OPC and XML-part fields under aggregate bounds. */
export async function pptxSnapshotToSqliteDatabase(snapshot:PptxSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);const opc=await measureOpcPackage(snapshot.opc,options),presentation=await measurePresentation(snapshot.presentation,options);rowsLimit(opc.rows+1+2*snapshot.xmlParts.length+presentation.rows+presentation.others,options);let bytes=opc.bytes+presentation.bytes+16+artifactSqliteTextBytes(snapshot.schema);artifactSqliteValueBudget(bytes,options);
 const documents=[];for(const[ordinal,part]of snapshot.xmlParts.entries()){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",ordinal,snapshot.xmlParts.length);bytes+=32+artifactSqliteTextBytes(part.contentType);artifactSqliteValueBudget(bytes,options);documents.push({schema:part.path,doc:part.document});}
 let shapeWork=0;for(const slide of snapshot.presentation.slides)for(const shape of slide.shapes){if(shapeWork++%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",shapeWork-1,0);if(shape.shapeKind==="other")documents.push({schema:"",doc:{root:shape.node,prolog:[],epilog:[]}});}
 const xml=await measureXmlDocuments(documents,options);rowsLimit(opc.rows+1+snapshot.xmlParts.length+xml.rows+presentation.rows,options);artifactSqliteValueBudget(bytes+xml.bytes,options);
 const database=await projectOpcPackage(snapshot.opc,await projectXmlDocuments(documents,sql,XML,options),OPC,options);const root:SqliteRow={rowid:1n,values:[1n,snapshot.schema,1n]},parts:SqliteRow[]=[];
 for(const[ordinal,part]of snapshot.xmlParts.entries()){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",ordinal,snapshot.xmlParts.length);const rowid=BigInt(ordinal+1);parts.push({rowid,values:[rowid,1n,BigInt(ordinal),part.contentType,rowid]});}
 const projected={tables:database.tables.map(table=>table.name==="pptx_document"?{name:table.name,sql:table.sql,rows:[root]}:table.name==="pptx_xml_part"?{name:table.name,sql:table.sql,rows:parts}:table)};const result=await projectPresentation(snapshot.presentation,projected,snapshot.xmlParts.length,options);await artifactSqliteTables(result,sql,options);return result;
}
/** 🧱️ Reconstructs owned package domains and globally owned XML roots without native-file normalization. */
export async function pptxSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<PptxSnapshot>{
 await artifactSqliteTables(database,sql,options);const row=artifactSqliteDocument(table(database,"pptx_document"));if(row.values.length!==3||artifactSqliteInteger(row,2)!==1n)throw new Error("PPTX package owner invalid");const schema=artifactSqliteText(row,1),opc=await reconstructOpcPackage(database,OPC,options);const parts=await artifactSqliteOrderedRowsControlled(table(database,"pptx_xml_part"),2,options);const documents=await reconstructXmlDocuments(database,sql,XML,options);if(parts.length>documents.length)throw new Error("PPTX XML part/document cardinality disagrees");const xmlParts:PptxSnapshot["xmlParts"]=[],identities=new Set<bigint>();
 for(const[ordinal,part]of parts.entries()){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordinal,parts.length);if(part.values.length!==5||part.rowid<=0n||artifactSqliteInteger(part,0)!==part.rowid||identities.has(part.rowid)||artifactSqliteInteger(part,1)!==1n||artifactSqliteInteger(part,4)!==BigInt(ordinal+1))throw new Error("PPTX XML part owner invalid");identities.add(part.rowid);const document=documents[ordinal]!;xmlParts.push({path:document.schema,contentType:artifactSqliteText(part,3),document:document.doc});}
 const presentation=await reconstructPresentation(database,documents,parts.length,options);return{schema,opc,xmlParts,presentation};
}
