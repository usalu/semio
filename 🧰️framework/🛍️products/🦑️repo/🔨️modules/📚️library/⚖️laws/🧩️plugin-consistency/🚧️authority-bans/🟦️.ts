/** 🚧️ Wave 4 plugin consistency policy: server-only db, OS state authority, pack files, raw spawn, null budget and MCP config bans.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { policyMaskLiterals, policyTestModSpans, policyLineInTestMod } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { POLICY_SKIP_DIRS, policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { AGENT_CLIENT_DECLARATION_MANIFEST, checkAgentClients, mcpServersOf, readDeclaredTools, type McpServer } from "../../../🤖️agent-clients/🟦️.ts";
import { existsSync, readdirSync, type Dirent } from "node:fs";
import { join } from "node:path";
import { policyDocumentAppUsages } from "../🧮️completeness/🟦️.ts";
import { policyAllRustFiles, policyExtractFnBody, policyPackageScriptModulePaths, policyRootScriptModulePaths } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleDbServerOnly
/** 🔎️Repo-wide `Cargo.toml` file paths (repo-relative), same walker + `POLICY_SKIP_DIRS` as the other policy file discoverers. */
export function policyDiscoverCargoTomlFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: Dirent[];
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "Cargo.toml") found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🛡️Directory prefixes allowed to carry a `db`/`db_*` Cargo dependency: the db family itself and the os hub server. */
const POLICY_DB_SERVER_ONLY_ALLOWED_PREFIXES = ["db/", "hub/"];

/** 🛡️True if `dir` (a repo-relative directory, trailing "/") may depend on `db`/`db_*`. */
function policyDbAllowedDir(dir: string): boolean {
  return POLICY_DB_SERVER_ONLY_ALLOWED_PREFIXES.some((prefix) => dir.startsWith(prefix));
}

const POLICY_DB_DEP_RE = /^(db(?:_[a-z0-9]+)*)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$/gm;

/**
 * 📏️db/ server-only rule: no `db`/`db_*` family Cargo dependency may live outside `db/` itself or the
 * `os-hub` server (`hub/`) — db is server-side storage for the hubs; clients keep local-first backbones (`vcs` + `store`) and
 * only ever reach db indirectly over the wire. `policyAppCouplingBreaches`'s plugin-tree coupling check
 * deliberately does not special-case `db/` so this stays the one gate that enforces it.
 */
