import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { getWorkspaceRoot, repositoryCargoTestPolicyV1 } from "../../../../../🟦️.ts";

const root = getWorkspaceRoot();
const owner = resolve(import.meta.dir, "../..");
const corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📋️owner-cmd-policy/🔣️.json"), "utf8")) as { version: number; cases: { id: string; manifest?: string; cwd?: string; manifestText?: string; classification: string; policy: string }[] };
const validateCorpus = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(owner, "🧬️schema/📋️owner-cmd-policy/🔣️.json"), "utf8")));
const neutralCargo = join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo");
const validatePolicy = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(neutralCargo, "🧬️schema/🔣️.json"), "utf8")));
const foreignPolicy = JSON.parse(readFileSync(join(neutralCargo, "🧫️fixtures/🔣️.json"), "utf8")).policies[0];
const child = 'await Bun.write(process.env.SEMIO_OWNER_POLICY_RECEIPT, JSON.stringify({policy: process.env.SEMIO_CARGO_TEST_POLICY ? JSON.parse(process.env.SEMIO_CARGO_TEST_POLICY) : null, context: JSON.parse(process.env.SEMIO_PROCESS_OWNER_CONTEXT)})); console.log("[native-owner-command-policy] child=" + (process.env.SEMIO_CARGO_TEST_POLICY ? "owned" : "absent"));';

test("native owner command portable corpus admits independent Ajv", () => {
  expect(validateCorpus(corpus), JSON.stringify(validateCorpus.errors)).toBe(true);
  expect(new Set(corpus.cases.map(row => row.id)).size).toBe(corpus.cases.length);
  expect(validatePolicy(foreignPolicy), JSON.stringify(validatePolicy.errors)).toBe(true);
});

for (const row of corpus.cases) test(row.id, () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(artifacts, { recursive: true });
  const output = mkdtempSync(join(artifacts, "native-owner-command-"));
  const receiptPath = join(output, "receipt.json");
  let manifest = row.manifest, cwd = row.cwd;
  if (row.manifestText) {
    const path = join(output, "Cargo.toml");
    writeFileSync(path, row.manifestText);
    manifest = relative(root, path).replaceAll("\\", "/");
    cwd = relative(root, output).replaceAll("\\", "/");
  }
  const document = toml.parse(readFileSync(resolve(root, manifest!), "utf8")) as { package?: { name?: string }; workspace?: object };
  const classification = document.package?.name ? "package" : document.workspace ? "workspace" : "invalid";
  expect(classification).toBe(row.classification);
  const env = { ...process.env, SEMIO_CARGO_TEST_POLICY: JSON.stringify(foreignPolicy), SEMIO_TEST_LEVEL: "fundamental", SEMIO_OWNER_POLICY_RECEIPT: receiptPath, SEMIO_TEST_ARTIFACT_DIR: output };
  const command = [process.execPath, join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", manifest!, "--cwd", cwd!, "--", process.execPath, "--eval", child];
  const result = Bun.spawnSync(command, { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  writeFileSync(join(output, "stdout.log"), result.stdout);
  writeFileSync(join(output, "stderr.log"), result.stderr);
  if (row.policy === "refused") {
    expect(result.exitCode).not.toBe(0);
    expect(new TextDecoder().decode(result.stderr)).toContain("Native owner requires a package or workspace manifest");
    expect(existsSync(receiptPath)).toBe(false);
  } else {
    expect(result.exitCode, new TextDecoder().decode(result.stderr)).toBe(0);
    const receipt = JSON.parse(readFileSync(receiptPath, "utf8"));
    expect(receipt.context.cwd).toBe(resolve(root, cwd!));
    if (row.policy === "owned") {
      expect(validatePolicy(receipt.policy), JSON.stringify(validatePolicy.errors)).toBe(true);
      expect(receipt.policy).toEqual(repositoryCargoTestPolicyV1(manifest!, resolve(root, cwd!), env));
      expect(receipt.policy.manifestPath).toBe(resolve(root, manifest!));
      expect(receipt.policy).not.toEqual(foreignPolicy);
    } else expect(receipt.policy).toBeNull();
  }
  console.log(`[native-owner-command-policy] ${row.id}: ${row.policy}; oracle=${classification}`);
}, 15000);

test("interactive dashboard delegates its native owner progress", () => {
  const project=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📋️project.json"),"utf8"));
  expect(project.targets.run.options.env?.SEMIO_NATIVE_OWNER_PROGRESS).toBe("delegated");
  expect(project.targets.build.options.env?.SEMIO_NATIVE_OWNER_PROGRESS).toBeUndefined();
});

test("native progress follows the declared owner and preserves independent child output", async () => {
  const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📣️progress/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/📣️progress/🔣️.json"),"utf8")));
  expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);
  const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!artifacts)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const output=mkdtempSync(join(artifacts,"native-progress-"));
  const workspace=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📋️owner-command-policy/🔣️.json"),"utf8")).cases.find((row:{classification:string})=>row.classification==="workspace");
  const manifest=join(root,workspace.manifest),cwd=join(root,workspace.cwd);
  const {build}=await import("esbuild");
  const compiled=await build({stdin:{contents:`setTimeout(()=>console.log("[DEBUG] interactive-ready"),${corpus.durationMs});`,loader:"js"},write:false,platform:"node",format:"esm"});
  const code=compiled.outputFiles[0]!.text;
  const collect=async(args:string[],environment:NodeJS.ProcessEnv)=>{
    const process=Bun.spawn(args,{cwd:root,env:environment,stdout:"pipe",stderr:"pipe"});
    const [stdout,stderr,status]=await Promise.all([new Response(process.stdout).text(),new Response(process.stderr).text(),process.exited]);
    return {stdout,stderr,status};
  };
  const wrapper=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts");
  const [oracle,...observed]=await Promise.all([collect(["node","--eval",code],process.env),...corpus.cases.map((row:{mode:string})=>collect([process.execPath,wrapper,"native","owner-command","--manifest",relative(root,manifest),"--cwd",relative(root,cwd)||".","--","node","--eval",code],{...process.env,SEMIO_NATIVE_OWNER_PROGRESS:row.mode,SEMIO_TEST_ARTIFACT_DIR:output}))]);
  expect(oracle.status,oracle.stderr).toBe(0);
  for(let index=0;index<corpus.cases.length;index++){
    const row=corpus.cases[index],result=observed[index]!;
    writeFileSync(join(output,`${row.id}.stdout.log`),result.stdout);writeFileSync(join(output,`${row.id}.stderr.log`),result.stderr);
    expect(result.status,result.stderr).toBe(0);
    expect(result.stdout.split(/\r?\n/).filter(line=>line.startsWith("[DEBUG] interactive-ready")).join("\n")+"\n").toBe(oracle.stdout);
    expect(result.stderr.includes("[native:owner-command] running elapsedMs="),row.id).toBe(row.wrapperProgress);
  }
},40000);

