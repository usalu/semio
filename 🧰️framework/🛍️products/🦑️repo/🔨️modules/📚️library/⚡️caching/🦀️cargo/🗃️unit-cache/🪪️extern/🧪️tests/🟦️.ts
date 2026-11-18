import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {externIdentityV1,markExternIdentityV1} from "../🟦️.ts";

const sha=(bytes:string)=>createHash("sha256").update(bytes).digest("hex");
test("a library enters by the identity of its unit and its crate hash, only while the file is the one that was marked",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"extern-")));
 try{
  const library=join(base,"libpm-abc.dylib"),rlib=join(base,"libx-abc.rlib"),hash="f".repeat(32),other="e".repeat(32);
  writeFileSync(library,"one");writeFileSync(rlib,"r");
  expect(externIdentityV1(library,sha("one"),undefined)).toBeUndefined();
  markExternIdentityV1([library,rlib],"a".repeat(64),[],undefined,()=>hash);
  const first=externIdentityV1(library,sha("one"),undefined);
  expect(first).toMatch(/^unit:[0-9a-f]{64}:[0-9a-f]{32}$/u);
  expect(externIdentityV1(rlib,sha("r"),undefined),"only dynamic libraries").toBeUndefined();
  markExternIdentityV1([library],"a".repeat(64),[],undefined,()=>other);
  expect(externIdentityV1(library,sha("one"),undefined),"the crate hash is part of the identity: another generation of one unit is another identity").not.toBe(first);
  markExternIdentityV1([library],"a".repeat(64),[],undefined,()=>undefined);
  expect(externIdentityV1(library,sha("one"),undefined),"a library whose crate hash is unknown keeps its last identity only while it names the same bytes").toBeDefined();
  writeFileSync(library,"two");
  expect(externIdentityV1(library,sha("two"),undefined),"a changed library is not trusted").toBeUndefined();
  markExternIdentityV1([library],"b".repeat(64),[],undefined,()=>undefined);
  expect(externIdentityV1(library,sha("two"),undefined),"no crate hash, no identity: the bytes count").toBeUndefined();
  markExternIdentityV1([library],"b".repeat(64),[],undefined,()=>hash);
  const second=externIdentityV1(library,sha("two"),undefined);
  expect(second).not.toBe(first);
  markExternIdentityV1([library],"b".repeat(64),["/x"],undefined,()=>hash);
  expect(externIdentityV1(library,sha("two"),undefined),"real paths are part of the identity").not.toBe(second);
 }finally{rmSync(base,{recursive:true,force:true});}
});
