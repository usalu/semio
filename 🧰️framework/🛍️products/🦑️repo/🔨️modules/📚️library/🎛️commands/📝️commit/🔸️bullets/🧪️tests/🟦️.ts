/** 🔸️ Contract: commit bullets keep their leading emoji, normalize authored text, refuse reserved emojis, cover the staged areas and travel through the git directory; the corpus is language-agnostic and `🐍️.py` classifies emoji by Unicode range and recomputes every verdict. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {MICRO_COMMIT_BULLET_RE,bulletEmojiValidationError,bulletLeadEmoji,formatMicroCommitBulletLine,listAddedTicketPaths,listCachedPaths,normalizeBulletLines,pathTokensForBulletCoverage,readPreparedBullets,ticketBullets,uncoveredStagedAreas,writePreparedBullets} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"bullets-")));git(root,"init","-q");return root;};
for(const row of corpus.lead)test("lead emoji of "+JSON.stringify(row.line),()=>{expect(bulletLeadEmoji(row.line)).toBe(row.expected);});
for(const row of corpus.format)test("bullet format "+JSON.stringify(row.line),()=>{expect(formatMicroCommitBulletLine(row.line)).toBe(row.expected);});
test("bullet text normalizes to emoji-first lines without headings, metrics and one-character lines",()=>{expect(normalizeBulletLines(corpus.normalize.text)).toEqual(corpus.normalize.expected);expect(normalizeBulletLines(corpus.normalize.capText)).toEqual(corpus.normalize.capExpected);});
test("the bullet pattern wants an emoji directly followed by text",()=>{for(const text of["🧪️x","🚀y","👩\u200d💻z"])expect(MICRO_COMMIT_BULLET_RE.test(text)).toBe(true);for(const text of["plain","-🚀x","🚀"])expect(MICRO_COMMIT_BULLET_RE.test(text)).toBe(false);});
test.failing("a space after an emoji with a variation selector is refused",()=>{expect(MICRO_COMMIT_BULLET_RE.test("🧪️ x")).toBe(false);});
for(const row of corpus.reserved)test("reserved lead emoji "+JSON.stringify(row.lead),()=>{expect(bulletEmojiValidationError([row.bullet])).toContain(row.lead);});
for(const[index,row]of corpus.validate.entries())test("emoji validation "+index,()=>{const error=bulletEmojiValidationError(row.bullets);if(row.reserved)expect(error).toContain("reserved");else expect(error).toBeNull();});
for(const row of corpus.tokens)test("coverage tokens of "+row.path,()=>{expect(pathTokensForBulletCoverage(row.path)).toEqual(row.expected);});
for(const[index,row]of corpus.staged.entries())test("uncovered areas "+index+": "+JSON.stringify(row.staged),()=>{expect(uncoveredStagedAreas(row.bullets,row.staged)).toEqual(row.expected);});
for(const[index,row]of corpus.stagedDefects.entries())test.failing("uncovered areas defect "+index,()=>{expect(uncoveredStagedAreas(row.bullets,row.staged)).toEqual(row.expected);});
test("prepared bullets round-trip through the git directory and normalize on read",()=>{
 const root=scratch();
 try{
  expect(readPreparedBullets(root)).toEqual([]);
  for(const bullets of corpus.prepared){writePreparedBullets(root,bullets);expect(readPreparedBullets(root)).toEqual(bullets);}
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("staged added ticket files become emoji-title bullets",()=>{
 const root=scratch();
 try{
  for(const[path,text]of Object.entries(corpus.tickets.staged)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  git(root,"add","-A");expect(listCachedPaths(root).length).toBe(Object.keys(corpus.tickets.staged).length);
  expect(listAddedTicketPaths(root).sort()).toEqual(Object.keys(corpus.tickets.staged).filter(path=>path.endsWith("ticket.json")).sort());
  expect(ticketBullets(root).sort()).toEqual([...corpus.tickets.expected].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
