/// <reference path="./🗄️.sql.d.ts" />
/** 📕️ XLSX literal package and XML relations. */
import sql from "./🗄️.sql" with {type:"text"};
import type {XlsxSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {artifactSqliteTables,artifactSqliteDocument,artifactSqliteInteger,artifactSqliteText,artifactSqliteTextBytes,artifactSqliteValueBudget,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type{SqliteDatabase,SqliteRow}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import{projectXmlDocuments,reconstructXmlDocuments,measureXmlDocuments,type XmlSqliteTables}from"../../../../../../../../📰️xml/🟦️.ts";
import{measureOpcPackage,projectOpcPackage,reconstructOpcPackage,type OpcSqliteTables}from"../../../../../../../../🎒️zip/📦️opc/🪶️sqlite/🟦️.ts";
export const XLSX_SQLITE_SCHEMA:string=sql;
const OPC:OpcSqliteTables={package:"xlsx_package",part:"xlsx_binary_part",defaultType:"xlsx_default_content_type",overrideType:"xlsx_override_content_type",relationshipOwner:"xlsx_relationship_owner",relationship:"xlsx_relationship"};
const XML:XmlSqliteTables={document:"xlsx_xml_document",node:"xlsx_xml_node",element:"xlsx_xml_element",text:"xlsx_xml_text",cdata:"xlsx_xml_cdata",comment:"xlsx_xml_comment",processingInstruction:"xlsx_xml_processing_instruction",attribute:"xlsx_xml_attribute",child:"xlsx_xml_child",documentMisc:"xlsx_xml_document_misc",declaration:"xlsx_xml_declaration",doctype:"xlsx_xml_doctype",entity:"xlsx_xml_entity"};
function rowsLimit(count:number,options:ArtifactSqliteOptions):void{if(!Number.isSafeInteger(count)||count>(options.maxRows??1_000_000))throw new Error("XLSX row limit");}
function table(database:SqliteDatabase,name:string):readonly SqliteRow[]{const found=database.tables.find(table=>table.name===name);if(!found)throw new Error("XLSX declared table missing");return found.rows;}
/** 🏗️ Projects exactly the persisted OPC and XML-part fields under aggregate bounds. */
export async function xlsxSnapshotToSqliteDatabase(snapshot:XlsxSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);const opc=await measureOpcPackage(snapshot.opc,options);rowsLimit(opc.rows+1+2*snapshot.xmlParts.length,options);let bytes=opc.bytes+16+artifactSqliteTextBytes(snapshot.schema);artifactSqliteValueBudget(bytes,options);
 const documents=[];for(const[ordinal,part]of snapshot.xmlParts.entries()){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",ordinal,snapshot.xmlParts.length);bytes+=32+artifactSqliteTextBytes(part.contentType);artifactSqliteValueBudget(bytes,options);documents.push({schema:part.path,doc:part.document});}
 const xml=await measureXmlDocuments(documents,options);rowsLimit(opc.rows+1+snapshot.xmlParts.length+xml.rows,options);artifactSqliteValueBudget(bytes+xml.bytes,options);
 const database=await projectOpcPackage(snapshot.opc,await projectXmlDocuments(documents,sql,XML,options),OPC,options);const root:SqliteRow={rowid:1n,values:[1n,snapshot.schema,1n]},parts:SqliteRow[]=[];
 for(const[ordinal,part]of snapshot.xmlParts.entries()){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",ordinal,snapshot.xmlParts.length);const rowid=BigInt(ordinal+1);parts.push({rowid,values:[rowid,1n,BigInt(ordinal),part.contentType,rowid]});}
 const result={tables:database.tables.map(table=>table.name==="xlsx_document"?{name:table.name,sql:table.sql,rows:[root]}:table.name==="xlsx_xml_part"?{name:table.name,sql:table.sql,rows:parts}:table)};await artifactSqliteTables(result,sql,options);return result;
}
/** 🧱️ Reconstructs owned package domains and globally owned XML roots without native-file normalization. */
export async function xlsxSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<XlsxSnapshot>{
 await artifactSqliteTables(database,sql,options);const row=artifactSqliteDocument(table(database,"xlsx_document"));if(row.values.length!==3||artifactSqliteInteger(row,2)!==1n)throw new Error("XLSX package owner invalid");const schema=artifactSqliteText(row,1),opc=await reconstructOpcPackage(database,OPC,options);const parts=await artifactSqliteOrderedRowsControlled(table(database,"xlsx_xml_part"),2,options);const documents=await reconstructXmlDocuments(database,sql,XML,options);if(parts.length!==documents.length)throw new Error("XLSX XML part/document cardinality disagrees");const xmlParts:XlsxSnapshot["xmlParts"]=[],identities=new Set<bigint>();
 for(const[ordinal,part]of parts.entries()){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordinal,parts.length);if(part.values.length!==5||part.rowid<=0n||artifactSqliteInteger(part,0)!==part.rowid||identities.has(part.rowid)||artifactSqliteInteger(part,1)!==1n||artifactSqliteInteger(part,4)!==BigInt(ordinal+1))throw new Error("XLSX XML part owner invalid");identities.add(part.rowid);const document=documents[ordinal]!;xmlParts.push({path:document.schema,contentType:artifactSqliteText(part,3),document:document.doc});}
 return{schema,opc,xmlParts};
}
