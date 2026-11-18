import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {cpSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {inspectTypescriptCompilerV1,typescriptCompilerV1} from "../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const area=realpathSync(mkdtempSync(join(tmpdir(),"semio-tsc-")));
afterAll(()=>rmSync(area,{recursive:true,force:true}));
const api=schema.definitions.Api.const,member=fileURLToPath(new URL("../",import.meta.url)),wantedVersion="5.9.3";
const put=(file:string,text:string)=>{mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const compilerSource=(version:string,omit:readonly string[]=[])=>`exports.version=${JSON.stringify(version)};${api.functions.filter(name=>!omit.includes(name)).map(name=>`exports.${name}=()=>${JSON.stringify(name)};`).join("")}${api.objects.filter(name=>!omit.includes(name)).map(name=>`exports.${name}={};`).join("")}`;
const install=(directory:string,version:string,options:{readonly omit?:readonly string[];readonly native?:boolean}={})=>{
 put(join(directory,"package.json"),JSON.stringify(options.native?{name:"typescript",version,main:"./lib/tsc.js",exports:{".":"./lib/version.cjs"}}:{name:"typescript",version,main:"./lib/typescript.js"}));
 put(join(directory,options.native?"lib/version.cjs":"lib/typescript.js"),options.native?`exports.version=${JSON.stringify(version)};`:compilerSource(version,options.omit));
};
let counter=0;
const tree=(options:{readonly declared?:string|null;readonly root?:{readonly version:string;readonly native?:boolean;readonly omit?:readonly string[]};readonly slot?:{readonly version:string;readonly native?:boolean;readonly omit?:readonly string[]}}={})=>{
 const root=join(area,"t"+counter++),parser=join(root,"m","bootstrap","📦️parsers");
 put(join(root,"bun.lock"),"{}");put(join(root,"package.json"),"{}");
 put(join(parser,"package.json"),JSON.stringify({name:"parsers",devDependencies:options.declared===null?{}:{typescript:options.declared??wantedVersion}}));
 if(options.root)install(join(root,"node_modules","typescript"),options.root.version,options.root);
 if(options.slot)install(join(parser,"node_modules","typescript"),options.slot.version,options.slot);
 return {root,parser,options:{root,parserDirectory:parser}};
};
const refusalOf=(run:()=>unknown):any=>{try{run();}catch(error:any){return error;}throw new Error("a refusal was expected");};
const ajv=new Ajv({strict:true}),check=ajv.compile({...schema.definitions.Refusal,definitions:schema.definitions} as object);

test("bare typescript in a tree whose root holds the native 7.x package has no compiler API (the hazard)",()=>{
 const fake=tree({root:{version:"7.0.2",native:true}}),bare=createRequire(join(fake.parser,"x.mjs"))("typescript");
 expect(bare.version).toBe("7.0.2");expect(bare.preProcessFile).toBeUndefined();
});

test("the declared install slot wins over a 7.x package at the root",()=>{
 const fake=tree({root:{version:"7.0.2",native:true},slot:{version:wantedVersion}}),compiler=typescriptCompilerV1(fake.options);
 expect(compiler.version).toBe(wantedVersion);expect(compiler.preProcessFile()).toBe("preProcessFile");
});

test("an empty slot falls to the direct dependency of the root, never to anything above it",()=>{
 install(join(area,"node_modules","typescript"),wantedVersion);
 expect(typescriptCompilerV1(tree({root:{version:wantedVersion}}).options).version).toBe(wantedVersion);
 expect(refusalOf(()=>typescriptCompilerV1(tree({}).options)).refusal.code).toBe("typescript-compiler-absent");
});

test("only a 7.x package refuses with the typed reason naming path, found and wanted version, retry never, English and German",()=>{
 const fake=tree({root:{version:"7.0.2",native:true}}),error=refusalOf(()=>typescriptCompilerV1(fake.options)),refusal=error.refusal,path=join(fake.root,"node_modules","typescript");
 expect(check(refusal)).toBe(true);
 expect(refusal.code).toBe("typescript-compiler-version");expect(refusal.retry).toBe("never");expect(error.retry).toBe("never");
 expect(refusal.resolved).toBe(path);expect(refusal.found).toBe("7.0.2");expect(refusal.wanted).toBe(wantedVersion);
 for(const text of [refusal.en,refusal.de])for(const part of [path,"7.0.2",wantedVersion])expect(text).toContain(part);
 expect(refusal.en).toContain("Retrying does not help");expect(refusal.de).toContain("Wiederholen hilft nicht");
 expect(refusal.tried.map((row:any)=>row.found)).toEqual([null,"7.0.2"]);
 expect(error.message).toContain(refusal.en);expect(error.message).toContain(refusal.de);
});

test("the declared version with a missing API refuses with the names that are missing",()=>{
 const fake=tree({root:{version:wantedVersion,omit:["preProcessFile","sys"]}}),refusal=refusalOf(()=>typescriptCompilerV1(fake.options)).refusal;
 expect(check(refusal)).toBe(true);expect(refusal.code).toBe("typescript-compiler-api");expect(refusal.missing).toEqual(["preProcessFile","sys"]);
 expect(refusal.en).toContain("preProcessFile, sys");expect(refusal.de).toContain("preProcessFile, sys");
});

test("an absent install and a declaration that is not exact are typed refusals",()=>{
 const absent=refusalOf(()=>typescriptCompilerV1(tree({}).options)).refusal;
 expect(check(absent)).toBe(true);expect(absent.code).toBe("typescript-compiler-absent");expect(absent.found).toBeNull();
 for(const declared of [null,"^5.9.3","latest"]){
  const refusal=refusalOf(()=>typescriptCompilerV1(tree({declared,root:{version:wantedVersion}}).options)).refusal;
  expect(check(refusal)).toBe(true);expect(refusal.code).toBe("typescript-compiler-declaration");expect(refusal.wanted).toBeNull();
 }
});

test("the inspection reads two package files and never loads the compiler",()=>{
 const fake=tree({root:{version:wantedVersion}});put(join(fake.root,"node_modules","typescript","lib","typescript.js"),"throw new Error('loaded');");
 const seen=inspectTypescriptCompilerV1(fake.options);
 expect(seen.ok).toBe(true);expect(seen.version).toBe(wantedVersion);
 const bad=inspectTypescriptCompilerV1(tree({root:{version:"7.0.2",native:true}}).options);
 expect(bad.ok).toBe(false);expect(bad.refusal.code).toBe("typescript-compiler-version");
});

test("a process whose files sit in a tree with a 7.x root derives its anchors from its own location",()=>{
 const run=(slot:boolean)=>{
  const fake=tree({root:{version:"7.0.2",native:true},...slot?{slot:{version:wantedVersion}}:{}}),place=join(fake.root,"m","bootstrap","🔷️typescript-compiler");
  mkdirSync(place,{recursive:true});cpSync(join(member,"🟨️.mjs"),join(place,"🟨️.mjs"));cpSync(join(member,"🧬️schema"),join(place,"🧬️schema"),{recursive:true});
  const child=Bun.spawnSync([process.execPath,"-e",`import {typescriptCompilerV1} from ${JSON.stringify(join(place,"🟨️.mjs"))};try{console.log("loaded "+typescriptCompilerV1().version)}catch(error){console.log("refused "+error.refusal.code+" "+error.refusal.found)}`],{cwd:fake.root});
  return child.stdout.toString().trim();
 };
 expect(run(true)).toBe("loaded "+wantedVersion);
 expect(run(false)).toBe("refused typescript-compiler-version 7.0.2");
});

test("on this checkout the interface loads the declared compiler and agrees with an independent import of the package",()=>{
 const compiler=typescriptCompilerV1(),declared=JSON.parse(readFileSync(join(member,...schema.definitions.Declaration.const.manifest),"utf8")).devDependencies.typescript;
 expect(compiler.version).toBe(declared);expect(compiler.version).toBe(ts.version);
 const source="import a from 'x';export * from './y';const z=import('w');",own=compiler.preProcessFile(source,true,true).importedFiles.map((row:any)=>row.fileName),oracle=ts.preProcessFile(source,true,true).importedFiles.map(row=>row.fileName);
 expect(own).toEqual(oracle);expect(own).toEqual(["x","./y","w"]);
 for(const name of [...api.functions,...api.objects])expect(name in compiler).toBe(true);
 expect(typescriptCompilerV1()).toBe(compiler);
});
