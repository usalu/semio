/** 🥒️ Parses the neutral restricted scenario contract shared by native test hosts. */
import { IMPLEMENTATIONS, TEST_MODES, type ComparisonProfile, type FeatureStep, type FeatureScenario, type TestMode, type Implementation } from "../🔌️adapter/🟦️.ts";
import { TEST_LEVELS, type TestLevel } from "../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
//#region 🥒️Gherkin
/** 🥒️ One `Given`/`When`/`Then` step. `And`/`But` inherit the previous step's canonical keyword. */


/** 🥒️ One executable scenario, after `Scenario Outline` expansion. */


/** 🥒️ The parsed, language-neutral behavioural contract of one test case. */
export type ParsedFeature = Readonly<{
  name: string;
  description: string;
  tags: readonly string[];
  capability: string | null;
  oracle: string | null;
  noOracleDecision: string | null;
  comparison: ComparisonProfile | null;
  /** 📥️ The produced subject artifact an oracle must consume, if it validates bytes rather than re-producing them. */
  oracleInput: "subject-raw" | null;
  /** 🦠️ The mutation catalog this feature claims to cover exhaustively, from `@mutations-<id>`. */
  mutationCatalog: string | null;
  background: readonly FeatureStep[];
  scenarios: readonly FeatureScenario[];
  errors: readonly string[];
}>;

const STEP_KEYWORDS = new Set(["Given", "When", "Then", "And", "But", "*"]);

function tagValue(tags: readonly string[], prefix: string): string | null {
  const hit = tags.find((tag) => tag.startsWith(prefix));
  return hit ? hit.slice(prefix.length) : null;
}

function tagValues(tags: readonly string[], prefix: string): string[] {
  return tags.filter((tag) => tag.startsWith(prefix)).map((tag) => tag.slice(prefix.length));
}

function splitTags(line: string): string[] {
  return line
    .split(/\s+/)
    .map((piece) => piece.trim())
    .filter((piece) => piece.startsWith("@"));
}

function splitTableRow(line: string): string[] {
  const trimmed = line.trim();
  const inner = trimmed.slice(1, trimmed.length - 1);
  return inner.split("|").map((cell) => cell.trim().replace(/\\\|/g, "|"));
}

function substitute(text: string, row: Readonly<Record<string, string>>): string {
  return text.replace(/<([^<>]+)>/g, (whole, key: string) => (key in row ? row[key]! : whole));
}

type FeatureBlock = { kind: "none" | "feature" | "background" | "scenario" | "outline"; name: string; tags: string[]; steps: FeatureStep[]; examples: Record<string, string>[]; line: number };

/**
 * 🥒️ Parses the repository's restricted Gherkin profile into one owned plan. The coordinator parses
 * a feature exactly once and hands every native host the resulting plan — no host re-reads or
 * reinterprets `component.feature`, which is what keeps five languages provably in agreement.
 * @see https://cucumber.io/docs/gherkin/reference/
 */
