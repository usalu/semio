/** 🔩️ Tool-job coverage: concrete factory resolution, microsecond budget laws, proof catalog and reserved routes.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { POLICY_SPLIT_SOURCE_ROOTS, policyModuleTreeFiles, policyReadFileSafe, policyReadModuleTreeSource } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { posix, relative, sep } from "node:path";
import { listSourceFamily, readSourceFamilySource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { policyAllRustFiles } from "../../🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { TOOL_JOB_FRAMEWORK_RESERVED_IDS, TOOL_JOB_HOST_CONFIGURATION_IDS, toolJobAppOwnedRows, toolJobDispositions, toolJobProductionProtocolSource, toolJobProductionSource, type ToolJobProof, toolJobProofIdentity, toolJobProofs, toolJobResolveTypeAlias, toolJobRustBlock, toolJobScanThenMonolithRows, type ToolJobStaticRow, toolJobStaticRows } from "../📋️inventory/🟦️.ts";
import { toolJobMountedDispatchOneTurnExact, toolJobRetainedDispatchSetup } from "../📤️publication/🟦️.ts";

//#region 🧭️ConcreteFactoryResolution
/** 🗿️ An artifact crate's own lib root (`🗿️artifacts/<artifact>/🦀️.rs`, declared by that artifact's
 * `📦️packages/🦀️rust/Cargo.toml` `[lib] path = "../../🦀️.rs"`), which owns the `#[path]` module tree
 * since the plugin package root became a re-export shell. */
function toolJobArtifactCrateRoot(file: string): string | undefined {
  const match = file.match(/^(.*\/🗿️artifacts\/[^/]+)\/🦀️\.rs$/);
  return match?.[1];
}

/** 🧭️ Resolves one crate module through its package's explicit taxonomy declarations. */
export function toolJobFactoryModuleFile(files: ReadonlyMap<string, string>, ownerFile: string, modulePath: string): string | undefined {
  const candidates = new Set<string>();
  const visit = (source: string, directory: string, prefix: string): void => {
    const declaration = /#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])/g;
    let match: RegExpExecArray | null;
    while ((match = declaration.exec(source))) {
      const target = posix.normalize(posix.join(directory, match[1]!));
      const name = `${prefix}::${match[2]}`;
      if (match[3] === ";") {
        const exportedComponent = prefix === modulePath && match[2] === "component" && /pub\s+use\s+component::\*\s*;/.test(source);
        if ((name === modulePath || exportedComponent) && files.has(target)) candidates.add(target);
        else if (modulePath.startsWith(`${name}::`) && files.has(target)) visit(files.get(target)!, posix.dirname(target), name);
      } else {
        const block = toolJobRustBlock(source, source.indexOf("{", match.index));
        if (!block) return;
        visit(block.body, target, name);
        declaration.lastIndex = block.end;
      }
    }
  };
  for (const [file, source] of files) {
    if (!file.endsWith("/🦀️.rs")) continue;
    const packageBoundary = file.lastIndexOf("/📦️packages/");
    const owningRoot = packageBoundary >= 0 ? file.slice(0, packageBoundary) : toolJobArtifactCrateRoot(file);
    if (owningRoot === undefined || !ownerFile.startsWith(`${owningRoot}/`)) continue;
    visit(source, posix.dirname(file), "crate");
  }
  return candidates.size === 1 ? [...candidates][0] : undefined;
}

