import {readFileSync, writeFileSync} from "node:fs";
import document from "../🔣️.json" with {type: "json"};
import {isMainModuleV1, onTerminationV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {readOracleTextV1, runReadOracleV1, windowsPlanV1, type ReadOracleReportV1} from "../🟦️.ts";
import {readOracleBothV1} from "../🏃️run/🟦️.ts";

/** ⌨️ Parses the command line of the read oracle into a request; the language is required, none is assumed. */
export function readOracleArgumentsV1(argv: readonly string[]): {readonly request: unknown; readonly out?: string} {
  const rest = argv.indexOf("--"), flags = rest < 0 ? argv : argv.slice(0, rest), command = rest < 0 ? [] : argv.slice(rest + 1);
  const values = new Map<string, string[]>();
  for (let at = 0; at < flags.length; at += 2) {
    const name = flags[at]!;
    if (!name.startsWith("--") || flags[at + 1] === undefined) throw new Error(readOracleBothV1("argumentUnexpected", {name}));
    values.set(name.slice(2), [...(values.get(name.slice(2)) ?? []), flags[at + 1]!]);
  }
  const one = (name: string): string | undefined => values.get(name)?.at(-1), list = one("allow-list");
  if (!list || !one("root") || !one("language")) throw new Error(readOracleBothV1("argumentsRequired"));
  const environment = Object.fromEntries((values.get("env") ?? []).map((pair) => [pair.slice(0, pair.indexOf("=")), pair.slice(pair.indexOf("=") + 1)]));
  return {
    out: one("out"),
    request: {version: 1, language: one("language"), root: one("root"), cwd: one("cwd") ?? "", allowed: readFileSync(list, "utf8").split("\n").filter(Boolean), ambient: values.get("allow-tree") ?? [], names: values.get("allow-name") ?? [], command, environment, rounds: Number(one("rounds") ?? 1), timeoutMs: Math.round(Number(one("timeout") ?? document.policy.defaultTimeoutSeconds) * 1000), backend: one("backend") ?? "auto", compare: one("compare") ?? "output", ...(one("max-profiles") ? {maxProfiles: Number(one("max-profiles"))} : {})},
  };
}

/** 🚪️ The entry point: runs the oracle, prints one line per round and the verdict, writes the report JSON; exit 0 clean, 1 not clean, 2 usage. */
export async function main(argv: readonly string[]): Promise<number> {
  let parsed: ReturnType<typeof readOracleArgumentsV1>;
  try {
    parsed = readOracleArgumentsV1(argv);
  } catch (error) {
    console.error(`${(error as Error).message}\n${readOracleTextV1("en", "usage")}\n${readOracleTextV1("de", "usage")}`);
    return 2;
  }
  const language = (parsed.request as {language: "en" | "de"}).language;
  const controller = new AbortController();
  onTerminationV1(() => controller.abort());
  const report: ReadOracleReportV1 = await runReadOracleV1(parsed.request, {progress: (text) => console.error(text), signal: controller.signal});
  if (report.backend === "mirror" && process.platform === "win32") for (const line of windowsPlanV1(language)) console.error(line);
  const values = {allowed: report.allowed, learned: report.learned.length, backend: report.backend, strength: report.strength, exit: report.rounds.at(-1)!.exit};
  console.error(readOracleTextV1(language, report.clean ? "clean" : "dirty", values));
  const text = JSON.stringify(report, null, 1);
  if (parsed.out) writeFileSync(parsed.out, text);
  else console.log(text);
  return report.clean ? 0 : 1;
}

if (isMainModuleV1(import.meta.url)) process.exit(await main(process.argv.slice(2)));
