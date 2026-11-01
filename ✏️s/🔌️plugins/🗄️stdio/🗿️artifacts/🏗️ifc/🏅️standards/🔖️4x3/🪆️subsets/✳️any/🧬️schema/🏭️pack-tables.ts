import {readFile,writeFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {encodeJsonPack,type PackJson} from "../../../../../📚️catalog/🛠️extract/📦️json-pack.ts";
import {compileIfc4x3Express} from "./⚖️validation/compiler.ts";
const here=(path:string):string=>fileURLToPath(new URL(path,import.meta.url));
/** 📂️ Committed neutral JSON and its IFCJ pack for every IFC4x3 ADD2 generated table. */
export const tableFiles={
 program:{json:here("⚖️validation/🌳️express/program.json"),pack:here("⚖️validation/🌳️express/program.ifcj")},
 templates:{json:here("🏷️templates/🔣️.json"),pack:here("🏷️templates/templates.ifcj")},
 mvd:{json:here("👁️mvd/🔣️.json"),pack:here("👁️mvd/📖️catalog/mvd.ifcj")},
} as const;
/** 🧮️ The canonical EXPRESS program compiled from the retained catalog; its digest is the cross-language identity. */
export async function compiledProgramJson():Promise<string>{const compilation=await compileIfc4x3Express();if(!compilation.complete)throw Error("ifc 4x3 program: "+JSON.stringify(compilation.diagnostics));return JSON.stringify(compilation.program);}
export const programDigest=(json:string):string=>createHash("sha256").update(json).digest("hex");
/** 🏭️ Regenerate `program.json` from the catalog and every `*.ifcj` from its neutral JSON (`bun 🏭️pack-tables.ts`); the output is deterministic. */
if(import.meta.main){
 const program=await compiledProgramJson();await writeFile(tableFiles.program.json,program+"\n");
 for(const[name,files]of Object.entries(tableFiles)){const pack=encodeJsonPack(JSON.parse(await readFile(files.json,"utf8")) as PackJson);await writeFile(files.pack,pack);console.log(name,pack.byteLength);}
 console.log("program sha256",programDigest(program));
}
