import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {externIdentityV1,markExternIdentityV1} from "../🟦️.ts";

const sha=(bytes:string)=>createHash("sha256").update(bytes).digest("hex");
test("a library enters by the identity of its unit, only while the file is the one that was marked",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"extern-")));
 try{
  const library=join(base,"libpm-abc.dylib"),rlib=join(base,"libx-abc.rlib");
  writeFileSync(library,"one");writeFileSync(rlib,"r");
  expect(externIdentityV1(library,sha("one"),undefined)).toBeUndefined();
  markExternIdentityV1([library,rlib],"a".repeat(64),[],undefined);
  const first=externIdentityV1(library,sha("one"),undefined);
  expect(first).toMatch(/^unit:[0-9a-f]{64}$/u);
  expect(externIdentityV1(rlib,sha("r"),undefined),"only dynamic libraries").toBeUndefined();
  writeFileSync(library,"two");
  expect(externIdentityV1(library,sha("two"),undefined),"a changed library is not trusted").toBeUndefined();
  markExternIdentityV1([library],"b".repeat(64),[],undefined);
  expect(externIdentityV1(library,sha("two"),undefined)).not.toBe(first);
  markExternIdentityV1([library],"b".repeat(64),["/x"],undefined);
  expect(externIdentityV1(library,sha("two"),undefined),"real paths are part of the identity").not.toBe(externIdentityV1(library,sha("two"),undefined)===undefined?"":"unit:"+sha(JSON.stringify(["extern-identity","b".repeat(64),[]])));
 }finally{rmSync(base,{recursive:true,force:true});}
});
