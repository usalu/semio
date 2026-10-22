/** 📭️ Contract: empty-folder discovery lists every directory without entries, deepest first, never inside protected, build or cache trees and never a symlink; the corpus is language-agnostic and `🐍️oracle.py` walks the same workspace. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {sep} from "node:path";
import {cleanCollectEmptyFolderRemovals,cleanDirectoryIsEmpty} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const posix=(path:string):string=>path.split(sep).join("/");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"folders-")));roots.push(root);for(const node of corpus.nodes){const target=join(root,...node.path.split("/"));if(node.kind==="directory")mkdirSync(target,{recursive:true});else{mkdirSync(dirname(target),{recursive:true});writeFileSync(target,(node as {text?:string}).text??"");}}return root;};
const root=world();
const absolute=(path:string):string=>join(root,...path.split("/"));
for(const row of corpus.empty.dirs)test("empty directory "+row.path,()=>{expect(cleanDirectoryIsEmpty(absolute(row.path))).toBe(row.expected);});
test("a symlink to an empty directory is not an empty directory",()=>{if(process.platform==="win32")return;const link=absolute("link-to-e");symlinkSync(absolute("e"),link);try{expect(cleanDirectoryIsEmpty(link)).toBe(false);}finally{rmSync(link);}});
for(const row of corpus.empty.collect)test("empty folders with protected "+JSON.stringify(row.prefixes),()=>{const rows=cleanCollectEmptyFolderRemovals(root,row.prefixes.map(absolute));expect(rows.map(item=>posix(item.path))).toEqual(row.expected);expect(rows.every(item=>item.kind==="empty-folder"&&item.bytes===0)).toBe(true);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
