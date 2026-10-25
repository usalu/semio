/** 🥒️ Which committed mutation fixture bundles an exhaustive mutation case declares. Pure text analysis of the restricted Gherkin profile: a step URI `<prefix><directory>/<fixture>/<facet>/…` names one bundle, and a `Scenario Outline` expands `<column>` placeholders from its `Examples` rows exactly as `parseFeature` does. Bundles are returned as `<directory>/<fixture>` with U+FE0F removed. @see https://cucumber.io/docs/gherkin/reference/ */
export interface MutationFixtureDeclarationPolicy {
  readonly stepUriPrefix: string;
  readonly bundleFacetDirectoryNames: readonly string[];
}

const PLACEHOLDER = /<([^<>]+)>/gu;

function tableCells(line: string): string[] {
  const trimmed = line.trim();
  return trimmed.slice(1, trimmed.length - 1).split("|").map((cell) => cell.trim().replace(/\\\|/gu, "|"));
}

function bundleOf(step: string, policy: MutationFixtureDeclarationPolicy): string | null {
  const at = step.indexOf(policy.stepUriPrefix);
  if (at < 0) return null;
  const segments = step.slice(at + policy.stepUriPrefix.length).split(/\s/u)[0]!.split("/");
  const facet = segments.findIndex((segment) => policy.bundleFacetDirectoryNames.includes(segment));
  return facet < 1 ? null : segments.slice(0, facet).join("/").replaceAll("️", "").normalize("NFC");
}

/** 🧫️ Every `<directory>/<fixture>` bundle the feature source declares through its step URIs. */
export function mutationFixturesDeclaredByFeature(source: string, policy: MutationFixtureDeclarationPolicy): ReadonlySet<string> {
  const declared = new Set<string>();
  let templates: string[] = [];
  let header: string[] | null = null;
  let examples = false;
  let outline = false;
  const flushPlain = (step: string): void => {
    const bundle = bundleOf(step, policy);
    if (bundle !== null && !/<[^<>]+>/u.test(bundle)) declared.add(bundle);
  };
  for (const raw of source.split(/\r?\n/u)) {
    const line = raw.trim();
    if (line === "" || line.startsWith("#") || line.startsWith("@")) continue;
    if (/^(Scenario Outline|Scenario Template):/u.test(line)) {
      outline = true;
      examples = false;
      header = null;
      templates = [];
      continue;
    }
    if (/^(Feature|Background|Scenario|Example):/u.test(line)) {
      outline = false;
      examples = false;
      header = null;
      templates = [];
      continue;
    }
    if (/^(Examples|Scenarios):/u.test(line)) {
      examples = true;
      header = null;
      continue;
    }
    if (line.startsWith("|")) {
      if (!outline || !examples) continue;
      const cells = tableCells(line);
      if (header === null) {
        header = cells;
        continue;
      }
      const row: Record<string, string> = Object.fromEntries(header.map((name, index) => [name, cells[index] ?? ""]));
      for (const template of templates) {
        const step = template.replace(PLACEHOLDER, (whole, key: string) => (key in row ? row[key]! : whole));
        const bundle = bundleOf(step, policy);
        if (bundle !== null) declared.add(bundle);
      }
      continue;
    }
    if (outline && !examples) {
      if (line.includes(policy.stepUriPrefix)) templates.push(line);
      continue;
    }
    if (!outline) flushPlain(line);
  }
  return declared;
}
