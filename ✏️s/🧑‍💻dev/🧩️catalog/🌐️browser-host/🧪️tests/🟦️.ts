import terminalSchema from "./🔚️terminal/🧬️schema/🔣️.json";
import terminalCorpus from "./🔚️terminal/🔣️.json";
import planCorpus from "../🏗️staging/📋️contract/🧪️tests/🔣️.json";
import {testBrowserHostPublicationPlanV1} from "../🏗️staging/📋️contract/🟦️.ts";
import type * as ts from "typescript";
import {describe,it,expect} from "bun:test";
import {createHash} from "node:crypto";
import {dirname,join,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,writeFileSync,rmSync} from "node:fs";
import {exactSpaceCreateArtifactArgs} from "../🏗️staging/🟦️.ts";
import {TEST_BROWSER_ACTIVATION_ROOT_ENV,TEST_BROWSER_HOST_RECEIPT_ENV,TEST_BROWSER_MODULE_ROOT_ENV,closeTestBrowserHostStagingV1,parseTestBrowserGisMaterializationReceiptV1,parseTestBrowserHostStagingReceiptV1,prepareTestBrowserHostRootsV1,resolveTestBrowserHostRootsV1,writeTestBrowserGisMaterializationReceiptV1} from "../🟦️.ts";
import {readActivationReceipt,ACTIVATION_RECEIPT_FILE} from "@semio-tech/framework-os-dev/activation";
  const ownedRelativeFiles = (root: string): string[] => {
    const files: string[] = [];
    const visit = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) visit(path);
        else if (entry.isFile()) files.push(relative(root, path).replaceAll("\\", "/"));
      }
    };
    visit(root);
    return files.sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  };

  describe("ticket-owned browser host staging", () => {
    it("matches the neutral schema and closes only the exact Space GIS and support DATA", async () => {
      const contractRoot = fileURLToPath(new URL("../",import.meta.url));
      const document = JSON.parse(readFileSync(join(contractRoot, "🧬️schema/🔣️.json"), "utf8"));
      const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8"));
      const { default: AjvDraft7 } = await import("ajv");
      const stagingAjv = new AjvDraft7({ strict: true, allErrors: true });
      stagingAjv.addSchema(document);
      const validate = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserHostStagingV1`)!;
      const validateMaterialization = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserGisMaterializationV1`)!;
      
      const provenanceFixture = JSON.parse(readFileSync(join(contractRoot, "🧪️tests/🧫️fixtures/🧬️selected-gis-byte-provenance-v1/🔣️.json"), "utf8"));
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      
      expect(parseTestBrowserHostStagingReceiptV1(fixture)).toEqual(fixture);
      for (const specimen of provenanceFixture.specimens) {
        const bytes = new TextEncoder().encode(specimen.utf8);
        expect(createHash("sha256").update(bytes).digest("hex")).toBe(specimen.sha256);
        expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(specimen.sha256);
      }
      expect(exactSpaceCreateArtifactArgs({ dialogs: [{ id: "createArtifact", args: [{ id: "name" }, { id: "kindChoice" }] }] })).toBe(true);
      expect(exactSpaceCreateArtifactArgs({ dialogs: [{ id: "createArtifact", args: [{ id: "name" }, { id: "kindId" }] }] })).toBe(false);
      const scriptSource = readFileSync(fileURLToPath(new URL("../🏗️staging/🟦️.ts", import.meta.url)), "utf8");
      const ts=await import("typescript"),tree=ts.createSourceFile("staging.ts",scriptSource,ts.ScriptTarget.Latest,true),calls:ts.CallExpression[]=[];
      const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);};visit(tree);
      for(const name of["withJavascriptDependencyReadStorageV1","withRepositoryCacheStoreV1","withArtifactScratchV1","withArtifactDestinationTaskV1","withComponentModelV1","withRepositoryArtifactDestinationV1","withRepositoryArtifactV1","withRepositoryArtifactTaskV1"])expect(calls.some(call=>call.expression.getText(tree)===name)).toBe(true);
      const publication=calls.find(call=>call.expression.getText(tree)==="stageRepositoryArtifacts");expect(publication?.arguments[0]?.getText(tree)).toBe("destination");
      expect(calls.some(call=>call.expression.getText(tree)==="buildComponentClosureV1")).toBe(true);
      expect(calls.some(call=>call.expression.getText(tree)==="runProbe")).toBe(false);
      expect(scriptSource).not.toContain("return resolveTestBrowserHostRootsV1");
      const hub=readFileSync(new URL("../../../../../🌎️hub/🧪️tests/🕹️gis-map-process/🟦️.ts",import.meta.url),"utf8"),hubTree=ts.createSourceFile("hub.ts",hub,ts.ScriptTarget.Latest,true),host=hubTree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="proveGisMapTwoAuthorShellProcess");expect(host&&ts.isFunctionDeclaration(host)).toBe(true);if(!host||!ts.isFunctionDeclaration(host)||!host.body)throw Error("Actual browser host consumer required");let staged=false;const visitStaging=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&node.expression.getText(hubTree)==="withTestBrowserHostV1")staged=true;ts.forEachChild(node,visitStaging);};visitStaging(host.body);expect(staged).toBe(true);

      const owner = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? (()=>{throw Error("Authored ticket fixture root required");})(), "h-"));
      const artifactRoot = owner;
      mkdirSync(artifactRoot, { recursive: true });
      try {
        const roots = prepareTestBrowserHostRootsV1(artifactRoot);
        const hostComponent = Buffer.from("space-cargo-component");
        const hostComponentSha256 = createHash("sha256").update(hostComponent).digest("hex");
        const hostCoreSha256 = createHash("sha256").update("space-component").digest("hex");
        const specimens = Object.fromEntries(provenanceFixture.specimens.map((row: any) => [row.id, row]));
        const files = new Map([
          ["🪞️vendor/.semio-artifact.json", "vendor"],
          ["🧵️shard/🟨️shard-worker.js", "shard"],
          ["🪐️space/semio_s_plugin_space_component.core.wasm", "space-component"],
          ["🪐️space/🛂️.descriptor.semio", "space-descriptor"],
          ["🪐️space/🔣️.json", JSON.stringify({ packageId: "semio:space", manifest: { pluginId: "space" } })],
          ["🪐️space/🧾️receipt.json", `${JSON.stringify({ schema: "semio.plugin.component-receipt/v1", pluginId: "space", packageId: "semio:space", profile: "release", target: "wasm32-wasip2", wasmSha256: hostComponentSha256, wasmBytes: hostComponent.byteLength, semanticDescriptorSha256: createHash("sha256").update("space-descriptor").digest("hex"), toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "3".repeat(64), provenanceSha256: "4".repeat(64) }, null, 2)}\n`],
          ["🪐️space/🌉️bridge.js", "space-bridge"],
          ["🌍️gis/semio_hub_gis_component.core.wasm", specimens["staged-core"].utf8],
          ["🌍️gis/🛂️.descriptor.semio", specimens["selected-descriptor"].utf8],
          ["🌍️gis/🌉️bridge.js", specimens["staged-bridge"].utf8],
        ]);
        for (const [name, body] of files) {
          const path = join(roots.moduleRoot, name);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, body);
        }
        const materialization = writeTestBrowserGisMaterializationReceiptV1(join(roots.moduleRoot, "🌍️gis"), { componentSha256: specimens["selected-component"].sha256, descriptorSha256: specimens["selected-descriptor"].sha256 });
        expect(validateMaterialization(materialization), JSON.stringify(validateMaterialization.errors)).toBe(true);
        expect(parseTestBrowserGisMaterializationReceiptV1(materialization)).toEqual(materialization);
        const current = { generationId: "a".repeat(64), currentSha256: "b".repeat(64), componentSha256: specimens["selected-component"].sha256, descriptorSha256: specimens["selected-descriptor"].sha256 };
        expect(() => closeTestBrowserHostStagingV1(artifactRoot, current, { byteLength: hostComponent.byteLength, sha256: "c".repeat(64) })).toThrow("receipt differs from its materialized owner");
        const closed = closeTestBrowserHostStagingV1(artifactRoot, current, { byteLength: hostComponent.byteLength, sha256: hostComponentSha256 });
        expect(validate(closed.receipt), JSON.stringify(validate.errors)).toBe(true);
        expect(closed.receipt.selectedGis).toMatchObject(current);
        expect(closed.receipt.selectedGis).toMatchObject({ stagedDescriptorSha256: specimens["selected-descriptor"].sha256, bridgeSha256: specimens["staged-bridge"].sha256, generatedCores: [{ relativePath: "semio_hub_gis_component.core.wasm", byteLength: specimens["staged-core"].utf8.length, sha256: specimens["staged-core"].sha256 }] });
        expect(closed.receipt.host).toMatchObject({ pluginId: "space", componentByteLength: hostComponent.byteLength, componentSha256: hostComponentSha256, coreSha256: hostCoreSha256 });
        expect(readActivationReceipt(closed.activationRoot).plugins.map((row) => row.pluginId)).toEqual(["gis", "space"]);
        const environment = {
          SEMIO_TEST_ARTIFACT_DIR: artifactRoot,
          [TEST_BROWSER_MODULE_ROOT_ENV]: closed.moduleRoot,
          [TEST_BROWSER_ACTIVATION_ROOT_ENV]: closed.activationRoot,
          [TEST_BROWSER_HOST_RECEIPT_ENV]: closed.receiptPath,
        };
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);
        expect(resolveTestBrowserHostRootsV1({})).toBeNull();
        expect(() => resolveTestBrowserHostRootsV1({ SEMIO_TEST_ARTIFACT_DIR: artifactRoot, [TEST_BROWSER_MODULE_ROOT_ENV]: closed.moduleRoot })).toThrow("complete closed root declaration");

        const descriptor = join(closed.moduleRoot, "🪐️space/🛂️.descriptor.semio");
        writeFileSync(descriptor, "substituted-space-descriptor");
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("module set changed");
        writeFileSync(descriptor, "space-descriptor");
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);

        const bridge = join(closed.moduleRoot, "🪐️space/🌉️bridge.js");
        writeFileSync(bridge, "substituted-space-module");
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("module set changed");
        writeFileSync(bridge, "space-bridge");
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);

        const receiptBytes = readFileSync(closed.receiptPath);
        const moduleSetSha256 = (): string => {
          const paths = ownedRelativeFiles(closed.moduleRoot);
          const hash = createHash("sha256");
          for (const path of paths) {
            const bytes = readFileSync(join(closed.moduleRoot, path));
            hash.update(`${JSON.stringify([path.replaceAll("\\", "/"), bytes.byteLength, createHash("sha256").update(bytes).digest("hex")])}\n`);
          }
          return hash.digest("hex");
        };
        for (const hostile of provenanceFixture.hostiles) {
          if (hostile.target === "host-receipt") {
            const changed: any = JSON.parse(JSON.stringify(closed.receipt));
            changed.selectedGis.componentSha256 = hostile.replacement;
            writeFileSync(closed.receiptPath, `${JSON.stringify(changed)}\n`);
          } else {
            const target = join(closed.moduleRoot, hostile.target === "staged-descriptor" ? "🌍️gis/🛂️.descriptor.semio" : "🌍️gis/🌉️bridge.js");
            const original = readFileSync(target);
            writeFileSync(target, hostile.replacement);
            const changed: any = JSON.parse(JSON.stringify(closed.receipt));
            changed.moduleSetSha256 = moduleSetSha256();
            writeFileSync(closed.receiptPath, `${JSON.stringify(changed)}\n`);
            expect(() => resolveTestBrowserHostRootsV1(environment), hostile.id).toThrow("GIS materialization differs");
            writeFileSync(target, original);
            writeFileSync(closed.receiptPath, receiptBytes);
            continue;
          }
          expect(() => resolveTestBrowserHostRootsV1(environment), hostile.id).toThrow("GIS identity changed");
          writeFileSync(closed.receiptPath, receiptBytes);
        }

        const activationPath = join(closed.activationRoot, ACTIVATION_RECEIPT_FILE);
        const activation = readFileSync(activationPath);
        writeFileSync(activationPath, `${activation.toString("utf8").trim()} `);
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("activation changed");
        writeFileSync(activationPath, activation);
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);
      } finally {
        rmSync(owner, { recursive: true, force: true });
      }
    });
  });


