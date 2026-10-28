import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import texts from "../📖️texts/🔣️.json";
import {buildPreflightContainerV1,buildPreflightRemediesV1} from "../🟦️.ts";
import {buildPreflightProblemsV1,readBuildPreflightPolicyV1} from "../../🟦️.ts";

/** 💊️ Every refused requirement of the preflight names what to do, in English and German: the texts pass their closed schema, an independent Python selection (same texts, own rules) agrees with the code on the whole grid of platforms, probe results, tools and container context, and the container markers are read without a process start. */
const python=testPythonV1()??Bun.which("python3")??"python3",policy=readBuildPreflightPolicyV1();
const platforms=["linux","darwin","win32"] as const,exchanges=["supported","unsupported","unprobed"] as const,cases=["yes","no","unprobed"] as const,toolPool=["cc","pkg-config","hdiutil","rust","developer-tools","msvc-linker"];

test("the texts pass the closed schema, have both languages everywhere and keep their placeholders",()=>{
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(schema,"remedies");
 const validate=ajv.compile({$ref:"remedies#/definitions/Texts"});expect(validate(texts),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...texts,exchange:{en:texts.exchange.en}})).toBe(false);expect(validate({...texts,extra:1})).toBe(false);
 const rows=[texts.exchange,texts.exchangeContainer,...Object.values(texts.caseFolding),...Object.values(texts.tools).flatMap(row=>Object.values(row))];
 for(const row of rows){expect(row.en.trim()).toBe(row.en);expect(row.de.trim()).toBe(row.de);expect(row.en).not.toBe(row.de);if(row.en.includes("<directory>"))expect(row.de.includes("<Verzeichnis>")).toBe(true);}
});

test("the remedy selection equals the independent Python selection over the whole grid",()=>{
 const grid:{facts:Record<string,unknown>;requiredExchange:boolean;requiredTools:string[];container:boolean}[]=[];
 for(const platform of platforms)for(const exchange of exchanges)for(const caseSensitive of cases)for(const container of [false,true])for(const missing of [[],["cc"],["cc","pkg-config","hdiutil"],["rust"],["developer-tools","rust"],["msvc-linker","rust"],toolPool])
  grid.push({facts:{platform,architecture:"x64",availableBytes:1,exchange,caseSensitive,tools:toolPool.filter(tool=>!missing.includes(tool))},requiredExchange:policy.platforms[platform].exchange,requiredTools:toolPool,container});
 const directory=mkdtempSync(join(tmpdir(),"semio-remedies-")),textsFile=join(directory,"texts.json"),casesFile=join(directory,"cases.json");
 try{
  writeFileSync(textsFile,JSON.stringify(texts));writeFileSync(casesFile,JSON.stringify(grid));
  const expected=JSON.parse(spawnSync(python,["-I",join(import.meta.dir,"🐍️oracle","🐍️.py"),textsFile,casesFile],{encoding:"utf8"}).stdout) as string[][];
  expect(expected.length).toBe(grid.length);let nonEmpty=0;
  grid.forEach((row,index)=>{const actual=buildPreflightRemediesV1(row.facts as never,row.requiredTools,row.requiredExchange,{container:row.container});expect(actual,JSON.stringify(row)).toEqual(expected[index]!);if(actual.length)nonEmpty++;expect(actual.length%2,"two languages per remedy").toBe(0);});
  expect(nonEmpty).toBeGreaterThan(grid.length/3);expect(expected.some(rows=>rows.length===0)).toBe(true);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a swap-less case-folding workspace gets the exact advice of its platform, a healthy one gets none, and every problem line of the existing policy has its remedy where one is declared",()=>{
 const sick={platform:"linux",architecture:"x64",availableBytes:99999999999,tools:["cc"],exchange:"unsupported",caseSensitive:"no"} as const;
 const lines=buildPreflightRemediesV1(sick,["cc","pkg-config"],true,{container:true});
 expect(lines).toEqual([`Remedy: ${texts.exchange.en}`,`Abhilfe: ${texts.exchange.de}`,`Remedy: ${texts.exchangeContainer.en}`,`Abhilfe: ${texts.exchangeContainer.de}`,`Remedy: ${texts.caseFolding.linux.en}`,`Abhilfe: ${texts.caseFolding.linux.de}`,`Remedy: ${texts.tools["pkg-config"].linux.en}`,`Abhilfe: ${texts.tools["pkg-config"].linux.de}`]);
 expect(buildPreflightRemediesV1({...sick,exchange:"supported",caseSensitive:"yes",tools:["cc","pkg-config"]},["cc","pkg-config"],true,{container:true})).toEqual([]);
 const problems=buildPreflightProblemsV1(policy.platforms.linux,{...sick,availableBytes:1,tools:[]});
 expect(problems.some(line=>line.includes("directory swap"))&&problems.some(line=>line.includes("case-sensitive"))).toBe(true);
 expect(buildPreflightRemediesV1({...sick,platform:"freebsd"},[ "cc" ],true,{container:false}).filter(line=>line.startsWith("Remedy")).length,"no case remedy for an undeclared platform").toBe(1);
});

test("container detection reads variables and marker files only",()=>{
 const none=()=>false;
 expect(buildPreflightContainerV1({},none)).toBe(false);
 for(const name of ["REMOTE_CONTAINERS","DEVCONTAINER","CODESPACES"])expect(buildPreflightContainerV1({[name]:"true"},none),name).toBe(true);
 expect(buildPreflightContainerV1({REMOTE_CONTAINERS:"false"},none)).toBe(false);
 expect(buildPreflightContainerV1({},path=>path==="/.dockerenv")).toBe(true);expect(buildPreflightContainerV1({},path=>path==="/run/.containerenv")).toBe(true);expect(buildPreflightContainerV1({},path=>path==="/etc/hosts")).toBe(false);
});
