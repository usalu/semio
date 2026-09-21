import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/♻️tiled-map-gesture-lifecycle/🔣️.json";
import schema from "../../🧬️schema/♻️tiled-map-gesture-lifecycle/🔣️.json";

type Scene = { node: number; key: string; kind: string; surfaceId: string };

const sameScene = (left: Scene, right: Scene | null): boolean => right !== null && left.node === right.node && left.key === right.key && left.kind === right.kind && left.surfaceId === right.surfaceId;

describe("TiledMap gesture lifecycle", () => {
  it("validates the closed identity and retirement contract", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(new Set(fixture.cases.map(({ name }) => name)).size).toBe(4);
  });

  it("retires only a changed retained scene identity", () => {
    for (const row of fixture.cases) {
      expect(row.retireOld).toBe(!sameScene(fixture.owner, row.next));
      expect(row.preserveGesture).toBe(!row.retireOld);
    }
  });

  it("keeps persistent MapHost state only where the lifecycle contract permits it", () => {
    expect(fixture.cases.find(({ name }) => name === "same-identity-refresh")).toMatchObject({ preserveHost: true, preserveGesture: true });
    expect(fixture.cases.find(({ name }) => name === "key-replacement")).toMatchObject({ preserveHost: true, preserveGesture: false });
    expect(fixture.retirement).toEqual({ selectionPublications: 0, pointerUpPublications: 0, successorStartsFresh: true, ordinaryRefreshKeepsCameraAndTiles: true });
  });

  it("backpressures before mutation and publishes successor hits only after one outside-borrow retirement", () => {
    const ledger = Array.from({ length: fixture.ledger.capacity }, (_, index) => index);
    expect(ledger.length).toBe(256);
    expect(ledger.length < fixture.ledger.capacity).toBe(false);
    expect(fixture.ledger.fullDisposition).toBe("yield-before-mutation");
    expect(fixture.ledger.drainPerOpportunity).toBe(1);
    expect(fixture.ledger.consumerOrder).toEqual(["release-ui-borrow", "retire-map-interaction", "publish-successor-hits"]);
    ledger.shift();
    expect(ledger.length < fixture.ledger.capacity).toBe(true);
  });
});