it("neutral funded browser host plan is mode distinct and binds every selected Source digest",async()=>{const{default:Ajv}=await import("ajv"),schema=JSON.parse(readFileSync(new URL("../🏗️staging/📋️contract/🧬️schema/🔣️.json",import.meta.url),"utf8")),validate=new Ajv({strict:true}).compile(schema);expect(validate(planCorpus.selected)).toBe(true);for(const invalid of planCorpus.invalid){expect(validate(invalid)).toBe(false);expect(()=>testBrowserHostPublicationPlanV1("/paid/diagnostics","prototype",invalid)).toThrow();}const{Database}=await import("bun:sqlite"),db=new Database(":memory:");const input=planCorpus.selected;try{for(const mode of["prototype","release"] as const){expect(planCorpus.modes).toContain(mode);const actual=testBrowserHostPublicationPlanV1("/paid/diagnostics",mode,input),expected=createHash("sha256").update(JSON.stringify([mode,input.generationId,input.currentSha256,input.componentSha256,input.descriptorSha256])).digest("hex").slice(0,24);expect(actual.directory).toBe(join("/paid/diagnostics","browser-host",mode,expected,"browser-host"));expect(db.query("select ?=? exact").get(actual.key,expected)).toEqual({exact:1});for(const field of Object.keys(input)){const changed={...input,[field]:planCorpus.mutatedDigest};expect(testBrowserHostPublicationPlanV1("/paid/diagnostics",mode,changed).key).not.toBe(actual.key);}expect(Object.isFrozen(actual)).toBe(true);}expect(()=>testBrowserHostPublicationPlanV1("/paid/diagnostics","prototype",{...input,extra:true})).toThrow();}finally{db.close();}});


