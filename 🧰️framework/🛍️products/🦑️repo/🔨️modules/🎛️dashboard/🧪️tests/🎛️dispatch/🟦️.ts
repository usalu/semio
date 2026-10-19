import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync,existsSync } from "node:fs";
import { join, resolve, relative, dirname } from "node:path";
import ts from "typescript";
import { parse as parseToml } from "@iarna/toml";
import schema from "../../🧬️schema/🎛️dispatch/🔣️.json" with { type: "json" };
import fixture from "../🧫️fixtures/🎛️dispatch/🔣️.json" with { type: "json" };

/** ⚖️ Runs the advertised observation selector against the actual carved Rust owner and received invocation. */
test("borrowed observation controls select the exact persistent library law with one Original",async()=>{
 expect(new Ajv2020({strict:true}).validate(schema,fixture)).toBe(true);const row=fixture.borrowedObservations,root=process.cwd(),text=readFileSync(join(root,row.script),"utf8"),tree=ts.createSourceFile(row.script,text,ts.ScriptTarget.Latest,true),owner=tree.statements.find((node):node is ts.ClassDeclaration=>ts.isClassDeclaration(node)&&node.name?.text==="BorrowedObservationScript");expect(owner).toBeDefined();
 const manifest=parseToml(readFileSync(join(root,row.manifest),"utf8"))as{package:{name:string;workspace:string};lib:{name:string};features?:unknown};expect(Bun.TOML.parse(readFileSync(join(root,row.manifest),"utf8"))).toEqual(manifest);expect(relative(root,resolve(dirname(join(root,row.manifest)),manifest.package.workspace))).toBe(row.workspace);expect(manifest.features).toBeUndefined();const database=new Database(":memory:");
 try{const selection=database.query("select ?1 package,?2 library,?3 law").get(manifest.package.name,manifest.lib.name,row.law)as{package:string;library:string;law:string};expect(selection).toEqual({package:row.package,library:row.library,law:row.law});expect(readFileSync(join(root,row.lawSource),"utf8")).toContain("fn "+selection.law.split("::").at(-1)+"(");const project=JSON.parse(readFileSync(join(root,row.script.replace("📜️script.ts","📋️project.json")),"utf8"));expect(project.targets[row.control+"-source"].metadata.semio.execution).toEqual({version:1,kind:"source"});
  const preparationBudget=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🏃️process/⏱️budget/🧬️schema/🔣️.json"),"utf8")).$defs.BuildPreparationDefault.const;const {transform}=await import("esbuild"),body=owner!.getText(tree).replace("export class ","class ").replace(/import\("\.\.\/\.\.\/📋️list\/🔗️persistent\/⚖️observation\/🧪️tests\/🟦️\.ts"\)/u,"loadOracle()"),programs=[ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,(await transform(body,{loader:"ts",target:"es2022"})).code];
  for(const program of programs)for(const vector of [...row.cases,...row.rejected.map(args=>({args,native:false,buildBudgetMs:0,rejected:true}))]){const original=Object.freeze({kind:"Original"}),phase=Object.freeze({kind:"same-Original phase"}),events:string[]=[],policy={buildBudgetMs:vector.buildBudgetMs,level:"quick",assertionBudgets:{quick:300000},rustMinStack:"8388608"};class BundleScript{root=dirname(join(root,row.script));repoRoot=root;invocation=original;}
   const ports={BundleScript,resolve,assert:(value:boolean)=>{if(!value)throw Error("selected arguments refused");},loadOracle:async()=>({borrowedObservationOracle:()=>events.push("oracle")}),readCargoTestPolicyV1:()=>policy,buildPreparationBudgetMs:()=>preparationBudget,scriptInvocationBudget:(value:unknown,maximum:number)=>{expect(value).toBe(original);expect([vector.buildBudgetMs||preparationBudget,300000]).toContain(maximum);return Math.min(17000,maximum);},configuredExactCargoLawPolicyV1:()=>({marker:"configured"}),exactCargoCallerControlV1:(value:unknown,label:string,maximum:number)=>{expect(value).toBe(original);expect([label,maximum]).toEqual(["exact-cargo:caller",0]);return phase;},runAdmittedExactCargoLawsV1:async(request:any)=>{expect(events).toEqual(["oracle"]);expect(request.control).toBe(phase);expect(request.invocation).toBe(original);expect(request.cwd).toBe(root);expect(request.policy).toEqual({marker:"configured",buildMilliseconds:Math.min(17000,vector.buildBudgetMs||preparationBudget),lawMilliseconds:17000});expect(request.nativeEnv).toEqual({RUST_MIN_STACK:policy.rustMinStack});expect(request.manifestPaths).toEqual({[selection.package]:resolve(root,row.manifest)});expect(request.groups).toEqual([{package:selection.package,target:{kind:"lib",name:selection.library},cargoArgs:["--locked"],laws:[selection.law]}]);events.push("native");await Promise.resolve();events.push("native-settled");}};
   const Command=new Function(...Object.keys(ports),program+";return BorrowedObservationScript;")(...Object.values(ports));if("rejected"in vector){await expect(new Command().run(vector.args)).rejects.toThrow("selected arguments refused");expect(events).toEqual([]);}else{await new Command().run(vector.args);expect(events).toEqual(vector.native?["oracle","native","native-settled"]:["oracle"]);}
  }
 }finally{database.close();}
});