export function policyDbServerOnlyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverCargoTomlFiles(repoRoot)) {
    const lastSlash = relPath.lastIndexOf("/");
    const dir = lastSlash === -1 ? "" : relPath.slice(0, lastSlash + 1);
    if (policyDbAllowedDir(dir)) continue;
    const text = policyReadFileSafe(repoRoot, relPath);
    let m: RegExpExecArray | null;
    POLICY_DB_DEP_RE.lastIndex = 0;
    while ((m = POLICY_DB_DEP_RE.exec(text))) {
      const depName = m[1]!;
      breaches.push({
        id: `db-server-only-${relPath}-${depName}`,
        summary: `"${relPath}" depends on "${depName}" outside db/'s server-only boundary`,
        kind: "protocol-migration/db-server-only",
        scope: relPath,
        priority: "high",
        reason: "db is server-side storage for the hubs — only db/ itself and hub/ may depend on a db/db_* crate; clients keep local-first backbones (vcs + store) and only ever reach db indirectly over the wire.",
        solution: `Remove the "${depName}" dependency from ${relPath}, or if this crate genuinely is a hub server, move or confirm it under db/ or hub/.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDbServerOnly

//#region 🔧️PolicyRuleOsStateAuthority
/** 🏛️OS product tree — the only place authoritative/session/host state may live after Wave 3 lands. */
const POLICY_OS_STATE_AUTHORITY_ROOT = "🧰️framework/🛍️products/💻️os/";
/** 🚫️Separate technologies excluded from OS-state scans (compose / mit-bestand / hub). */
const POLICY_OS_STATE_AUTHORITY_EXCLUDED_PREFIXES = ["compose/", "♻️mit-bestand/", "🌎️hub/"] as const;
const POLICY_OS_STATE_AUTHORITY_NAME_RE = /(Store|Registry|Host|Session|Engine|Kernel|World|Scene|State|Cache)$/;
const POLICY_OS_STATE_INTERIOR_TY_RE = /\b(?:OnceLock|OnceCell|LazyLock|Mutex|RwLock|RefCell|Cell)\s*</;
const POLICY_OS_STATE_STATIC_MUT_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+mut\b/;
const POLICY_OS_STATE_STATIC_CONST_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:static|const)\s+(?:mut\s+)?\w+\s*:\s*([^;=]+)/;
const POLICY_OS_STATE_FN_ITEM_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+\w+/;
const POLICY_OS_STATE_STRUCT_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+(\w+)\b/;
const POLICY_OS_STATE_FIELD_MAP_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:r#)?\w+\s*:\s*[^=\n]*(?:\bHashMap\s*<|\bBTreeMap\s*<)/;
const POLICY_OS_STATE_SEQ_U32_RE = /\bseq\s*:\s*u32\b/;
const POLICY_OS_STATE_ATOMIC_ID_RE = /\bAtomicU(?:32|64)\b/;
const POLICY_OS_STATE_THREAD_LOCAL_RE = /\bthread_local!\s*(?:\{|\()/;
const POLICY_OS_STATE_LAZY_STATIC_RE = /\blazy_static!\s*\{/;

/** 🛡️True when `relPath` is in scope for OS-exclusive state authority (outside OS product; not compose/mit-bestand/hub). */
function policyOsStateAuthorityPathInScope(relPath: string): boolean {
  const n = relPath.replace(/^\.\//, "");
  if (n.startsWith(POLICY_OS_STATE_AUTHORITY_ROOT)) return false;
  if (POLICY_OS_STATE_AUTHORITY_EXCLUDED_PREFIXES.some((p) => n.startsWith(p))) return false;
  return true;
}



/**
 * 📏️OS-exclusive state authority: outside `🧰️framework/🛍️products/💻️os/`, no item-scope interior
 * mutability / TLS / lazy globals, no Store|Registry|Host|… types with HashMap/BTreeMap fields, and no
 * `seq: u32` / `AtomicU32`/`AtomicU64` id minting. Skips test mods; excludes compose/mit-bestand/hub.
 */
export function policyOsStateAuthorityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!policyOsStateAuthorityPathInScope(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const lines = content.split(/\r?\n/);
    const testSpans = policyTestModSpans(lines);
    let depth = 0;
    const fnBodyDepths: number[] = [];
    const structStack: { name: string; depth: number; authority: boolean }[] = [];

    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      if (policyLineInTestMod(testSpans, lineNo)) {
        const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
        depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
        while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
        while (structStack.length > 0 && depth <= structStack[structStack.length - 1]!.depth) structStack.pop();
        return;
      }

      const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
      const inFn = fnBodyDepths.some((d) => depth >= d);
      const opensFn = !inFn && POLICY_OS_STATE_FN_ITEM_RE.test(raw) && /\{/.test(codeOnly);
      const structMatch = !inFn ? raw.match(POLICY_OS_STATE_STRUCT_RE) : null;

      if (!inFn) {
        if (POLICY_OS_STATE_STATIC_MUT_RE.test(raw)) {
          breaches.push({
            id: `os-state-authority-static-mut-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" declares item-scope static mut outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: item-scope static mut is forbidden outside 🧰️framework/🛍️products/💻️os/ — host DocumentStore / engine cache own authoritative state.",
            solution: `Move the state behind an OS host API (DocumentStore dispatch, draft lane, or content-addressed engine), or relocate the item under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        if (POLICY_OS_STATE_THREAD_LOCAL_RE.test(codeOnly)) {
          breaches.push({
            id: `os-state-authority-thread-local-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" uses thread_local! outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: thread_local! registries/caches are forbidden outside the OS product.",
            solution: `Replace thread_local! with host-owned session/draft state under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        if (POLICY_OS_STATE_LAZY_STATIC_RE.test(codeOnly)) {
          breaches.push({
            id: `os-state-authority-lazy-static-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" uses lazy_static! outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: lazy_static! process globals are forbidden outside the OS product.",
            solution: `Replace lazy_static! with an OS host registry / engine cache under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        const staticTy = raw.match(POLICY_OS_STATE_STATIC_CONST_RE)?.[1] ?? "";
        if (staticTy && POLICY_OS_STATE_INTERIOR_TY_RE.test(staticTy)) {
          const ty = staticTy.match(/\b(OnceLock|OnceCell|LazyLock|Mutex|RwLock|RefCell|Cell)\s*</)?.[1] ?? "interior";
          breaches.push({
            id: `os-state-authority-static-${ty}-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" declares item-scope ${ty}<…> outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: item-scope OnceLock/OnceCell/LazyLock/Mutex/RwLock/RefCell/Cell are forbidden outside the OS product.",
            solution: `Delete the static/const and route ownership through the OS host (DocumentStore, draft lane, or EngineCache) under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        if (POLICY_OS_STATE_SEQ_U32_RE.test(codeOnly) || POLICY_OS_STATE_ATOMIC_ID_RE.test(codeOnly)) {
          const kind = POLICY_OS_STATE_SEQ_U32_RE.test(codeOnly) ? "seq: u32" : "AtomicU32/64";
          breaches.push({
            id: `os-state-authority-id-mint-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" mints ids via ${kind} outside the OS product`,
            kind: "os-state-authority/id-minting",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: handle/id counters (seq: u32, AtomicU32/AtomicU64) must live in the OS host, not in plugins/s-modules/framework leaves.",
            solution: `Replace local id minting with content-addressed handles or OS-issued ids under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        const authorityStruct = structStack.length > 0 && depth > structStack[structStack.length - 1]!.depth ? structStack[structStack.length - 1] : undefined;
        if (authorityStruct?.authority && POLICY_OS_STATE_FIELD_MAP_RE.test(raw)) {
          breaches.push({
            id: `os-state-authority-map-field-${relPath}-${lineNo}-${authorityStruct.name}`,
            summary: `"${relPath}:${lineNo}" ${authorityStruct.name} holds a HashMap/BTreeMap field outside the OS product`,
            kind: "os-state-authority/authority-struct-map",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: types named *Store|*Registry|*Host|*Session|*Engine|*Kernel|*World|*Scene|*State|*Cache must not own HashMap/BTreeMap maps outside the OS product.",
            solution: `Move ${authorityStruct.name}'s map ownership into OS DocumentStore / host session / engine cache, or rename if it is not authoritative state.`,
          });
        }
      }

      if (opensFn) fnBodyDepths.push(depth + 1);
      if (structMatch && /\{/.test(codeOnly)) {
        structStack.push({ name: structMatch[1]!, depth, authority: POLICY_OS_STATE_AUTHORITY_NAME_RE.test(structMatch[1]!) });
      }

      depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
      while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
      while (structStack.length > 0 && depth <= structStack[structStack.length - 1]!.depth) structStack.pop();
    });
  }
  return breaches;
}

