/** 🧭️ Public help is readable before any generated storage exists. */
import {expect,test} from "bun:test";
import {createRequire} from "node:module";
import Ajv from "ajv";
import {buildInvocationRequestsHelpV1} from "../🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./📋️schema.json";

test("portable help preserves task argument boundaries and agrees with yargs",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const parse=createRequire(import.meta.url)("yargs-parser");
 for(const row of fixture.help){const boundary=row.args.indexOf("--"),head=row.args.slice(0,boundary<0?undefined:boundary),oracle=parse(head,{boolean:["help"],alias:{help:"h"}});expect(buildInvocationRequestsHelpV1(row.args)).toBe(row.expected);expect(head.length===0||Boolean(oracle.help)).toBe(row.expected);}
});

test("the actual build CLI reads help without provisioning cache partitions",async()=>{
 for(const tool of ["nx","bazel"]){const child=Bun.spawn([process.execPath,"repo",`--tool=${tool}`,"--help"],{cwd:process.cwd(),stdout:"pipe",stderr:"pipe"}),[stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);expect(status,stderr).toBe(0);expect(stdout).toContain("bun repo [--tool=nx|bazel]");expect(stderr).not.toContain("image.json");console.log(`[DEBUG] ${tool} build CLI help completed without a cache receiver`);}
});
