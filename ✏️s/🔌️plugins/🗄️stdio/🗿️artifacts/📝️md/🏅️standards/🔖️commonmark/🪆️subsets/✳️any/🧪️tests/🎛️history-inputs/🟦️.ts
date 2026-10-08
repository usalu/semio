import {expect,test} from "bun:test";
import Ajv from "ajv";
import MarkdownIt from "markdown-it";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
const root=resolve(import.meta.dir,"../..");const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));const parser=new MarkdownIt("commonmark");
function snapshot(text:string){
 const tokens=parser.parse(text,{});const blocks=[];
 for(let index=0;index<tokens.length;index++){
  const token=tokens[index]!;if(token.level!==0||token.nesting!==1)continue;
  if(token.type!=="paragraph_open"&&token.type!=="heading_open")throw Error(`Unexpected block ${token.type}`);
  const children=tokens[index+1]!.children!;expect(children.every(child=>child.type==="text")).toBe(true);
  const inlines=children.map(child=>({kind:"text",text:child.content}));
  blocks.push(token.type==="heading_open"?{kind:"heading",level:Number(token.tag.slice(1)),inlines}:{kind:"paragraph",inlines});
 }
 return {schema:"stdio.md",blocks};
}
const rows=[
 {id:"inlines",leaf:"✏️set-inlines",mutation:{mutation:"setInlines",path:[],index:0,inlines:[{kind:"text",text:"Hallo 😀"}]},afterText:"Hallo 😀\n\nTwo.\n"},
 {id:"replace",leaf:"🔁replace-block",mutation:{mutation:"replaceBlock",path:[],index:0,block:{kind:"heading",level:2,inlines:[{kind:"text",text:"Title"}]}},afterText:"## Title\n\nTwo.\n"},
 {id:"insert",leaf:"➕insert-block",mutation:{mutation:"insertBlock",path:[],index:1,block:{kind:"paragraph",inlines:[{kind:"text",text:"Middle."}]}},afterText:"One.\n\nMiddle.\n\nTwo.\n"},
 {id:"remove",leaf:"➖remove-block",mutation:{mutation:"removeBlock",path:[],index:0},afterText:"Two.\n"}
];
test("Markdown committed inline and block intents match independent CommonMark documents",()=>{
 const ajv=new Ajv({strict:false});const validateSnapshot=ajv.compile(read("🧬️schema/📸️snapshot/🔣️.json"));
 const before=snapshot("One.\n\nTwo.\n");expect(validateSnapshot(before)).toBe(true);
 for(const row of rows){
  const after=snapshot(row.afterText);expect(validateSnapshot(after)).toBe(true);
  expect(ajv.compile(read(`🧬️schema/🧬️mutations/${row.leaf}/🧬️schema/🔣️.json`))(row.mutation)).toBe(true);expect(after).not.toEqual(before);
  const path=resolve(root,`🧫️fixtures/🧬️history-edits/${row.id}/🦠️mutation/🔣️.json`);expect(existsSync(path)).toBe(true);
  expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation:row.mutation,before,after});
  console.log("[DEBUG] Markdown committed native intent matches independent markdown-it",row.id);
 }
});