/** 🧬️True when `struct Name` in `content` declares at least one field (non-ZST unit/`{}` structs return false). */
function policyStructDeclaresFields(content: string, structName: string): { line: number; hasFields: boolean } | undefined {
  const re = new RegExp(String.raw`(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+${structName}\b`, "g");
  const m = re.exec(content);
  if (!m) return undefined;
  const line = policyLineOfIndex(content, m.index);
  const rest = content.slice(m.index + m[0].length);
  const open = rest.match(/^\s*([(;{])/);
  if (!open) return { line, hasFields: false };
  if (open[1] === ";") return { line, hasFields: false };
  if (open[1] === "(") {
    const tup = rest.match(/^\s*\(([^)]*)\)/);
    const inner = (tup?.[1] ?? "").replace(/\/\/[^\n]*/g, "").trim();
    return { line, hasFields: inner.length > 0 };
  }
  const body = policyExtractFnBody(content, m.index);
  if (!body.startsWith("{")) return { line, hasFields: false };
  const stripped = body
    .slice(1, -1)
    .replace(/\/\/[^\n]*/g, "")
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .trim();
  return { line, hasFields: stripped.length > 0 };
}

/**
 * 📏️DocumentApp shape law: every `impl DocumentApp for X` app type must be a ZST (no fields).
 * `register_document_app(` factory-closure args are checked only after the receiverless API lands —
 * today `PluginBundle::register_document_app` still takes `factory: impl Fn() -> A`, so that arm is a
 * deliberate no-op until Wave 1b deletes the factory.
 */
export function policyDocumentAppShapeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const n = relPath.replace(/^\.\//, "");
    if (POLICY_OS_STATE_AUTHORITY_EXCLUDED_PREFIXES.some((p) => n.startsWith(p))) continue;

    const content = policyReadFileSafe(repoRoot, relPath);
    const lines = content.split(/\r?\n/);
    const testSpans = policyTestModSpans(lines);

    for (const usage of policyDocumentAppUsages(relPath, content)) {
      if (policyLineInTestMod(testSpans, usage.line)) continue;
      const decl = policyStructDeclaresFields(content, usage.appType);
      if (!decl?.hasFields) continue;
      breaches.push({
        id: `document-app-shape-${relPath}-${usage.appType}`,
        summary: `"${relPath}" impl DocumentApp for ${usage.appType} but ${usage.appType} declares fields (must be a ZST)`,
        kind: "os-state-authority/document-app-shape",
        scope: relPath,
        line: decl.line,
        priority: "high",
        reason: "DocumentApp types must be receiverless ZSTs — fields become guest-owned state that bypasses the host DocumentStore.",
        solution: `Strip fields from ${usage.appType} (move session/scratch into OS draft lane / host DocumentSession) so it is a unit struct.`,
      });
    }

    // Receiverless register_document_app API not landed yet (`factory: impl Fn() -> A` still required) —
    // factory-argument breach scanning stays a no-op until Wave 1b deletes the factory parameter.
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleOsStateAuthority

//#region 🔧️PolicyRuleNoPackFiles
/** 🔎️Repo-wide `*.pack` file paths (repo-relative), same walker + `POLICY_SKIP_DIRS` as `policyDiscoverExampleJsonFiles`/`policyDiscoverOpsFiles`. */
function policyDiscoverPackFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: Dirent[];
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".pack")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/**
 * 📏️pack/ rollout rule: no `*.pack` binary file may ever be committed. Pack is authoritative-but-
 * regeneratable local storage + export/import (`FolderTextStorage`/`FolderSqliteStorage`'s pack
 * columns/files, both dev-disposable), never a checked-in artifact — examples stay committed DSL text
 * so diffs and golden hashes stay human-legible (golden fixtures are text blake3 hashes, not binary
 * blobs). `target/` and other build-artifact dirs are already excluded via `POLICY_SKIP_DIRS`.
 */
export function policyNoPackFilesBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverPackFiles(repoRoot)) {
    breaches.push({
      id: `no-pack-file-${relPath}`,
      summary: `"${relPath}" is a committed *.pack binary file — pack is dev-disposable and must never be committed`,
      kind: "pack-migration/no-pack-file",
      scope: relPath,
      priority: "high",
      reason: "pack's disk role is regeneratable local storage + export/import, not a committed artifact; committing binary .pack blobs would also break the text-diffability the DSL-mirror/golden-hash design relies on.",
      solution: `Delete ${relPath} from the repo (git rm) and rely on its DSL-text mirror / regenerate it via DocumentPack::encode_pack.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoPackFiles

//#region 🔧️PolicyRuleNoRawSpawn
const POLICY_RAW_SPAWN_EXEMPT = new Set(["./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts"]);
const POLICY_RAW_SPAWN_RE = /\b(spawnSync|execSync|execFileSync|Bun\.spawn|spawn)\s*\(/g;

/** 🔎️Strips TS/JS comments and string literals so policy regexes only see executable code. */
export function policyStripTsCommentsAndStrings(content: string): string {
  const out: string[] = [];
  let i = 0;
  while (i < content.length) {
    const ch = content[i]!;
    const next = content[i + 1];
    if (ch === "/" && next === "/") {
      while (i < content.length && content[i] !== "\n") {
        out.push(" ");
        i++;
      }
      continue;
    }
    if (ch === "/" && next === "*") {
      i += 2;
      while (i < content.length && !(content[i] === "*" && content[i + 1] === "/")) {
        out.push(" ");
        i++;
      }
      if (i < content.length) i += 2;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      const quote = ch;
      out.push(" ");
      i++;
      while (i < content.length) {
        if (content[i] === "\\") {
          out.push(" ");
          out.push(" ");
          i += 2;
          continue;
        }
        if (content[i] === quote) {
          out.push(" ");
          i++;
          break;
        }
        out.push(" ");
        i++;
      }
      continue;
    }
    out.push(ch);
    i++;
  }
  return out.join("");
}

/** 🔎️Repo-wide `script.ts` file paths (repo-relative), skipping node_modules/target/.🧬semio and other policy skip dirs. */
function policyDiscoverScriptTsFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: Dirent[];
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "📜️script.ts") found.push(childRel);
    }
  };
  walk("");
  return [...new Set([...found, ...policyRootScriptModulePaths(repoRoot).filter((path) => existsSync(join(repoRoot, path))), ...policyPackageScriptModulePaths(repoRoot).filter((path) => existsSync(join(repoRoot, path)))])].sort();
}

/**
 * 📏️Budgeted-spawn rule: every `script.ts` must route subprocesses through the budgeted runners in
 * `repo/lib/js/index.ts` — raw `spawn`/`spawnSync`/`execSync`/`execFileSync`/`Bun.spawn` bypass
 * wall-clock budgets and are forbidden outside the budget implementation itself.
 */
export function policyRawSpawnBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverScriptTsFiles(repoRoot)) {
    if (POLICY_RAW_SPAWN_EXEMPT.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const stripped = policyStripTsCommentsAndStrings(content);
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    let m: RegExpExecArray | null;
    POLICY_RAW_SPAWN_RE.lastIndex = 0;
    while ((m = POLICY_RAW_SPAWN_RE.exec(stripped))) {
      const token = m[1] ?? "spawn";
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-raw-spawn-${relPath}-${line}`,
        summary: `"${relPath}:${line}" uses raw ${token}( — route subprocesses through runCmd/runCmdStatus/runProbe/runRepositoryTestCommand from repo/lib/js/index.ts`,
        kind: "budget/no-raw-spawn",
        scope: relPath,
        line,
        priority: "high",
        reason: "Raw child_process/Bun.spawn calls bypass the repo's wall-clock budget layer; script.ts files must use the budgeted runners so orchestrators, tool calls and tests cannot hang forever.",
        solution: `Replace ${token}( at ${relPath}:${line} with runCmd, runCmdStatus, runProbe, or runRepositoryTestCommand from repo/lib/js/index.ts (or orchestratorBudgetOpts/daemonBudgetOpts for long-running classes).`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoRawSpawn

//#region 🔧️PolicyRuleNoBudgetNull
/**
 * 📏️Budget-null rule: `budgetMs: null` is not a supported escape hatch — use orchestratorBudgetOpts()
 * or daemonBudgetOpts() (or an explicit positive budget) so every subprocess has a named budget class.
 */
export function policyBudgetNullBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverScriptTsFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    const re = /budgetMs:\s*null/g;
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) {
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-budget-null-${relPath}-${line}`,
        summary: `"${relPath}:${line}" passes budgetMs: null — use orchestratorBudgetOpts() or daemonBudgetOpts() instead`,
        kind: "budget/no-budget-null",
        scope: relPath,
        line,
        priority: "high",
        reason: "budgetMs: null disables wall-clock budgeting for that subprocess; named orchestrator/daemon budget classes keep long-running work bounded without anonymous unbounded escapes.",
        solution: `At ${relPath}:${line}, replace budgetMs: null with orchestratorBudgetOpts() for build/script fan-outs or daemonBudgetOpts() for dev servers/MCP stdio, or pass an explicit positive budgetMs.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoBudgetNull

//#region 🔧️PolicyRuleMcpConfig
/** 🔎️True when an MCP server starts through the cross-platform Bun bootstrap that builds the native binary on demand. */
function policyMcpServerUsesBootstrap(server: McpServer): boolean {
  const args = server.args;
  return server.command === "bun" && args[0] === "./📜️script.ts" && args[1] === "dev" && args[2] === "mcp" && args[3] === "stdio" && Boolean(args[4]);
}

/**
 * 📏️MCP-config rule: every client file is derived from the root dashboard declaration (`bun ./📜️script.ts agents write`),
 * and every declared server launches through the Bun bootstrap so missing platform-specific binaries
 * are built before stdio is handed to the native MCP server.
 * @see ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🟦️.ts
 */
export function policyMcpConfigBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = checkAgentClients(repoRoot).map((drift) => ({
    id: `mcp-config-drift-${drift.path}`,
    summary: `"${drift.path}" ${drift.reason === "missing" ? "is missing" : "differs from the root dashboard declaration"}`,
    kind: "runtime/mcp-config-derived",
    scope: drift.path,
    priority: "high",
    reason: "Every agent and editor client file is derived from `metadata.semio.dashboard.tools[].mcp` of the root 📋️project.json; a hand edit or a stale copy makes the clients disagree.",
    solution: "Run `bun ./📜️script.ts agents write`.",
  }));
  for (const [client, servers] of Object.entries(mcpServersOf(readDeclaredTools(repoRoot)))) {
    for (const server of servers.filter((entry) => !policyMcpServerUsesBootstrap(entry))) {
      breaches.push({
        id: `mcp-config-${client}-${server.name}`,
        summary: `MCP server "${server.name}" of client ${client} does not use the portable repo MCP bootstrap`,
        kind: "runtime/mcp-config-repo-bootstrap",
        scope: AGENT_CLIENT_DECLARATION_MANIFEST,
        priority: "high",
        reason: "A checked-in native executable is platform-specific and may be absent in a fresh checkout; the Bun router resolves or builds the correct binary without writing protocol noise to stdout.",
        solution: `In ${AGENT_CLIENT_DECLARATION_MANIFEST}, set the tool command to ["bun", "./📜️script.ts", "dev", "mcp", "stdio", "<kind>"].`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleMcpConfig
