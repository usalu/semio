import {buildModeEnvironment} from "../../🎚️mode/🟦️.ts";
import * as checker from "../🟦️.ts";
import {test,expect} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";
import ts from "typescript";
import Ajv from "ajv";
import {compilerStateDirectory} from "../../🗃️state/🟦️.ts";
import {resourceLeasePathV1} from "../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import stateSchema from "../../🗃️state/🧬️schema/🔣️.json";
import checkSchema from "../🧬️schema/🔣️.json";
import cases from "./🧫️fixtures/🔣️.json";

const root=resolve(import.meta.dir,"../../../../../.."),output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!output)throw Error("Compiler tests require the original artifact directory");

test("neutral runtime compiler and routing receivers keep repository tooling outside their imports",()=>{
 for(const entry of cases.entrypoints){const path=resolve(root,entry),source=readFileSync(path,"utf8"),file=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true),imports=file.statements.flatMap(node=>(ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier)?[node.moduleSpecifier.text]:[]);expect(imports.filter(value=>value.startsWith("."))).toEqual(ts.preProcessFile(source,true,true).importedFiles.map(row=>row.fileName).filter(value=>value.startsWith(".")));for(const imported of imports.filter(value=>value.startsWith("."))){const target=resolve(dirname(path),imported);expect(target.includes("/🛍️products/"),entry+" -> "+imported).toBe(false);expect(target.includes("/✏️s/")).toBe(false);expect(target.includes("/🌎️hub/")).toBe(false);}}
});

test("neutral compiler state DATA partitions agree with Ajv and an independent hash layout",()=>{
 const validate=new Ajv({strict:true}).compile(stateSchema),state={root:output,owner:"compiler-test",compiler:"typescript:1",configuration:"one",mode:"prototype" as const,platform:"test",architecture:"test"},directories=new Set<string>();
 for(const change of[{},...cases.partitions]){const candidate={...state,...change};expect(validate(candidate)).toBe(true);const identity=createHash("sha256").update(JSON.stringify([candidate.owner,candidate.compiler,candidate.configuration,candidate.platform,candidate.architecture])).digest("hex").slice(0,24),expected=join(resolve(candidate.root),"compiler",candidate.platform+"-"+candidate.architecture,candidate.mode,identity);expect(compilerStateDirectory(candidate as typeof state)).toBe(expected);directories.add(expected);}
 expect(directories.size).toBe(cases.partitions.length+1);
 for(const change of cases.invalidOwners){const candidate={...state,...change};expect(validate(candidate)).toBe(false);expect(()=>compilerStateDirectory(candidate as typeof state)).toThrow();}
});

test("neutral TypeScript request DATA requires selected intent and rejects caller lease namespaces",()=>{
 const oracle=new Ajv({strict:true}).compile(checkSchema),request={workspace:root,stateRoot:output,cwd:root,project:"tsconfig.json",owner:"compiler-test",timeoutMs:30000,mode:"prototype"};
 for(const mode of["prototype","release"]){const value={...request,mode};expect(oracle(value)).toBe(true);expect(validateJsonSchemaSubset(checkSchema,value)).toEqual([]);}
 for(const change of cases.invalidChecks){const value={...request,...change};expect(oracle(value)).toBe(false);expect(validateJsonSchemaSubset(checkSchema,value).length).toBeGreaterThan(0);}
 const {mode:_mode,...missing}=request;expect(oracle(missing)).toBe(false);expect(validateJsonSchemaSubset(checkSchema,missing).length).toBeGreaterThan(0);
});

test("independent TypeScript semantic oracle distinguishes both authored Source vectors without output",()=>{
 const options:ts.CompilerOptions={strict:true,noLib:true,noEmit:true,types:[]};let previous:ts.Program|undefined;
 for(const[source,expected]of[[cases.source,[]],[cases.source,[]],[cases.invalidSource,[2322]]]as const){const file="value.ts",host=ts.createCompilerHost(options);host.getSourceFile=name=>name===file?ts.createSourceFile(file,source,ts.ScriptTarget.ES2022,true):undefined;host.fileExists=name=>name===file;host.readFile=name=>name===file?source:undefined;host.writeFile=()=>{throw Error("Source oracle must not emit");};const program=ts.createProgram({rootNames:[file],options,host,...(previous?{oldProgram:previous}:{})});expect(program.getSemanticDiagnostics(program.getSourceFile(file)).map(row=>row.code)).toEqual([...expected]);previous=program;}
});

