import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { emitTypeScriptDeclarations } from "../../🧬️declarations/🟦️.ts";

/** 🧬️ A bare workspace package import is inlined into the flat declaration closure instead of staying unresolvable. */
test("declaration closure inlines workspace package imports", () => {
 const root=realpathSync(mkdtempSync(join(tmpdir(),"semio-declarations-"))),write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 try{
  write("package.json",'{"name":"workspace","private":true}');
  write("lib/package.json",'{"name":"@scope/lib","type":"module","exports":{"./contract":"./contract/entry.ts"}}');
  write("lib/contract/entry.ts",'export { Shape } from "../shape.ts";\n');
  write("lib/shape.ts","export interface Shape { readonly side: number }\n");
  write("app/index.ts",'import type { Shape } from "@scope/lib/contract";\nexport const square: Shape = { side: 1 };\n');
  mkdirSync(join(root,"node_modules/@scope"),{recursive:true});
  symlinkSync(join(root,"lib"),join(root,"node_modules/@scope/lib"),"dir");
  symlinkSync(dirname(createRequire(import.meta.url).resolve("typescript/package.json")),join(root,"node_modules/typescript"),"dir");
  const files=emitTypeScriptDeclarations(join(root,"app/index.ts"),root),text=[...files.values()].join("\n");
  expect(text).not.toContain("@scope/lib");
  expect(text).toContain("side: number");
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
