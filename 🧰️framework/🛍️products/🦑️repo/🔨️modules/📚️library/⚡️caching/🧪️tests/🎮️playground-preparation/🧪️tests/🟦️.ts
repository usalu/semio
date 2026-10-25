import publicationSchema from "@semio-tech/plugin-registry-publication/schema.json";
import contributionSchema from "@semio-tech/plugin-registry-publication/build/schema/contribution.json";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {createRequire} from "node:module";
import ts from "typescript";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {parseCatalogBuildContributionV1} from "@semio-tech/plugin-registry-publication/build";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
const require=createRequire(import.meta.url);
const artifactRoot=()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Original ticket artifact directory required");return resolve(root);};
test("runtime fixture callers forward their original invocation into preparation",()=>{
 for(const name of ["🧊️live-activation","🧊️native-runtime"]){const file=new URL("../../"+name+"/🟦️.ts",import.meta.url),source=ts.createSourceFile(file.pathname,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true),calls:ts.CallExpression[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="preparePlaygroundFixtureTargetsV1")calls.push(node);ts.forEachChild(node,visit);};visit(source);expect(calls.length,name).toBe(1);expect(calls[0]!.arguments.map(node=>node.getText(source))).toEqual(name==="🧊️native-runtime"?["workspace","sourceRoot","invocation","actualPhase"]:["workspace","root","invocation","phaseControl"]);}
 const callers=readFileSync(new URL("../../⚡️cache-contracts/🧾️command-inputs/🟦️.ts",import.meta.url),"utf8");expect(callers).toContain("testWgpuLiveActivation(workspace, output, invocation, phaseControl)");expect(callers).toContain("testNativeRuntime(workspace, output, invocation, phaseControl)");
});
test("closed playground fixture drives real preparation and independent manifest/output oracles",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);expect(new Ajv({strict:true}).addSchema(publicationSchema,"https://json.schemas.assets.semio-tech.com/os/plugin/registry/publication/schema.json").compile(contributionSchema)(fixture.catalog)).toBe(true);const parsed:unknown=parseCatalogBuildContributionV1(fixture.catalog);expect(parsed).toEqual(fixture.catalog);
 const root=mkdtempSync(join(artifactRoot(),"cp-")),db=new Database(":memory:");
 try{const location=new URL("../🟦️.ts",import.meta.url).href,{preparePlaygroundFixtureTargetsV1}=await import(location);await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"cache-playground-fixture-source",maximumElapsedMilliseconds:20000},{workspaceRoot:root},Date.now()),async invocation=>{const targets=await preparePlaygroundFixtureTargetsV1(process.cwd(),root,invocation,createIssuedResourceControlV1(invocation,"cache:received-preparation",20000));db.run("CREATE TABLE expected(name TEXT PRIMARY KEY, outputs TEXT)");for(const row of fixture.expected)db.run("INSERT INTO expected VALUES(?,?)",[row.name,JSON.stringify(row.outputs)]);for(const row of db.query("SELECT name, outputs FROM expected ORDER BY name").all() as {name:string;outputs:string}[])expect(targets[row.name].outputs,row.name).toEqual(JSON.parse(row.outputs));for(const profile of fixture.profiles){expect(targets[`prepare-fixture-wgpu-${profile}`].dependsOn).toContain("renderer:wasm");expect(targets[`prepare-fixture-native-${profile}`].dependsOn).toEqual(["@semio-tech/plugin-registry:session-fixture","sample:component"]);}const manifest=require("@iarna/toml").parse(readFileSync(join(root,fixture.manifest),"utf8"));expect(manifest.package.metadata.semio["component-kind"]).toBe("plugin");expect(manifest.package.metadata.semio.playground[0].ports.wgpu).toBe(12345);});}finally{db.close();rmSync(root,{recursive:true,force:true});}
});
test("expired original invocation refuses fixture creation before receiving files",async()=>{
 const root=mkdtempSync(join(artifactRoot(),"ce-"));try{const location=new URL("../🟦️.ts",import.meta.url).href,{preparePlaygroundFixtureTargetsV1}=await import(location);await expect(withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"cache-playground-fixture-expiry",maximumElapsedMilliseconds:50},{workspaceRoot:root},Date.now()),async invocation=>{await Bun.sleep(80);await preparePlaygroundFixtureTargetsV1(process.cwd(),root,invocation,createIssuedResourceControlV1(invocation,"cache:received-preparation",20000));})).rejects.toThrow();expect(readdirSync(root)).toEqual([]);}finally{rmSync(root,{recursive:true,force:true});}
});

test("container bootstrap receives source listeners before compiled catalog publication",async()=>{
 const {containerPlaygroundDeclarationsV1}=await import("../../../📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts");await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"cache-container-source-listeners",maximumElapsedMilliseconds:30000},{workspaceRoot:process.cwd()},Date.now()),async invocation=>{const rows=await containerPlaygroundDeclarationsV1(process.cwd(),invocation),fixture=require("../../../📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json"),config=Bun.JSONC.parse(readFileSync(join(process.cwd(),".devcontainer/devcontainer.json"),"utf8")) as {forwardPorts:number[]};expect(rows.length).toBeGreaterThan(0);for(const command of fixture.forwardedCommands.filter((row:{playground?:string})=>row.playground!==undefined)){const row=rows.find(row=>row.variant===command.playground);expect(row,command.playground).toBeDefined();const port=command.slot===0?require("lodash/get")(row,["ports",command.surface]):require("lodash/get")(row,["userPorts",command.surface,command.slot-1]);expect(Number.isSafeInteger(port)).toBe(true);expect(config.forwardPorts).toContain(port);}});
},30000);
