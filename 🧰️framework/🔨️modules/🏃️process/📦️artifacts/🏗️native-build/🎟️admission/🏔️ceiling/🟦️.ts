import schema from "../🧬️schema/🔣️.json" with { type: "json" };

/** 🏔️ The compile-memory budget of a worker class (`Policy.unitCeiling.classes`): the library budget, the higher allowance of a final `cdylib` link unit and the host the class stands for. */
export type CompileBudgetClassV1 = Readonly<{ hostMegabytes: number; libraryMegabytes: number; finalMegabytes: number; finalBackend: string }>;

/** 🏷️ The budget kind of a unit: `final` for a linked unit (`cdylib`, `bin`, `dylib`, `staticlib`; a `--test` executable is a `bin`), `library` for everything else. */
export type CompileBudgetKindV1 = "library" | "final";

/** 🪚️ One entry of the closed carving list: a unit known to exceed its kind's budget, its measured peak and the budget the host warden enforces for it while it runs. */
export type CompileCarvingEntryV1 = Readonly<{ crate: string; target: string; measuredMegabytes: number; budgetMegabytes: number }>;

/** 🧬️ One carving family: homogeneous generated shards (`pattern` is a regular expression that must match a whole crate name, on `target`) whose peaks vary from run to run. `measuredMegabytes` is the class measurement, the largest peak observed over any shard of the family (`source` says which warden log lines), and `budgetMegabytes` the budget every shard of the family may reach: the class measurement plus the variance margin of the policy law (at most 10 percent). */
export type CompileCarvingFamilyV1 = Readonly<{ family: string; pattern: string; target: string; measuredMegabytes: number; budgetMegabytes: number; action: string; source: string }>;

/** 🏗️ The compile-memory policy: the active worker class, the budgets of every class, the closed carving list and the closed carving families. */
export type CompileBudgetPolicyV1 = Readonly<{ workerClass: string; classes: Readonly<Record<string, CompileBudgetClassV1>>; carving: readonly CompileCarvingEntryV1[]; families: readonly CompileCarvingFamilyV1[] }>;

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

/** 🪚️ The carving declaration of a unit: its own entry (`crate` and `target` equal) before the family whose pattern matches the whole crate name on its target (then `measuredMegabytes` is the class measurement), `undefined` for an unlisted unit. */
export function carvingOfUnitV1(policy: CompileBudgetPolicyV1, unit: Readonly<{ crate: string; target: string }>): Readonly<{ measuredMegabytes: number; budgetMegabytes: number }> | undefined {
  return policy.carving.find((one) => one.crate === unit.crate && one.target === unit.target) ?? policy.families.find((one) => one.target === unit.target && new RegExp(`^(?:${one.pattern})$`, "u").test(unit.crate));
}

/** 🔎️ The unit a compiler command line (`rustc`, `rustdoc`, `clippy-driver`, arguments split at white space) compiles: its crate (`--crate-name`), its target (`--target`, `host` without) and its crate types (`--crate-type`, comma lists split, plus `bin` for a `--test` executable); `null` for a command that names no crate (a probe such as `rustc -vV`). */
export function compileUnitOfCommandV1(command: string): Readonly<{ crate: string; target: string; types: readonly string[] }> | null {
  const tokens = command.split(/\s+/u).filter(Boolean), crate = flagValues(tokens, "--crate-name")[0];
  if (!crate) return null;
  return { crate, target: flagValues(tokens, "--target")[0] || "host", types: [...flagValues(tokens, "--crate-type").flatMap((value) => value.split(",")).filter(Boolean), ...tokens.includes("--test") ? ["bin"] : []] };
}

/** 📦️ The artifact directory named by a build-script rustc invocation before its command text is shortened. */
export function buildScriptOutputDirectoryV1(command: string): string | undefined {
  if (compileUnitOfCommandV1(command)?.crate !== "build_script_build") return undefined;
  const match = /(?:^|\s)--out-dir(?:=|\s+)(?:"([^"]+)"|(.+?))(?=\s+-|$)/u.exec(command);
  return (match?.[1] ?? match?.[2])?.trimEnd();
}

/** 🏔️ The ceiling a running compile unit may reach (the runtime form of the compile-memory law of the admission policy): the budget of its kind in the active worker class (a linked unit is `final`, everything else `library`); a unit on the carving list may reach its declared `budgetMegabytes` (never less than the budget of its kind). `null` for a command that names no crate.
 * @see ../🧬️schema/🔣️.json */
export function runtimeCeilingOfCommandV1(policy: CompileBudgetPolicyV1, command: string): RuntimeCeilingV1 | null {
  const unit = compileUnitOfCommandV1(command);
  if (!unit) return null;
  const key = `${unit.crate}@${unit.crate}|${unit.target}|-|${unit.types.join(",") || "-"}`, base = unitBudgetMegabytesV1(policy, key), entry = carvingOfUnitV1(policy, unit);
  return { unit: `${unit.crate}|${unit.target}`, kind: unitBudgetKindV1(key), megabytes: entry ? Math.max(entry.budgetMegabytes, base) : base, listed: entry !== undefined, ...entry ? { measuredMegabytes: entry.measuredMegabytes } : {} };
}

