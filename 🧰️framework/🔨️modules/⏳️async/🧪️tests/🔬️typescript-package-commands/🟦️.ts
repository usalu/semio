import type { ScriptInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand, resolveTypescriptCompilerV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";

/** 🏃️ Runs the `ApiContractScript` command under the caller's roots and invocation. */
export async function runApiContractScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw Error("test-api-contract accepts no arguments");
  await runOwnedCommand(process.execPath, ["test", resolve(context.root, "../../📏️api/🧪️tests/🟦️.ts")], context.repoRoot, "async:public-api", 30_000);
}

/** 🏃️ Runs the `PublicationContractScript` command under the caller's roots and invocation. */
export async function runPublicationContractScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-publication-contract");
  const tests = [resolve(context.root, "../../🔐️publication/🧪️tests/🟦️.ts"), resolve(context.root, "../../🔐️publication/🛂️checkpoint/🧪️tests/🟦️.ts")];
  await runOwnedCommand(process.execPath, [resolveTypescriptCompilerV1(context.root), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], context.root, "async:publication:strict", 30000);
  await runOwnedCommand(process.execPath, ["test", ...tests], context.root, "async:publication:contract", 30000);
}

/** 🏃️ Runs the `SinglePollCheckScript` command under the caller's roots and invocation. */
export async function runSinglePollCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw Error("single-poll-check accepts no arguments");
  const test = resolve(context.root, "../../🔂️poll/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath, [resolveTypescriptCompilerV1(context.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: context.repoRoot, budgetMs: 30000, throwOnFailure: true});
  await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: context.repoRoot, budgetMs: 15000, throwOnFailure: true});
}
