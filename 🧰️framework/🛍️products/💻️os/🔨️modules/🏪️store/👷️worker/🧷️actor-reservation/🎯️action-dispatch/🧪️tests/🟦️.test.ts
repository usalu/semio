import { expect, test } from "bun:test";
import Ajv from "ajv";
import { decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { BrowserActorActionMailboxV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { ReservationActionDispatch } from "../🟦️.ts";
import { ReservationPollLanes } from "../../🛤️poll-lanes/🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };
import schema from "../../../../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json" with { type: "json" };

const packet = (actionSequence: number, operationId = fixture.operationId) => ({
  ...fixture.owner, actionSequence, kind: "browser-actor-action", payload: { kind: "segmented-download", operationId },
});

function setup(retireAfter = false) {
  let current = true;
  const calls: [string[], unknown[]][] = [];
  const child = {
    async invoke(path: string[], args: unknown[]) {
      calls.push([path, args]);
      const expected = fixture.chunks[calls.length - 1]!;
      if (retireAfter) current = false;
      return expected === null ? undefined : { tag: "some", val: Uint8Array.from(expected) };
    },
  };
  const session = {
    pendingUiPatch: null, viewRefresh: null, child, closed: false, pollTail: Promise.resolve(), turnTail: Promise.resolve(),
    lease: { fields: () => ({ scope: fixture.owner.scope, surface: { surfaceId: fixture.owner.verifiedSurfaceId, windowKindId: "terrain" } }) },
    renderedSurfaceRevisions: new Map([["terrain", 3]]), generation: BigInt(fixture.owner.activationGeneration),
    lastActionSequence: 0,
  };
  const hub = {
    session, backbone: { assertDocumentOwnerCurrent() { if (!current) throw new Error("owner retired"); } },
  } as unknown as ConstructorParameters<typeof ReservationActionDispatch>[0];
  Object.assign(hub, { lanes: new ReservationPollLanes(hub) });
  const dispatch = new ReservationActionDispatch(hub);
  return { dispatch, calls, retire: () => { current = false; } };
}

test("the scoped lane drains the existing guest lease once per packet including EOF", async () => {
  const { dispatch, calls } = setup();
  const ajv = new Ajv({ strict: true });
  ajv.addSchema(schema);
  const requestSchema = ajv.getSchema(schema.$id + "#/definitions/request")!;
  const resultSchema = ajv.getSchema(schema.$id + "#/definitions/result")!;
  let mailbox: BrowserActorActionMailboxV1;
  mailbox = new BrowserActorActionMailboxV1(request => {
    expect(requestSchema(request)).toBe(true);
    const received = decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ ...request, clientInstanceId: fixture.clientInstanceId }));
    if (received.kind !== "browser-actor-action") throw new Error("wrong action envelope");
    const { clientInstanceId, ...action } = received;
    void dispatch.dispatchAction(action).then(result => {
      expect(resultSchema(result)).toBe(true);
      const reply = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ ...result, clientInstanceId }));
      if (reply.kind !== "browser-actor-action-result") throw new Error("wrong result envelope");
      const { clientInstanceId: recipient, ...receipt } = reply;
      expect(recipient).toBe(fixture.clientInstanceId);
      expect(mailbox.settle(receipt)).toBe(true);
    });
  });
  const owner = parseBrowserActorActionRequestV1(packet(1));
  for (const bytes of fixture.chunks) {
    const actual = await mailbox.takeSegmentedDownloadChunk(owner, BigInt(fixture.operationId));
    expect(actual).toEqual(bytes === null ? undefined : Uint8Array.from(bytes));
  }
  expect(calls).toEqual(fixture.chunks.map(() => [["jobs", "takeSegmentedDownloadChunk"], [0, BigInt(fixture.operationId)]]));
  const replay = await dispatch.dispatchAction(parseBrowserActorActionRequestV1(packet(fixture.replaySequence)));
  expect(replay.outcome).toBe("rejected");
  expect(calls).toHaveLength(fixture.chunks.length);
});

test("foreign, unpainted or retired owners cannot gain a download chunk", async () => {
  for (const stale of fixture.staleCases) {
    const { dispatch, calls, retire } = setup(stale === "retire-after");
    const request = packet(1);
    if (stale === "documentId") request.scope = { ...request.scope, documentId: "other-grid" };
    if (stale === "activationGeneration") request.activationGeneration = "13";
    if (stale === "instanceId") request.instanceId = 1;
    if (stale === "surfaceRevision") request.surfaceRevision = 4;
    if (stale === "retire-before") retire();
    const result = await dispatch.dispatchAction(parseBrowserActorActionRequestV1(request));
    expect(result.outcome).toBe("rejected");
    expect(result.downloadChunk).toBeUndefined();
    expect(calls).toHaveLength(stale === "retire-after" ? 1 : 0);
  }
  for (const operation of fixture.invalidOperationIds) expect(() => parseBrowserActorActionRequestV1(packet(1, operation))).toThrow();
});

test("a different export or a mutation receipt cannot settle a download read", async () => {
  const { dispatch } = setup();
  const request = parseBrowserActorActionRequestV1(packet(1));
  const response = await dispatch.dispatchAction(request);
  const { downloadChunk, ...actionResult } = response;
  expect(downloadChunk).toBeDefined();
  expect(() => parseBrowserActorActionResultV1({ ...response, mutationCount: 1 })).toThrow();
  expect(() => parseBrowserActorActionResultV1({ ...response, downloadChunk: { operationId: fixture.operationId, bytes: new Array(4097).fill(0) } })).toThrow();
  const mailbox = new BrowserActorActionMailboxV1(() => {});
  const pending = mailbox.takeSegmentedDownloadChunk(request, BigInt(fixture.operationId));
  expect(mailbox.settle(actionResult)).toBe(false);
  expect(mailbox.settle({ ...response, downloadChunk: { operationId: "1", bytes: null } })).toBe(false);
  expect(mailbox.settle(response)).toBe(true);
  expect(await pending).toEqual(Uint8Array.from(fixture.chunks[0]!));
});
