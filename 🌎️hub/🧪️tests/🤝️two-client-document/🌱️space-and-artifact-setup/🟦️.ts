/** 🌱️ Signing in both authors, creating the space, measuring the directory pages and creating the artifact.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | the two-client document e2e}: a phase of the scenario over the same bindings, moved verbatim. */
import { startLocalHub } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { setTimeout as sleep } from "node:timers/promises";
import { expect } from "vitest";

/** 🧬️ Signs in both authors, creates the space and artifact and measures the directory pages. */
export async function createSpaceAndArtifact({ signIn, ADA, BO, cmd, fixture, fetchTimed, origin, sealSpaceArtifactCreateV1, run }: {
  readonly signIn: (email: string, password: string, device: string) => Promise<string>;
  readonly ADA: { email: string; password: string; display: string; };
  readonly BO: { email: string; password: string; display: string; };
  readonly cmd: (token: string, body: unknown) => Promise<{ status: number; json: any; }>;
  readonly fixture: any;
  readonly fetchTimed: (url: string, init?: RequestInit, ms?: number) => Promise<Response>;
  readonly origin: string;
  readonly sealSpaceArtifactCreateV1: any;
  readonly run: Awaited<ReturnType<typeof startLocalHub>>;
}) {
  const scenario = {} as { directoryTimings: { spaceListMs: number; directoryPageMs: number[] }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string };
  scenario.directoryTimings = { spaceListMs: 0, directoryPageMs: [] };
  const tokenA = await signIn(ADA.email, ADA.password, "device-ada");
  const tokenB = await signIn(BO.email, BO.password, "device-bo");
  const created = await cmd(tokenA, { kind: "create-space", name: fixture.spaceName, spaceKind: "studio", visibility: "private" });
  expect(created.status).toBe(202);
  scenario.spaceId = created.json?.events?.find((e: any) => e.body?.kind === "space.created")?.body?.spaceId as string;
  if (!scenario.spaceId) {
    const listed = await (await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
    scenario.spaceId = listed.find((r: any) => r?.space?.name === fixture.spaceName)?.space?.id;
  }
  expect(scenario.spaceId).toBeTruthy();
  expect((await cmd(tokenA, { kind: "upsert-member", spaceId: scenario.spaceId, email: BO.email, role: "author" })).status).toBe(202);
  for (let index = 0; index < fixture.directoryPages.spaces; index++) {
    const extra = await cmd(tokenA, { kind: "create-space", name: `${fixture.spaceName} ${index}`, spaceKind: index % 2 ? "atelier" : "studio", visibility: index % 4 ? "private" : "public" });
    expect(extra.status).toBe(202);
  }
  const directoryPageMs: number[] = [];
  let spaceListMs = Number.POSITIVE_INFINITY;
  for (let sample = 0; sample < fixture.directoryPages.samples; sample++) {
    const startedAt = Date.now();
    const listed = await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA}` } });
    const rows = (await listed.json()) as unknown[];
    spaceListMs = Math.min(spaceListMs, Date.now() - startedAt);
    expect(listed.status).toBe(200);
    expect(rows.length).toBeGreaterThan(fixture.directoryPages.spaces);
  }
  for (let after = 0; ; ) {
    const startedAt = Date.now();
    const paged = await fetchTimed(`${origin}/directory/event-page/v1?after=${after}`, { headers: { authorization: `Bearer ${tokenA}` } });
    const page = (await paged.json()) as { throughSeqInclusive: number; hasMore: boolean };
    directoryPageMs.push(Date.now() - startedAt);
    expect(paged.status).toBe(200);
    if (!page.hasMore || page.throughSeqInclusive === after) break;
    after = page.throughSeqInclusive;
  }
  expect(spaceListMs, `the space list of ${fixture.directoryPages.spaces}+ spaces answered within the page budget`).toBeLessThanOrEqual(fixture.directoryPages.pageBudgetMs);
  expect(Math.max(...directoryPageMs), `every directory event page answered within the page budget: ${JSON.stringify(directoryPageMs)}`).toBeLessThanOrEqual(fixture.directoryPages.pageBudgetMs);
  scenario.directoryTimings = { spaceListMs, directoryPageMs };

  const route = `/spaces/${encodeURIComponent(scenario.spaceId)}/artifact-creations`;
  const catalog = await (await fetchTimed(`${origin}${route}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
  const kind = catalog.kinds.find((row: any) => row.kindId === fixture.artifactKindId);
  expect(kind, `catalog kinds=${catalog.kinds.map((row: any) => row.kindId).join(",")}`).toBeTruthy();
  const request = sealSpaceArtifactCreateV1({
    requestId: [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join(""),
    expectedCatalogGenerationId: catalog.catalogGenerationId,
    kindId: kind.kindId,
    name: "Two Client Note",
  });
  expect((await fetchTimed(`${origin}${route}`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${tokenA}` }, body: JSON.stringify(request) })).ok).toBe(true);

  scenario.documentId = "";
  scenario.surfaceId = fixture.surfaceId as string;
  scenario.artifactSchema = "";
  scenario.packSchemaHashHex = "";
  const deadline = Date.now() + 300_000;
  while (Date.now() < deadline) {
    const polled = await (await fetchTimed(`${origin}${route}/${request.requestId}`, { headers: { authorization: `Bearer ${tokenA}` } }, 60_000)).json();
    if (polled.phase === "ready") {
      scenario.documentId = polled.ready.artifactId;
      scenario.artifactSchema = polled.ready.artifactSchema ?? kind.schema;
      break;
    }
    if (["failed", "cancelled", "indeterminate"].includes(polled.phase)) {
      throw new Error(`creation ${polled.phase}: ${JSON.stringify(polled)}\nhub-output:\n${run.output().slice(-12000)}`);
    }
    await sleep(1000);
  }
  expect(scenario.documentId.startsWith("artifact-")).toBe(true);
  return { scenario, tokenA, tokenB };
}