export function parseFeature(source: string): ParsedFeature {
  const errors: string[] = [];
  const lines = source.split(/\r?\n/);

  let featureName = "";
  const descriptionLines: string[] = [];
  let featureTags: string[] = [];
  let pendingTags: string[] = [];
  const background: FeatureStep[] = [];
  const scenarios: FeatureScenario[] = [];

  let block: FeatureBlock = { kind: "none", name: "", tags: [], steps: [], examples: [], line: 0 };
  let lastKeyword: "Given" | "When" | "Then" = "Given";
  let exampleHeader: string[] | null = null;
  let inExamples = false;

  const flush = (): void => {
    if (block.kind === "background") background.push(...block.steps);
    if (block.kind === "scenario") scenarios.push(...materializeScenario(block, null, errors));
    if (block.kind === "outline") {
      if (block.examples.length === 0) errors.push(`Scenario Outline "${block.name}" (line ${block.line}) has no Examples rows`);
      block.examples.forEach((row, index) => scenarios.push(...materializeScenario(block, { row, index }, errors)));
    }
    block = { kind: "none", name: "", tags: [], steps: [], examples: [], line: 0 };
    exampleHeader = null;
    inExamples = false;
  };

  for (let i = 0; i < lines.length; i += 1) {
    const raw = lines[i]!;
    const line = raw.trim();
    const lineNo = i + 1;
    if (line === "" || line.startsWith("#")) continue;

    if (line.startsWith("@")) {
      pendingTags = [...pendingTags, ...splitTags(line)];
      continue;
    }

    const header = line.match(/^(Feature|Background|Scenario Outline|Scenario Template|Scenario|Example|Examples|Scenarios):\s*(.*)$/);
    if (header) {
      const keyword = header[1]!;
      const name = header[2]!.trim();
      if (keyword === "Examples" || keyword === "Scenarios") {
        if (block.kind !== "outline") errors.push(`Examples at line ${lineNo} does not follow a Scenario Outline`);
        inExamples = true;
        exampleHeader = null;
        pendingTags = [];
        continue;
      }
      if (keyword === "Feature") {
        flush();
        featureName = name;
        featureTags = pendingTags;
        pendingTags = [];
        block = { kind: "feature", name, tags: [], steps: [], examples: [], line: lineNo };
        continue;
      }
      flush();
      if (keyword === "Background") {
        block = { kind: "background", name, tags: [], steps: [], examples: [], line: lineNo };
      } else if (keyword === "Scenario Outline" || keyword === "Scenario Template") {
        block = { kind: "outline", name, tags: pendingTags, steps: [], examples: [], line: lineNo };
      } else {
        block = { kind: "scenario", name, tags: pendingTags, steps: [], examples: [], line: lineNo };
      }
      pendingTags = [];
      lastKeyword = "Given";
      continue;
    }

    if (line.startsWith("|")) {
      const cells = splitTableRow(line);
      if (inExamples) {
        if (exampleHeader === null) {
          exampleHeader = cells;
        } else {
          const row: Record<string, string> = {};
          exampleHeader.forEach((key, index) => {
            row[key] = cells[index] ?? "";
          });
          block.examples.push(row);
        }
        continue;
      }
      const target = block.steps[block.steps.length - 1];
      if (!target) {
        errors.push(`Data table at line ${lineNo} does not follow a step`);
        continue;
      }
      block.steps[block.steps.length - 1] = { ...target, dataTable: [...(target.dataTable ?? []), cells] };
      continue;
    }

    if (line === '"""' || line === "```") {
      const closer = line;
      const body: string[] = [];
      let j = i + 1;
      while (j < lines.length && lines[j]!.trim() !== closer) {
        body.push(lines[j]!);
        j += 1;
      }
      if (j >= lines.length) errors.push(`Unterminated doc string opened at line ${lineNo}`);
      const target = block.steps[block.steps.length - 1];
      if (!target) errors.push(`Doc string at line ${lineNo} does not follow a step`);
      else block.steps[block.steps.length - 1] = { ...target, docString: dedent(body) };
      i = j;
      continue;
    }

    const stepMatch = line.match(/^(Given|When|Then|And|But|\*)\s+(.*)$/);
    if (stepMatch && STEP_KEYWORDS.has(stepMatch[1]!)) {
      const rawKeyword = stepMatch[1]!;
      const keyword: "Given" | "When" | "Then" = rawKeyword === "Given" || rawKeyword === "When" || rawKeyword === "Then" ? rawKeyword : lastKeyword;
      lastKeyword = keyword;
      if (block.kind === "none" || block.kind === "feature") {
        errors.push(`Step at line ${lineNo} is outside a Background or Scenario`);
        continue;
      }
      block.steps.push({ keyword, rawKeyword, text: stepMatch[2]!.trim() });
      continue;
    }

    if (block.kind === "feature") {
      descriptionLines.push(raw.trim());
      continue;
    }
    errors.push(`Unrecognized line ${lineNo}: ${JSON.stringify(line)}`);
  }
  flush();

  if (featureName === "") errors.push("Feature has no `Feature:` header");
  const capability = tagValue(featureTags, "@capability-");
  const oracle = tagValue(featureTags, "@oracle-");
  const noOracleDecision = tagValue(featureTags, "@no-oracle-");
  const comparisonRaw = tagValue(featureTags, "@comparison-");
  const oracleInput = tagValue(featureTags, "@oracle-input-");
  const mutationCatalog = tagValue(featureTags, "@mutations-");
  // 🧭️The parser records the declared profile; whether that profile EXISTS is registry knowledge and
  // is checked in the contract phase, so the Gherkin profile stays independent of which formats the
  // repository happens to own today.
  const comparison: ComparisonProfile | null = comparisonRaw;

  const seen = new Set<string>();
  for (const scenario of scenarios) {
    if (seen.has(scenario.id)) errors.push(`Duplicate scenario id @id-${scenario.id}`);
    seen.add(scenario.id);
  }

  if (oracleInput !== null && oracleInput !== "subject-raw") errors.push(`Unknown oracle input @oracle-input-${oracleInput}`);
  return { name: featureName, description: descriptionLines.join("\n").trim(), tags: featureTags, capability, oracle, noOracleDecision, comparison, oracleInput: oracleInput === "subject-raw" ? oracleInput : null, mutationCatalog, background, scenarios, errors };
}

