import { readTestSandboxProtocolV1, type TestSandboxProtocolV1 } from "../📐️layout/🟦️.ts";

/** 🔍️ Names a language the scanner understands. */
export type ScanLanguageV1 = "ts" | "go" | "rs" | "py" | "sh";

/** 🔎️ One finding: where a test breaks the concurrency law. */
export type ScanFindingV1 = Readonly<{ file: string; rule: string; severity: "error" | "warn"; line: number }>;

type RuleV1 = TestSandboxProtocolV1["rules"][number];
type CompiledRuleV1 = Readonly<{ rule: RuleV1; pattern?: RegExp; also: readonly RegExp[]; exempt: readonly RegExp[]; all: readonly RegExp[]; none: readonly RegExp[] }>;

/** ⚖️ Carries the compiled rule table and path policy of one law. */
export type CompiledLawV1 = Readonly<{ declared: TestSandboxProtocolV1; rules: readonly CompiledRuleV1[]; comment: Readonly<Record<string, RegExp>>; suite: Readonly<Record<string, RegExp>>; extensions: ReadonlyMap<string, ScanLanguageV1> }>;

/** 🔨️ Compiles the rule table once; every pattern must be valid in the JavaScript and the Python dialect the oracle replays. */
export function compileLawV1(declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): CompiledLawV1 {
  const compile = (source: string, flags: string): RegExp => new RegExp(source, flags);
  const rules = declared.rules.map((rule): CompiledRuleV1 => {
    const row = rule as RuleV1 & { flags?: string; pattern?: string; also?: readonly string[]; exempt?: readonly string[]; all?: readonly string[]; none?: readonly string[] }, flags = row.flags ?? "";
    return { rule, pattern: row.pattern === undefined ? undefined : compile(row.pattern, flags), also: (row.also ?? []).map(source => compile(source, flags)), exempt: (row.exempt ?? []).map(source => compile(source, flags)), all: (row.all ?? []).map(source => compile(source, flags)), none: (row.none ?? []).map(source => compile(source, flags)) };
  });
  const extensions = new Map<string, ScanLanguageV1>();
  for (const [language, list] of Object.entries(declared.scan.extensions)) for (const extension of list) extensions.set(extension, language as ScanLanguageV1);
  const mapped = (table: Readonly<Record<string, string>>): Record<string, RegExp> => Object.fromEntries(Object.entries(table).map(([language, source]) => [language, compile(source, "")]));
  return { declared, rules, comment: mapped(declared.scan.commentLine), suite: mapped(declared.scan.suite), extensions };
}

/** ⏩️ Resolves the language of a path, or undefined when the scanner skips it. */
export function scanLanguageV1(law: CompiledLawV1, path: string): ScanLanguageV1 | undefined {
  const base = path.split("/").pop() ?? "";
  return base.includes(".") ? law.extensions.get(base.slice(base.lastIndexOf(".") + 1)) : undefined;
}

/** 🛤️ Tells whether a tracked path is a test source: below the taxonomy test segment, a test-named file, and outside generated or fixture trees. */
export function scanIsTestPathV1(law: CompiledLawV1, path: string, declared: ReadonlySet<string> = new Set()): boolean {
  const parts = path.split("/"), base = parts[parts.length - 1] ?? "";
  if (parts.some(part => law.declared.scan.skipSegments.includes(part))) return false;
  return parts.slice(0, -1).includes(law.declared.scan.testSegment) || declared.has(path) || base.endsWith("_test.go") || /\.(test|spec)\.[a-z]+$/u.test(base);
}

/** 📄️ Tells whether a source file is a runnable suite; provers and helpers that callers drive are not scanned. */
export function scanIsSuiteV1(law: CompiledLawV1, language: ScanLanguageV1, text: string): boolean {
  return law.suite[language]!.test(text);
}

/** 🪵️ Scans one source against the line and file rules of its language. */
export function scanSourceV1(law: CompiledLawV1, file: string, language: ScanLanguageV1, text: string): readonly ScanFindingV1[] {
  if (!scanIsSuiteV1(law, language, text)) return [];
  const rows: ScanFindingV1[] = [], lines = text.split("\n"), comment = law.comment[language]!, code = lines.map(line => comment.test(line) ? "" : line).join("\n");
  for (const compiled of law.rules) {
    const { rule } = compiled;
    if (!(rule.languages as readonly string[]).includes(language)) continue;
    if (rule.scope === "line") {
      lines.forEach((line, index) => {
        if (comment.test(line) || !compiled.pattern!.test(line) || !compiled.also.every(also => also.test(line)) || compiled.exempt.some(exempt => exempt.test(line))) return;
        rows.push({ file, rule: rule.id, severity: rule.severity as "error" | "warn", line: index + 1 });
      });
    } else if (rule.scope === "file" && compiled.all.every(all => all.test(code)) && !compiled.none.some(none => none.test(code))) {
      const first = lines.findIndex(line => compiled.all[0]!.test(line));
      rows.push({ file, rule: rule.id, severity: rule.severity as "error" | "warn", line: first < 0 ? 1 : first + 1 });
    }
  }
  return rows;
}

/** 🧪️ Scans the sources of one test owner as a unit: owner rules need a pattern somewhere and no sandbox pattern anywhere. */
export function scanOwnerV1(law: CompiledLawV1, owner: string, sources: readonly Readonly<{ language: ScanLanguageV1; text: string }>[]): readonly ScanFindingV1[] {
  const rows: ScanFindingV1[] = [];
  for (const compiled of law.rules) {
    if (compiled.rule.scope !== "owner") continue;
    const text = sources.filter(source => (compiled.rule.languages as readonly string[]).includes(source.language)).map(source => source.text).join("\n");
    if (text !== "" && compiled.all.every(all => all.test(text)) && !compiled.none.some(none => none.test(text))) rows.push({ file: `${owner}/${law.declared.scan.testSegment}`, rule: compiled.rule.id, severity: compiled.rule.severity as "error" | "warn", line: 1 });
  }
  return rows;
}

/** ▶️ Runs the whole scan over a listing; the caller supplies listing and reading so the scan stays tool-neutral. */
export function scanTestsV1(law: CompiledLawV1, listing: readonly string[], read: (path: string) => string | undefined, declared: ReadonlySet<string> = new Set()): Readonly<{ scanned: number; helpers: number; findings: readonly ScanFindingV1[] }> {
  const findings: ScanFindingV1[] = [], owners = new Map<string, { language: ScanLanguageV1; text: string }[]>();
  let scanned = 0, helpers = 0;
  for (const path of listing) {
    const language = scanLanguageV1(law, path);
    if (!language || !scanIsTestPathV1(law, path, declared)) continue;
    const text = read(path);
    if (text === undefined || text.length > law.declared.scan.maximumBytes) continue;
    const parts = path.split("/"), at = parts.indexOf(law.declared.scan.testSegment);
    if (at >= 0) { const owner = parts.slice(0, at).join("/"); (owners.get(owner) ?? owners.set(owner, []).get(owner)!).push({ language, text }); }
    if (!scanIsSuiteV1(law, language, text)) { helpers++; continue; }
    scanned++;
    findings.push(...scanSourceV1(law, path, language, text));
  }
  for (const [owner, sources] of owners) findings.push(...scanOwnerV1(law, owner, sources));
  return { scanned, helpers, findings };
}
