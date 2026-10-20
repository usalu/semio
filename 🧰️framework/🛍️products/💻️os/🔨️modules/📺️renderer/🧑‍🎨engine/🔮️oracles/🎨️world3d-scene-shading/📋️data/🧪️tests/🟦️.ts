import { world3dInlineSurfaceFixture as surfaceFixture } from "@semio-tech/ui/testing/fixtures";
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020.js";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{runInNewContext}from"node:vm";
import ts from"typescript";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import{oracleRgbaV1,oraclePixelDeltaV1,oracleSamplerV1,oracleBrowserDataPreludeV1,oracleSurfaceCasesV1}from"../🟦️.ts";
test("oracle DATA agrees with Ajv and SQLite, including generated browser prelude",()=>{
 const validate=new Ajv2020().compile(schema),db=new Database(":memory:");
 try{for(const row of corpus){
  const data={rgba:row.rgba,sampler:row.sampler};let valid=true;
  try{oracleRgbaV1(row.rgba);oracleSamplerV1("baseColor",{baseColorTexture:row.sampler});}catch{valid=false;}
  expect(valid).toBe(row.valid);expect(Boolean(validate(data))).toBe(row.valid);
  let browserValid=true;try{runInNewContext(oracleBrowserDataPreludeV1()+"oracleSamplerV1('baseColor',input)",{input:{baseColorTexture:row.sampler}},{timeout:100});}catch{browserValid=false;}
  let samplerValid=true;try{oracleSamplerV1("baseColor",{baseColorTexture:row.sampler});}catch{samplerValid=false;}
  expect(browserValid).toBe(samplerValid);
  const sql=db.query(`select json_array_length(?1)=4 and not exists(select 1 from json_each(?1) where type!='integer' or atom<0 or atom>255) and not exists(select 1 from json_each(?2) where key not in ('wrapS','wrapT','magFilter','minFilter') or case when key in ('wrapS','wrapT') then atom not in (33071,33648,10497) when key='magFilter' then atom not in (9728,9729) else atom not in (9728,9729,9984,9985,9986,9987) end) as valid`).get(JSON.stringify(row.rgba),JSON.stringify(row.sampler));
  expect(sql).toEqual({valid:Number(row.valid)});
 }
 const actual=[0,128,255,64],expected=[1,127,253,64];
 const sql=db.query(`select a.atom-b.atom as delta from json_each(?1) a join json_each(?2) b on a.key=b.key order by a.key`).all(JSON.stringify(actual),JSON.stringify(expected));
 expect(sql).toEqual(oraclePixelDeltaV1(actual,expected).map(delta=>({delta})));
 expect(()=>oraclePixelDeltaV1(actual,[1,2])).toThrow();expect(()=>oracleSamplerV1("unknown",{})).toThrow();expect(()=>oracleSamplerV1("normal",{normalTexture:{wrapS:null}})).toThrow();
 expect(()=>oracleSamplerV1("normal",{normalTexture:{get wrapS(){throw Error("getter");}}})).toThrow();
 expect(oracleSurfaceCasesV1({pixelCases:[{id:"a"}]})).toEqual([{id:"a"}]);
 for(const value of [null,{}, {pixelCases:[null]}, {pixelCases:[[]]}])expect(()=>oracleSurfaceCasesV1(value)).toThrow();
 }finally{db.close();}
});
test("actual oracle consumes validated DATA and self-contained browser prefix",()=>{
 const oracles=["📜️script.ts","📐️scene-shading-oracle/🟦️.ts","🔭️reference-visual-oracle/🟦️.ts","🪟️authored-surface-oracle/🟦️.ts","🕸️grid-visual-oracle/🟦️.ts"].map(path=>readFileSync(new URL("../../"+path,import.meta.url),"utf8")),source=oracles.join("\n");
 const calls:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))calls.push(node.expression.text);ts.forEachChild(node,visit);};for(const [index,text] of oracles.entries())visit(ts.createSourceFile("oracle"+index+".ts",text,ts.ScriptTarget.Latest,true));
 expect(calls.filter(name=>name==="oraclePixelDeltaV1").length).toBe(4);
 expect(calls).toContain("oracleSamplerV1");expect(calls).toContain("oracleSurfaceCasesV1");expect(calls).toContain("oracleBrowserDataPreludeV1");
 expect(source).toContain('/// <reference types="@webgpu/types" />');
 expect(source).not.toContain("await build.outputs[0].text()");
 const helper=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),helperSyntax=ts.createSourceFile("data.ts",helper,ts.ScriptTarget.Latest,true);
 const sampler=helperSyntax.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="oracleSamplerV1");
 if(!sampler||!ts.isFunctionDeclaration(sampler))throw Error("Actual sampler declaration required");
 const code=ts.transpileModule(sampler.getText(helperSyntax).replace(/^export /,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText.replace(/^export /m,"");
 expect(JSON.parse(JSON.stringify(runInNewContext(code+";oracleSamplerV1('normal',{});",{},{timeout:100})))).toEqual(oracleSamplerV1("normal",{}));
});

test("committed surface sampler choices preserve the installed reference mapping",()=>{
 const db=new Database(":memory:");try{
 for(const row of oracleSurfaceCasesV1(surfaceFixture)){
  const maps=row.maps;if(maps===null||typeof maps!=="object"||Array.isArray(maps))throw Error("Committed maps required");
  for(const role of Object.keys(maps)){
   const actual=oracleSamplerV1(role,row.textureSamplers),browser=runInNewContext(oracleBrowserDataPreludeV1()+"oracleSamplerV1(role,input)",{role,input:row.textureSamplers},{timeout:100});
   expect(JSON.parse(JSON.stringify(browser))).toEqual(actual);
   const source=JSON.stringify(row.textureSamplers??{}),field=actual.field;
   const expected=db.query(`select coalesce(json_extract(?1,'$.'||?2||'.wrapS'),33071) as wrapS,coalesce(json_extract(?1,'$.'||?2||'.wrapT'),33071) as wrapT,coalesce(json_extract(?1,'$.'||?2||'.magFilter'),9728) as magFilter,coalesce(json_extract(?1,'$.'||?2||'.minFilter'),9728) as minFilter`).get(source,field);
   expect(expected).toEqual({wrapS:actual.wrapS,wrapT:actual.wrapT,magFilter:actual.magFilter,minFilter:actual.minFilter});
  }
 }
 }finally{db.close();}
});