function dedent(body: readonly string[]): string {
  const indents = body.filter((line) => line.trim() !== "").map((line) => line.length - line.trimStart().length);
  const shift = indents.length === 0 ? 0 : Math.min(...indents);
  return body.map((line) => line.slice(shift)).join("\n");
}

function materializeScenario(block: FeatureBlock, example: { row: Record<string, string>; index: number } | null, errors: string[]): FeatureScenario[] {
  const tags = block.tags;
  const baseId = tagValue(tags, "@id-");
  const levels = tagValues(tags, "@level-").filter((value) => (TEST_LEVELS as readonly string[]).includes(value)) as TestLevel[];
  const modes = tagValues(tags, "@mode-").filter((value) => (TEST_MODES as readonly string[]).includes(value)) as TestMode[];
  const where = `"${block.name}" (line ${block.line})`;
  if (baseId === null) errors.push(`Scenario ${where} is missing its @id-<stable-id> tag`);
  if (levels.length !== 1) errors.push(`Scenario ${where} must carry exactly one @level-<fundamental|quick|long|exhaustive> tag (found ${levels.length})`);
  if (modes.length !== 1) errors.push(`Scenario ${where} must carry exactly one @mode-<differential|conformance|round-trip|property|error> tag (found ${modes.length})`);
  if (baseId === null || levels.length !== 1 || modes.length !== 1) return [];
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(baseId)) {
    errors.push(`Scenario id @id-${baseId} is not kebab-case`);
    return [];
  }
  const row = example?.row ?? {};
  if (example !== null && !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.id ?? "")) {
    errors.push(`Scenario Outline ${where} Examples row ${example.index + 1} needs a kebab-case \`id\` cell: its scenario id is @id-${baseId}-<id>, and a row index would rename every later row when one is inserted`);
    return [];
  }
  const id = example === null ? baseId : `${baseId}-${row.id}`;
  const implementations = tagValues(tags, "@implementation-").filter((value) => (IMPLEMENTATIONS as readonly string[]).includes(value)) as Implementation[];
  const steps = block.steps.map((step) => ({ ...step, text: substitute(step.text, row), docString: step.docString === undefined ? undefined : substitute(step.docString, row), dataTable: step.dataTable?.map((cells) => cells.map((cell) => substitute(cell, row))) }));
  return [
    {
      id,
      name:
        example === null
          ? block.name
          : `${block.name} [${Object.entries(row)
              .map(([key, value]) => `${key}=${value}`)
              .join(", ")}]`,
      level: levels[0]!,
      mode: modes[0]!,
      tags,
      steps,
      seed: tagValue(tags, "@seed-") ?? undefined,
      platforms: tagValues(tags, "@platform-"),
      requires: tagValues(tags, "@requires-"),
      implementations: implementations.length > 0 ? implementations : undefined,
      outlineOf: example === null ? undefined : baseId,
      line: block.line,
    },
  ];
}
//#endregion 🥒️Gherkin
