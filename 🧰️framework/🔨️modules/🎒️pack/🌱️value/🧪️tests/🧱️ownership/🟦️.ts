/** 📦️ Admits the actual canonical Record module and its genuine Pack package. */
import {test,expect} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import TOML from "@iarna/toml";
import fixture from "../../🧫️fixtures/🧩️ownership/🔣️.json";
type Ownership={package:string;module:string;manifest:string;library:{name:string;path:string};mount:string;dependencies:string[];publicEntries:string[];forbiddenProviders:string[]};
const contract=fixture as unknown as Ownership,owner=resolve(import.meta.dir,"../.."),read=(path:string)=>readFileSync(resolve(owner,path),"utf8");

test("Record ownership examples name the actual package and public module",()=>{
 expect(contract.package).toBe("semio-framework-pack");expect(contract.module).toBe("record");
 expect(new Set(contract.dependencies).intersection(new Set(contract.forbiddenProviders)).size).toBe(0);
 for(const entry of contract.publicEntries)expect(read("🦀️.rs")).toContain("pub fn "+entry+"(");
});

test("one actual Pack library mounts Record and exposes no product source authority",()=>{
 expect(contract.manifest).toBe("../📦️packages/🦀️rust/Cargo.toml");
 const path=resolve(owner,contract.manifest);expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8"),native=Bun.TOML.parse(source) as {package:{name:string};lib:{name:string;path:string};dependencies:Record<string,unknown>};
 expect(native).toEqual(TOML.parse(source) as typeof native);expect(native.package.name).toBe(contract.package);expect(native.lib).toEqual(contract.library);
 expect(Object.keys(native.dependencies).sort()).toEqual([...contract.dependencies].sort());expect(native.dependencies).not.toHaveProperty(contract.package);
 const library=readFileSync(resolve(dirname(path),contract.library.path),"utf8");expect(library).toContain('#[path = "'+contract.mount+'"]');expect(library).toContain("pub mod "+contract.module+";");
 expect(resolve(dirname(path),contract.mount)).toBe(resolve(owner,"🦀️.rs"));
 for(const path of ["🦀️.rs","🛫️encode/🦀️.rs","🏭️schema/🦀️.rs","🔎️scalar-witness/🦀️.rs"])for(const forbidden of ["crate::os_","semio_framework_os_kernel::"])expect(read(path)).not.toContain(forbidden);
});
