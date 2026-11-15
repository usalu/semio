import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {embeddedPathValuesV1,pathSensitiveV1} from "../🟦️.ts";

test("a path value is counted, a file that is only included is not",()=>{
 expect(embeddedPathValuesV1('const D:&str=env!("CARGO_MANIFEST_DIR");')).toBe(1);
 expect(embeddedPathValuesV1('fn d()->PathBuf{PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("x")}')).toBe(1);
 expect(embeddedPathValuesV1('const T:&str=include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../x.json"));')).toBe(0);
 expect(embeddedPathValuesV1('include!(concat!(env!("OUT_DIR"),"/gen.rs"));')).toBe(0);
 expect(embeddedPathValuesV1('include!(concat!(env!("OUT_DIR"),"/gen.rs")); const O:&str=env!("OUT_DIR");')).toBe(1);
 expect(embeddedPathValuesV1('fn f(){ let _=option_env!("CARGO_MANIFEST_DIR"); }')).toBe(1);
 expect(embeddedPathValuesV1('fn f(){ let _=env!("CARGO_PKG_NAME"); }')).toBe(0);
});
test("the scan follows modules and reports the real paths only when a file of the unit embeds one",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"embedding-")));
 try{
  mkdirSync(join(base,"src"),{recursive:true});
  writeFileSync(join(base,"src","lib.rs"),"mod inner;\npub fn a()->u32{1}\n");
  writeFileSync(join(base,"src","inner.rs"),'pub fn dir()->&\'static str{env!("CARGO_MANIFEST_DIR")}\n');
  expect(pathSensitiveV1({source:join(base,"src","lib.rs"),manifest:base,cwd:base})).toEqual([base]);
  writeFileSync(join(base,"src","inner.rs"),"pub fn b()->u32{2}\n");
  expect(pathSensitiveV1({source:join(base,"src","lib.rs"),manifest:base,cwd:base})).toEqual([]);
  expect(pathSensitiveV1({source:join(base,"src","missing.rs"),manifest:base,cwd:base})).toEqual([base]);
 }finally{rmSync(base,{recursive:true,force:true});}
});
