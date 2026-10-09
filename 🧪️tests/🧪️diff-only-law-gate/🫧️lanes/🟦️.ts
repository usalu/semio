import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {policyDiffOnlyFileBreaches,policyInverseSumLawUntestedLeafBreaches} from "../../../📜️script.ts";

const root=resolve(import.meta.dir,"../../.."),owner=join(root,"🧫️fixtures/🧫️diff-only-law-gate"),vectors=JSON.parse(readFileSync(join(owner,"🔣️.json"),"utf8")) as {codes:Record<string,string>;cases:{id:string;segments:string[];lines:string[];breaches:{line:number;rule:string}[]}[];leafTrees:{id:string;files:Record<string,string[]>;breaches:string[]}[]},fixture=JSON.parse(readFileSync(join(owner,"🫧️lanes/🔣️.json"),"utf8")) as {version:1;cases:{id:string;expected:string[]}[]},validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🫧️lanes/🧬️schema/🔣️.json"),"utf8")));

/** 🌳️ Derives whole-state type and restore tokens independently through the TypeScript lexical scanner. */
function oracle(source:string):string[]{
 const scanner=ts.createScanner(ts.ScriptTarget.Latest,true,ts.LanguageVariant.Standard,source),tokens:{text:string;line:number}[]=[];
 for(let kind=scanner.scan();kind!==ts.SyntaxKind.EndOfFileToken;kind=scanner.scan())tokens.push({text:scanner.getTokenText(),line:source.slice(0,scanner.getTokenStart()).split("\n").length});
 const found=new Set<string>();
 for(let index=0;index<tokens.length;index++){
  const token=tokens[index]!;
  if(token.text==="MutationDiff"&&tokens[index+1]?.text==="<"&&tokens[index+3]?.text===">"&&tokens[index+4]?.text==="for"&&tokens[index+2]?.text===tokens[index+5]?.text)found.add(token.line+"|R13");
  if(token.text==="type"&&tokens[index+1]?.text==="Diff"&&tokens[index+2]?.text==="=")found.add(token.line+"|R13");
  if(["Restore","Snapshot"].includes(token.text)&&tokens[index+1]?.text==="{")found.add(token.line+"|R14");
 }
 return [...found].sort();
}

test("strict lane vectors retain their original programs and independent closed schema",()=>{
 expect(validate(fixture)).toBe(true);expect(validate({...fixture,unknown:true})).toBe(false);expect(validate({...fixture,cases:[]})).toBe(false);
 expect(new Set(fixture.cases.map(row=>row.id)).size).toBe(4);
 for(const row of fixture.cases){const original=vectors.cases.find(candidate=>candidate.id===row.id);expect(original).toBeDefined();expect(oracle(original!.lines.join("\n"))).toEqual(row.expected);}
});

test("R13 and R14 reject whole-state diff and restore in every original ephemeral lane and type",()=>{
 for(const row of fixture.cases){const original=vectors.cases.find(candidate=>candidate.id===row.id)!;const reports=policyDiffOnlyFileBreaches(original.segments.join("/"),original.lines.join("\n")).map(record=>{const match=/:(\d+) (R1[34]):/u.exec(record.summary);return match?match[1]+"|"+match[2]:null;}).filter(Boolean).sort();expect(reports).toEqual(row.expected);console.log("[DEBUG] strict diff lane "+row.id+" breaches="+reports.length);}
});

test("complete original per-file mutation programs retain every exact rule and design-code expectation",()=>{
 for(const row of vectors.cases){
  const path=row.segments.join("/"),reports=policyDiffOnlyFileBreaches(path,row.lines.join("\n")+"\n"),actual=reports.map(record=>{const match=/^(.*):(\d+) (R\d+):([A-Z0-9-]+) /u.exec(record.summary);expect(match).not.toBeNull();expect(match![1]).toBe(path);expect(match![4]).toBe(vectors.codes[match![3]!]);return match![2]+"|"+match![3];}).sort();
  expect(actual).toEqual(row.breaches.map(record=>record.line+"|"+record.rule).sort());console.log("[DEBUG] complete original diff program "+row.id+" breaches="+actual.length);
 }
});

test("complete original R15 leaf trees retain exact authored ownership without Git mutation or source transport",()=>{
 for(const tree of vectors.leafTrees){const files=Object.keys(tree.files),reports=policyInverseSumLawUntestedLeafBreaches(files,path=>(tree.files[path]??[]).join("\n"));expect(reports.map(row=>row.scope).sort()).toEqual([...tree.breaches].sort());for(const report of reports)expect(/ R15:([A-Z0-9-]+) /u.exec(report.summary)?.[1]).toBe(vectors.codes.R15);console.log("[DEBUG] complete original diff leaf tree "+tree.id+" breaches="+reports.length);}
});
