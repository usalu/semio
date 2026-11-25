import schema from "../🧬️schema/🔣️.json" with { type: "json" };

/** 🏔️ The compile-memory budget of a worker class (`Policy.unitCeiling.classes`): the library budget, the higher allowance of a final `cdylib` link unit and the host the class stands for. */
export type CompileBudgetClassV1 = Readonly<{ hostMegabytes: number; libraryMegabytes: number; finalMegabytes: number; finalBackend: string }>;

/** 🏷️ The budget kind of a unit: `final` for a linked unit (`cdylib`, `bin`, `dylib`, `staticlib`; a `--test` executable is a `bin`), `library` for everything else. */
export type CompileBudgetKindV1 = "library" | "final";

/** 🪚️ One entry of the closed carving list: a unit known to exceed its kind's budget, its measured peak and the budget the host warden enforces for it while it runs. */
export type CompileCarvingEntryV1 = Readonly<{ crate: string; target: string; measuredMegabytes: number; budgetMegabytes: number }>;

/** 🏗️ The compile-memory policy: the active worker class, the budgets of every class and the closed carving list. */
export type CompileBudgetPolicyV1 = Readonly<{ workerClass: string; classes: Readonly<Record<string, CompileBudgetClassV1>>; carving: readonly CompileCarvingEntryV1[] }>;

/** 🎚️ The compile-memory policy of the admission schema (`Policy.unitCeiling`). */
export const UNIT_CEILING_POLICY_V1: CompileBudgetPolicyV1 = schema.definitions.Policy.const.unitCeiling;

/** 🚧️ The crate and target of a unit key (`<package>@<crate>|<target>|<profile>|<crate types>`), or `null` for a malformed key. */
export function unitKeyPartsV1(key: string): Readonly<{ crate: string; target: string }> | null {
  const [identity = "", target = "", profile, types, ...rest] = key.split("|"), crate = identity.slice(identity.indexOf("@") + 1);
  return identity.includes("@") && crate && target && profile !== undefined && types !== undefined && !rest.length ? { crate, target } : null;
}

/** 🔗️ The crate types whose unit links an artifact and therefore carries the `final` budget. */
export const LINKED_CRATE_TYPES_V1: readonly string[] = ["cdylib", "bin", "dylib", "staticlib"];

/** 🏷️ The budget kind of a unit key: a key whose crate types hold a linked type (`LINKED_CRATE_TYPES_V1`) is a `final` link unit, every other key a `library` unit. */
export function unitBudgetKindV1(key: string): CompileBudgetKindV1 {
  return (key.split("|")[3] ?? "").split(",").some((type) => LINKED_CRATE_TYPES_V1.includes(type)) ? "final" : "library";
}

/** 📏️ The budget in megabytes of a unit key in the active worker class of `policy`. */
export function unitBudgetMegabytesV1(policy: CompileBudgetPolicyV1, key: string): number {
  const budget = policy.classes[policy.workerClass];
  if (!budget) throw Error(`Unknown worker class ${policy.workerClass}`);
  return unitBudgetKindV1(key) === "final" ? budget.finalMegabytes : budget.libraryMegabytes;
}

/** 🏔️ The runtime ceiling of a running compile unit: `unit` is `<crate>|<target>`, `kind` its budget kind, `megabytes` the most its resident footprint may reach, `listed` whether it is on the carving list (then `measuredMegabytes` is its measured peak). */
export type RuntimeCeilingV1 = Readonly<{ unit: string; kind: CompileBudgetKindV1; megabytes: number; listed: boolean; measuredMegabytes?: number }>;

const flagValues = (tokens: readonly string[], flag: string): string[] => tokens.flatMap((token, index) => token === flag ? [tokens[index + 1] ?? ""] : token.startsWith(flag + "=") ? [token.slice(flag.length + 1)] : []);

/** 🔎️ The unit a compiler command line (`rustc`, `rustdoc`, `clippy-driver`, arguments split at white space) compiles: its crate (`--crate-name`), its target (`--target`, `host` without) and its crate types (`--crate-type`, comma lists split, plus `bin` for a `--test` executable); `null` for a command that names no crate (a probe such as `rustc -vV`). */
export function compileUnitOfCommandV1(command: string): Readonly<{ crate: string; target: string; types: readonly string[] }> | null {
  const tokens = command.split(/\s+/u).filter(Boolean), crate = flagValues(tokens, "--crate-name")[0];
  if (!crate) return null;
  return { crate, target: flagValues(tokens, "--target")[0] || "host", types: [...flagValues(tokens, "--crate-type").flatMap((value) => value.split(",")).filter(Boolean), ...tokens.includes("--test") ? ["bin"] : []] };
}

/** 🏔️ The ceiling a running compile unit may reach (the runtime form of the compile-memory law of the admission policy): the budget of its kind in the active worker class (a linked unit is `final`, everything else `library`); a unit on the carving list may reach its declared `budgetMegabytes` (never less than the budget of its kind). `null` for a command that names no crate.
 * @see ../🧬️schema/🔣️.json */
export function runtimeCeilingOfCommandV1(policy: CompileBudgetPolicyV1, command: string): RuntimeCeilingV1 | null {
  const unit = compileUnitOfCommandV1(command);
  if (!unit) return null;
  const key = `${unit.crate}@${unit.crate}|${unit.target}|-|${unit.types.join(",") || "-"}`, base = unitBudgetMegabytesV1(policy, key), entry = policy.carving.find((one) => one.crate === unit.crate && one.target === unit.target);
  return { unit: `${unit.crate}|${unit.target}`, kind: unitBudgetKindV1(key), megabytes: entry ? Math.max(entry.budgetMegabytes, base) : base, listed: entry !== undefined, ...entry ? { measuredMegabytes: entry.measuredMegabytes } : {} };
}

/** 🚛️ Whether a unit is known to need more memory than a machine of `hostMegabytes` can carry: its largest known peak (the carving entry measured for its crate and target, or the peak learned on this machine, `learnedMegabytes`) is above the final budget of the worker class and the machine is no larger than that class. Such a unit belongs to a bigger machine and is queued for one instead of compiling here; a machine larger than the class, and a unit never measured, answer `false`.
 * @see ../🧬️schema/🔣️.json */
export function unitBeyondHostV1(policy: CompileBudgetPolicyV1, unit: Readonly<{ crate: string; target: string }>, learnedMegabytes: number | undefined, hostMegabytes: number): boolean {
  const budget = policy.classes[policy.workerClass];
  if (!budget) throw Error(`Unknown worker class ${policy.workerClass}`);
  const entry = policy.carving.find((one) => one.crate === unit.crate && one.target === unit.target), peak = Math.max(entry?.measuredMegabytes ?? 0, learnedMegabytes !== undefined && Number.isFinite(learnedMegabytes) ? learnedMegabytes : 0);
  return peak > budget.finalMegabytes && hostMegabytes <= budget.hostMegabytes;
}
