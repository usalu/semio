import {readFile,writeFile,mkdir} from "node:fs/promises";
import {dirname} from "node:path";
import {createHash} from "node:crypto";
import {extractExpressCatalog,type ExpressSourceManifest,type ExpressExtractionOptions} from "./🟦️.ts";
import {renderRustCatalog} from "./🦀️render.ts";
import type {IfcCatalog} from "../🟦️.ts";
/** 📂️ Caller-owned source, manifest and retained declaration output paths. */
export interface CatalogGenerationFiles{source:string;manifest:string;json:string;rust:string;}
/** 🏭️ Reproduce a catalog only from a manifest's unchanged normative source bytes. */
export async function generateCatalog(files:CatalogGenerationFiles,options:ExpressExtractionOptions={}):Promise<{schema:string;sha256:string;entities:number;types:number;functions:number;rules:number}>{const[source,manifestText]=await Promise.all([readFile(files.source),readFile(files.manifest,"utf8")]),manifest=JSON.parse(manifestText) as ExpressSourceManifest&{validation?:IfcCatalog["validation"]},sha256=createHash("sha256").update(source).digest("hex");if(sha256!==manifest.source.sha256)throw Error("express catalog: normative source checksum mismatch");const catalog=extractExpressCatalog(new TextDecoder("utf-8",{fatal:true}).decode(source),manifest,options);if(manifest.validation)catalog.validation={...manifest.validation};const json=JSON.stringify(catalog)+"\n",rust=renderRustCatalog(catalog);await Promise.all([mkdir(dirname(files.json),{recursive:true}),mkdir(dirname(files.rust),{recursive:true})]);await writeFile(files.json,json);await writeFile(files.rust,rust);return {schema:catalog.schema,sha256,entities:Object.keys(catalog.entities).length,types:Object.keys(catalog.types).length,functions:Object.keys(catalog.functions).length,rules:Object.keys(catalog.rules).length};}
