import { test, expect } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

test("native publication and launch select the admitted variant and executable", async () => {
  const workspace = process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd();
  const require = createRequire(import.meta.url), ts = require("typescript");
  const {repoTestArtifactEnvironment} = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts")).href);
  const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoTestArtifactEnvironment(workspace, "native-entrypoint-catalog").SEMIO_TEST_ARTIFACT_DIR!;
  const root = mkdtempSync(join(artifactDirectory, "native-session-catalog-"));
  const put = (path: string, value: unknown) => {
    mkdirSync(dirname(join(root,path)), {recursive:true});
    writeFileSync(join(root,path), JSON.stringify(value));
  };
  const corpus = JSON.parse(readFileSync(join(import.meta.dir, "../../../../../🧪️tests/🧫️fixtures/🧊️wgpu/⌨️native-entrypoint/📇️catalog/🔣️.json"), "utf8"));
  const registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
  const scoped = registry + "/dist/sessions/" + corpus.variant;
  const native = resolve(import.meta.dir, "../..");
  const {readGeneratedCatalogProjection} = await import(pathToFileURL(join(workspace, registry, "📖️catalog-view/🟦️.ts")).href);
  const launch = await import(pathToFileURL(join(native, "🎟️launch/🟦️.ts")).href);
  const nativeLaunchDeclaration = JSON.parse(readFileSync(join(native,"🎟️launch/🔣️.json"), "utf8"));
  const {withScriptProcessEnvelope, createScriptProcessEnvelope} = await import(pathToFileURL(join(workspace, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")).href);
  put(registry + "/🤖️generated/🔌️plugins.json", []);
  put(registry + "/🤖️generated/🎠️playgrounds.json", []);
  put(scoped + "/🔌️plugins.json", [corpus.entry]);
  put(scoped + "/🎠️playgrounds.json", [corpus.playground]);

  const signal = new AbortController().signal;
  const publicationControl = {signal};
  const rendererControl = {signal};
  const scopedProjection = () => readGeneratedCatalogProjection(join(root,scoped));
  let published: unknown[] = [], launched: unknown[] = [], renderer: unknown[] = [];
  let configuredHost: string | undefined;
  const dependencies = {
    ...launch, nativeLaunchDeclaration, join, resolve, readFileSync, process, console,
    BundleScript: class { repoRoot = root; invocation = undefined; },
    existsSync: () => true,
    nativeRuntimeDirectory: () => join(root, "runtime"),
    readNativeCallerAuthorityV1: () => ({original:{buildMode:corpus.renderer.mode}, repositoryRoot:root}),
    createRequestedIssuedResourceControlV1: (control: unknown) => control,
    withPlaygroundSessionV1: async (_workspace: string, _invocation: unknown, variant: string, body: Function) => {
      const projection = scopedProjection();
      const plugins = projection.entries.map((row: {pluginId:string}) => ({pluginId:row.pluginId}));
      return body({variant,plugins},projection,publicationControl);
    },
    publishNativeRuntime: async (...args: unknown[]) => { published = args; },
    withNativeRuntimeV1: async (_workspace: string, _invocation: unknown, _variant: string, control: unknown, body: Function) =>
      body(join(root,"runtime"), {}, control),
    playgroundNativeHostArtifactV1: () => configuredHost,
    withNativeRendererBinaryV1: async (packageRoot: string, mode: string, cwd: string, invocation: unknown, control: unknown, body: Function) => {
      renderer = [packageRoot, mode, cwd, invocation, control];
      await body(join(root, corpus.renderer.binary[process.platform === "win32" ? "windows" : "posix"]), rendererControl);
    },
    runNativeSession: async (...args: unknown[]) => { launched = args; },
  };
  const load = (file: string, names: string[], entry: string) => {
    const source = ts.createSourceFile(file, readFileSync(file,"utf8"), ts.ScriptTarget.Latest,true);
    const declarations = names.map(name => {
      const statement = source.statements.find((node: any) => node.name?.text === name ||
        node.declarationList?.declarations.some((row: any) => row.name?.text === name));
      if (!statement) throw Error("Missing native runner declaration " + name);
      return statement.getText(source).replace(/^export /u,"");
    }).join("\n");
    const {outputText} = ts.transpileModule(declarations, {compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}});
    return new Function(...Object.keys(dependencies),outputText + "\nreturn " + entry)(...Object.values(dependencies));
  };
  const Publish = load(join(native,"📦️modules/📜️script.ts"), ["PublishScript"],"PublishScript");
  const run = load(join(native,"🟦️.ts"), ["BOOT_AXIS_FLAGS","selection","runRunScript"],"runRunScript");
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:native-catalog",maximumElapsedMilliseconds:5000},{},Date.now()), async invocation => {
    const publish = new Publish();
    publish.invocation = invocation;
    await publish.run([corpus.variant, corpus.renderer.mode]);
    const expected = require("lodash/find")([corpus.entry], {pluginId:corpus.entry.pluginId});
    expect(published).toEqual([root, invocation, corpus.variant, [{pluginId:expected.pluginId,cratePath:expected.cratePath}], publicationControl]);

    const args = [corpus.variant, corpus.renderer.mode, ...corpus.nativeArgs];
    await run({root:native,repoRoot:root,invocation}, args);
    expect(renderer).toEqual([resolve(native,"../📦️packages/🦀️rust"), corpus.renderer.mode, root, invocation, publicationControl]);
    expect(launched[0]).toBe(join(root,corpus.renderer.binary[process.platform === "win32" ? "windows" : "posix"]));
    expect(launched[5]).toBe(rendererControl);
    const environment = launched[2] as Record<string,string>;
    expect(JSON.parse(environment.SEMIO_NATIVE_LAUNCH_ADMISSION).scriptOwner).toBe("fixture:native-catalog");
    expect(environment.SEMIO_PLUGIN_MODULES).toBe(join(root,"runtime"));
    expect(launched[1]).toEqual(["--plugin",corpus.variant,"--example","🎬️demo","--app",corpus.playground.app,"--brand",corpus.playground.brand,"--smoke"]);

    configuredHost = join(root,corpus.renderer.configured);
    renderer = [];
    await run({root:native,repoRoot:root,invocation}, args);
    expect(renderer).toEqual([]);
    expect(launched[0]).toBe(configuredHost);
    expect(launched[5]).toBe(publicationControl);
  });
});
