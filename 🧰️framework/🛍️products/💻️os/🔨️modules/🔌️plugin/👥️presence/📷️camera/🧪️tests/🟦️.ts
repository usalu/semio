import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { encodePresencePeer, decodePresencePeer } from "@semio-tech/framework-replication";
import { orbitPointToScreen } from "@semio-tech/framework-replication";
import { PerspectiveCamera, Vector3 } from "three";
import { publishLocalPresenceWindowViewV1, collectLocalPresenceWindowViewsV1, clearLocalPresenceWindowViewV1, publishArtifactPresenceRosterV1, artifactPresenceRosterV1, clearArtifactPresenceRosterV1, publishLocalPresenceActorV1, localPresenceActorV1, publishLocalActiveToolV1, collectLocalActiveToolV1 } from "../../../../📺️renderer/🧑‍🎨engine/🧱️elements/👕️canvas-presence/🟦️.ts";

/** 🧪️ Canonical viewport wire/schema and exact document/window authority against independent Ajv. */
export async function testCameraPresenceContract(repoRoot: string): Promise<number> {
  const root = resolve(import.meta.dir, "..");
  const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🔣️.json"), "utf8"));
  const ajv = semioSchemaAjvV1();
  const projections = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/👕️peer-overlay-v1/🔣️.json"), "utf8")).projectionCases;
  for (const row of projections) {
    const actual = orbitPointToScreen(row, row.size, row.world);
    assert.deepEqual(actual, row.expected);
    const camera = new PerspectiveCamera(row.fov, row.size[0] / row.size[1], 0.1, 1000);
    camera.position.fromArray(row.position);
    camera.up.fromArray(row.up);
    camera.lookAt(new Vector3().fromArray(row.target));
    camera.updateMatrixWorld();
    const determinant = camera.projectionMatrix.determinant();
    const projected = new Vector3().fromArray(row.world).project(camera);
    const oracle = !Number.isFinite(determinant) || determinant === 0 ? null : [(projected.x + 1) * row.size[0] / 2, (1 - projected.y) * row.size[1] / 2];
    assert.deepEqual(actual, oracle, `independent Three projection ${row.id}`);
  }
  const validate = ajv.compile(JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8")));
  for (const row of fixture.admission) {
    assert.equal(validate(row.value), row.valid, `canonical view schema ${row.id}`);
    if (row.valid) {
      const peer = { actor: "source", connectedAtMs: 1, views: [row.value] };
      assert.deepEqual(decodePresencePeer(Uint8Array.from(encodePresencePeer(peer)), [0]).views, peer.views.map(view => ({ ...view, pointer: view.pointer, rayOrigin: view.rayOrigin })));
    }
  }
  for (const owner of [
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema",
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema",
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema",
    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema",
    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema",
  ]) {
    const validateEmpty = ajv.compile(JSON.parse(readFileSync(resolve(repoRoot, owner, "🔣️.json"), "utf8")));
    const module = await import(resolve(repoRoot, owner, "🟦️.ts"));
    const parse = module.parseNoPresence ?? module.parsePuzzle2dPresence ?? module.parsePuzzle3dPresence ?? module.parsePuzzle5dPresence;
    for (const value of [{}, { selectedIds: [] }, { cameraX: 0 }, { activeUtilityId: "select" }, [], null]) {
      assert.equal(validateEmpty(value), value !== null && !Array.isArray(value) && Object.keys(value).length === 0);
      if (validateEmpty(value)) assert.deepEqual(parse(value), {});
      else assert.throws(() => parse(value));
    }
  }
  const { documentA, documentB, window, actorA, actorB } = fixture.isolation;
  const a = fixture.admission[0].value;
  const b = fixture.admission[1].value;
  publishLocalPresenceWindowViewV1(documentA, window, a);
  publishLocalPresenceWindowViewV1(documentB, window, b);
  assert.deepEqual(collectLocalPresenceWindowViewsV1(documentA), [a]);
  assert.deepEqual(collectLocalPresenceWindowViewsV1(documentB), [b]);
  clearLocalPresenceWindowViewV1(window, documentA);
  assert.deepEqual(collectLocalPresenceWindowViewsV1(documentA), []);
  assert.deepEqual(collectLocalPresenceWindowViewsV1(documentB), [b]);
  publishArtifactPresenceRosterV1(documentA, [{ actor: actorA, connectedAtMs: 1, views: [] }]);
  publishArtifactPresenceRosterV1(documentB, [{ actor: actorB, connectedAtMs: 1, views: [] }]);
  assert.equal(artifactPresenceRosterV1(documentA)[0]?.actor, actorA);
  assert.equal(artifactPresenceRosterV1(documentB)[0]?.actor, actorB);
  assert.deepEqual(artifactPresenceRosterV1("local"), []);
  publishLocalPresenceActorV1(documentA, actorA);
  publishLocalPresenceActorV1(documentB, actorB);
  assert.equal(localPresenceActorV1(documentA), actorA);
  assert.equal(localPresenceActorV1("local"), null);
  publishLocalActiveToolV1("local", "select");
  assert.equal(collectLocalActiveToolV1(documentA), undefined);
  clearLocalPresenceWindowViewV1(window, documentB);
  clearArtifactPresenceRosterV1(documentA);
  clearArtifactPresenceRosterV1(documentB);
  publishLocalPresenceActorV1(documentA, null);
  publishLocalPresenceActorV1(documentB, null);
  publishLocalActiveToolV1("local", null);
  return 83;
}
