import assert from "node:assert/strict";
import {readFileSync,existsSync} from "node:fs";
import {join,resolve} from "node:path";
import {parse as parseToml} from "smol-toml";
import Ajv from "ajv";
import ts from "typescript";
import {parsePlaygroundBlock} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";
import {parseDevContribution} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🟦️.ts";

/** 🎥️ Binds the authored shooting route to its real Specific browser contribution. */
export function testShootingRouteV1(workspace:string):void{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const text=readFileSync(join(workspace,fixture.manifest),"utf8"),block=text.split("[[package.metadata.semio.playground]]")[1].split("\n[")[0];
 const actual=parsePlaygroundBlock(block,"shooting",fixture.cratePath);
 const oracle=(parseToml(text) as any).package.metadata.semio.playground.find((row:any)=>row.variant===fixture.variant);
 assert.equal(actual?.devContribution,fixture.contribution,"declared shooting route requires its actual Specific browser contribution");
 assert.equal(actual?.devContribution,oracle.devContribution,"independent TOML oracle");
 assert.equal(actual?.app,fixture.app);assert.deepEqual(actual?.ports,fixture.ports);
 const contribution=JSON.parse(readFileSync(join(workspace,fixture.contribution),"utf8")),parsed=parseDevContribution(contribution);
 const schemaRoot=join(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
 const ajv=new Ajv({strict:true});ajv.addKeyword({keyword:"x-semio-formats",schemaType:"array",valid:true});ajv.addSchema(JSON.parse(readFileSync(join(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🔣️.json"),"utf8")));ajv.addSchema(JSON.parse(readFileSync(join(schemaRoot,"🚀️local-hub/🧬️schema/🔣️.json"),"utf8")));
 const validate=ajv.compile(JSON.parse(readFileSync(join(schemaRoot,"🧩️contribution/🧬️schema/🔣️.json"),"utf8")));
 assert.ok(validate(contribution),JSON.stringify(validate.errors));assert.deepEqual(parsed,contribution);
 assert.equal(parsed.ownerRoot,fixture.ownerRoot);assert.equal(parsed.localHub,undefined,"shooting contribution declares no local hub provider");
 for(const key of ["viteConfig","browserEntry"] as const){assert.ok(existsSync(join(workspace,parsed[key])));assert.ok(resolve(workspace,parsed[key]).length<=256);}
 const entryText=readFileSync(join(workspace,parsed.browserEntry),"utf8"),entry=ts.createSourceFile("entry.ts",entryText,ts.ScriptTarget.Latest,true);
 const imports=entry.statements.filter(ts.isImportDeclaration).map(node=>(node.moduleSpecifier as ts.StringLiteral).text);
 assert.ok(imports.includes("@semio-tech/s-dev-catalog-ts"));assert.ok(imports.includes("@semio-tech/framework-os-dev"));
 const calls:ts.CallExpression[]=[];const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);};visit(entry);
 const boot=calls.find(call=>ts.isIdentifier(call.expression)&&call.expression.text==="bootFrameworkOsDev");
 assert.ok(boot&&ts.isObjectLiteralExpression(boot.arguments[0]),"Specific entry passes explicit boot configuration");
 const properties=boot.arguments[0].properties;
 assert.ok(properties.some(property=>ts.isShorthandPropertyAssignment(property)&&property.name.text==="variant"));
 const catalogue=properties.find(property=>ts.isPropertyAssignment(property)&&property.name.getText(entry)==="catalogRows");
 assert.ok(catalogue&&ts.isPropertyAssignment(catalogue)&&ts.isCallExpression(catalogue.initializer)&&ts.isIdentifier(catalogue.initializer.expression)&&catalogue.initializer.expression.text==="composeSpecificOsCatalogV1","Specific entry consumes its actual catalog composition");
 assert.ok(!properties.some(property=>property.name?.getText(entry)==="hostConfig"),"entry leaves host configuration to catalogue-owned selection");
 const config=readFileSync(join(workspace,parsed.viteConfig),"utf8"),configAst=ts.createSourceFile("config.ts",config,ts.ScriptTarget.Latest,true),configCalls:ts.CallExpression[]=[];
 const configVisit=(node:ts.Node):void=>{if(ts.isCallExpression(node))configCalls.push(node);ts.forEachChild(node,configVisit);};configVisit(configAst);
 assert.ok(configCalls.some(call=>ts.isIdentifier(call.expression)&&call.expression.text==="specificOsDevBuildInventoryV1"),"Specific builder selects its actual installed inventory");
 for(const source of [entryText,config])assert.deepEqual(ts.transpileModule(source,{reportDiagnostics:true,compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).diagnostics?.filter(row=>row.category===ts.DiagnosticCategory.Error),[]);
}
