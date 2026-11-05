import "./🧾️publication/🟦️.ts";
import {CARGO_BUILD_OWNER_PID_ENV} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { EventEmitter } from "node:events";
import { PassThrough } from "node:stream";
import { createInterface } from "node:readline";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";

const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const source = readFileSync(new URL("../../../🟦️.ts", import.meta.url), "utf8");
const ast = ts.createSourceFile("mcp.ts", source, ts.ScriptTarget.Latest, true);
const names = ["mcpStagingEnvironment", "stageMcpBinaryAsync", "nativeMcpBinaryStagingFor"];
const declarations = ast.statements.filter(node => ts.isFunctionDeclaration(node) ? names.includes(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some(declaration => ts.isIdentifier(declaration.name) && names.includes(declaration.name.text)));
const code = ts.transpileModule(declarations.map(node => node.getText(ast).replace(/^export /u, "")).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;

test("native staging neutral commands agree with Ajv and SQLite", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      const oracle = db.query("SELECT 'repo' command, json_array('run','@semio-tech/framework-os-mcp-rs:build','--output=stream') arguments").get() as { command: string; arguments: string };
      expect(row.command).toBe(oracle.command);
      expect(row.arguments).toEqual(JSON.parse(oracle.arguments));
    }
    expect(validate({ ...fixture, cases: fixture.cases.map((row: object) => ({ ...row, command: "bun" })) })).toBe(false);
  } finally { db.close(); }
});

for (const row of fixture.cases) test(`native staging actual ports retain argv, env, progress and signal on ${row.platform}`, async () => {
  expect(declarations.length).toBe(3);
  const calls: Array<{ command: string; args: string[]; options: Record<string, unknown> }> = [];
  const processPort = { env: { SEMIO_SENTINEL: "opaque", SEMIO_SCRIPT_INVOKE: "original-envelope", CARGO_INCREMENTAL: "1" }, pid: 99 };
  const startToolV1 = (command: string, args: string[], options: Record<string, unknown>) => {
    calls.push({ command, args, options });
    const child = Object.assign(new EventEmitter(), { stdout: new PassThrough(), stderr: new PassThrough() });
    setTimeout(() => { child.stdout.end("child-output\n"); child.stderr.end("child-diagnostic\n"); child.emit("close", 0); }, 0);
    return child;
  };
  const runToolSyncV1 = (command: string, args: string[], options: Record<string, unknown>) => {
    calls.push({ command, args, options });
    return { stdout: "child-output\n", stderr: "child-diagnostic\n", status: 0 };
  };
  const ports = new Function("startToolV1", "runToolSyncV1", "createInterface", "process", "CARGO_BUILD_OWNER_PID_ENV", "MCP_BINARY_TARGET", `${code}; return { stageMcpBinaryAsync, nativeMcpBinaryStagingFor };`)(startToolV1, runToolSyncV1, createInterface, processPort, CARGO_BUILD_OWNER_PID_ENV, row.arguments[1]);
  const controller = new AbortController(), progress: string[] = [];
  expect(await ports.stageMcpBinaryAsync("owned-root", (line: string) => progress.push(line), controller.signal)).toBe(0);
  expect(ports.nativeMcpBinaryStagingFor("owned-root").stage((line: string) => progress.push(line))).toBe(0);
  for (const call of calls) {
    expect(call.command).toBe(row.command);
    expect(call.args).toEqual(row.arguments);
    expect(call.options.cwd).toBe("owned-root");
    expect(call.options.shell).toBe(false);
    expect(call.options.env).toEqual({ ...processPort.env, [CARGO_BUILD_OWNER_PID_ENV]: "99" });
  }
  expect(calls.length).toBe(2);
  expect(calls[0].options.signal).toBe(controller.signal);
  expect(progress.filter(line => line === "child-output").length).toBe(2);
  expect(progress.filter(line => line === "child-diagnostic").length).toBe(2);
  expect(progress.filter(line => line.startsWith("[semio-os-mcp] staging via repo run")).length).toBe(2);
});

