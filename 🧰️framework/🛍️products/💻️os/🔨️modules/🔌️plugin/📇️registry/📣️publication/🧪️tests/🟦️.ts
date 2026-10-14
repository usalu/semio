import {test,expect} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixtures from "./🧫️fixtures/🔣️.json";
import {parseCatalogPublicationV1,publicationPackageRootsV1} from "../🟦️.ts";
const oracle=new Ajv({strict:true}),validate=oracle.compile(schema);
function referenceAdmission(value:unknown):boolean{
  if(!validate(value))return false;
  const row=value as {ownerRoot:string;outputRoot:string;contributions:{ownerRoot:string;packageRoots:string[]}[]};
  const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
  if(!oracle.compile({type:"string",pattern:"^"+escape(row.ownerRoot)+"/"})(row.outputRoot))return false;
  if(row.contributions.some(owner=>!oracle.compile({type:"array",items:{type:"string",pattern:"^"+escape(owner.ownerRoot)+"(?:/|$)"},uniqueItems:true})(owner.packageRoots)))return false;
  return Boolean(oracle.compile({type:"array",uniqueItems:true})(row.contributions.flatMap(owner=>owner.packageRoots)));
}
for(const row of fixtures.cases)test(row.name,()=>{expect(referenceAdmission(row.value)).toBe(true);expect(publicationPackageRootsV1(parseCatalogPublicationV1(row.value))).toEqual(row.roots);const oracle=row.value.contributions.flatMap(contribution=>contribution.packageRoots);expect(oracle).toEqual(row.roots);console.log("[DEBUG] publication contract",row.name,row.roots.length);});
for(const row of fixtures.invalid)test("rejects invalid authority "+JSON.stringify(row),()=>{expect(referenceAdmission(row)).toBe(false);expect(()=>parseCatalogPublicationV1(row)).toThrow();});
test("Ajv rejects unknown authority fields",()=>{expect(validate(fixtures.invalid[2])).toBe(false);});

import {mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {readGeneratedCatalogProjection,catalogBuildInventoryV1} from "../../📖️catalog-view/🟦️.ts";
for(const row of fixtures.runtimeCases)test(row.name,()=>{
  const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Ticket artifact directory required");
  const before=process.env.SEMIO_PLUGIN_CATALOG;
  try{
    if(row.catalogRoot===null)delete process.env.SEMIO_PLUGIN_CATALOG;
    else{const path=join(root,row.catalogRoot);process.env.SEMIO_PLUGIN_CATALOG=path;if(row.catalogRoot!=="missing"){mkdirSync(path,{recursive:true});writeFileSync(join(path,"🔌️plugins.json"),row.catalogRoot==="malformed"?"{}":[].toString()||"[]");writeFileSync(join(path,"🎠️playgrounds.json"),"[]");}}
    if("reject" in row)expect(()=>readGeneratedCatalogProjection()).toThrow();
    else{expect(readGeneratedCatalogProjection()).toEqual(row.result);expect(catalogBuildInventoryV1(readGeneratedCatalogProjection()).plugins).toEqual([]);}
    console.log("[DEBUG] runtime catalog authority",row.name);
  }finally{if(before===undefined)delete process.env.SEMIO_PLUGIN_CATALOG;else process.env.SEMIO_PLUGIN_CATALOG=before;}
});

import {renderCatalogFiles} from "../../📽️projection/🟦️.ts";
test("neutral publication does not consult any specific owner",()=>{
  const empty=parseCatalogPublicationV1(fixtures.cases[0]!.value),forbidden=()=>{throw Error("Empty roster consulted repository inputs");};
  const result=renderCatalogFiles("absent-repository",{entries:forbidden,kind:forbidden,readText:forbidden,readBytes:forbidden},"exclude",empty);
  expect(result.entries).toEqual([]);expect(result.playgrounds).toEqual([]);expect(JSON.parse(result.files["🔌️plugins.json"]!)).toEqual([]);
  expect(result.files["🧩️plugins/🟦️.ts"]).not.toContain("s-hub");
  console.log("[DEBUG] original neutral projection rendered",Object.keys(result.files).length,"files without reading owner inputs");
});
