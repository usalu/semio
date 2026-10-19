import environmentSchema from "@semio-tech/process/build/environment/schema.json";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import type { ArtifactCratePlan } from "../🗺️plan/🧾️model/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const protocol = schema.definitions.Protocol.const;
const environment = environmentSchema.definitions.Protocol.const.environment;

type TargetId = "build" | "check" | "test";

/** 🚀️ One crate that gets editor routes; `project` names the build project that runs it and defaults to the crate name, `targets` limits the routes and `arguments` replaces the cargo arguments of a target. */
export type CrateLaunchSubject = Readonly<{ name: string; project?: string; shape: keyof typeof protocol.shapes; targets?: readonly TargetId[]; arguments?: Readonly<Partial<Record<TargetId, readonly string[]>>> }>;

/** 🛣️ One editor route, in the key order of the catalog. */
export type CrateLaunchRoute = Readonly<{ name: string; type: string; request: string; command: string; cwd: string; env: Readonly<Record<string, string>>; presentation: Readonly<{ group: string; order: number }> }>;

/** 🔢️ The next free order of each route group: one above the highest order any entry of the group holds. */
export function nextLaunchRouteOrders(configurations: readonly { presentation?: { group?: unknown; order?: unknown } }[]): Record<string, number> {
  const orders: Record<string, number> = Object.fromEntries(protocol.targets.map((target) => [target.group, target.orderBase]));
  for (const { presentation } of configurations) {
    const group = presentation?.group;
    if (typeof group === "string" && group in orders && typeof presentation?.order === "number") orders[group] = Math.max(orders[group]!, Math.floor(presentation.order) + 1);
  }
  return orders;
}

/** 🏭️ The routes of the given crates, prototype before release, taking consecutive orders from `orders` upwards. */
export function crateLaunchRoutes(subjects: readonly CrateLaunchSubject[], orders: Readonly<Record<string, number>>): CrateLaunchRoute[] {
  const next = { ...orders };
  const routes: CrateLaunchRoute[] = [];
  for (const subject of subjects) {
    const problems = validateJsonSchemaSubset(schema.definitions.Subject, subject, schema);
    if (problems.length) throw new Error(`launch routes: ${JSON.stringify(subject)} is no crate subject: ${problems.join("; ")}`);
    for (const target of protocol.targets) {
      if (subject.targets && !subject.targets.includes(target.id as TargetId)) continue;
      const cargoArguments = subject.arguments?.[target.id as TargetId] ?? protocol.shapes[subject.shape][target.id as keyof typeof protocol.shapes.lib];
      for (const configuration of protocol.configurations) {
        routes.push({
          name: `${protocol.namePrefix} ${subject.name} ${target.icon}${target.id} ${configuration.icon}${configuration.id}`,
          type: protocol.type,
          request: protocol.request,
          command: protocol.command.replace("{project}", subject.project ?? subject.name).replace("{target}", target.id).replace("{configuration}", configuration.id).replace("{arguments}", cargoArguments.join(" ")),
          cwd: protocol.cwd,
          env: { ...environment },
          presentation: { group: target.group, order: next[target.group]! },
        });
        next[target.group] = next[target.group]! + 1;
      }
    }
  }
  return routes;
}

/** 🔎️ Every way an entry of the crate route family departs from the protocol: the shape of the entry, its group and the build environment. */
export function crateLaunchRouteProblems(configurations: readonly Record<string, any>[]): string[] {
  const problems: string[] = [];
  for (const entry of configurations) {
    if (typeof entry?.name !== "string" || !entry.name.startsWith(`${protocol.namePrefix} `) || typeof entry.command !== "string" || !entry.command.startsWith("bun repo run ") || !/ --configuration=(prototype|release) --parallel=1 -- /u.test(entry.command)) continue;
    for (const problem of validateJsonSchemaSubset(schema.definitions.Route, entry, schema)) problems.push(`${entry.name}: ${problem}`);
    for (const problem of validateJsonSchemaSubset(environmentSchema.definitions.Environment, entry.env ?? {}, environmentSchema)) problems.push(`${entry.name}: env ${problem}`);
    const target = protocol.targets.find((row) => entry.command.includes(`:${row.id} `));
    if (target && entry.presentation?.group !== target.group) problems.push(`${entry.name}: group ${String(entry.presentation?.group)} is not ${target.group}`);
  }
  return problems;
}

/** 🔪️ The crates of a carve plan that get routes: one with a library is built as a library, a crate of tests only as tests. */
export function crateLaunchSubjects(plan: ArtifactCratePlan): CrateLaunchSubject[] {
  return plan.crates.map((crate) => ({ name: crate.name, shape: crate.lib ? "lib" : "tests" }));
}
