/** 🎨️ A package's stylesheet export resolves to its physical file in the declaration phase too, where no declaration file can exist. */
import { expect,test } from "bun:test";
import { resolve } from "node:path";
import ts from "typescript";
import { workspacePackageModuleV1 } from "../🟨️.mjs";
import law from "./stylesheet-export.json";

test("a stylesheet export resolves in the runtime and the declaration phase",()=>{
 const root=resolve("."),pkg=resolve(root,law.package),containing=resolve(pkg,law.containing),output=resolve(pkg,law.output);
 for(const declarations of [false,true])expect(workspacePackageModuleV1(containing,law.specifier,root,ts,ts.sys,ts.ModuleKind.ESNext,declarations)).toBe(output);
});