test("dashboard test dispatch retains one current context through every selected suite", async () => {
  const oracle = new Ajv2020({ strict: true }).compile(schema);
  expect(oracle(fixture), JSON.stringify(oracle.errors)).toBe(true);
  expect(oracle({ cases: [{ segments: ["quick"] }] })).toBe(false);
  const file = new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), text = readFileSync(file, "utf8"), tree = ts.createSourceFile(file.pathname, text, ts.ScriptTarget.Latest, true);
  expect(ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true }).diagnostics).toEqual([]);
  const command = tree.statements.find((node): node is ts.ClassDeclaration => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
  expect(command).toBeDefined();
  const compiled = ts.transpileModule(command!.getText(tree), { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true });
  expect(compiled.diagnostics).toEqual([]);
  const database = new Database(":memory:");
  try { for (const row of fixture.cases) {
    const invocation = Object.freeze({ fixture: "received", control: { signal: new AbortController().signal } }), control = Object.freeze({ signal: new AbortController().signal }), options = Object.freeze({ invocation, control, workspace: process.cwd(), mode: "prototype", network: { offline: true } });
    const selected: unknown[][] = [], owned: unknown[][] = [], battles: unknown[][] = [], installed: unknown[][] = [], native: unknown[][] = [], factories: unknown[][] = [];
    let leased = false, authorityCustody = false;
    const active = new Set<unknown>(), events: string[] = [], authorityCall = (call: unknown[]) => (call[1] as string[]).some(value => value.replaceAll("\\", "/").endsWith("🧭️authority/🟦️.ts"));
    const ports = {
      BundleScript: class { root = join(process.cwd(), "dashboard"); repoRoot = process.cwd(); invocation = invocation; },
      process: { env: {}, execPath: process.execPath }, join, crates: ["dashboard-fixture"], BATTLE_SUITES: ["coverage", "journeys", "load", "smoke"],
      dashboardOptions: (...args: unknown[]) => { factories.push(args); return options; },
      dashboardExecutable: async (...args: unknown[]) => { expect(args[2]).toBe(options); events.push("select"); selected.push(args); return "installed-dashboard"; },
      devToolingEnv: (env: unknown = {}) => env, repoTestArtifactEnvironment: () => ({}),
      scriptInvocationEnvironment: (value: unknown, env: unknown) => { expect(value).toBe(invocation); return { ...(env as object), received: invocation }; },
      compilerAdmissionBudgetV1: (value: unknown) => { expect(value).toBe(control); return {}; },
      mkdir: async () => {},
      runOwnedCommand: async (...args: unknown[]) => { const env = (args[5] as { env: { received: unknown; SEMIO_DASHBOARD_BIN?: string } }).env; expect(active.size).toBe(0); active.add(args[3]); expect(env.received).toBe(invocation); if (authorityCall(args)) authorityCustody = leased; if (env.SEMIO_DASHBOARD_BIN) expect(leased).toBe(true); const event = authorityCall(args) ? "authority" : String(args[3]); events.push(event); owned.push(args); await Promise.resolve(); if (env.SEMIO_DASHBOARD_BIN) expect(leased).toBe(true); active.delete(args[3]); events.push(event+"-settled"); },
      battle: async (...args: unknown[]) => { expect(args[3]).toBe(options); events.push("battle"); battles.push(args); },
      resolveTestLevel: (segments: string[]) => ({ rest: segments.slice(1) }),
      withInstalledDashboardV1: async (workspace: unknown, context: unknown, body: (path: string) => Promise<void>) => { expect(context).toBe(options); events.push("installed"); installed.push([workspace, context]); leased = true; try { await body("installed-dashboard"); } finally { leased = false; events.push("released"); } },
      runRepositoryCargoTests: async (...args: unknown[]) => { events.push("native"); native.push(args); },
    };
    const Command = new Function(...Object.keys(ports), compiled.outputText + ";return TestScript;")(...Object.values(ports));
    await new Command().run([...row.segments]);
    expect(factories).toEqual([[invocation, process.cwd()]]);
    expect(selected.length, row.segments.join(" ")).toBe(row.executableCalls);
    for (const call of selected) expect(call.slice(1)).toEqual([process.cwd(), options]);
    expect(owned.filter(authorityCall)).toHaveLength(1);
    expect(events.slice(0, 5)).toEqual(["select", "installed", "authority", "authority-settled", "released"]);
    expect(events.slice(5)).not.toContain("authority");
    expect(authorityCustody).toBe(row.authorityCustody);
    expect(leased).toBe(false);
    expect(active.size).toBe(0);
    expect(events).toEqual(row.events);
    expect(owned.length).toBe(row.ownedCalls);
    for (const call of owned) expect((call[5] as { signal: unknown }).signal).toBe(control.signal);
    expect(battles.map(call => call[2])).toEqual(row.battle ? [row.battle] : []);
    expect(installed.length).toBe(row.installedCalls);
    expect(native.length).toBe(Number(row.native));
    if (row.native) expect(native[0]![2]).toBe(invocation);
    const actual = { executableCalls: selected.length, owned: owned.length > 0, battle: battles[0]?.[2] ?? null, installed: installed.length > 0, native: native.length > 0, ownedCalls: owned.length, installedCalls: installed.length, authorityCustody };
    const independent = database.query("with input as (select ?1 in ('quick','long','exhaustive','') expanded) select case when expanded then 2 else 1 end executableCalls, 1 owned, case when ?1 in ('coverage','journeys','load','smoke') then ?1 else null end battle, 1 installed, expanded native, case when expanded then 4 when ?1='execution' then 2 else 1 end ownedCalls, case when expanded then 2 else 1 end installedCalls, 1 authorityCustody from input").get(row.segments[0] ?? "") as { executableCalls: number; owned: number; battle: string | null; installed: number; native: number; ownedCalls: number; installedCalls: number; authorityCustody: number };
    expect(actual).toEqual({ ...independent, owned: Boolean(independent.owned), installed: Boolean(independent.installed), native: Boolean(independent.native), authorityCustody: Boolean(independent.authorityCustody) });
    const refusal = new Error("authority fixture refusal");
    events.length = 0;
    const Refusing = new Function(...Object.keys(ports), compiled.outputText + ";return TestScript;")(...Object.values({ ...ports, runOwnedCommand: async (...args: unknown[]) => { expect(authorityCall(args)).toBe(true); expect(leased).toBe(true); events.push("authority"); await Promise.resolve(); expect(leased).toBe(true); throw refusal; } }));
    await expect(new Refusing().run([...row.segments])).rejects.toBe(refusal);
    expect(events).toEqual(["select", "installed", "authority", "released"]);
    expect(leased).toBe(false);
  } } finally { database.close(); }
  console.log(`[DEBUG] Actual dashboard test dispatcher retained the received context across ${fixture.cases.length} shared suite vectors; process ports were isolated`);
});