/** 🧬️ Resolves a compiler type token, including an explicit import alias, to one concrete source type; a path through a carved sibling crate's re-export resolves to the one definition of that name inside the owner's artifact. */
function toolJobFactoryType(files: ReadonlyMap<string, string>, ownerFile: string, reference: string): { file: string; name: string } | undefined {
  if (!/^(?:crate::)?[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*$/.test(reference)) return undefined;
  const ownerSource = files.get(ownerFile) ?? "";
  let path = reference;
  if (!reference.includes("::")) {
    const imports = [...ownerSource.matchAll(/(?:^|\n)\s*use\s+(crate(?:::[A-Za-z_][A-Za-z0-9_]*)+)(?:\s+as\s+([A-Za-z_][A-Za-z0-9_]*))?\s*;/g)]
      .filter((entry) => (entry[2] ?? entry[1]!.split("::").at(-1)) === reference);
    const local = new RegExp(`(?:struct|enum)\\s+${reference}\\b`).test(ownerSource);
    if (local && imports.length === 0) return { file: ownerFile, name: reference };
    if (local || imports.length !== 1) return undefined;
    path = imports[0]![1]!;
  }
  const name = path.split("::").at(-1)!;
  if (TOOL_JOB_EDITOR_ROOT.test(ownerFile) && ownerSource.match(new RegExp(`(?:struct|enum)\\s+${name}\\b`, "g"))?.length === 1) return { file: ownerFile, name };
  if (!path.startsWith("crate::")) return undefined;
  const file = toolJobFactoryModuleFile(files, ownerFile, path.slice(0, -(name.length + 2)));
  if (file) return { file, name };
  const artifact = ownerFile.slice(0, Math.max(ownerFile.indexOf("/🏅️standards/"), 0));
  const definitions = artifact ? [...files].filter(([candidate, text]) => candidate.startsWith(`${artifact}/`) && new RegExp(`(?:struct|enum)\\s+${name}\\b`).test(text)) : [];
  return definitions.length === 1 ? { file: definitions[0]![0], name } : undefined;
}

/** 🔐️ Binds the compiler witness to its actual factory, exact owner, route, and live registration site. */
function toolJobConcreteFactoryExact(files: ReadonlyMap<string, string>, proof: ToolJobProof, ownerBody: string): boolean {
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(proof.factory) || !proof.factoryType) return false;
  const witness = toolJobFactoryType(files, proof.ownerFile, proof.factoryType);
  if (!witness || witness.name !== proof.factory) return false;
  const source = files.get(witness.file) ?? "";
  const implementation = (trait: string, namespace: string): string | undefined => {
    const start = source.search(new RegExp(`impl\\s+(?:${namespace}::)?${trait}\\s+for\\s+${witness.name}\\s*\\{`));
    return start < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", start))?.body;
  };
  const factory = implementation("ToolJobFactory", "semio_framework(?:_plugin)?");
  const owned = implementation("ArtifactOwnedToolJobFactory", "semio_framework_plugin");
  const canonicalOwner = (owner: string) => owner.replaceAll(/\s+/g, "").replace(/^semio_framework_plugin::/, "");
  if (!factory || !owned || !/type\s+Job\s*=/.test(factory) || !/fn\s+create_job\s*\(/.test(factory) || !/fn\s+execution_contract\s*\(/.test(factory)
    || canonicalOwner(toolJobResolveTypeAlias(source, owned.match(/type\s+Owner\s*=\s*([^;]+);/)?.[1] ?? "")) !== canonicalOwner(proof.ownerTypeName)) return false;
  const listExpression = owned.match(/const\s+TOOL_IDS\s*:[^=]+=\s*([^;]+);/)?.[1]?.trim();
  const listName = listExpression?.match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)$/)?.[1];
  const listPattern = listName ? new RegExp(`const\\s+${listName}\\s*:[^=]+=\\s*&\\[([^\\]]*)\\]`) : undefined;
  const carved = listPattern ? [...files.values()].flatMap((text) => (listPattern.exec(text)?.[1] ?? [])) : [];
  const list = listPattern ? (source.match(listPattern)?.[1] ?? (carved.length === 1 ? carved[0] : undefined)) : listExpression;
  if (!list) return false;
  const strings = new Map([...[source, ...files.values()].flatMap((text) => [...text.matchAll(/const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]+)"/g)]).map((row): [string, string] => [row[1]!, row[2]!])]);
  const toolIds = [...list.matchAll(/"([^"]+)"|\b([A-Z][A-Z0-9_]*)\b/g)].map((row) => row[1] ?? strings.get(row[2]!));
  if (!toolIds.includes(proof.toolId)) return false;
  const registeredIn = (body: string, file: string) => [...body.matchAll(/registry\.register\(\s*((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)::new\s*\(/g)].some((entry) => {
    const registered = toolJobFactoryType(files, file, entry[1]!);
    return registered?.file === witness.file && registered.name === witness.name;
  });
  if (registeredIn(ownerBody, proof.ownerFile)) return true;
  const selfRegistered = [...ownerBody.matchAll(/\b([A-Za-z_][A-Za-z0-9_]*)::register\(\s*registry\s*\)/g)].some((entry) => {
    if (entry[1] !== witness.name) return false;
    const inherent = source.match(new RegExp(`\\bimpl\\s+${witness.name}\\s*\\{`));
    const inherentBody = inherent ? toolJobRustBlock(source, source.indexOf("{", inherent.index))?.body : undefined;
    return !!inherentBody && new RegExp(`registry\\.register\\(\\s*(?:Self|${witness.name})::new\\s*\\(`).test(inherentBody);
  });
  if (selfRegistered) return true;
  return [...ownerBody.matchAll(/(crate(?:::[A-Za-z_][A-Za-z0-9_]*)+)::register\(registry\)/g)].some((entry) => {
    const carvedFamily = witness.file === proof.ownerFile && TOOL_JOB_EDITOR_ROOT.test(proof.ownerFile);
    if (!carvedFamily && toolJobFactoryModuleFile(files, proof.ownerFile, entry[1]!) !== witness.file) return false;
    return [...source.matchAll(/\bfn\s+register\s*\(/g)].some((delegated) => {
      const body = toolJobRustBlock(source, source.indexOf("{", delegated.index))?.body;
      return !!body && registeredIn(body, witness.file);
    });
  });
}




/** 🔌️ The text the plugin root file once held: every authored file of the plugin folder in path order, production and canonical tests, so a source-text law sees the facade, the carved crates and the laws that pin them as one text. */
export function toolJobPluginFamilySource(root: string): string {
  return readSourceFamilySource(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, { tests: true });
}

const TOOL_JOB_EDITOR_ROOT = /^(.*\/(?:✏️editor|👁️viewer))\/(?:🎹️app\/)?🦀️\.rs$/u;

/** 👪️ The repo-relative `.rs` files of the carved owner family below an editor or viewer root: the root first, then every authored file of the folders carved below it in path order; `🧪️tests` and `🧪️testing` stay out. */
export function toolJobOwnerFamilyFiles(root: string, ownerFile: string): string[] {
  const directory = ownerFile.match(TOOL_JOB_EDITOR_ROOT)?.[1] ?? posix.dirname(ownerFile);
  const family = listSourceFamily(`${root}/${directory}`).map((absolute) => relative(root, absolute).split(sep).join("/"));
  return [ownerFile, ...family.filter((file) => file !== ownerFile)];
}

/** 🧩️ The production text one owner row stands for: an editor or viewer root stands for its whole carved family (the `#[path]` children and the sibling crates carved below it), each member stripped of its test items on its own so one member's unbalanced braces cannot blank a later member, and so the typed command rows, the interactive-job dispositions, the proof catalogs, the owner impl and the factories stay in one text; any other file stands for its own module tree. */
export function toolJobOwnerProductionText(root: string, file: string): string {
  if (TOOL_JOB_EDITOR_ROOT.test(file)) return toolJobOwnerFamilyFiles(root, file).map((member) => { const text = toolJobProductionProtocolSource(policyReadFileSafe(root, member)); return text.endsWith("\n") ? text : `${text}\n`; }).join("");
  return toolJobProductionProtocolSource(POLICY_SPLIT_SOURCE_ROOTS.includes(file) ? policyReadModuleTreeSource(root, file) : policyReadFileSafe(root, file));
}

/** 🧩️ Alias for toolJobOwnerProductionText. */
export const toolJobOwnerSourceText = toolJobOwnerProductionText;


/** 🧱️ Mounts an artifact crate root that builds the app definition (`fn create_…(` returning `AppDefinition`) into that artifact's one editor family, so the declared actions and their dispositions are read beside the owner impl and its proofs. */
function toolJobMountDefinitionBuilders(sources: Map<string, string>): void {
  const mounted = new Set<string>();
  for (const [file, text] of [...sources]) {
    const artifact = file.match(/^(.*\/🗿️artifacts\/[^/]+)\/🦀️\.rs$/u)?.[1];
    if (!artifact || !/\bfn\s+create_[a-z0-9_]+\s*\([^)]*\)\s*->\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*AppDefinition\b/.test(text)) continue;
    const editors = [...sources.keys()].filter((candidate) => candidate.startsWith(`${artifact}/`) && /\/✏️editor\/(?:🎹️app\/)?🦀️\.rs$/u.test(candidate));
    if (editors.length !== 1) continue;
    sources.set(editors[0]!, `${sources.get(editors[0]!)}\n${text}`);
    mounted.add(file);
  }
  for (const file of mounted) sources.delete(file);
}

/** 🧬️ Appends the expansion of every single-arm declarative macro invocation to the text of the file that invokes it, so a factory generated by a shared `macro_rules!` is read as the `struct` and `impl` blocks it stands for. */
function toolJobExpandDeclarativeMacros(files: Map<string, string>): void {
  const macros = new Map<string, { params: string[]; body: string }>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/macro_rules!\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{\s*\(([^)]*)\)\s*=>\s*\{/g)) {
      const block = toolJobRustBlock(source, match.index + match[0].length - 1);
      if (block) macros.set(match[1]!, { params: [...match[2]!.matchAll(/\$([A-Za-z_][A-Za-z0-9_]*)\s*:\s*[a-z]+/g)].map((param) => param[1]!), body: block.body });
    }
  }
  if (macros.size === 0) return;
  for (const [file, source] of files) {
    let expansion = "";
    for (const [name, definition] of macros) {
      for (const invocation of source.matchAll(new RegExp(`(?:[A-Za-z_][A-Za-z0-9_]*::)*${name}!\\s*\\(([^()]*)\\)\\s*;`, "g"))) {
        const args = invocation[1]!.split(",").map((arg) => arg.trim());
        if (args.length === definition.params.length) expansion += `\n${definition.params.reduce((text, param, index) => text.replaceAll(`$${param}`, args[index]!), definition.body).replaceAll("$crate::", "crate::")}`;
      }
    }
    if (expansion) files.set(file, `${source}${expansion}`);
  }
}

/** 🪓️ The plugin production sources the tool-job census reads over a repo-relative `.rs` listing: a declared split root stands for its whole module tree and an editor or viewer root for its whole carved family; their members are no census entries of their own, so owner, proof, factory and row evidence stays in one text. */
export function toolJobProductionSourcesOf(root: string, listing: readonly string[]): Map<string, string> {
  const files = listing.filter((file) => file.includes("✏️s/🔌️plugins/") && !file.includes("/🧪️tests/"));
  const mounted = new Set([
    ...POLICY_SPLIT_SOURCE_ROOTS.filter((path) => path.includes("✏️s/🔌️plugins/")).flatMap((path) => policyModuleTreeFiles(root, path).slice(1)),
    ...files.filter((file) => TOOL_JOB_EDITOR_ROOT.test(file)).flatMap((file) => toolJobOwnerFamilyFiles(root, file).slice(1)),
  ]);
  const sources = new Map(files.filter((file) => !mounted.has(file)).map((file) => [file, toolJobOwnerProductionText(root, file)]));
  toolJobMountDefinitionBuilders(sources);
  toolJobExpandDeclarativeMacros(sources);
  return sources;
}

/** 🪓️ The plugin production sources of the whole repository. */
export function toolJobPluginProductionSources(root: string): Map<string, string> {
  return toolJobProductionSourcesOf(root, policyAllRustFiles(root));
}

/** 🧬️ The `*_SCHEMA` string constants declared by artifact crates outside the plugin folder, which a plugin's owner impl may name as its `DOCUMENT_SCHEMA`. */
export function toolJobExternalSchemaConstants(files: ReadonlyMap<string, string>): Map<string, string> {
  const constants = new Map<string, string>();
  for (const [file, source] of files) {
    if (file.includes("✏️s/🔌️plugins/") || !file.includes("/🗿️artifacts/")) continue;
    for (const match of source.matchAll(/(?:pub\s+)?const\s+([A-Z][A-Z0-9_]*_SCHEMA[A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]+)"/g)) constants.set(match[1]!, match[2]!);
  }
  return constants;
}

/** 🧪️ Scans every production app catalog for an exact compiler factory witness and rejects custom-covered generic rows. */
export function toolJobFactoryProofActivationScan(root: string): { owners: number; customRows: number; genericRows: number; failures: string[] } {
  const sources = toolJobPluginProductionSources(root);
  const proofs = toolJobProofs(sources);
  const failures = toolJobProofCatalogFailures(sources, toolJobStaticRows(sources), toolJobDispositions(sources), proofs).filter((failure) => failure.startsWith("forged bounded reducer factory or compiler witness"));
  return { owners: new Set(proofs.map((proof) => proof.ownerFile)).size, customRows: proofs.filter((proof) => proof.factoryType !== undefined).length, genericRows: proofs.filter((proof) => proof.factoryType === undefined).length, failures };
}

//#region ⏱️MicrosecondBudgetLaws
/** 🪢️ The production sources the exact microsecond worker law binds: typed dispatch, the mounted operation and its job, the worker authority, the job runtime, the trace clock and the step driver. */
export interface ToolJobMicrosecondSources {
  readonly dispatch: string;
  readonly mounted: string;
  readonly operation: string;
  readonly worker: string;
  readonly job: string;
  readonly trace: string;
  readonly loan: string;
}

/** 🪢️ Reads the production sources of the exact microsecond worker law from their owning crates. */
export function toolJobMicrosecondWorkerSources(root: string): ToolJobMicrosecondSources {
  const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
  const job = "🧰️framework/🔨️modules/🧵️job";
  return {
    dispatch: policyReadModuleTreeSource(root, `${plugin}/🧵️instance/📨️dispatch/🦀️.rs`),
    mounted: policyReadModuleTreeSource(root, `${plugin}/🧵️retained-command/🦀️.rs`),
    operation: policyReadModuleTreeSource(root, `${plugin}/👷️job/🦀️.rs`),
    worker: policyReadModuleTreeSource(root, `${job}/👷️worker/🎟️authority/🦀️.rs`),
    job: policyReadModuleTreeSource(root, `${job}/🦀️.rs`),
    trace: policyReadModuleTreeSource(root, "🧰️framework/🔨️modules/⏱️trace/🦀️.rs"),
    loan: policyReadFileSafe(root, `${job}/📬️outcome/🤝️loan/🦀️.rs`),
  };
}

/** ⏱️ Cross-checks exact microsecond deadlines with strict Ajv and independent BigInt arithmetic. */
export function toolJobMicrosecondWorkerExact(raw: ToolJobMicrosecondSources): boolean {
  const dispatch = toolJobProductionSource(raw.dispatch);
  const mounted = toolJobProductionSource(raw.mounted);
  const operation = toolJobProductionSource(raw.operation);
  const worker = toolJobProductionSource(raw.worker);
  const job = toolJobProductionSource(raw.job);
  const trace = toolJobProductionSource(raw.trace);
  const loan = toolJobProductionSource(raw.loan);
  const body = (source: string, marker: string): string | undefined => {
    if (source.split(marker).length !== 2) return undefined;
    return toolJobRustBlock(source, source.indexOf("{", source.indexOf(marker)))?.body;
  };
  const compact = (text: string | undefined): string | undefined => text?.replace(/\s+/g, "");
  const setup = body(dispatch, "pub async fn start_typed_command_operation(");
  const step = compact(body(mounted, "pub fn drive_worker_step("));
  const job_step = body(operation, "impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>");
  const driver = compact(body(loan, "pub fn drive_step<"));
  const fault = compact(body(worker, "fn publish_original_worker_fault<"));
  const authority = compact(body(worker, "fn drive_worker_job_authority<"));
  const duration = body(job, "pub fn from_duration(");
  const clock = body(job, "pub fn default_now_us(");
  const preflight = "ifcx.fuel_remaining()==0||cx.latest_us().map(|now|now>=cx.deadline_us()).unwrap_or_else(||cx.deadline_exceeded())";
  const loaned = "cx.with_original_work_loan(step_fuel,step_deadline_us,|same|session.pump_one_for_interactive_turn(same))";
  return !!setup && !!step && !!job_step && !!driver && !!authority && !!fault && !!duration && !!clock
    && setup.includes("contract: Some(admission.proof.contract()),")
    && setup.includes("ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes)")
    && step.includes("letcontract=self.contract.ok_or_else(")
    && step.includes("fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)}")
    && step.includes("letstep_fuel=cx.fuel_remaining().min(contract.max_work_units_per_step);")
    && step.includes("letstep_deadline_us=cx.now_us().map_or(cx.deadline_us(),|now|now.saturating_add(u64::from(contract.max_step_micros)));")
    && step.split(loaned).length === 2 && step.split("pump_one_for_interactive_turn(").length === 2
    && ["self.decoded_items > self.contract.max_decoded_items", "self.contract.max_work_units_per_step == 0", "self.contract.max_output_bytes"].every((token) => job_step.includes(token))
    && duration.includes("start_us.checked_add(duration_us).map(")
    && authority.includes("letretained=authority.issued_retained;")
    && authority.includes("letstart_us=(params.now_us)();")
    && authority.includes("start_us.and_then(|start_us|StepBudget::from_duration(config.fuel_per_step,start_us,config.step_budget_us,retained))")
    && authority.includes("authority.clock.begin(start_us)") && authority.includes("authority.last_step_end_us=start_us;")
    && fault.includes("JobOutcomeBorrow::admit_original_fault(params.operation,params.generation,authority.issued_retained,&mutauthority.retained_step_progress,")
    && driver.includes(preflight) && driver.indexOf(preflight) < driver.indexOf("job.step(cx)")
    && driver.includes("letend=(cx.now_us)();")
    && clock.includes("semio_framework_trace::try_now_us()")
    && /fn default_clock_us\(\) -> Option<u64> \{\s*None\s*\}/.test(trace)
    && trace.includes("elapsed().as_micros()).ok()");
}

//#endregion ⏱️MicrosecondBudgetLaws










/** 🧾️ Reports source-only exact route and publication evidence without running a compiler. */
export function toolJobOwnerSourceEvidence(sources: ReadonlyMap<string, string>) {
  const proofs = toolJobProofs(sources);
  const rows = toolJobStaticRows(sources);
  const dispositions = toolJobDispositions(sources);
  return {
    rows,
    proofs,
    dispositions: [...dispositions].map(([key, disposition]) => ({ key, disposition })),
    appOwned: toolJobAppOwnedRows(sources),
    scanThenMonolith: toolJobScanThenMonolithRows(sources, proofs),
    failures: toolJobProofCatalogFailures(sources, rows, dispositions, proofs),
  };
}
//#endregion 🧭️ConcreteFactoryResolution

/** 🧬️ Enforces an exact dynamic proof/declaration bijection without a shared magic row count. */
export function toolJobProofCatalogFailures(files: ReadonlyMap<string, string>, rows: readonly ToolJobStaticRow[], dispositions: ReadonlyMap<string, string>, proofs: readonly ToolJobProof[], externalSchemaConstants: ReadonlyMap<string, string> = new Map()): string[] {
  const failures: string[] = [];
  const appOwned = toolJobAppOwnedRows(files);
  const identities = new Set<string>();
  const publicKeys = new Set<string>();
  const constants = new Map<string, string>(externalSchemaConstants);
  for (const source of files.values()) for (const match of source.matchAll(/(?:pub\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]+)"/g)) constants.set(match[1]!, match[2]!);
  const associatedConstants = new Map<string, Array<{ file: string; value?: string }>>();
  for (const [file, source] of files) {
    for (const alias of source.matchAll(/(?:pub\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*([A-Za-z_][A-Za-z0-9_]*)::([A-Z][A-Z0-9_]*)\s*;/g)) {
      const values: string[] = [];
      const implementations = new RegExp(`impl\\s+${alias[2]}\\s*\\{`, "g");
      for (const implementation of source.matchAll(implementations)) {
        const body = toolJobRustBlock(source, source.indexOf("{", implementation.index))?.body ?? "";
        const value = body.match(new RegExp(`const\\s+${alias[3]}\\s*:\\s*&(?:'static\\s+)?str\\s*=\\s*"([^"]+)"`))?.[1];
        if (value !== undefined) values.push(value);
      }
      const entries = associatedConstants.get(alias[1]!) ?? [];
      entries.push({ file, value: values.length === 1 ? values[0] : undefined });
      associatedConstants.set(alias[1]!, entries);
    }
  }
  if (proofs.length === 0) failures.push("bounded reducer proof catalogs contain no exact owner-local rows");
  for (const proof of proofs) {
    const identity = toolJobProofIdentity(proof);
    const publicKey = `${proof.controllerId}\0${proof.toolId}`;
    if (!identities.add(identity)) failures.push(`duplicate bounded reducer proof identity ${identity}`);
    if (!publicKeys.add(publicKey)) failures.push(`ambiguous bounded reducer public key ${publicKey}`);
    const source = files.get(proof.sourceFile) ?? "";
    const editorOwner = proof.ownerTypeName.match(/^(?:semio_framework_plugin::)?EditorApp<([A-Za-z_][A-Za-z0-9_]*)>$/)?.[1];
    const viewerOwner = proof.ownerTypeName.match(/^(?:semio_framework_plugin::)?ViewerApp<([A-Za-z_][A-Za-z0-9_]*)>$/)?.[1];
    const directOwner = editorOwner || viewerOwner ? undefined : proof.ownerTypeName.match(/^(?:semio_framework_plugin::)?([A-Za-z_][A-Za-z0-9_]*)$/)?.[1];
    const owner = editorOwner ?? viewerOwner ?? directOwner;
    const ownerTrait = editorOwner ? "ArtifactEditor" : viewerOwner ? "ArtifactViewer" : directOwner ? "ArtifactApp" : undefined;
    const ownerPattern = owner && ownerTrait ? new RegExp(`impl\\s+(?:semio_framework_plugin::)?${ownerTrait}\\s+for\\s+${owner}\\s*\\{`) : undefined;
    const ownerStart = ownerPattern ? source.search(ownerPattern) : -1;
    const ownerOpen = ownerStart < 0 ? -1 : source.indexOf("{", ownerStart);
    const ownerImpl = ownerOpen < 0 ? undefined : toolJobRustBlock(source, ownerOpen);
    const schemaExpression = ownerImpl?.body.match(/const\s+DOCUMENT_SCHEMA\s*:\s*&'static\s+str\s*=\s*(?:"([^"]+)"|((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*))/);
    const schemaConstant = schemaExpression?.[2]?.split("::").at(-1) ?? "";
    let declaredSchema = schemaExpression?.[1] ?? constants.get(schemaConstant);
    if (!schemaExpression?.[1] && associatedConstants.has(schemaConstant)) {
      const paths = new Set<string>();
      const expression = schemaExpression?.[2] ?? "";
      if (expression.startsWith("crate::")) paths.add(expression.slice(0, -(schemaConstant.length + 2)));
      else {
        for (const imported of source.matchAll(/(?:^|\n)\s*use\s+(crate(?:::[A-Za-z_][A-Za-z0-9_]*)+)::\{([^}]+)\}\s*;/g)) {
          if (imported[2]!.split(",").some((entry) => entry.trim() === schemaConstant)) paths.add(imported[1]!);
        }
        for (const imported of source.matchAll(new RegExp(`(?:^|\\n)\\s*use\\s+(crate(?:::[A-Za-z_][A-Za-z0-9_]*)+)::${schemaConstant}\\s*;`, "g"))) paths.add(imported[1]!);
      }
      const candidates = associatedConstants.get(schemaConstant)!.filter((entry) => entry.file === proof.sourceFile || [...paths].some((path) => toolJobFactoryModuleFile(files, proof.sourceFile, path) === entry.file));
      declaredSchema = candidates.length === 1 ? candidates[0]!.value : undefined;
    }
    if (proof.sourceFile !== proof.ownerFile || !ownerImpl) failures.push(`forged bounded reducer owner/file authority ${identity}`);
    if (declaredSchema !== proof.documentSchema) failures.push(`forged bounded reducer document schema ${identity}`);
    const concreteFactoryExact = !!ownerImpl && toolJobConcreteFactoryExact(files, proof, ownerImpl.body);
    const generic = proof.factory === "BoundedFirstStepCommandJobFactory" && proof.factoryType === undefined
      && !appOwned.some((row) => row.ownerFile === proof.ownerFile && row.toolId === proof.toolId);
    if (!generic && !concreteFactoryExact) failures.push(`forged bounded reducer factory or compiler witness ${identity}`);
    const hostConfiguration = generic && TOOL_JOB_HOST_CONFIGURATION_IDS.includes(proof.toolId);
    if (!hostConfiguration && (!rows.some((row) => row.file === proof.ownerFile && row.id === proof.toolId) || dispositions.get(`${proof.ownerFile}\0${proof.toolId}`) !== "Migrated")) failures.push(`extra bounded reducer proof lacks its exact Migrated declaration ${identity}`);
  }
  for (const row of rows) {
    if (dispositions.get(`${row.file}\0${row.id}`) !== "Migrated") continue;
    const matches = proofs.filter((proof) => proof.ownerFile === row.file && proof.toolId === row.id);
    if (matches.length !== 1) failures.push(`Migrated declaration has ${matches.length} exact bounded reducer proofs ${row.file}\0${row.id}`);
  }
  return failures;
}

export type ToolJobReservedSpec = { id: string; job: string; factory: string; line: number; values: readonly number[]; expressions: readonly string[]; lanes: readonly string[] };

export function toolJobReservedSpecs(source: string): ToolJobReservedSpec[] {
  const specs: ToolJobReservedSpec[] = [];
  const row = /framework_reserved_job!\(\s*([A-Za-z0-9_]+),\s*([A-Za-z0-9_]+),\s*"([^"]+)",\s*\d+,\s*([\d_]+|[A-Za-z_][A-Za-z0-9_:]*(?:\s*\*\s*\d+)?),\s*([\d_]+|[A-Za-z_][A-Za-z0-9_:]*(?:\s*\*\s*\d+)?),\s*([\d_]+|[A-Za-z_][A-Za-z0-9_:]*(?:\s*\*\s*\d+)?),\s*([\d_]+|[A-Za-z_][A-Za-z0-9_:]*(?:\s*\*\s*\d+)?),\s*lanes:\s*\[([A-Za-z,\s]+)\],?\s*\);/g;
  for (const match of source.matchAll(row)) {
    const values = match.slice(4, 8).map((value) => (/^[\d_]+$/u.test(value!) ? Number(value!.replaceAll("_", "")) : Number.NaN));
    const lanes = match[8]!.split(",").map((lane) => lane.trim());
    if (values.every((value) => Number.isNaN(value) || (Number.isFinite(value) && value > 0)) && lanes.every((lane) => ["HostOnly", "Artifact", "Child"].includes(lane)) && new Set(lanes).size === lanes.length) specs.push({ id: match[3]!, job: match[1]!, factory: match[2]!, line: source.slice(0, match.index).split("\n").length, values, expressions: match.slice(4, 8).map((value) => value!.trim()), lanes });
  }
  return specs;
}

export function toolJobFrameworkReservedRoutesExact(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_action");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  const macroIds = TOOL_JOB_FRAMEWORK_RESERVED_IDS.filter((id) => id !== "configuration-binary");
  const specs = toolJobReservedSpecs(source).filter((spec) => macroIds.includes(spec.id as (typeof macroIds)[number]));
  const ids = new Set(specs.map((spec) => spec.id));
  const uniqueJobs = new Set(specs.map((spec) => spec.job));
  const uniqueFactories = new Set(specs.map((spec) => spec.factory));
  if (!block || specs.length !== macroIds.length || ids.size !== macroIds.length || uniqueJobs.size !== macroIds.length || uniqueFactories.size !== macroIds.length || macroIds.some((id) => !ids.has(id))) return false;
  if (specs.some((spec) => spec.lanes.join("\0") !== (["cut", "paste", "import-media"].includes(spec.id) ? ["Artifact", "Child"] : ["HostOnly"]).join("\0"))) return false;
  const reservedStart = source.lastIndexOf("async fn dispatch_framework_reserved_action");
  const reservedOpen = reservedStart < 0 ? -1 : source.indexOf("{", reservedStart);
  const reserved = reservedOpen < 0 ? undefined : toolJobRustBlock(source, reservedOpen);
  const runnerStart = source.lastIndexOf("async fn run_framework_reserved_job");
  const runnerOpen = runnerStart < 0 ? -1 : source.indexOf("{", runnerStart);
  const runner = runnerOpen < 0 ? undefined : toolJobRustBlock(source, runnerOpen);
  if (!reserved || !runner) return false;
  const noDirectBranches = !block.body.includes("else if HISTORY_ACTION_IDS.contains(&action)") && !block.body.includes("else if CLIPBOARD_ACTION_IDS.contains(&action)") && !block.body.includes("self.store.dispatch(command)");
  const exactGuard = block.body.includes("return self.dispatch_framework_reserved_action(action, args, meta).await");
  const exactRoutes = specs.every(
    (spec) => source.includes(`registry.register(${spec.factory}::<A>::new(&controller_id))`) && reserved.body.includes(`${spec.job}::new(`),
  );
  const configurationExact =
    source.includes("impl<A: ArtifactApp> semio_framework_job::InteractiveJob for FrameworkConfigurationBinaryJob<A>") &&
    source.includes("registry.register(FrameworkConfigurationBinaryJobFactory::<A>::new(&controller_id))") &&
    source.includes('const TOOL_IDS: &\'static [&\'static str] = &["configuration-binary"]') &&
    source.includes('run_framework_reserved_job("configuration-binary"') &&
    source.includes("FrameworkConfigurationBinaryStage::Envelope") &&
    source.includes("FrameworkConfigurationBinaryStage::Decode") &&
    source.includes("FrameworkConfigurationBinaryStage::Complete");
  const pluginSeamExact =
    source.includes("fn register_tool_job_factories(_registry: &mut ArtifactToolFactoryRegistry<'_, Self>)") &&
    source.includes("fn build_reserved_tool_job(_request: ArtifactReservedToolJobRequest<Self>)") &&
    source.includes("A::register_tool_job_factories(&mut app_tool_registry)") &&
    source.includes("A::build_reserved_tool_job(request)") &&
    source.includes("registry.register(FrameworkCopyJobFactory::<A>::new(&controller_id))") &&
    source.includes("registry.register(FrameworkCutJobFactory::<A>::new(&controller_id))") &&
    source.includes("registry.register(FrameworkPasteJobFactory::<A>::new(&controller_id))") &&
    !source.includes("registry.register(FrameworkImportMediaJobFactory::<A>::new(&controller_id))");
  const routeStateMachines = !/impl\s+FrameworkReservedCursor\s*\{[\s\S]*?fn\s+step\s*\(/.test(source) && source.includes('cx.set_stage(concat!("framework-reserved-", $route, "-envelope"))');
  const exactRunner =
    runner.body.includes("admit_exact_wire") &&
    runner.body.includes("proof.admits::<A>") &&
    ((runner.body.includes("WorkerJobSession::new") && runner.body.includes("session.step(&pool")) ||
      (runner.body.includes("MountedWorkerJobSession::try_new") && runner.body.includes("session.pump_one(&pool") && runner.body.includes("session.close_step("))) &&
    runner.body.includes("CheckpointReady(checkpoint)") &&
    runner.body.includes("candidate.output.len() > proof.contract().max_output_bytes");
  const commitPermit = source.includes("struct FrameworkReservedCommitPermit") && source.includes("validate_framework_reserved_commit(action, &permit).await?") && source.indexOf("permit.finish();", reservedStart) > source.indexOf("let result = if HISTORY_ACTION_IDS", reservedStart);
  const importStart = source.lastIndexOf("async fn dispatch_import_media");
  const importOpen = importStart < 0 ? -1 : source.indexOf("{", importStart);
  const importBlock = importOpen < 0 ? undefined : toolJobRustBlock(source, importOpen);
  const configStart = source.lastIndexOf("async fn dispatch_config_command_inner");
  const configOpen = configStart < 0 ? -1 : source.indexOf("{", configStart);
  const configBlock = configOpen < 0 ? undefined : toolJobRustBlock(source, configOpen);
  const binaryFailClosed =
    !!importBlock &&
    !!configBlock &&
    importBlock.body.includes("build_artifact_reserved_media_job") &&
    importBlock.body.includes('run_framework_reserved_job("import-media"') &&
    !importBlock.body.includes("A::import_media(") &&
    configBlock.body.includes("FrameworkConfigurationBinaryJob::<A>") &&
    configBlock.body.includes('run_framework_reserved_job("configuration-binary"') &&
    !configBlock.body.includes("decode_op(") &&
    source.includes("ArtifactCommand<A::ConfigMutation> as ::protocol::OpBinary>::decode_op(&self.raw)");
  const boundedPreparationAndHeldCommit =
    source.includes("FrameworkReservedStage::Admitted") &&
    source.includes("FrameworkReservedStage::Cursor") &&
    source.includes("FrameworkReservedStage::Retiring") &&
    reserved.body.includes("framework_reserved_work_items(action, args).await?") &&
    reserved.body.includes("validate_framework_reserved_commit(action, &permit).await?") &&
    reserved.body.includes("permit.finish()") &&
    source.includes("if permit.is_cancelled().await") &&
    !reserved.body.includes("dispatch_history_action(") &&
    !reserved.body.includes("dispatch_clipboard_action(") &&
    !importBlock?.body.includes("serde_json::to_vec(&(port, media))") &&
    !importBlock?.body.includes("media.clone()") &&
    source.includes("framework reserved mounted worker transition was rejected") &&
    source.includes("commit_framework_shared_host_route") &&
    source.includes("self.validate_framework_reserved_commit(action, permit).await?") &&
    source.includes("framework route '{verb}' completed against a stale revision or generation");
  return block.body.includes("interactive-job.unknown-key") && noDirectBranches && exactGuard && exactRoutes && configurationExact && pluginSeamExact && routeStateMachines && exactRunner && commitPermit && binaryFailClosed && boundedPreparationAndHeldCommit;
}