/** 🚛️ Whether a unit is known to need more memory than a machine of `hostMegabytes` can carry: its largest known peak (the carving entry measured for its crate and target, or the peak learned on this machine, `learnedMegabytes`) is above the final budget of the worker class and the machine is no larger than that class. Such a unit belongs to a bigger machine and is queued for one instead of compiling here; a machine larger than the class, and a unit never measured, answer `false`, except that a unit whose peak is above what the machine can ever hold beside its reserves (`holdableMegabytes`, see `hostHoldableMegabytesV1`) is always beyond it.
 * @see ../🧬️schema/🔣️.json */
export function unitBeyondHostV1(policy: CompileBudgetPolicyV1, unit: Readonly<{ crate: string; target: string }>, learnedMegabytes: number | undefined, hostMegabytes: number, holdableMegabytes?: number): boolean {
  const budget = policy.classes[policy.workerClass];
  if (!budget) throw Error(`Unknown worker class ${policy.workerClass}`);
  const entry = carvingOfUnitV1(policy, unit), peak = Math.max(entry?.measuredMegabytes ?? 0, learnedMegabytes !== undefined && Number.isFinite(learnedMegabytes) ? learnedMegabytes : 0);
  return peak > budget.finalMegabytes && hostMegabytes <= budget.hostMegabytes || holdableMegabytes !== undefined && peak > holdableMegabytes;
}

/** 📈️ The memory facts of one machine that decide whether it can hold a unit: its total, the free-memory floor and the interactive reserve it keeps back (`hostFloorMegabytesV1`, `hostInteractiveReserveMegabytesV1` of the host-capacity policy), the share of the rest the policy lets reservations take, and, for the machine the unit runs on, what is available now. */
export type GrowthHostV1 = Readonly<{ origin: string; totalMegabytes: number; floorMegabytes: number; interactiveReserveMegabytes: number; budgetBasisPoints: number; availableMegabytes?: number }>;

/** 📈️ What a unit needing more than its memory grant is told: `fits` (the grant carries it), `grow` (this host has the room now: ask for `extraMegabytes` more), `wait` (this host can hold it but the free memory beside the reserves is short by now: wait for growth), `beyond-host` (no memory this host can ever hold carries it, a larger machine of the fleet does: queue it there), `refused` (no machine of the fleet can hold it; `largestMegabytes` is the most the largest holds). */
export type GrantGrowthVerdictV1 = Readonly<{ kind: "fits" } | { kind: "grow"; extraMegabytes: number; roomMegabytes: number; holdableMegabytes: number } | { kind: "wait"; extraMegabytes: number; roomMegabytes: number; holdableMegabytes: number } | { kind: "beyond-host"; holdableMegabytes: number; largestMegabytes: number; largestOrigin: string } | { kind: "refused"; largestMegabytes: number; largestOrigin: string }>;

/** 🧱️ The most memory reservations on one machine can ever hold: its total less the floor and the interactive reserve, times the share the policy lets reservations take (the same arithmetic as the host-capacity decision).
 * @see ../../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts (`decideHostCapacityV1`) */
export function hostHoldableMegabytesV1(host: Pick<GrowthHostV1, "totalMegabytes" | "floorMegabytes" | "interactiveReserveMegabytes" | "budgetBasisPoints">): number {
  return Math.floor(Math.max(0, host.totalMegabytes - host.floorMegabytes - host.interactiveReserveMegabytes) * host.budgetBasisPoints / 10000);
}

/** 📈️ The law of grant growth: a unit that needs `needMegabytes` under a grant of `grantMegabytes` is carried (`fits`), grown into when this host has the free room above its floor and interactive reserve (`grow`), waited for while the room is short (`wait`), queued for a larger machine when this host can never hold it (`beyond-host`) and refused only when no machine of the fleet can (`refused`). `fleet` lists the other machines (`undefined` when the fleet is unknown: the unit is then queued, never refused).
 * @see ../🧬️schema/🔣️.json (`Policy.growth`) */
export function decideGrantGrowthV1(input: Readonly<{ needMegabytes: number; grantMegabytes: number; local: GrowthHostV1; fleet: readonly GrowthHostV1[] | undefined }>): GrantGrowthVerdictV1 {
  const { needMegabytes, grantMegabytes, local } = input;
  if (needMegabytes <= grantMegabytes) return { kind: "fits" };
  const holdable = hostHoldableMegabytesV1(local), extra = needMegabytes - grantMegabytes;
  if (needMegabytes <= holdable) {
    const room = Math.max(0, (local.availableMegabytes ?? Infinity) - local.floorMegabytes - local.interactiveReserveMegabytes);
    return { kind: extra <= room ? "grow" : "wait", extraMegabytes: extra, roomMegabytes: Number.isFinite(room) ? room : holdable, holdableMegabytes: holdable };
  }
  const others = (input.fleet ?? []).filter((host) => host.origin !== local.origin).map((host) => ({ origin: host.origin, megabytes: hostHoldableMegabytesV1(host) })), largest = others.reduce((best, host) => host.megabytes > best.megabytes ? host : best, { origin: local.origin, megabytes: holdable });
  if (input.fleet === undefined || largest.megabytes >= needMegabytes) return { kind: "beyond-host", holdableMegabytes: holdable, largestMegabytes: largest.megabytes, largestOrigin: largest.origin };
  return { kind: "refused", largestMegabytes: largest.megabytes, largestOrigin: largest.origin };
}