test("dashboard window tests select the native test owners after the crate split", async () => {
  const oracle = new Ajv2020({ strict: true }).compile(schema);
  expect(oracle(fixture), JSON.stringify(oracle.errors)).toBe(true);
  const file = new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), source = readFileSync(file, "utf8"), tree = ts.createSourceFile(file.pathname, source, ts.ScriptTarget.Latest, true);
  const body = tree.statements.find((node): node is ts.ClassDeclaration => ts.isClassDeclaration(node) && node.name?.text === "TestScript")!;
  const declaration = tree.statements.find((node): node is ts.VariableStatement => ts.isVariableStatement(node) && node.declarationList.declarations.some(value => value.name.getText(tree) === "crates"))!;
  const crates = new Function("crate", ts.transpileModule(declaration.getText(tree), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText + ";return crates;")("semio-framework-repo-dashboard");
  expect(crates).toEqual(fixture.unit);
  const invocation = Object.freeze({ fixture: "received" }), control = Object.freeze({ signal: new AbortController().signal }), options = Object.freeze({ invocation, control }), native: unknown[][] = [], commands: unknown[][] = [];
  let contexts = 0;
  const ports = {
    BundleScript: class { root = join(process.cwd(), "dashboard"); repoRoot = process.cwd(); invocation = invocation; },
    process: { env: {}, execPath: process.execPath }, join, crate: "semio-framework-repo-dashboard", crates,
    dashboardOptions: (value: unknown, workspace: string) => { contexts++; expect(value).toBe(invocation); expect(workspace).toBe(process.cwd()); return options; },
    compilerAdmissionBudgetV1: (value: unknown) => { expect(value).toBe(control); return 0; },
    scriptInvocationEnvironment: (value: unknown, env: unknown) => { expect(value).toBe(invocation); return { ...(env as object), received: invocation }; },
    resolveTestLevel: (segments: string[], level?: string) => { expect(segments).toEqual(level ? ["--nocapture"] : ["quick", "--nocapture"]); if (level) expect(level).toBe("quick"); return { rest: ["--nocapture"] }; },
    runOwnedCommand: async (...args: unknown[]) => { expect((args[5] as { signal: unknown; env: { received: unknown } }).signal).toBe(control.signal); expect((args[5] as { env: { received: unknown } }).env.received).toBe(invocation); commands.push(args); await Promise.resolve(); expect(native).toHaveLength(0); }, devToolingEnv: (env: unknown) => env, repoTestArtifactEnvironment: () => ({}),
    runRepositoryCargoTests: async (...args: unknown[]) => { native.push(args); },
  };
  const compiled = ts.transpileModule(body.getText(tree), { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true });
  expect(compiled.diagnostics).toEqual([]);
  const Command = new Function(...Object.keys(ports), compiled.outputText + ";return TestScript;")(...Object.values(ports));
  await new Command().run(["windows", "--nocapture"]);
  expect(commands).toHaveLength(1);
  expect(native).toHaveLength(fixture.windows.length);
  for (const [index, row] of fixture.windows.entries()) {
    expect(row.manifests.map(path => (parseToml(readFileSync(join(process.cwd(), path), "utf8")).package as { name: string }).name)).toEqual(row.packages);
    expect(native[index]).toEqual([row.packages, process.cwd(), invocation, [...row.extraArgs, "--test-threads", "1", "--success-output", "immediate", "--nocapture"]]);
  }
  const terminal = parseToml(readFileSync(join(process.cwd(), fixture.windows[0]!.manifests[0]!), "utf8"));
  expect((terminal.dependencies as Record<string, { path: string }>).dashboard_core!.path).toBe("../../../🧱️invocation-catalog/📦️packages/🦀️rust");
  expect(readFileSync(new URL("../../🖥️terminal/🧪️tests/🔬️unit/🦀️.rs", import.meta.url), "utf8")).toContain("fn docking_tabs_reach_all_four_corners");
  native.length = 0; commands.length = 0;
  await new Command().run(["unit", "quick", "--nocapture"]);
  expect(native).toEqual([[fixture.unit, process.cwd(), invocation, ["--nocapture"]]]);
  expect(commands).toHaveLength(0);
  expect(contexts).toBe(1);
  native.length = 0;
  const refusal = new Error("window precheck refusal"), Refusing = new Function(...Object.keys(ports), compiled.outputText + ";return TestScript;")(...Object.values({ ...ports, runOwnedCommand: async () => { await Promise.resolve(); throw refusal; } }));
  await expect(new Refusing().run(["windows", "--nocapture"])).rejects.toBe(refusal);
  expect(native).toHaveLength(0);
  console.log(`[DEBUG] Actual window dispatcher selected ${fixture.windows.flatMap(row => row.packages).length} native owners, verified through independent TOML parsing`);
});


