/** ⚖️ Laws of every Play interaction matrix: the matrix meets its schema, it has exactly one mount file and exactly one file per row (so a suite spreads rows over machines one file at a time), every file declares the row it names, and no row file locates anything by CSS or XPath. */
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {describe,expect,test} from "bun:test";
import {parseMatrixV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧮️matrix/🟦️.ts";

const root=new URL("..",import.meta.url).pathname,matrices=readdirSync(root,{withFileTypes:true}).filter(entry=>entry.isDirectory()&&!entry.name.includes("laws")&&!entry.name.includes("texts")).map(entry=>entry.name);

describe("play interaction matrices",()=>{
 test("at least one matrix exists",()=>{expect(matrices.length).toBeGreaterThan(0);});
 for(const name of matrices)test(name,()=>{
  const directory=join(root,name),matrix=parseMatrixV1(JSON.parse(readFileSync(join(directory,"🔣️.json"),"utf8")));
  const files=readdirSync(join(directory,"🧪️tests"),{recursive:true,encoding:"utf8"}).filter(file=>file.endsWith("🟦️.ts")).map(file=>readFileSync(join(directory,"🧪️tests",file),"utf8"));
  const declared=files.map(source=>/playMatrixTestsV1\([^)]*?(?:\{row:"([a-z0-9-]+)"\}|"mount")\)/u.exec(source)?.[1]??(/playMatrixTestsV1\([^)]*"mount"\)/u.test(source)?"mount":null));
  expect(declared.filter(row=>row==="mount")).toHaveLength(1);
  expect(declared.filter(row=>row!=="mount").sort()).toEqual(matrix.rows.map(row=>row.id).sort());
  for(const source of files)expect(source).not.toMatch(/querySelector|locator\(|xpath|\$\(/iu);
  expect(JSON.stringify(matrix)).not.toMatch(/"(selector|css|xpath|testId)"/iu);
 });
});
