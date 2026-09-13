import { type CoverageRow, type DiscoveredCase, type FixtureManifest, type Implementation, type MutationManifest, discoverTestCases } from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 🎛️ Narrows discovery to the cases a command was pointed at (`--case`, `--owner`, `--project`). */
export function selectCases(repoRoot: string, segments: readonly string[]): DiscoveredCase[] {
  const all = discoverTestCases(repoRoot);
  const value = (flag: string): string | null => {
    const index = segments.indexOf(flag);
    return index === -1 ? null : (segments[index + 1] ?? null);
  };
  const owner = value("--owner");
  const caseSlug = value("--case");
  const project = value("--project");
  // 🎛️`--owner` matches the exact owner path, a trailing segment of it, OR any ancestor segment, so
  // `--owner 🗄️stdio` selects every artifact owned beneath that plugin rather than nothing.
  const matchesOwner = (entry: DiscoveredCase): boolean => owner === null || entry.owner === owner || entry.owner.endsWith(`/${owner}`) || entry.owner.split("/").includes(owner) || entry.owner.includes(`${owner}/`);
  return all.filter((entry) => matchesOwner(entry) && (caseSlug === null || entry.case === caseSlug) && (project === null || entry.projectName === project));
}

/** 🎛️ Implementations a command should exercise: every claimed adapter unless `--implementation` narrows it. */
export function selectImplementations(discovered: DiscoveredCase, segments: readonly string[]): Implementation[] {
  const index = segments.indexOf("--implementation");
  const requested = index === -1 ? null : segments[index + 1];
  const claimed = Object.keys(discovered.adapters) as Implementation[];
  return requested ? claimed.filter((impl) => impl === requested) : claimed;
}

/**
 * 🎛️ The full v2 selector set. Every phase accepts every selector, so a CI shard is expressible at
 * the exact coordinate a report row is keyed by — never merely at artifact level.
 */
export type Selectors = Readonly<{
  artifact: string | null;
  standard: string | null;
  subset: string | null;
  mutation: string | null;
  outcome: string | null;
  case: string | null;
  fixtureClass: string | null;
  fixtureFamily: string | null;
  oracle: string | null;
  probe: string | null;
  implementation: string | null;
  platform: string | null;
  agent: string | null;
  run: string | null;
  status: string | null;
}>;

export function readSelectors(segments: readonly string[]): Selectors {
  const value = (flag: string): string | null => {
    const index = segments.indexOf(flag);
    return index === -1 ? null : (segments[index + 1] ?? null);
  };
  return {
    artifact: value("--artifact"),
    standard: value("--standard"),
    subset: value("--subset"),
    mutation: value("--mutation"),
    outcome: value("--outcome"),
    case: value("--case"),
    fixtureClass: value("--fixture-class"),
    fixtureFamily: value("--fixture-family"),
    oracle: value("--oracle"),
    probe: value("--probe"),
    implementation: value("--implementation"),
    platform: value("--platform"),
    agent: value("--agent"),
    run: value("--run"),
    status: value("--status"),
  };
}

export function matchesTarget(manifest: MutationManifest, selectors: Selectors): boolean {
  return (
    (selectors.artifact === null || manifest.artifact === selectors.artifact || manifest.artifact.endsWith(`.${selectors.artifact}`)) &&
    (selectors.standard === null || manifest.standard === selectors.standard) &&
    (selectors.subset === null || manifest.subset === selectors.subset || manifest.mutations.some((mutation) => mutation.subset === selectors.subset)) &&
    (selectors.mutation === null || manifest.mutations.some((mutation) => mutation.id === selectors.mutation))
  );
}

export function matchesFixture(fixture: FixtureManifest, selectors: Selectors): boolean {
  return (
    (selectors.artifact === null || fixture.target.artifact === selectors.artifact || fixture.target.artifact.endsWith(`.${selectors.artifact}`)) &&
    (selectors.standard === null || fixture.target.standard === selectors.standard) &&
    (selectors.subset === null || fixture.target.subset === selectors.subset) &&
    (selectors.mutation === null || fixture.mutation === selectors.mutation) &&
    (selectors.outcome === null || fixture.outcome === selectors.outcome) &&
    (selectors.fixtureClass === null || fixture.class === selectors.fixtureClass) &&
    (selectors.fixtureFamily === null || fixture.family === selectors.fixtureFamily)
  );
}

export function matchesRow(row: CoverageRow, selectors: Selectors): boolean {
  return (
    (selectors.artifact === null || row.artifact === selectors.artifact || row.artifact.endsWith(`.${selectors.artifact}`)) &&
    (selectors.standard === null || row.standard === selectors.standard) &&
    (selectors.subset === null || row.subset === selectors.subset) &&
    (selectors.mutation === null || row.mutation === selectors.mutation) &&
    (selectors.outcome === null || row.outcome === selectors.outcome) &&
    (selectors.oracle === null || row.oracle === selectors.oracle) &&
    (selectors.implementation === null || row.implementation === selectors.implementation) &&
    (selectors.platform === null || row.platform === selectors.platform) &&
    (selectors.fixtureClass === null || row.fixtureClass === selectors.fixtureClass) &&
    (selectors.status === null || row.status === selectors.status)
  );
}