test("dashboard-selected owner tests allocate their output without manual environment setup", async () => {
  const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📣️progress/🔣️.json"),"utf8")),ts=(await import("typescript")).default;
  const script=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts"),source=ts.createSourceFile(script,readFileSync(script,"utf8"),ts.ScriptTarget.Latest,true);
  const selected=source.statements.find((node:any)=>node.name?.text==="TestScript").members.find((node:any)=>node.name?.text==="run").body.statements.find((node:any)=>ts.isIfStatement(node)&&node.expression.getText(source)==='segments[0] === "native-owner-command-policy"');
  const environmentPath=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts"),environmentSyntax=ts.createSourceFile(environmentPath,readFileSync(environmentPath,"utf8"),ts.ScriptTarget.Latest,true),environmentSource=environmentSyntax.statements.find((node:any)=>node.name?.text==="repoTestArtifactEnvironment").getText(environmentSyntax).replace(/^export /,"");
  const compilers=[(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText];
  for(const compile of compilers){
    const allocate=new Function("resolve","mkdirSync",compile(environmentSource)+";return repoTestArtifactEnvironment;")(resolve,()=>{});
    let observed:NodeJS.ProcessEnv|undefined;
    const run=new Function("process","resolveTestLevel","join","repoTestArtifactEnvironment","runRepositoryTestCommand",compile(`async function execute(segments:string[]){${selected.getText(source)}}`)+";return execute;")({env:{},execPath:process.execPath},()=>{},join,(repo:string,route:string)=>allocate(repo,route,{}),async(_command:string,_args:string[],options:{env:NodeJS.ProcessEnv})=>{observed=options.env;});
    await run.call({repoRoot:root},[corpus.artifactEnvironment.route]);
    expect(observed?.SEMIO_TEST_ARTIFACT_DIR).toBe(resolve(root,corpus.artifactEnvironment.defaultRelativeRoot,corpus.artifactEnvironment.route));
  }
  console.log("[DEBUG] dashboard-selected owner test output checked through Bun and TypeScript compilers");
});
