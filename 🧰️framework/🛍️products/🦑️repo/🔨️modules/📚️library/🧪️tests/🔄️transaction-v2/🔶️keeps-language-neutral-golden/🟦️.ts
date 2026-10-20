/** 🔶️ The keeps language neutral golden tests of the transaction plan journal v2 aggregate suite. */
import { expect, test } from "bun:test";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { noFollowTreeDigest } from "../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { ownedFilePaths, ownedFilesystemEntries } from "../../🔍️filesystem/🟦️.ts";
import { EXACT_BOUNDARY_KEYS, FAILURE_STAGES, KILL_PHASES, LEASE_PHASES, type ProtocolGolden, RESTORE_PHASES, TICKET_REL, TRANSACTION_LEDGER_BOUNDARIES_GOLDEN, TRANSACTION_PROTOCOL_GOLDEN, transactionGolden } from "../📜️boundary-contracts/🟦️.ts";
import { fixtureRunRoot, retainFixture } from "../🏗️fixture-builders/🟦️.ts";

/** 🔷 Registers the keeps language neutral golden tests of the transaction plan journal v2 aggregate suite. */
export function registerKeepsLanguageNeutralGoldenTests(): void {
  test("keeps the language-neutral golden aligned with owned no-follow enumeration", async () => {
    const harness = JSON.parse(readFileSync(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json"), "utf8"));
    const ts = await import("typescript");
    const launcher = ts.createSourceFile(harness.launcherPath, readFileSync(join(getWorkspaceRoot(), harness.launcherPath), "utf8"), ts.ScriptTarget.Latest, true);
    const allocationSource = ts.createSourceFile(harness.allocationPath, readFileSync(join(getWorkspaceRoot(), harness.allocationPath), "utf8"), ts.ScriptTarget.Latest, true);
    const testSource = ts.createSourceFile(harness.testPath, readFileSync(join(getWorkspaceRoot(), harness.testPath), "utf8"), ts.ScriptTarget.Latest, true);
    const builderSource = ts.createSourceFile(harness.builderPath, readFileSync(join(getWorkspaceRoot(), harness.builderPath), "utf8"), ts.ScriptTarget.Latest, true);
    const runValidator = builderSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === harness.runRootValidator);
    expect(runValidator).toBeDefined();
    const filterWaves: string[][][] = [], staticTitles: string[] = [];
    const visitLauncher = (node: import("typescript").Node): void => {
      if (ts.isVariableDeclaration(node) && node.name.getText(launcher) === harness.filterWavesConstant && node.initializer && ts.isArrayLiteralExpression(node.initializer)) filterWaves.push(node.initializer.elements.map((wave) => {
        if (!ts.isArrayLiteralExpression(wave) || wave.elements.some((entry) => !ts.isStringLiteral(entry))) throw new Error("Transaction shard filters must be closed literal strings");
        return wave.elements.map((entry) => (entry as import("typescript").StringLiteral).text);
      }));
      ts.forEachChild(node, visitLauncher);
    };
    const visitTests = (node: import("typescript").Node): void => {
      if (ts.isCallExpression(node) && ["test", "test.concurrent"].includes(node.expression.getText(testSource)) && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) staticTitles.push(node.arguments[0].text);
      ts.forEachChild(node, visitTests);
    };
    visitLauncher(launcher);
    visitTests(testSource);
    expect(filterWaves).toEqual([[harness.shards.map((entry: { filter: string }) => entry.filter)]]);
    expect(staticTitles).toHaveLength(14);
    const stages = FAILURE_STAGES.map((stage) => ({ title: `rolls back ${stage} with non-empty phase authority`, boundaries: [`rolledback:${stage}`] }));
    const phases = [...KILL_PHASES, ...RESTORE_PHASES, ...LEASE_PHASES].map((phase) => ({ title: `recovers parent-killed ${phase}`, boundaries: [`killed:${phase}`, `recovered:${phase}`] }));
    const terminal = ["committed", "rolled-back"].flatMap((state) => [`killed:transaction-terminal-${state}-stage-removed`, `recovered:transaction-terminal-${state}-stage-removed`]);
    const staticBoundaries: Record<string, string[]> = {
      "recovers parent-killed committed and rolled-back backup-only terminal cleanup": terminal,
      "rolls back a process-tree-killed mixed generator and commits ordinal two": ["killed:process-tree-mixed-generator", "recovered:process-tree-mixed-generator", "committed:process-tree-mixed-generator"],
      "keeps double-plan bytes stable, cancellation exact, and committed second apply immutable": ["rolledback:cancellation"],
      "recovers caught allocation and journal previous-image callback failures": ["rolledback:caught-attempt-canonical-published", "rolledback:caught-journal-previous-exchanged"],
    };
    const cases = [...staticTitles.map((title) => ({ title, boundaries: staticBoundaries[title] ?? [] })), ...stages, ...phases];
    expect(cases).toHaveLength(62);
    expect(new Set(cases.map((entry) => entry.title)).size).toBe(62);
    expect(cases.flatMap((entry) => entry.boundaries).sort()).toEqual(EXACT_BOUNDARY_KEYS);
    const filters = filterWaves[0][0].map((filter) => new RegExp(filter, "u"));
    for (const entry of cases) {
      expect(filters.filter((filter) => filter.test(entry.title))).toHaveLength(1);
      expect(filters.filter((filter) => filter.test(`transaction plan journal v2 aggregate ${entry.title}`))).toHaveLength(1);
    }
    expect(filters.map((filter) => cases.filter((entry) => filter.test(entry.title)).length)).toEqual(harness.shards.map((entry: { tests: number }) => entry.tests));
    expect(filters.map((filter) => cases.filter((entry) => filter.test(entry.title)).flatMap((entry) => entry.boundaries).length)).toEqual(harness.shards.map((entry: { boundaries: number }) => entry.boundaries));
    const allocationConstant = allocationSource.statements.find((node) => ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => declaration.name.getText(allocationSource) === harness.runOwnerConstant));
    const allocation = allocationSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === harness.bundleRootFunction);
    expect(allocationConstant).toBeDefined();
    expect(allocation).toBeDefined();
    const retention = builderSource.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === harness.retentionFunction);
    expect(retention).toBeDefined();
    const propagation: string[] = [];
    const inspectPropagation = (node: import("typescript").Node): void => {
      if (ts.isPropertyAssignment(node) && node.name.getText(launcher) === harness.runRootEnvironment) propagation.push(node.initializer.getText(launcher));
      ts.forEachChild(node, inspectPropagation);
    };
    inspectPropagation(launcher);
    expect(propagation).toEqual(["options.runRoot"]);
    const functionSource = [allocationConstant!.getText(allocationSource).replace(/^export\s+/u, ""), allocation!.getText(allocationSource).replace(/^export\s+/u, ""), runValidator!.getText(builderSource).replace(/^export\s+/u, ""), retention!.getText(builderSource).replace(/^export\s+/u, "")].join("\n");
    const compiled = [new Bun.Transpiler({ loader: "ts" }).transformSync(functionSource), ts.transpileModule(functionSource, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText];
    const roots: string[] = [];
    const allocationRepo = mkdtempSync(join(fixtureRunRoot(), "🧪️allocation-"));
    for (const code of compiled) {
      let currentRoot = "";
      const helpers = new Function("mkdirSync", "lstatSync", "join", "resolve", "isAbsolute", "parse", "relative", "sep", "dirname", "basename", "fixtureRunRoot", `${code}\nreturn { allocate: ${harness.bundleRootFunction}, validate: ${harness.runRootValidator}, retain: ${harness.retentionFunction} };`)(mkdirSync, lstatSync, join, resolve, isAbsolute, parse, relative, sep, dirname, basename, () => currentRoot) as { allocate: (repoRoot: string, runId: string) => string; validate: (repoRoot: string, runId: string, runRoot: string) => string; retain: (root: string) => void };
      const ids = [0, 1].map(() => `${process.pid}-${crypto.randomUUID()}`), pair: string[] = [];
      for (const id of ids) {
        const bundle = helpers.allocate(allocationRepo, id), root = dirname(bundle);
        pair.push(root);
        roots.push(root);
        expect(dirname(root)).toBe(join(allocationRepo, harness.runOwnerDirectory));
        expect(basename(root)).toBe(harness.runRootPrefix + id.slice(id.indexOf("-") + 1));
        expect(basename(bundle)).toBe(harness.bundleDirectory);
        expect(lstatSync(root).isDirectory()).toBe(true);
        expect(lstatSync(root).isSymbolicLink()).toBe(false);
        expect(helpers.validate(allocationRepo, id, root)).toBe(root);
        writeFileSync(join(bundle, "🔤️.txt"), "retained allocation input\n");
        expect(() => helpers.allocate(allocationRepo, id)).toThrow();
      }
      currentRoot = pair[0]!;
      const namesBefore = readdirSync(dirname(currentRoot)).sort();
      for (const id of harness.invalidRunIds) {
        expect(() => helpers.allocate(allocationRepo, id)).toThrow();
        expect(() => helpers.validate(allocationRepo, id, currentRoot)).toThrow();
      }
      expect(() => helpers.allocate("relative-root", ids[0]!)).toThrow();
      expect(() => helpers.validate(allocationRepo, ids[0]!, relative(allocationRepo, currentRoot))).toThrow();
      expect(() => helpers.validate(allocationRepo, ids[1]!, currentRoot)).toThrow();
      expect(readdirSync(dirname(currentRoot)).sort()).toEqual(namesBefore);
      const proofRepo = join(currentRoot, "🧪️ownership"), proofOwner = join(proofRepo, harness.runOwnerDirectory);
      mkdirSync(proofOwner, { recursive: true });
      const proofId = `${process.pid}-${crypto.randomUUID()}`, proofRoot = join(proofOwner, harness.runRootPrefix + proofId.slice(proofId.indexOf("-") + 1));
      symlinkSync(currentRoot, proofRoot, process.platform === "win32" ? "junction" : "dir");
      expect(() => helpers.allocate(proofRepo, proofId)).toThrow();
      expect(() => helpers.validate(proofRepo, proofId, proofRoot)).toThrow();
      expect(lstatSync(proofRoot).isSymbolicLink()).toBe(true);
      const aliasRepo = join(currentRoot, "🧪️alias");
      symlinkSync(proofRepo, aliasRepo, process.platform === "win32" ? "junction" : "dir");
      expect(() => helpers.allocate(aliasRepo, `${process.pid}-${crypto.randomUUID()}`)).toThrow();
      expect(() => helpers.validate(aliasRepo, proofId, join(aliasRepo, harness.runOwnerDirectory, basename(proofRoot)))).toThrow();
      const fileId = `${process.pid}-${crypto.randomUUID()}`, fileRoot = join(proofOwner, harness.runRootPrefix + fileId.slice(fileId.indexOf("-") + 1));
      writeFileSync(fileRoot, "not a directory\n");
      expect(() => helpers.allocate(proofRepo, fileId)).toThrow();
      expect(() => helpers.validate(proofRepo, fileId, fileRoot)).toThrow();
      expect(readFileSync(fileRoot, "utf8")).toBe("not a directory\n");
      for (const row of harness.cases) {
        const parent = row.parent === "run" ? currentRoot : row.parent === "ticket" ? join(getWorkspaceRoot(), TICKET_REL) : join(currentRoot, "🧪️nested");
        const root = join(parent, row.name);
        if (row.accepted) mkdirSync(root);
        if (row.accepted) expect(() => helpers.retain(root)).not.toThrow();
        else expect(() => helpers.retain(root)).toThrow();
      }
      const foreign = join(pair[1]!, "🧪️foreign"), fixtureLink = join(currentRoot, "🧪️link");
      mkdirSync(foreign);
      symlinkSync(foreign, fixtureLink, process.platform === "win32" ? "junction" : "dir");
      expect(() => helpers.retain(foreign)).toThrow();
      expect(() => helpers.retain(fixtureLink)).toThrow();
      expect(lstatSync(fixtureLink).isSymbolicLink()).toBe(true);
      for (const root of pair) expect(readFileSync(join(root, harness.bundleDirectory, "🔤️.txt"), "utf8")).toBe("retained allocation input\n");
    }
    expect(new Set(roots).size).toBe(4);
    const protocolGolden = JSON.parse(readFileSync(TRANSACTION_PROTOCOL_GOLDEN, "utf8")) as ProtocolGolden;
    expect(Object.keys(transactionGolden).sort()).toEqual(["boundaries", "schemaVersion", "transactionLedgers", "workspaceLedgers"]);
    expect(Object.keys(protocolGolden).sort()).toEqual(["failureStages", "journalStates", "schemaVersion", "virtualPreimageNodes"]);
    expect(ownedFilePaths(resolve(TRANSACTION_LEDGER_BOUNDARIES_GOLDEN, ".."))).toEqual(["🔣️.json"]);
    expect(ownedFilePaths(resolve(TRANSACTION_PROTOCOL_GOLDEN, ".."))).toEqual(["🔣️.json"]);
    expect(protocolGolden.failureStages).toEqual(FAILURE_STAGES);
    expect(protocolGolden.journalStates).toHaveLength(11);
    expect(Object.keys(transactionGolden.boundaries).sort()).toEqual(EXACT_BOUNDARY_KEYS);
    for (const [owner, ledgers] of [["transaction", transactionGolden.transactionLedgers], ["workspace", transactionGolden.workspaceLedgers]] as const) for (const [digest, nodes] of Object.entries(ledgers ?? {})) {
      expect(new Bun.CryptoHasher("sha256").update(canonicalJson(nodes)).digest("hex")).toBe(digest);
      for (const node of nodes) {
        if (node.kind === "symlink") expect(new Bun.CryptoHasher("sha256").update(node.target).digest("hex")).toBe(node.targetHash);
        if (node.kind === "file") {
          const bytes = Buffer.from(node.bytesBase64, "base64");
          expect(bytes.length).toBe(node.size);
          expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(node.sha256);
          if (node.json !== undefined) { if (owner === "transaction") expect(node.canonicalJson).toBe(true); expect(bytes.toString("utf8")).toBe(`${canonicalJson(node.json)}\n`); }
        }
      }
    }
    const referencedTransactions = [...new Set(Object.values(transactionGolden.boundaries).map((entry) => entry.transaction))].sort(), referencedWorkspaces = [...new Set(Object.values(transactionGolden.boundaries).map((entry) => entry.workspace))].sort();
    expect(Object.keys(transactionGolden.transactionLedgers).sort()).toEqual(referencedTransactions);
    expect(Object.keys(transactionGolden.workspaceLedgers).sort()).toEqual(referencedWorkspaces);
    for (const reference of Object.values(transactionGolden.boundaries)) {
      expect(transactionGolden.transactionLedgers[reference.transaction]).toBeDefined();
      expect(transactionGolden.workspaceLedgers[reference.workspace]).toBeDefined();
    }
    const root = mkdtempSync(join(fixtureRunRoot(), "🧪️parity-"));
    try {
      mkdirSync(join(root, "evidence", "directory"), { recursive: true });
      writeFileSync(join(root, "evidence", "file.txt"), "sentinel\n");
      symlinkSync("../file.txt", join(root, "evidence", "link"));
      expect(ownedFilesystemEntries(join(root, "evidence"), true).map(({ path }) => path === "." ? "evidence" : `evidence/${path}`)).toEqual(["evidence", "evidence/directory", "evidence/file.txt", "evidence/link"]);
      expect(noFollowTreeDigest(root, "evidence")).toMatchObject({ directories: 2, files: 1, symlinks: 1 });
      expect(protocolGolden.virtualPreimageNodes.some((entry) => entry.state === "symlink")).toBe(true);
    } finally { retainFixture(root); }
  });
}