/** 🗂️ Verifies the actual native wrapper selects explicit workspace manifests rather than inheriting repository cwd. */
test("dashboard native test selections retain their physical owning workspaces under one Original",async()=>{
 expect(new Ajv2020({strict:true}).validate(schema,fixture)).toBe(true);const root=process.cwd(),workspaceApi=await import("../../../📚️library/🗂️workspaces/🦀️cargo/🟦️.ts"),source=readFileSync(new URL("../../../📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("cargo.ts",source,ts.ScriptTarget.Latest,true),entry=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="runRepositoryCargoTests");expect(entry).toBeDefined();const compiled=ts.transpileModule(entry!.getText(tree).replace(/^export\s+/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022},reportDiagnostics:true});expect(compiled.diagnostics).toEqual([]);const database=new Database(":memory:");
 try{for(const row of [...fixture.windows,{packages:fixture.unit,manifests:fixture.unitManifests,workspace:fixture.unitWorkspace,extraArgs:[]}]){
  const entries=row.manifests.map((manifest,index)=>({name:row.packages[index]!,manifest}));expect(entries).toHaveLength(row.packages.length);
  for(const entry of entries){const manifest=resolve(root,entry.manifest),data=parseToml(readFileSync(manifest,"utf8")),pkg=data.package as {name:string;workspace:string},directory=resolve(dirname(manifest),pkg.workspace),workspace=relative(root,directory).replaceAll("\\","/")||".",owner=parseToml(readFileSync(join(directory,"Cargo.toml"),"utf8")),members=(owner.workspace as {members:string[]}).members,local=relative(directory,dirname(manifest)).replaceAll("\\","/")||".";expect(pkg.name).toBe(entry.name);expect(workspace).toBe(row.workspace);expect(workspaceApi.cargoWorkspaceForManifest(root,entry.manifest).directory).toBe(workspace);expect((database.query("select exists(select 1 from json_each(?1) where value=?2) included").get(JSON.stringify(members),local)as{included:number}).included).toBe(1);}
  const invocation=Object.freeze({kind:"received"}),phase=Object.freeze({kind:"phase"}),preparation=Object.freeze({kind:"preparation"}),events:string[]=[],requests:unknown[]=[],environment=Object.freeze({kind:"environment"});
  const ports={createIssuedResourceControlV1:(value:unknown,command:string,budget:number)=>{expect(value).toBe(invocation);expect([command,budget]).toEqual(["cargo:repository-tests",0]);return phase;},createRequestedIssuedResourceControlV1:(value:unknown,budget:number)=>{expect(value).toBe(phase);expect(budget).toBe(1000);return preparation;},checkResourceLeaseControl:(value:unknown)=>expect(value).toBe(phase),getWorkspaceRoot:()=>root,cargoRepositoryPackageSelections:(value:string,packages:string[])=>{expect(value).toBe(root);expect(packages).toEqual(row.packages);return entries;},cargoWorkspaceForManifest:workspaceApi.cargoWorkspaceForManifest,resolve,relative,join,repositoryCargoPreparationStorageV1:(value:string)=>{expect(value).toBe(root);return "storage";},prepareCargoWorkspaceInvocation:async(storage:unknown,value:string,args:string[],cwd:string,env:unknown,control:unknown)=>{expect(storage).toBe("storage");expect(value).toBe(root);expect(args).toEqual(["test","--manifest-path",resolve(root,row.manifests[0]!),...row.packages.flatMap(name=>["-p",name])]);expect(cwd).toBe(root);expect(env).toBe(environment);expect(control).toBe(preparation);events.push("prepare");await Promise.resolve();events.push("prepared");},scriptInvocationEnvironment:(value:unknown,env:unknown)=>{expect(value).toBe(invocation);expect(env).toBe(environment);return env;},testLevelBudgetMs:()=>1000,repositoryCargoTestPolicyV1:()=>({}),TEST_LEVELS:["quick"],packageTestBudgetMs:()=>1000,runRepositoryCargoTestsV1:async(request:unknown,_policy:unknown,value:string,original:unknown,control:unknown)=>{expect(events).toEqual(["prepare","prepared"]);expect(value).toBe(root);expect(original).toBe(invocation);expect(control).toBe(phase);requests.push(request);}};
  const execute=new Function(...Object.keys(ports),compiled.outputText+";return runRepositoryCargoTests;")(...Object.values(ports));await execute(row.packages,root,invocation,row.extraArgs,environment);expect(requests).toEqual([{manifestPath:resolve(root,row.manifests[0]!),packages:row.packages,cwd:root,extraArgs:row.extraArgs,environment}]);
 }}finally{database.close();}
});


/** 🌱️ Every advertised value law reaches its physical crate instead of a facade's dependency tests. */
test("value native controls dispatch every carved law through the exact owner and same Original",async()=>{
 expect(new Ajv2020({strict:true}).validate(schema,fixture)).toBe(true);const row=fixture.valueNativeRoutes,root=process.cwd(),text=readFileSync(join(root,row.script),"utf8"),tree=ts.createSourceFile(row.script,text,ts.ScriptTarget.Latest,true),preparation=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🏃️process/⏱️budget/🧬️schema/🔣️.json"),"utf8")).$defs.BuildPreparationDefault.const,{transform}=await import("esbuild"),assert=(await import("node:assert/strict")).default,database=new Database(":memory:");expect(existsSync(join(root,row.removedHelper))).toBe(false);
 try{for(const route of row.routes){
  const owner=tree.statements.find((node):node is ts.ClassDeclaration=>ts.isClassDeclaration(node)&&node.name?.text===route.class);expect(owner,route.class).toBeDefined();expect(text).toContain(`.register("${route.control}",${route.class})`);
  const expected:any[]=[],manifests:Record<string,string>={};for(const group of route.groups){const actual=parseToml(readFileSync(join(root,group.manifest),"utf8"))as{package:{name:string};lib:{name?:string;path:string}};expect(Bun.TOML.parse(readFileSync(join(root,group.manifest),"utf8"))).toEqual(actual);const selected=database.query("select ?1 package,?2 library").get(actual.package.name,actual.lib.name??actual.package.name.replaceAll("-","_"))as{package:string;library:string};expect(selected).toEqual({package:group.package,library:group.library});for(const[ordinal,law]of group.laws.entries())expect(readFileSync(join(root,group.lawSources[ordinal]!),"utf8"),law).toContain("fn "+law.split("::").at(-1)+"(");manifests[group.package]=join(root,group.manifest);expected.push({package:group.package,target:{kind:"lib",name:group.library},laws:group.laws});}
  const body=owner!.getText(tree).replace("export class ","class ").replace(/import\("\.\.\/\.\.\/✨️derive\/🛬️receive\/🔀️enum\/🧪️tests\/🟦️\.ts"\)/u,"loadPayloadOracle()"),programs=[ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,(await transform(body,{loader:"ts",target:"es2022"})).code];
  for(const program of programs)for(const vector of row.cases){const original=Object.freeze({kind:"Original",control:{signal:new AbortController().signal}}),phase=Object.freeze({kind:"same Original"}),calls:any[]=[],policy={manifestPath:join(root,row.script.replace("📜️script.ts","Cargo.toml")),buildBudgetMs:vector.buildBudgetMs,level:"quick",assertionBudgets:{quick:300000},rustMinStack:"8388608"};class BundleScript{root=dirname(join(root,row.script));repoRoot=root;invocation=original;}
   let release!:()=>void,reached!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;}),nativeReached=new Promise<void>(resolve=>{reached=resolve;});const ports={BundleScript,resolve,assert,readFileSync,semioSchemaAjvV1:()=>new Ajv2020({strict:true}),resolveTypescriptCompilerV1:()=>"fixture-tsc",loadPayloadOracle:async()=>({nativePayloadEnumOracle:()=>{}}),readCargoTestPolicyV1:()=>policy,buildPreparationBudgetMs:()=>preparation,scriptInvocationBudget:(value:unknown,maximum:number)=>{expect(value).toBe(original);return maximum===0?17000:Math.min(17000,maximum);},configuredExactCargoLawPolicyV1:()=>({marker:"configured"}),exactCargoCallerControlV1:(value:unknown,label:string,maximum:number)=>{expect(value).toBe(original);expect([label,maximum]).toEqual(["exact-cargo:caller",0]);return phase;},runOwnedCommand:async(_command:string,_args:string[],_cwd:string,_label:string,_budget:number,options:any)=>{expect(options.signal).toBe(original.control.signal);},runAdmittedExactCargoLawsV1:async(request:any)=>{calls.push(request);reached();await gate;}};
   const Command=new Function(...Object.keys(ports),program+`;return ${route.class};`)(...Object.values(ports));let settled=false;const work=new Command().run([vector.argument]);void work.then(()=>{settled=true;},()=>{});if(vector.argument==="native"){await Promise.race([nativeReached,work.then(()=>{throw Error(route.class+" returned without native law");})]);await Promise.resolve();expect(settled,route.class+" must await native completion").toBe(false);release();await work;expect(calls,route.class).toHaveLength(1);const request=calls[0];expect(request.control).toBe(phase);expect(request.invocation).toBe(original);expect(request.cwd).toBe(root);expect(request.policy.buildMilliseconds).toBe(Math.min(17000,vector.buildBudgetMs||preparation));expect(request.manifestPaths).toEqual(manifests);expect(request.groups.map(({cargoArgs,...group}:any)=>{if(cargoArgs!==undefined)expect(cargoArgs).toEqual(["--locked"]);return group;})).toEqual(expected);}else{await work;expect(calls,route.class).toEqual([]);}
  }
 }}finally{database.close();}
 console.log(`[DEBUG] Both TypeScript and esbuild dispatched ${row.routes.length} actual value controls to exact TOML owners with preserved Original custody`);
},60_000);
