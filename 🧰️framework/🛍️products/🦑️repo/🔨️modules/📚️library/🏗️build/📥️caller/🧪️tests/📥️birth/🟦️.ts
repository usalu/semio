import {expect,test} from "bun:test";
import {join,resolve} from "node:path";
import {mkdtemp,readFile,rm} from "node:fs/promises";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./📋️schema.json";
import{readDefaultBuildCliPolicyV1}from"../../../📜️script.ts";
import ts from"typescript";
import {createBuildBootstrapCapabilitiesV1} from "../../📤️child/🟦️.ts";
import type {NativeBuildCallerCapabilitiesV1} from "../../🟦️.ts";
import {registerBuildTestLawV1} from "../../../\ud83e\uddea\ufe0ftests/\ud83d\udce5\ufe0finvocation/\ud83d\udfe6\ufe0f.ts";
import {readIssuedScriptProcessEnvelopeV1,scriptProcessEnvironment,scriptInvocationBudget,advanceScriptInvocation} from "../../../../../../../../\ud83d\udd28\ufe0fmodules/\ud83c\udfc3\ufe0fprocess/\ud83d\udce6\ufe0fpackages/\ud83d\udfe6\ufe0ftypescript/\ud83e\udded\ufe0frouting/\ud83d\udfe6\ufe0f.ts";
import {runOwnedCommand} from "../../../../../../../../\ud83d\udd28\ufe0fmodules/\ud83c\udfc3\ufe0fprocess/\ud83c\udf9b\ufe0fowned-execution/\ud83d\udfe6\ufe0f.ts";

test("prebirth build grants bind every finite literal Nx launch and preserve opaque tails",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(':memory:');try{
 for(const row of fixture.cases){const value=await createBuildBootstrapCapabilitiesV1(process.cwd(),['repo',...row.args],{SEMIO_BUILD_MODE:row.mode}),grant=value as {buildMode:string;launches:readonly(readonly string[])[]};expect(grant.buildMode).toBe(row.mode);
 const oracle=db.query(`with rows(value,ordinal) as (
 select json(?1),0 where json_type(?1)!='null' union all
 select json_array('nx','run','workspace:deps-javascript','--output-style=stream'),1 where ?2 union all
 select json_array('nx','watch','--all','--includeGlobalWorkspaceFiles','--verbose','--','bun','nx','run',?3,'--output-style=stream'),2 where ?3 is not null union all
 select json_array('nx','run',?3,'--output-style=stream'),3 where ?3 is not null union all
 select json_array('nx','daemon','--start'),4 where ?3 is not null)
 select json_group_array(json(value)) as value from (select value from rows order by ordinal)`).get(JSON.stringify(row.main),Number(row.javascript),row.watch??null) as {value:string};expect(grant.launches.filter(args=>args[0]==='nx')).toEqual(JSON.parse(oracle.value));}
 }finally{db.close();}
});

registerBuildTestLawV1("closed Nx child data preserves private Original and actual Node receipt with authored ceilings","caller-birth",async original=>{
 const root=process.cwd(),before=readIssuedScriptProcessEnvelopeV1(original),args=fixture.childArguments,defaults=await createBuildBootstrapCapabilitiesV1(root,['repo',...args],{SEMIO_BUILD_MODE:'release'}) as NativeBuildCallerCapabilitiesV1,parent=(before.capabilities as Partial<NativeBuildCallerCapabilitiesV1>).native??defaults.native;
 const native={...parent,transport:{maximumBytes:Math.min(parent.transport.maximumBytes,fixture.ceilings.transport.maximumBytes),maximumLines:Math.min(parent.transport.maximumLines,fixture.ceilings.transport.maximumLines)},child:{maximumElapsedMilliseconds:parent.child.maximumElapsedMilliseconds===0?fixture.ceilings.child.maximumElapsedMilliseconds:Math.min(parent.child.maximumElapsedMilliseconds,fixture.ceilings.child.maximumElapsedMilliseconds)},network:{offline:true}},declared={...defaults,native},capabilities=await createBuildBootstrapCapabilitiesV1(root,['repo',...args],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(declared)}),directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),'caller-birth-')),receipt=join(directory,'birth.json'),env=scriptProcessEnvironment({...before,capabilities},{...process.env,SEMIO_SCRIPT_CAPABILITIES:undefined});
 let failed=false;try{await runOwnedCommand(process.execPath,[join(import.meta.dir,'📜️script.ts'),receipt],root,'build:caller-birth',scriptInvocationBudget(original,10000),{env,signal:original.control.signal,onProgress:()=>advanceScriptInvocation(original,'build:caller-birth','running')});const result=JSON.parse(await readFile(receipt,'utf8'));expect(result).toMatchObject({version:1,policy:before.policy,deadline:before.deadlineEpochMilliseconds,native,originalUnchanged:true,forgedRefused:true,undeclaredRefused:true});expect(result.node.capabilities.native).toEqual(native);expect(readIssuedScriptProcessEnvelopeV1(original)).toBe(before);}catch(error){failed=true;throw error;}finally{try{await rm(directory,{recursive:true,force:true});}catch(error){if(!failed)throw error;}}
});


/** 🏛️ Keeps ordinary callers on the same canonical authored policy as the real build entry. */
test("ordinary build CLI policy has one actual authored default before process birth",async()=>{const policy=readDefaultBuildCliPolicyV1(),source=await readFile(new URL("../../../📜️script.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("build.ts",source,ts.ScriptTarget.Latest,true),entry=tree.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text==="runBuildCliV1");expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);expect(fixture.defaultPolicy).toEqual(policy);expect(Object.isFrozen(policy)).toBe(true);expect(entry).toBeDefined();expect(entry!.getText(tree)).toContain("readDefaultBuildCliPolicyV1()");expect(entry!.getText(tree)).not.toContain('owner:"repository:build"');const db=new Database(":memory:");try{expect((db.query("select json_object('version',1,'owner','repository:build','maximumElapsedMilliseconds',0) policy").get()as{policy:string}).policy).toBe(JSON.stringify(policy));}finally{db.close();}});
