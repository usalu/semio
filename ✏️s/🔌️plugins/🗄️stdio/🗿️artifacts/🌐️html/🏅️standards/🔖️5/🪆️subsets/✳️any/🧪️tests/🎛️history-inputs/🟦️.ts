import {expect,test} from "bun:test";
import Ajv from "ajv";
import {parse} from "parse5";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";

type Node={nodeName:string;tagName?:string;value?:string;data?:string;name?:string;attrs?:{name:string;value:string}[];childNodes?:Node[]};
const root=resolve(import.meta.dir,"../..");
const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
function snapshot(text:string){
 const doc=parse(text) as unknown as Node;
 const convert=(node:Node,parent?:string):unknown=>node.nodeName==="#text"?(parent==="script"||parent==="style"?{kind:"rawText",parentKind:parent,text:node.value}:{kind:"text",text:node.value}):node.nodeName==="#comment"?{kind:"comment",text:node.data}:{kind:"element",name:node.tagName,attributes:node.attrs??[],children:(node.childNodes??[]).map(child=>convert(child,node.tagName))};
 const doctype=doc.childNodes!.find(node=>node.nodeName==="#documentType");
 return {schema:"stdio.html",...(doctype?{doctype:`DOCTYPE ${doctype.name}`} : {}),root:convert(doc.childNodes!.find(node=>node.tagName==="html")!)};
}
const beforeHtml="<!DOCTYPE html><html><head></head><body><p class=\"lead\">One</p></body></html>";
const rows=[
 {id:"text",leaf:"✍️set-text",mutation:{mutation:"setText",path:[1,0,0],text:"Hallo 😀"},afterHtml:"<!DOCTYPE html><html><head></head><body><p class=\"lead\">Hallo 😀</p></body></html>"},
 {id:"doctype",leaf:"📜set-doctype",mutation:{mutation:"setDoctype",doctype:null},afterHtml:"<html><head></head><body><p class=\"lead\">One</p></body></html>"},
 {id:"target",leaf:"🔖set-attribute",beforeHtml:"<!DOCTYPE html><html><head></head><body><p class=\"lead\">One</p><p class=\"tail\">Two</p></body></html>",mutation:{mutation:"setAttribute",path:[1,0],name:"class",value:"note"},afterHtml:"<!DOCTYPE html><html><head></head><body><p class=\"note\">One</p><p class=\"tail\">Two</p></body></html>"},
 {id:"attribute",leaf:"🔖set-attribute",mutation:{mutation:"setAttribute",path:[1,0],name:"class",value:"note"},afterHtml:"<!DOCTYPE html><html><head></head><body><p class=\"note\">One</p></body></html>"}
];
test("HTML committed text doctype and attribute intents match independent HTML5 documents",()=>{
 const ajv=new Ajv({strict:false});const validateSnapshot=ajv.compile(read("🧬️schema/📸️snapshot/🔣️.json"));
 const before=snapshot(beforeHtml);expect(validateSnapshot(before)).toBe(true);
 for(const row of rows){
  const before=snapshot(row.beforeHtml??beforeHtml);expect(validateSnapshot(before)).toBe(true);
  const after=snapshot(row.afterHtml);expect(validateSnapshot(after)).toBe(true);
  const leaf=read(`🧬️schema/🧬️mutations/${row.leaf}/🧬️schema/🔣️.json`);expect((ajv.getSchema(leaf.$id)??ajv.compile(leaf))(row.mutation)).toBe(true);
  expect(after).not.toEqual(before);
  const path=resolve(root,`🧫️fixtures/🧬️history-edits/${row.id}/🦠️mutation/🔣️.json`);expect(existsSync(path)).toBe(true);
  expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation:row.mutation,before,after});
  console.log("[DEBUG] HTML committed native intent matches independent parse5",row.id);
 }
});
