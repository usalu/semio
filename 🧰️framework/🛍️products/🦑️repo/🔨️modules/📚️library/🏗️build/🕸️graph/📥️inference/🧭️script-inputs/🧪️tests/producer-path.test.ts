/** 🧭️ A native producer resolves despite TypeScript's portable path separators. */
import { expect,test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname,resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { workspacePackageModuleV1 } from "../🟨️.mjs";
import law from "./producer-path.json";
import schema from "./producer-path.schema.json";

test("a declared unbuilt artifact package resolves through native paths",()=>{
 expect(new Ajv().validate(schema,law)).toBe(true);
 const root=resolve("."),pkg=resolve(root,law.package),containing=resolve(pkg,law.containing),output=resolve(pkg,law.output);
 expect(readFileSync(resolve(pkg,"📋️project.json"),"utf8")).toContain("typescript-artifact-package");
 const host={...ts.sys,fileExists:(name:string)=>resolve(name)===output||ts.sys.fileExists(name),directoryExists:(name:string)=>resolve(name)===dirname(output)||ts.sys.directoryExists(name),realpath:(name:string)=>resolve(name)===output?output:ts.sys.realpath!(name)};
 const options={moduleResolution:ts.ModuleResolutionKind.NodeNext,module:ts.ModuleKind.NodeNext,allowJs:true,resolveJsonModule:true,noDtsResolution:true};
 const oracle=ts.resolveModuleName(law.specifier,containing,options,host,undefined,undefined,ts.ModuleKind.ESNext).resolvedModule?.resolvedFileName;
 expect(oracle && resolve(oracle)).toBe(output);
 const unbuilt={...ts.sys,fileExists:(name:string)=>resolve(name)===output?false:ts.sys.fileExists(name)};
 expect(resolve(workspacePackageModuleV1(containing,law.specifier,root,ts,unbuilt,ts.ModuleKind.ESNext))).toBe(output);
});

test("relative resolver answers remain outside workspace authority",()=>{
 const compiler={...ts,resolveModuleName:()=>({resolvedModule:{resolvedFileName:"relative.js"},affectingLocations:[]})};
 expect(workspacePackageModuleV1(resolve("example.ts"),"external",resolve("."),compiler,ts.sys,ts.ModuleKind.ESNext)).toBeUndefined();
});
