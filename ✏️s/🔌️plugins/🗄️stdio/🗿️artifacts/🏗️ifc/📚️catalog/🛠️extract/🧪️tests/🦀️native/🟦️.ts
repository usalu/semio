import {readFileSync} from "node:fs";
import {parserOwnerHarnessV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧪️tests/🧰️owner-harness/🟦️.ts";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {renderRustCatalog} from "../../🦀️render.ts";
import {ifc2x3Catalog} from "../../../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🟦️.ts";
import {ifc4Catalog} from "../../../../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts";
import {ifc4x3Catalog} from "../../../../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts";
import policy from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json";
import corpus from "./🔣️.json";
const catalogs=[ifc2x3Catalog,ifc4Catalog,ifc4x3Catalog],root=new URL("../../../../",import.meta.url);
for(const [index,row] of corpus.catalogs.entries())test("bounded native catalog preserves every original declaration: "+row.name,()=>{const result=renderRustCatalog(catalogs[index]!,{maximumSourceBytes:policy.storage.sourceBoundary.maxSourceBytes});expect(typeof result).toBe("object");expect(readFileSync(new URL(row.path+"/data.rs",root),"utf8")).toBe(result.assembler);expect(Buffer.byteLength(result.assembler)).toBeLessThanOrEqual(policy.storage.sourceBoundary.maxSourceBytes);expect(result.components.length).toBeGreaterThan(1);const database=new Database(":memory:");try{database.exec("create table expected(kind text,name text,digest text,primary key(kind,name));create table observed(kind text,name text,digest text,primary key(kind,name))");for(const item of row.rows)database.query("insert into expected values(?,?,?)").run(item.kind,item.key,item.sha256);for(const component of result.components){expect(readFileSync(new URL(row.path+"/🧩️components/"+component.folder+"/🦀️.rs",root),"utf8")).toBe(component.rust);expect(Buffer.byteLength(component.rust)).toBeLessThanOrEqual(policy.storage.sourceBoundary.maxSourceBytes);expect(result.assembler).toContain(component.packageName.replaceAll("-","_")+"::populate");for(const line of component.rust.split("\n")){const match=line.match(/^(entities|types|functions|rules)\.insert\(("(?:\\.|[^"\\])*")/);if(!match)continue;const key=JSON.parse(match[2]!);if(typeof key!=="string")throw Error("Original catalog declaration name required");database.query("insert into observed values(?,?,?)").run(match[1]!,key,createHash("sha256").update(line).digest("hex"));}}expect(database.query("select * from expected except select * from observed").all()).toEqual([]);expect(database.query("select * from observed except select * from expected").all()).toEqual([]);}finally{database.close();}});
test("native catalog emission retains cancellation before publication",()=>{expect(()=>renderRustCatalog(ifc2x3Catalog,{maximumSourceBytes:policy.storage.sourceBoundary.maxSourceBytes,progress:()=>false})).toThrow("cancelled");});
test("one declaration cannot evade the native Source byte boundary",()=>{expect(()=>renderRustCatalog(ifc2x3Catalog,{maximumSourceBytes:128})).toThrow("declaration");});

const owner=await parserOwnerHarnessV1(new URL("../../",import.meta.url));
test("the complete physical native catalog producer compiles strictly",()=>{expect(owner.files).toHaveLength(3);expect(owner.diagnostics()).toEqual([]);},30000);
