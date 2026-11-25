import { expect, test } from "bun:test";
import fixture from "./🧫️fixtures/🪟️standalone-panel-owner.json" with { type: "json" };
import { spacePanelOwnerLeafIdV1 } from "../🟦️.ts";

type App = Parameters<typeof spacePanelOwnerLeafIdV1>[0];

/** 🪟️ A standalone program host (no host app) builds its space panel from the mounted session's app, so Open File can focus the opened program instead of throwing. */
test("the space panel owner is the host app, else the mounted session's app", () => {
  for (const row of fixture.cases) {
    const resolve = () => spacePanelOwnerLeafIdV1(row.host ? fixture.hostApp as unknown as App : undefined, row.session ? fixture.sessionApp as unknown as App : undefined);
    if ("refusal" in row) expect(resolve, row.name).toThrow(row.refusal);
    else expect(resolve(), row.name).toBe(row.leaf);
  }
});
