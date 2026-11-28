/** ♿️ Locating by role and accessible name: the in-repo resolver returns the vector ids in jsdom, `@testing-library/dom` (dom-accessibility-api) returns the same elements for every query, and the page script is the very same function. */
import {describe,expect,test} from "bun:test";
import {JSDOM} from "jsdom";
import {queryAllByRole} from "@testing-library/dom";
import vectors from "./🔣️.json" with {type:"json"};
import {accessibleInstallScriptV1,describeAccessibleTargetV1,installAccessibleV1,type AccessibleTargetV1} from "../🟦️.ts";

type Query=AccessibleTargetV1&{ids:string[]};
const ids=(elements:readonly Element[])=>elements.map(element=>element.getAttribute("data-v")??"?");

describe("accessible locator",()=>{
 for(const row of vectors.cases)test(row.name,()=>{
  const dom=new JSDOM(`<!doctype html><body>${row.html}</body>`),window=dom.window as unknown as Window&typeof globalThis,api=installAccessibleV1(window);
  for(const query of row.queries as Query[]){
   const ours=api.queryAll(window.document.body,query);
   expect(ids(ours),describeAccessibleTargetV1(query)).toEqual(query.ids);
   if(query.within!==undefined||(row as {oracle?:boolean}).oracle===false)continue;
   const theirs=queryAllByRole(window.document.body,query.role as never,{name:query.exact===false?(name:string)=>name.toLowerCase().includes(query.name.toLowerCase()):query.name,hidden:false});
   expect(ids(theirs),describeAccessibleTargetV1(query)+" testing-library").toEqual(ids(ours));
  }
 });
 test("the page script installs the same resolver on window",()=>{
  const dom=new JSDOM("<!doctype html><body><button data-v='a' aria-label='Weiter'>→</button></body>",{runScripts:"outside-only"});
  dom.window.eval(accessibleInstallScriptV1());
  const api=(dom.window as unknown as {__semioAccessibleV1:ReturnType<typeof installAccessibleV1>}).__semioAccessibleV1;
  expect(ids(api.queryAll(dom.window.document.body,{role:"button",name:"Weiter"}))).toEqual(["a"]);
  expect(accessibleInstallScriptV1()).not.toMatch(/querySelector\(["'`][^*]/u);
 });
 test("targets render with their chain",()=>{
  expect(describeAccessibleTargetV1({role:"button",name:"Delete:",exact:false,within:{role:"region",name:"Design"},nth:1})).toBe('region "Design" > button "Delete:" (contains) #1');
 });
});