it("Shell birth setup and terminal join remain inside its actual requested phase",async()=>{
 const {default:Ajv}=await import("ajv"),{Database}=await import("bun:sqlite"),ts=await import("typescript");
 const validate=new Ajv({strict:true}).compile(terminalSchema);expect(validate(terminalCorpus)).toBe(true);
 const db=new Database(":memory:");try{db.exec("create table events(kind text,position integer,event text,primary key(kind,position))");for(const row of terminalCorpus.cases){row.trace.forEach((event,position)=>db.query("insert into events values(?,?,?)").run(row.kind,position,event));expect(db.query("select (select position from events where kind=? and event='observed')=(select position from events where kind=? and event='born')+1 observed,(select position from events where kind=? and event='close')<(select position from events where kind=? and event='phase-retired') joined").get(row.kind,row.kind,row.kind,row.kind)).toEqual({observed:1,joined:1});expect(row.primary).toBe(row.kind==="success"?"none":row.kind==="error"?"same-error":"undefined");}}finally{db.close();}
 const source=readFileSync(new URL("../../../../../🌎️hub/🧪️tests/🕹️gis-map-process/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("hub.ts",source,ts.ScriptTarget.Latest,true),peer=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="startGisMapShellPeerV1");if(!peer||!ts.isFunctionDeclaration(peer)||!peer.body)throw Error("Actual Shell peer required");let body:ts.Block|undefined;const seek=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&node.expression.getText(tree)==="withRequestedIssuedResourceControlV1"){const fn=node.arguments[2];if(fn&&ts.isArrowFunction(fn)&&ts.isBlock(fn.body))body=fn.body;}ts.forEachChild(node,seek);};seek(peer.body);if(!body)throw Error("Actual requested Shell phase required");
 const named=(statement:ts.Statement,name:string)=>ts.isVariableStatement(statement)&&statement.declarationList.declarations.some(declaration=>ts.isIdentifier(declaration.name)&&declaration.name.text===name),spawn=body.statements.findIndex(statement=>named(statement,"child")),terminal=body.statements[spawn+1];expect(spawn).toBeGreaterThanOrEqual(0);expect(terminal&&named(terminal,"terminal")).toBe(true);if(!terminal)throw Error("Immediate actual child terminal observation required");const observed:string[]=[];const walk=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isPropertyAccessExpression(node.expression)&&node.expression.name.text==="once"){const event=node.arguments[0];if(event&&ts.isStringLiteral(event))observed.push(event.text);}ts.forEachChild(node,walk);};walk(terminal);expect(observed.sort()).toEqual(["close","error"]);
 const guarded=body.statements.find(ts.isTryStatement);expect(guarded?.catchClause!==undefined&&guarded.finallyBlock!==undefined).toBe(true);if(!guarded?.finallyBlock)throw Error("Actual child setup cleanup scope required");const joins:string[]=[];const awaitWalk=(node:ts.Node):void=>{if(ts.isAwaitExpression(node))joins.push(node.expression.getText(tree));ts.forEachChild(node,awaitWalk);};awaitWalk(guarded.finallyBlock);expect(joins).toContain("terminal");expect(joins).toContain("progressWork");expect(guarded.tryBlock.getText(tree)).toContain("startedResolve()");expect(guarded.finallyBlock.getText(tree)).toContain("terminateOwnedChildTree(child)");expect(body.getText(tree)).toContain("if(failed)throw failure");
});
