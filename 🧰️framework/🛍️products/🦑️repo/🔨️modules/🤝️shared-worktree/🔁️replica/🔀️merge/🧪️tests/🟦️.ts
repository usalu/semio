import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {mergeBytesLimitV1,mergeEditsLimitV1,mergeTextV1} from "../🟦️.ts";

const scratch=mkdtempSync(join(tmpdir(),"replica-merge-")),gitTest=Bun.spawnSync(["git","--version"]).exitCode===0?test:test.skip;
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
const bytes=(text:string):Uint8Array=>Buffer.from(text,"utf8");
const ours=(base:string,mine:string,yours:string):string|null=>{const outcome=mergeTextV1(bytes(base),bytes(mine),bytes(yours));return outcome.clean?Buffer.from(outcome.merged).toString("utf8"):null;};
const git=(base:string,mine:string,yours:string):string|null=>{
 const [b,o,t]=["base","ours","theirs"].map(name=>join(scratch,name)) as [string,string,string];
 writeFileSync(b,base);writeFileSync(o,mine);writeFileSync(t,yours);
 const result=Bun.spawnSync(["git","merge-file","-p",o,b,t],{stdout:"pipe",stderr:"pipe"});
 return result.exitCode===0?result.stdout.toString("utf8"):null;
};

test("the fixture follows its schema",()=>{
 const validate=new Ajv({strict:true,allowUnionTypes:true}).compile(schema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
});

test("the merge matches every vector",()=>{
 for(const vector of fixture.vectors)expect(ours(vector.base,vector.ours,vector.theirs),vector.name).toBe(vector.merged);
});

gitTest("every vector agrees with git merge-file",()=>{
 for(const vector of fixture.vectors)expect(git(vector.base,vector.ours,vector.theirs),vector.name).toBe(vector.merged);
});

gitTest("a clean merge of random edits is the merge git makes",()=>{
 let seed=20261123;
 const next=(limit:number):number=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;};
 const edit=(lines:string[]):string[]=>{const copy=[...lines];for(let count=1+next(3);count>0;count--){const at=next(copy.length+1),kind=next(3);if(kind===0)copy.splice(at,0,`new ${next(1000)}\n`);else if(kind===1&&at<copy.length)copy.splice(at,1);else if(at<copy.length)copy[at]=`changed ${next(1000)}\n`;}return copy;};
 let clean=0;
 for(let round=0;round<300;round++){
  const base=Array.from({length:4+next(30)},(_,index)=>`line ${index}\n`),mine=edit(base).join(""),yours=edit(base).join(""),merged=ours(base.join(""),mine,yours);
  if(merged===null)continue;
  clean++;
  expect(git(base.join(""),mine,yours),`round ${round}`).toBe(merged);
 }
 expect(clean).toBeGreaterThan(50);
},120_000);

test("binary, oversized and far apart sides are never merged",()=>{
 expect(mergeTextV1(bytes("a\n"),Buffer.from([97,0,10]),bytes("b\n"))).toEqual({clean:false,reason:"binary"});
 const huge=new Uint8Array(mergeBytesLimitV1+1).fill(97);
 expect(mergeTextV1(bytes("a\n"),huge,bytes("b\n"))).toEqual({clean:false,reason:"too-large"});
 const base=Array.from({length:mergeEditsLimitV1+2},(_,index)=>`${index}\n`).join(""),rewritten=Array.from({length:mergeEditsLimitV1+2},(_,index)=>`x${index}\n`).join("");
 expect(mergeTextV1(bytes(base),bytes(rewritten),bytes(`${base}tail\n`))).toEqual({clean:false,reason:"too-different"});
});

test("a large file with a few edits on each side merges quickly",()=>{
 const base=Array.from({length:20000},(_,index)=>`fn item_${index}() -> usize { ${index} }\n`),mine=[...base],yours=[...base];
 mine[10]="fn item_10() -> usize { 1010 }\n";yours[19990]="fn item_19990() -> usize { 0 }\n";
 const started=performance.now(),outcome=mergeTextV1(bytes(base.join("")),bytes(mine.join("")),bytes(yours.join("")));
 expect(performance.now()-started).toBeLessThan(500);
 expect(outcome.clean&&Buffer.from(outcome.merged).toString("utf8").split("\n").slice(0,base.length).filter((line,index)=>line!==base[index]!.slice(0,-1)).length).toBe(2);
});
