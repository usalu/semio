/** 🧭️ Public help is readable before any generated storage exists. */
import {expect,test} from "bun:test";
import {createRequire} from "node:module";
import Ajv from "ajv";
import {buildInvocationRequestsHelpV1,resolveBuildModeV1} from "../🟦️.ts";
import {resolveNxBuildMode} from "../../../⚡️caching/🚀️bootstrap/🎚️mode/🟦️.ts";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./📋️schema.json";

test("portable help preserves task argument boundaries and agrees with yargs",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const parse=createRequire(import.meta.url)("yargs-parser");
 for(const row of fixture.help){const boundary=row.args.indexOf("--"),head=row.args.slice(0,boundary<0?undefined:boundary),oracle=parse(head,{boolean:["help"],alias:{help:"h"}});expect(buildInvocationRequestsHelpV1(row.args)).toBe(row.expected);expect(head.length===0||Boolean(oracle.help)).toBe(row.expected);}
});

test("the actual build CLI reads help without provisioning cache partitions",async()=>{
 for(const tool of ["nx","bazel"]){const child=Bun.spawn([process.execPath,"repo",`--tool=${tool}`,"--help"],{cwd:process.cwd(),stdout:"pipe",stderr:"pipe"}),[stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);expect(status,stderr).toBe(0);expect(stdout).toContain("bun repo [--tool=nx|bazel]");expect(stderr).not.toContain("image.json");console.log(`[DEBUG] ${tool} build CLI help completed without a cache receiver`);}
});


test("Neutral E2E targets retain the original compiler mode across both actual resolvers",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const require=createRequire(import.meta.url),parse=require("yargs-parser"),{fixedTargetBuildMode,targetBuildModes}=require("../../🎚️mode/🟨️.mjs"),db=new Database(":memory:");
 try{
 for(const row of fixture.originalModeCases){const parsed=parse(row.args,{string:["configuration"],alias:{configuration:"c"}}),encoded=row.args[0]==="run"?row.args[1]!.split(":")[2]:undefined,explicit=parsed.configuration??encoded??null,values=[explicit,row.authored,row.inherited].filter(value=>value!==null),oracle=db.query("select case when count(distinct value)>1 then null else coalesce(min(value),'prototype') end mode from json_each(?)").get(JSON.stringify(values))as{mode:string|null};expect(oracle.mode,row.id).toBe(row.expected);for(const[name,resolve]of Object.entries({generic:resolveBuildModeV1,nx:resolveNxBuildMode})){const selection={args:[...row.args],env:row.authored===null?{}:{SEMIO_BUILD_MODE:row.authored}},environment=row.inherited===null?{}:{SEMIO_BUILD_MODE:row.inherited};if(row.expected===null)expect(()=>resolve(selection,environment),name+":"+row.id).toThrow(/Conflicting.*build modes/u);else{const actual=resolve(selection,environment);expect(actual.env.SEMIO_BUILD_MODE,name+":"+row.id).toBe(oracle.mode);expect(actual.args).toEqual(row.args);expect(selection.env).toEqual(row.authored===null?{}:{SEMIO_BUILD_MODE:row.authored});console.log(`[DEBUG] ${name} neutral E2E ${row.id} retained ${actual.env.SEMIO_BUILD_MODE}`);}}}
 for(const row of fixture.fixedTargetCases)expect(fixedTargetBuildMode(row.name)??null,row.name).toBe(row.mode);const target=targetBuildModes("test-e2e",{cache:true});expect(target.defaultConfiguration).toBe("prototype");expect(target.metadata.semio.buildMode.fixed).toBeUndefined();for(const mode of["prototype","release"])expect(target.configurations[mode].env.SEMIO_BUILD_MODE).toBe(mode);
 }finally{db.close();}
});
