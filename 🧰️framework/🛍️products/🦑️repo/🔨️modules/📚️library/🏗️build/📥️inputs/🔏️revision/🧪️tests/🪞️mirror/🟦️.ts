/** 🪞️ A test that loads a copy of the revision reader needs the modules it imports at the same relative places: this lays its local dependency closure out under a scratch root exactly as the repository does, and returns the directory the copy of the reader belongs in. */
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,extname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {isBuiltin} from "node:module";
import {TranspilerV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🔬️source/🟦️.ts";

const framework=fileURLToPath(new URL("../../../../../../../../../",import.meta.url));
const reader=fileURLToPath(new URL("../../",import.meta.url));

/** 📂️ Mirrors the reader's transitive local dependency closure while leaving its own module and schema under the fixture's explicit control. */
export function mirrorRevisionReaderDirectoryV1(root:string):string{
 const own=join(reader,"🟨️.mjs"),ownSchema=join(reader,"🧬️schema"),seen=new Set<string>(),pending=[own];
 while(pending.length){const path=resolve(pending.pop()!),name=relative(framework,path);if(isAbsolute(name)||name===".."||name.startsWith(".."+sep))throw Error("Revision fixture dependency leaves framework");if(seen.has(path))continue;seen.add(path);const bytes=readFileSync(path),extension=extname(path);
  if(path!==own&&!path.startsWith(ownSchema+sep)){const copy=join(root,name);if(copy.length>256)throw Error("Revision fixture dependency path exceeds256");mkdirSync(dirname(copy),{recursive:true});writeFileSync(copy,bytes,{flag:"wx"});}
  if(extension!==".json")for(const row of new TranspilerV1({loader:extension===".ts"||extension===".tsx"?"ts":"js"}).scanImports(bytes.toString("utf8"))){if(row.path.startsWith("."))pending.push(resolve(dirname(path),row.path));else if(!isBuiltin(row.path)&&row.path!=="bun"&&row.path!=="bun:test")throw Error("Explicit revision fixture dependency required: "+row.path);}
 }
 const directory=join(root,relative(framework,reader));mkdirSync(directory,{recursive:true});return directory;
}
