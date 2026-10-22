import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {pythonProjectForRootV1, toolchainHostV1, toolchainIdentityV1, type ToolchainKindV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import {taskFamilyV1} from "../../../🏃️execution/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};
import table from "./🔣️.json" with {type: "json"};

if (validateJsonSchemaSubset(schema, table, schema).length) throw new Error("Closed Bazel native toolchain classes required / Geschlossene Bazel-Klassentabelle der nativen Toolchains erforderlich");

/** 🧰️ The toolchain classes a Bazel task can belong to besides Go and Cargo, which have their own identities. */
export type BazelNativeToolchainKindV1 = Exclude<ToolchainKindV1, "go">;
/** 🧾️ What the classifier reads of one task. */
export type BazelNativeToolchainTaskV1 = {project?: string; target: string; root: string; definition: {execution: {kind: string; command?: string}}};

/** 🧾️ The authored class table: the task family or command pattern that makes a task a member of a toolchain class. */
export const BAZEL_NATIVE_TOOLCHAINS_V1 = table;

/** 🧮️ Names the toolchain class of a task, decided by the same ordered family rules that classify every task for coverage, so Bazel and the neutral contract agree on what is Python, .NET or CMake; `undefined` for every other task. */
export function bazelNativeToolchainClassV1(task: BazelNativeToolchainTaskV1): BazelNativeToolchainKindV1 | undefined {
  const execution = task.definition.execution, family = taskFamilyV1({...(task.project !== undefined ? {project: task.project} : {}), target: task.target, root: task.root, kind: execution.kind === "noop" ? "noop" : "command", ...(execution.command !== undefined ? {command: execution.command} : {}), cache: {cache: true}});
  return table.classes.find(row => ("family" in row && row.family === family) || (row.command !== undefined && execution.command !== undefined && new RegExp(row.command, "u").test(execution.command)))?.kind as BazelNativeToolchainKindV1 | undefined;
}

/** 🔑️ Names the identity of the toolchain by one digest that re-keys every action of the class when the pin, the host or a declared variable changes; a Python task is keyed by the interpreter of its own project. */
export function bazelNativeToolchainDigestV1(kind: BazelNativeToolchainKindV1, environment: Readonly<Record<string, string | undefined>> = process.env, root: string = ".", platform: string = process.platform, architecture: string = process.arch): string {
  const project = kind === "python" ? pythonProjectForRootV1(root)?.id : undefined;
  return toolchainIdentityV1(kind, {host: toolchainHostV1(platform, architecture), ...(project === undefined ? {} : {project}), environment}).digest;
}