test("compiler and SQLite path DATA obey the resolved UTF16 boundary before materialization",()=>{
 const maximum=cases.admission.maximumPathUnits,short=join(output,"path-data"),absolute=join(short,"b".repeat(maximum-short.length-1-80)),leaf=resourceLeasePathV1(absolute,"boundary"),oracle=new Ajv({strict:true}).compile({type:"string",maxLength:maximum});expect(existsSync(absolute)).toBe(false);expect(oracle(leaf+"-journal")).toBe(true);expect((leaf+"-journal").length).toBe(maximum);
 const oversized=join(short,"b".repeat(maximum-short.length));expect(()=>resourceLeasePathV1(oversized,"boundary")).toThrow(/path/i);expect(()=>compilerStateDirectory({root:oversized,owner:"path",compiler:"typescript:test",configuration:"path",mode:"prototype"})).toThrow(/path|at most 256 character/i);expect(existsSync(absolute)).toBe(false);
});


test("actual TypeScript argument admission preserves the authored no-output compiler boundary",()=>{
 const path=resolve(import.meta.dir,"../🟦️.ts"),source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),entry=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="typecheckProject");if(!entry||!ts.isFunctionDeclaration(entry)||!entry.body)throw Error("Actual TypeScript owner required");let statements:readonly ts.Statement[]|undefined;
 const locate=(node:ts.Node):void=>{if(ts.isBlock(node)&&node.statements.some(statement=>ts.isVariableStatement(statement)&&statement.declarationList.declarations.some(row=>ts.isIdentifier(row.name)&&row.name.text==="args")))statements=node.statements;ts.forEachChild(node,locate);};locate(entry.body);if(!statements)throw Error("Actual compiler argument admission required");const index=statements.findIndex(statement=>ts.isVariableStatement(statement)&&statement.declarationList.declarations.some(row=>ts.isIdentifier(row.name)&&row.name.text==="args")),declaration=statements[index]!,next=statements[index+1],guard=next&&ts.isIfStatement(next)?next.getText(source):"",body=declaration.getText(source)+guard+";return args;",helper=(checker as unknown as Record<string,unknown>).readTypeScriptCheckArgumentsV1,admit=new Function("options","readTypeScriptCheckArgumentsV1",ts.transpile(body,{target:ts.ScriptTarget.ES2022})),oracle=new Ajv({strict:true}).compile(checkSchema),observed=cases.arguments.map(row=>{let accepted=false;try{admit({args:row.value},helper);accepted=true;}catch{}const request={workspace:root,stateRoot:output,cwd:root,project:"tsconfig.json",owner:"argument-law",timeoutMs:30000,mode:"prototype",args:row.value};return {...row,actual:accepted,schema:oracle(request)};});
 for(const row of observed){expect(row.actual,JSON.stringify(row.value)).toBe(row.accepted);expect(row.schema,JSON.stringify(row.value)).toBe(row.accepted);if(row.accepted){const parsed=ts.parseCommandLine(["--noEmit","-p","tsconfig.json",...row.value,"--incremental","true","--tsBuildInfoFile","owned/state.tsbuildinfo"]);expect(parsed.errors).toEqual([]);expect(parsed.options.noEmit).toBe(true);expect(parsed.options.incremental).toBe(true);expect(parsed.options.tsBuildInfoFile).toBe("owned/state.tsbuildinfo");}}
});


test("actual TypeScript child environment disables independent bytecode cache writers",()=>{
 const path=resolve(import.meta.dir,"../🟦️.ts"),source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);let expression:ts.Expression|undefined;const locate=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="runOwnedCommand"){const options=node.arguments[5];if(options&&ts.isObjectLiteralExpression(options)){const env=options.properties.find(row=>ts.isPropertyAssignment(row)&&ts.isIdentifier(row.name)&&row.name.text==="env");if(env&&ts.isPropertyAssignment(env))expression=env.initializer;}}ts.forEachChild(node,locate);};locate(source);if(!expression)throw Error("Actual compiler child environment required");const evaluate=new Function("mode","compilerEnvironment","buildModeEnvironment","typeScriptCheckEnvironmentV1","return "+expression.getText(source)),helper=(checker as unknown as Record<string,unknown>).typeScriptCheckEnvironmentV1;
 for(const mode of["prototype","release"]as const)for(const environment of cases.environments){const result=evaluate(mode,environment,buildModeEnvironment,helper);for(const[key,value]of Object.entries(checkSchema.definitions.CompilerEnvironment.const))expect(result[key],key).toBe(value);expect(Object.hasOwn(result,"NODE_COMPILE_CACHE")).toBe(false);expect(result.SEMIO_BUILD_MODE).toBe(mode);expect(result.AUTHORED_TEST_VALUE).toBe((environment as Record<string,unknown>).AUTHORED_TEST_VALUE);}
});