test("catalog launch names its producer and consumes only native server arguments", () => {
 const row=JSON.parse(readFileSync(new URL("./🧫️fixtures/catalog.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:false}).validate(schema.definitions.CatalogLaunch,row)).toBe(true);
 const selected=ast.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="takeMcpPluginCatalog");
 if(!selected)throw Error("Actual catalog launch parser required");
 const source=ts.transpileModule(selected.getText(ast).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 const parser=new Function("resolve",source+";return takeMcpPluginCatalog;")((root:string,path:string)=>root+"/"+path);
 const actual=parser(row.arguments,"workspace");
 expect(actual).toEqual({args:row.forwarded,env:{SEMIO_PLUGIN_CATALOG:"workspace/"+row.catalog},target:row.target});
 for(const invalid of [["--catalog-target",row.target],["--catalog",row.catalog,"--catalog-target","invalid"],[...row.arguments,"--catalog-target",row.target]])expect(()=>parser(invalid,"workspace")).toThrow();
 const db=new Database(":memory:");
 try{expect(db.query("select json_array(?1,?2) arguments").get(...row.forwarded)).toEqual({arguments:JSON.stringify(actual.args)});}finally{db.close();}
});

test("interactive MCP compiler intent is authored without a shell environment prerequisite", () => {
 const selected=ast.statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(value=>ts.isIdentifier(value.name)&&value.name.text==="MCP_STDIO_BUILD_MODE_V1"));
 if(!selected)throw Error("Authored interactive compiler intent required");
 const code=ts.transpileModule(selected.getText(ast).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 const mode=new Function(code+";return MCP_STDIO_BUILD_MODE_V1;")();
 expect(new Ajv({strict:false}).validate(schema.definitions.LaunchMode,mode)).toBe(true);
 const db=new Database(":memory:");try{expect(db.query("select coalesce(null, ?1) mode").get(mode)).toEqual({mode:"prototype"});}finally{db.close();}
});

test("catalog staging retains the projected Original environment and abort signal", async () => {
 const row=JSON.parse(readFileSync(new URL("./🧫️fixtures/catalog.json",import.meta.url),"utf8")),controller=new AbortController(),environment={SEMIO_SCRIPT_PROCESS_ENVELOPE:"projected-original",SEMIO_SCRIPT_PROVIDERS:"original-provider-designation",SEMIO_BUILD_MODE:"prototype"};
 let captured:any;const startToolV1=(command:string,args:string[],options:any)=>{captured={command,args,options};const child=Object.assign(new EventEmitter(),{stdout:new PassThrough(),stderr:new PassThrough()});queueMicrotask(()=>{child.stdout.end();child.stderr.end();child.emit("close",0);});return child;};
 const actual=new Function("startToolV1","runToolSyncV1","createInterface","process","CARGO_BUILD_OWNER_PID_ENV","MCP_BINARY_TARGET",code+";return stageMcpBinaryAsync;")(startToolV1,()=>{},createInterface,{env:{},pid:99},CARGO_BUILD_OWNER_PID_ENV,"@semio-tech/framework-os-mcp-rs:build");
 expect(await actual("root",()=>{},controller.signal,row.target,environment)).toBe(0);expect(captured.command).toBe("repo");expect(captured.args).toEqual(["run",row.target,"--output=stream"]);expect(captured.options.signal).toBe(controller.signal);expect(captured.options.env).toEqual({...environment,CARGO_INCREMENTAL:"0",[CARGO_BUILD_OWNER_PID_ENV]:"99"});
});

test("normal and interactive staging call one owned artifact producer", () => {
 const producerSource=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),producerAst=ts.createSourceFile("producer.ts",producerSource,ts.ScriptTarget.Latest,true);
 const owner=producerAst.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="buildMcpArtifactV1");if(!owner)throw Error("Shared MCP artifact producer required");
 const build=readFileSync(new URL("../../../📦️packages/🦀️rust/📜️script.ts",import.meta.url),"utf8");expect(build.includes("await buildMcpArtifactV1(this.repoRoot, this.invocation")).toBe(true);
 const dev=readFileSync(new URL("../../../../../../🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts",import.meta.url),"utf8");expect(dev.includes("await buildMcpArtifactV1(this.root,this.invocation,scriptCompilerAdmissionControlV1")).toBe(true);
 const compiled=ts.transpileModule(owner.getText(producerAst).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,calls:unknown[][]=[];
 const producer=new Function("resolve","producer","buildAdmittedCargoArtifactsV1","verifyCapturedCargoArtifactsV1","scriptInvocationEnvironment",compiled+";return buildMcpArtifactV1;")((root:string,path:string)=>root+"/"+path,schema.definitions.Producer.const,(...args:unknown[])=>{calls.push(args);return Promise.resolve();},()=>Promise.resolve(),(original:unknown)=>({original}));
 const original={},control={},diagnostics:string[]=[],diagnostic=(line:string)=>{diagnostics.push(line);};void producer("workspace",original,control,diagnostic);expect(calls).toHaveLength(1);const[manifest,args,root,request]=calls[0] as[string,string[],string,Record<string,unknown>];
 expect(manifest).toBe("workspace/"+schema.definitions.Producer.const.manifest);expect(root).toBe("workspace");expect(args).toEqual(["--package",schema.definitions.Producer.const.package,"--bin",schema.definitions.Producer.const.binary]);expect(request.invocation).toBe(original);expect(request.control).toBe(control);expect(request.sourcesRecord).toBe(schema.definitions.Producer.const.sourcesRecord);expect(request.output).toBeUndefined();expect(request.environment).toEqual({original});expect(request.diagnostic).toBe(diagnostic);(request.diagnostic as(line:string)=>void)("compiler diagnostic");expect(diagnostics).toEqual(["compiler diagnostic"]);
 const db=new Database(":memory:");try{const row=db.query("select json_array(?, ?, ?, ?) arguments").get("--package","semio-framework-os-mcp","--bin","semio-os-mcp")as{arguments:string};expect(args).toEqual(JSON.parse(row.arguments));}finally{db.close();}
});

test("native artifact diagnostics use the explicit owned sink without touching protocol stdout",()=>{
 const row=JSON.parse(readFileSync(new URL("./🧫️fixtures/diagnostics.json",import.meta.url),"utf8"));expect(new Ajv({strict:false}).validate(schema.definitions.Diagnostics,row)).toBe(true);
 const text=readFileSync(new URL("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("native.ts",text,ts.ScriptTarget.Latest,true),owner=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="writeCargoArtifactDiagnosticV1");if(!owner)throw Error("Owned diagnostic output port required");
 const code=ts.transpileModule(owner.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,out:string[]=[],err:string[]=[],write=new Function("process",code+";return writeCargoArtifactDiagnosticV1;")({stdout:{write:(line:string)=>out.push(line)}});
 for(const line of row.lines)write({diagnostic:(value:string)=>err.push(value)},line);expect(out).toEqual(row.stdout);expect(err).toEqual(row.stderr);
 const db=new Database(":memory:");try{expect(db.query("select json_group_array(value) output from json_each(?)").get(JSON.stringify(row.lines))).toEqual({output:JSON.stringify(err)});}finally{db.close();}
});
