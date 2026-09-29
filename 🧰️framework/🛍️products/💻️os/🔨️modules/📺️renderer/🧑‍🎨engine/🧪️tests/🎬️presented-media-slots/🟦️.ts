/** 🎬️ Canonical media owner and browser slot descriptors share a third-party schema oracle. */
import { describe, expect, test } from "vitest";
import { createHash } from "node:crypto";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../🧫️fixtures/🎬️presented-media-slots/🔣️.json";
import fixtureSchema from "../../🧫️fixtures/🎬️presented-media-slots/🧬️schema/🔣️.json";
import contract from "../../🎯️targets/🧊️wgpu/🎬️media-slots/🧬️contract/🔣️.json";
import mediaContract from "../../../../🔌️plugin/🪟️window-kits/🎬️media/🧬️contract/🔣️.json";
import reservation from "../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🎬️media-transport-reservation/🔣️.json";

describe("🎬️ presented media slots", () => {
  test("trusted identity transitions reject stale and retired replies", () => {
    const ajv = new Ajv2020({ strict: true });
    const validateFixture = ajv.compile(fixtureSchema);
    expect(validateFixture(fixture), JSON.stringify(validateFixture.errors)).toBe(true);
    let request = 0;
    let active = false;
    let document: string | null = null;
    for (const row of fixture.identityTransitions) {
      if (row.operation === "begin") { request++; active = true; document = null; expect(request).toBe(row.request); }
      else if (row.operation === "retire") { active = false; document = null; }
      else {
        const accepted = active && row.request === request && row.instance === 17;
        expect(ajv.compile({ const: row.accepted })(accepted)).toBe(true);
        if (accepted) document = row.replyDocument ?? null;
      }
      expect(ajv.compile({ const: row.document })(document)).toBe(true);
    }
  });
  test("neutral occlusion includes foreground chrome and later siblings", () => {
    const ajv = new Ajv2020({ strict: true });
    for (const vector of fixture.occlusionCases) {
      const frame = vector.frames.filter((frame) => vector.body.x >= frame.x && vector.body.y >= frame.y && vector.body.x + vector.body.width <= frame.x + frame.width && vector.body.y + vector.body.height <= frame.y + frame.height).sort((left, right) => left.width * left.height - right.width * right.height)[0] ?? vector.body;
      const media = vector.media;
      const result = media.x < frame.x + frame.width && media.x + media.width > frame.x && media.y < frame.y + frame.height && media.y + media.height > frame.y;
      expect(ajv.compile({ const: vector.occluded })(result), vector.name).toBe(true);
    }
  });
  test("canonical descriptors meet schema and language-neutral owner vectors", () => {
    const ajv = new Ajv2020({ allErrors: true, strict: true }).addSchema(mediaContract);
    const validate = ajv.compile(contract);
    const validateOwner = ajv.compile({ type: "object", required: ["controllerId", "appInstanceId", "parentDocumentId"], properties: { controllerId: { const: "video.viewer" }, appInstanceId: { const: 17 }, parentDocumentId: { const: "document-7" } } });
    for (const vector of fixture.cases) {
      const props = structuredClone(reservation.cases[0].props);
      Object.assign(props.resource!, "resourceOverride" in vector ? vector.resourceOverride : {});
      const ownerMatches = props.resource!.controllerId === "video.viewer" && props.resource!.appInstanceId === 17 && props.resource!.parentDocumentId === "document-7";
      expect(validateOwner(props.resource), vector.name).toBe(ownerMatches);
      const parent = "parentRect" in vector ? vector.parentRect : null;
      const scroll = "scrollOffset" in vector ? vector.scrollOffset : null;
      const x = vector.body.x + (parent?.x ?? 0) + vector.localRect.x - (scroll?.x ?? 0);
      const y = vector.body.y + (parent?.y ?? 0) + vector.localRect.y - (scroll?.y ?? 0);
      const viewport = parent ? { x: vector.body.x + parent.x, y: vector.body.y + parent.y, width: parent.width, height: parent.height } : vector.body;
      const clip = { x: Math.max(x, viewport.x), y: Math.max(y, viewport.y), width: Math.max(0, Math.min(x + vector.localRect.width, viewport.x + viewport.width) - Math.max(x, viewport.x)), height: Math.max(0, Math.min(y + vector.localRect.height, viewport.y + viewport.height) - Math.max(y, viewport.y)) };
      expect(Number(ownerMatches), vector.name).toBe(vector.expectedCount);
      if (vector.expectedCount === 1) {
        const slot = { token: createHash("sha256").update(JSON.stringify(fixture.tokenIdentity)).digest("hex"), windowId: "media.video.viewer", nodeId: "1", nodeKey: "media-slot", rect: { x, y, width: vector.localRect.width, height: vector.localRect.height }, clip, paintOrder: 0, occluded: clip.width <= 0 || clip.height <= 0, pluginId: "media", controllerId: "video.viewer", appInstanceId: 17, parentDocumentId: "document-7", props };
        expect(slot.token).toBe(fixture.token);
        expect(slot.occluded, vector.name).toBe(vector.expectedOccluded);
        expect(validate([slot]), JSON.stringify(validate.errors)).toBe(true);
        expect(slot.rect).toEqual("expectedRect" in vector ? vector.expectedRect : null);
        expect(slot.clip).toEqual("expectedClip" in vector ? vector.expectedClip : null);
        expect(validate(Array.from({ length: fixture.slotCapacity + 1 }, () => slot))).toBe(false);
        expect(validate([{ ...slot, candidateBounds: slot.rect }])).toBe(false);
      }
    }
  });
});
