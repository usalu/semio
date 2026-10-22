/** 🚦️ Census gates behind `verify production-placeholders|docstrings|agnostic|mutation-caps|debug-tags|interface-owners`.
 * @see 📜️script.ts
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import {sayVerifyV1,refuseVerifyV1} from "../📖️texts/🟦️.ts";

/** 🧮️ The two production source censuses as gates (acceptance ledger 1.8 / 1.9): `placeholders` fails on any
 * `unimplemented!(` / `todo!(` in a production (non-test) Rust source; `commands` fails on any production command declared
 * `InteractiveJobClassification::BatchOnlyPendingRewrite` (unreachable from every control of the running app). Both
 * cross-check their scanner against `git grep`, print their table, and publish the acceptance record.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts `runSourceCensus` */
export async function runSourceCensusGate(root: string, which: "placeholders" | "commands"): Promise<void> {
  const { acceptanceCheckResult, publishAcceptanceCheckResult } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🟦️.ts");
  const { runSourceCensus } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🧮️placeholder-and-job-census/🟦️.ts");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  const startedAt = new Date();
  try {
    const census = runSourceCensus(root, controller.signal, (line) => sayVerifyV1("census.source",{line:String(line)},"error"));
    const oracleAgrees = census.placeholderDisagreements.length === 0 && census.callDisagreements.length === 0;
    if (which === "placeholders") {
      const production = census.placeholders.filter((hit) => !hit.testOnly && !hit.commented);
      for (const hit of census.placeholders.filter((entry) => !entry.commented)) sayVerifyV1("census.production-placeholders",{production:String(hit.testOnly ? "test" : "PRODUCTION"),macro:String(hit.macro),path:String(hit.path),line:String(hit.line)});
      for (const disagreement of census.placeholderDisagreements) sayVerifyV1("census.production-placeholders-oracle-disagreement",{disagreement:String(disagreement)});
      const status = production.length === 0 && oracleAgrees ? "pass" : "fail";
      sayVerifyV1("census.production-placeholders-files-production-test",{files:String(census.files),production:production.length,test:census.placeholders.filter((hit) => hit.testOnly && !hit.commented).length,commented:census.placeholders.filter((hit) => hit.commented).length,oracle:String(oracleAgrees ? "agrees" : "DISAGREES")});
      publishAcceptanceCheckResult(root, acceptanceCheckResult({
        check: "production-placeholders",
        status,
        startedAt,
        measured: { files: census.files, production: production.length, testOnly: census.placeholders.filter((hit) => hit.testOnly && !hit.commented).length, oracleAgrees },
        summary: { en: `${production.length} production unimplemented!/todo! placeholders in ${census.files} Rust sources${production.length ? `: ${production.slice(0, 4).map((hit) => `${hit.path}:${hit.line}`).join(", ")}` : ""}`, de: `${production.length} produktive unimplemented!/todo!-Platzhalter in ${census.files} Rust-Quellen${production.length ? `: ${production.slice(0, 4).map((hit) => `${hit.path}:${hit.line}`).join(", ")}` : ""}` },
      }));
      if (status !== "pass") process.exitCode = 1;
      return;
    }
    const production = census.jobs.filter((job) => !job.testOnly);
    const unreachable = production.filter((job) => job.classification === "BatchOnlyPendingRewrite");
    const perOwner = new Map<string, { migrated: number; unreachable: string[]; other: number }>();
    for (const job of production) {
      const owner = /^✏️s\/🔌️plugins\/([^/]+)/u.exec(job.path)?.[1] ?? job.path.split("/").slice(0, 3).join("/");
      const row = perOwner.get(owner) ?? { migrated: 0, unreachable: [], other: 0 };
      if (job.classification === "Migrated") row.migrated += 1;
      else if (job.classification === "BatchOnlyPendingRewrite") row.unreachable.push(job.command);
      else row.other += 1;
      perOwner.set(owner, row);
    }
    sayVerifyV1("census.owner-migrated-unreachable-other-unreachable");
    for (const [owner, row] of [...perOwner].sort((left, right) => right[1].unreachable.length - left[1].unreachable.length)) console.log(`| ${owner} | ${row.migrated} | ${row.unreachable.length} | ${row.other} | ${row.unreachable.join(", ")} |`);
    for (const line of census.unparsed) sayVerifyV1("census.interactivity-commands-non-literal-declaration",{line:String(line)});
    for (const disagreement of census.callDisagreements) sayVerifyV1("census.interactivity-commands-oracle-disagreement",{disagreement:String(disagreement)});
    const status = unreachable.length === 0 && oracleAgrees && census.unparsed.length === 0 ? "pass" : "fail";
    sayVerifyV1("census.interactivity-commands-declarations-unreachable-non",{declarations:production.length,unreachable:unreachable.length,nonLiteral:census.unparsed.length,oracle:String(oracleAgrees ? "agrees" : "DISAGREES")});
    publishAcceptanceCheckResult(root, acceptanceCheckResult({
      check: "command-reachability",
      status,
      startedAt,
      measured: { declarations: production.length, unreachable: unreachable.length, nonLiteral: census.unparsed.length, oracleAgrees },
      summary: { en: `${unreachable.length}/${production.length} production commands unreachable (BatchOnlyPendingRewrite)${unreachable.length ? `: ${[...perOwner].filter(([, row]) => row.unreachable.length).map(([owner, row]) => `${owner} ${row.unreachable.length}`).join(", ")}` : ""}`, de: `${unreachable.length}/${production.length} produktive Befehle unerreichbar (BatchOnlyPendingRewrite)${unreachable.length ? `: ${[...perOwner].filter(([, row]) => row.unreachable.length).map(([owner, row]) => `${owner} ${row.unreachable.length}`).join(", ")}` : ""}` },
    }));
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}

/** 🏷️ The docstring census as gates (acceptance ledger 5.11): `at-emoji` fails on any docstring that opens with the `@emoji`
 * residue token instead of its emoji; `emoji-first` fails on any docstring that opens with neither an emoji nor a symbol
 * glyph. Both scan every tracked Rust / TypeScript source, cross-check the `@emoji` openers against `git grep`, print the
 * findings and publish the acceptance record.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts `runDocstringCensus` */
export async function runDocstringCensusGate(root: string, rule: string | undefined): Promise<void> {
  if (rule !== "at-emoji" && rule !== "emoji-first") throw refuseVerifyV1("usage","census.usage-docstrings-at-emoji-emoji");
  const { acceptanceCheckResult, publishAcceptanceCheckResult } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🟦️.ts");
  const { runDocstringCensus } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  const startedAt = new Date();
  try {
    const census = runDocstringCensus(root, controller.signal, (line) => sayVerifyV1("census.docstrings",{line:String(line)},"error"));
    const atEmoji = census.hits.filter((hit) => hit.rule === "at-emoji");
    const noEmoji = census.hits.filter((hit) => hit.rule === "no-emoji");
    const failing = rule === "at-emoji" ? atEmoji : census.hits;
    for (const hit of failing.slice(0, 200)) sayVerifyV1("census.docstrings-2",{rule:String(hit.rule),path:String(hit.path),line:String(hit.line),text:String(hit.text)});
    for (const disagreement of census.disagreements) sayVerifyV1("census.docstrings-oracle-disagreement",{disagreement:String(disagreement)});
    const status = failing.length === 0 && census.disagreements.length === 0 ? "pass" : "fail";
    sayVerifyV1("census.docstrings-rule-files-at-emoji",{rule:String(rule),files:String(census.files),atEmoji:atEmoji.length,noEmoji:noEmoji.length,oracle:String(census.disagreements.length === 0 ? "agrees" : "DISAGREES")});
    publishAcceptanceCheckResult(root, acceptanceCheckResult({
      check: rule === "at-emoji" ? "docstring-at-emoji" : "docstring-emoji-first",
      status,
      startedAt,
      measured: { files: census.files, atEmoji: atEmoji.length, noEmoji: noEmoji.length, oracleAgrees: census.disagreements.length === 0 },
      summary: { en: `${failing.length} docstrings break the ${rule} rule in ${census.files} Rust/TypeScript sources (@emoji ${atEmoji.length}, no emoji ${noEmoji.length})`, de: `${failing.length} Docstrings verletzen die Regel ${rule} in ${census.files} Rust-/TypeScript-Quellen (@emoji ${atEmoji.length}, ohne Emoji ${noEmoji.length})` },
    }));
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}

/** ❄️ The per-file docstring-emoji gate (acceptance ledger 5.11, design §21.2): fails on any emoji that opens more than one
 * docstring of one source. Without `--files-from <list>` it scans every tracked Rust / TypeScript source and publishes the
 * acceptance record; with it (one repository-relative path per line, e.g. a ticket's touched files) it scans exactly those. Prints
 * the reusing files with their repeat counts, most first.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts `runDocstringEmojiCensus` */
export async function runDocstringEmojiGate(root: string, args: readonly string[]): Promise<void> {
  const listAt = args.indexOf("--files-from");
  if ((listAt >= 0 && args[listAt + 1] === undefined) || args.length !== (listAt >= 0 ? 2 : 0)) throw refuseVerifyV1("usage","census.usage-docstrings-emoji-unique-files");
  const { acceptanceCheckResult, publishAcceptanceCheckResult } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🟦️.ts");
  const { runDocstringEmojiCensus } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts");
  const paths = listAt >= 0 ? readFileSync(resolve(root, args[listAt + 1]!), "utf8").split("\n").map((line) => line.trim()).filter(Boolean) : undefined;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  const startedAt = new Date();
  try {
    const census = runDocstringEmojiCensus(root, controller.signal, (line) => sayVerifyV1("census.docstrings",{line:String(line)},"error"), paths);
    const perFile = new Map<string, { repeats: number; emojis: string[] }>();
    for (const reuse of census.reuse) {
      const row = perFile.get(reuse.path) ?? { repeats: 0, emojis: [] };
      perFile.set(reuse.path, { repeats: row.repeats + reuse.lines.length - 1, emojis: [...row.emojis, `${reuse.emoji}×${reuse.lines.length} :${reuse.lines.join(",")}`] });
    }
    const ranked = [...perFile].sort(([leftPath, left], [rightPath, right]) => right.repeats - left.repeats || leftPath.localeCompare(rightPath));
    for (const [path, row] of ranked.slice(0, 200)) sayVerifyV1("census.docstrings-emoji-unique-repeats",{path:String(path),repeats:String(row.repeats),emojis:row.emojis.join(" ")});
    const repeats = ranked.reduce((sum, [, row]) => sum + row.repeats, 0);
    const status = repeats === 0 ? "pass" : "fail";
    sayVerifyV1("census.docstrings-rule-emoji-unique-files",{files:String(census.files),reusing:ranked.length,repeats:String(repeats),over:String(paths === undefined ? "" : ` over ${args[listAt + 1]}`)});
    if (paths === undefined) {
      publishAcceptanceCheckResult(root, acceptanceCheckResult({
        check: "docstring-emoji-unique",
        status,
        startedAt,
        measured: { files: census.files, reusing: ranked.length, repeats },
        summary: { en: `${repeats} docstrings reuse their file's opening emoji in ${ranked.length} of ${census.files} Rust/TypeScript sources`, de: `${repeats} Docstrings wiederholen das Eröffnungs-Emoji ihrer Datei in ${ranked.length} von ${census.files} Rust-/TypeScript-Quellen` },
      }));
    }
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}

/** 🪢️ The artifact-agnostic gates of the framework contract (design §22.31 c, §22.32 d). `framework-names` fails on every line of
 * framework code — outside tests, fixtures, oracles, generated trees and doc comments — that names an artifact, and prints each
 * `path:line`; `utility-machines` fails on every artifact that declares a `UtilityDefinition` and uses no tool-machine vocabulary,
 * and prints every utility artifact with the vocabulary it uses. Neither publishes an acceptance record.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts `runArtifactNameCensus`, `runUtilityMachineCensus` */
export async function runAgnosticGate(root: string, args: readonly string[]): Promise<void> {
  if (args.length !== 1 || (args[0] !== "framework-names" && args[0] !== "utility-machines")) throw refuseVerifyV1("usage","census.usage-agnostic-framework-names-utility");
  const { runArtifactNameCensus, runUtilityMachineCensus } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🪢️artifact-neutrality/🟦️.ts");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  try {
    if (args[0] === "framework-names") {
      const census = runArtifactNameCensus(root, controller.signal, (line) => sayVerifyV1("census.agnostic",{line:String(line)},"error"));
      for (const hit of census.hits) sayVerifyV1("census.agnostic-framework-names",{path:String(hit.path),line:String(hit.line),name:String(hit.name),text:String(hit.text)});
      for (const disagreement of census.disagreements) sayVerifyV1("census.agnostic-oracle-disagreement",{disagreement:String(disagreement)});
      sayVerifyV1("census.agnostic-rule-framework-names-files",{files:String(census.files),naming:new Set(census.hits.map((hit) => hit.path)).size,lines:census.hits.length,oracle:String(census.disagreements.length === 0 ? "agrees" : "DISAGREES")});
      if (census.hits.length > 0 || census.disagreements.length > 0) process.exitCode = 1;
      return;
    }
    const census = runUtilityMachineCensus(root, controller.signal, (line) => sayVerifyV1("census.agnostic",{line:String(line)},"error"));
    for (const artifact of census.artifacts) sayVerifyV1("census.agnostic-utility-machines-utilities-vocabulary",{role:String(artifact.machine.length === 0 ? "OFFENDER" : "machine"),artifact:String(artifact.artifact),utilities:String(artifact.utilities),vocabulary:String(artifact.machine.join(",") || "none")});
    sayVerifyV1("census.agnostic-rule-utility-machines-files",{files:String(census.files),artifacts:census.artifacts.length,offenders:census.offenders.length});
    if (census.offenders.length > 0) process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}

/** 🐘️ The structural cap gate (audit F3): every mutation leaf whose payload schema caps its inverse rows
 * (`x-semio-inverse-rows.bounded`) is wrapped by a non-generic `#[derive(Mutations)]` aggregate of its artifact, so the derived
 * payload law — the per-leaf oracle of the cap — exists. Prints the bounded leaves per plugin and every `capLawMissing` leaf;
 * `--under <repository-relative path>` narrows it and refuses a path that holds no bounded leaf. Publishes no acceptance record.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🐘️mutation-caps/🟦️.ts `mutationCapReport` */
export async function runMutationCapGate(root: string, args: readonly string[]): Promise<void> {
  if (!(args.length === 0 || (args.length === 2 && args[0] === "--under" && args[1] !== ""))) throw refuseVerifyV1("usage","census.usage-mutation-caps-under-repository");
  const { mutationCapReport } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🧬️schema/📋️orchestration/🐘️mutation-caps/🟦️.ts");
  const report = mutationCapReport(root, args[1] ?? "");
  if (args.length === 2 && report.leaves.length === 0) throw refuseVerifyV1("failed","census.mutation-caps-under-holds-no",{path:String(JSON.stringify(args[1]))});
  const plugins = new Map<string, number>();
  for (const leaf of report.leaves) plugins.set(leaf.directory.split("/").slice(0, 3).join("/"), (plugins.get(leaf.directory.split("/").slice(0, 3).join("/")) ?? 0) + 1);
  for (const [plugin, count] of [...plugins].sort(([left], [right]) => left.localeCompare(right))) sayVerifyV1("census.mutation-caps-bounded-missing",{plugin:String(plugin),bounded:String(count),missing:report.findings.filter((finding) => finding.directory.startsWith(`${plugin}/`)).length});
  for (const finding of report.findings) sayVerifyV1("census.mutation-caps",{code:String(finding.code),directory:String(finding.directory),detail:String(finding.detail)});
  sayVerifyV1("census.mutation-caps-bounded-caplawmissing",{bounded:report.leaves.length,capLawMissing:report.findings.length});
  if (report.findings.length > 0) process.exitCode = 1;
}

/** 🐞️ The debug-tag census as a gate (acceptance ledger 5.12): fails on any tracked line outside the ticket tree and Markdown
 * prose that carries the tag AGENTS.md reserves for temporary logs, cross-checks the scanner against `git grep -c`, prints
 * the findings and publishes the acceptance record.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts `runDebugTagCensus` */
export async function runDebugTagCensusGate(root: string): Promise<void> {
  const { acceptanceCheckResult, publishAcceptanceCheckResult } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🟦️.ts");
  const { runDebugTagCensus } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  const startedAt = new Date();
  try {
    const census = runDebugTagCensus(root, controller.signal, (line) => sayVerifyV1("census.debug-tags",{line:String(line)},"error"));
    for (const hit of census.hits.slice(0, 200)) sayVerifyV1("census.debug-tags-2",{path:String(hit.path),line:String(hit.line),text:String(hit.text)});
    for (const disagreement of census.disagreements) sayVerifyV1("census.debug-tags-oracle-disagreement",{disagreement:String(disagreement)});
    const status = census.hits.length === 0 && census.disagreements.length === 0 ? "pass" : "fail";
    sayVerifyV1("census.debug-tags-files-lines-oracle",{files:String(census.files),lines:census.hits.length,oracle:String(census.disagreements.length === 0 ? "agrees" : "DISAGREES")});
    publishAcceptanceCheckResult(root, acceptanceCheckResult({
      check: "debug-tags",
      status,
      startedAt,
      measured: { files: census.files, lines: census.hits.length, oracleAgrees: census.disagreements.length === 0 },
      summary: { en: `${census.hits.length} lines in ${census.files} tracked sources still carry the temporary-log tag`, de: `${census.hits.length} Zeilen in ${census.files} verfolgten Quellen tragen noch die Markierung für temporäre Protokolle` },
    }));
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}

/** 🧱️ The interface-ownership census as a gate (acceptance ledger 1.10 / 5.9): fails on any production TypeScript / JavaScript
 * source that imports an interface-owned external library (three.js → the ui module) from outside its owner, cross-checks
 * the scanner against `git grep`, prints the findings and publishes the acceptance record.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts `runInterfaceImportCensus` */
export async function runInterfaceImportGate(root: string): Promise<void> {
  const { acceptanceCheckResult, publishAcceptanceCheckResult } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🟦️.ts");
  const { runInterfaceImportCensus } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🔌️interface-import-census/🟦️.ts");
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  const startedAt = new Date();
  try {
    const census = runInterfaceImportCensus(root, controller.signal, (line) => sayVerifyV1("census.interface-owners",{line:String(line)},"error"));
    for (const hit of census.hits) sayVerifyV1("census.interface-owners-imports-outside",{path:String(hit.path),line:String(hit.line),specifier:String(hit.specifier),owner:String(hit.owner)});
    for (const disagreement of census.disagreements) sayVerifyV1("census.interface-owners-oracle-disagreement",{disagreement:String(disagreement)});
    const status = census.hits.length === 0 && census.disagreements.length === 0 ? "pass" : "fail";
    sayVerifyV1("census.interface-owners-files-imports-oracle",{files:String(census.files),imports:census.hits.length,oracle:String(census.disagreements.length === 0 ? "agrees" : "DISAGREES")});
    publishAcceptanceCheckResult(root, acceptanceCheckResult({
      check: "interface-owned-imports",
      status,
      startedAt,
      measured: { files: census.files, imports: census.hits.length, oracleAgrees: census.disagreements.length === 0 },
      summary: { en: `${census.hits.length} production imports of an interface-owned library outside its owner in ${census.files} sources`, de: `${census.hits.length} produktive Importe einer schnittstellengebundenen Bibliothek außerhalb ihres Besitzers in ${census.files} Quellen` },
    }));
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}
