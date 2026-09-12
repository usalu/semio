/** 🗞️ The wgpu bridge's own law for a MULTI-FRAME reply: a `Migrated` interactive job answers with
 * more than its `Invocation` frame — the mounted operation's result pages ride the very same
 * `Effect::SendMessage{Shell{instance}}` endpoint, and the guest parks until each is acknowledged.
 *
 * ⚖️ Regression pinned here: handing one of those pages to `decodeAppFrame` reports
 * `decodeAppFrame: unknown tag 115`, because `115` is the `'s'` of `semio.typed-operation-page.v1`.
 * That is exactly what `flowEvalTick` produced on `http://127.0.0.1:6118/?plugin=generation3d`
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Oracle: the SAME neutral fixture the native renderer's
 * `🗞️typed-result-page/🦀️.rs` law and the shared `🖼️wire-turn/🟦️.ts` twin read. */
import { describe, expect, it } from "vitest";
import lanes from "../../../../🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json";
import { decodeAppFrame, encodeAppFrame } from "../../../../../🟦️.ts";
import { TYPED_OPERATION_PAGE_MAGIC, scanTypedOperationPages, shellFrameBytes } from "../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { WGPU_TYPED_OPERATION_DRAIN_BUDGET, WGPU_TYPED_OPERATION_EFFECT_CAPACITY, WGPU_TYPED_OPERATION_SETTLE_LIMIT, WgpuTypedOperationDrive, invocationResponseJson } from "../../🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts";

const stream = lanes.shellMessageStream;
const token = stream.token;

function encodePage(lane: number, payload: string): Uint8Array {
  const body = new TextEncoder().encode(payload);
  const bytes = new Uint8Array(TYPED_OPERATION_PAGE_MAGIC.length + lanes.page.headerBytes + body.length);
  bytes.set(TYPED_OPERATION_PAGE_MAGIC);
  const header = new DataView(bytes.buffer, TYPED_OPERATION_PAGE_MAGIC.length, lanes.page.headerBytes);
  header.setUint32(0, token.receiver, true);
  header.setBigUint64(4, BigInt(token.operation), true);
  header.setBigUint64(12, BigInt(token.generation), true);
  header.setUint32(20, token.sequence, true);
  header.setUint8(24, token.attempt);
  header.setUint8(lanes.page.laneOffset, lane);
  header.setUint32(lanes.page.lengthOffset, body.length, true);
  bytes.set(body, TYPED_OPERATION_PAGE_MAGIC.length + lanes.page.headerBytes);
  return bytes;
}

const shellMessage = (payload: Uint8Array) => ({ tag: "send-message", val: { target: { tag: "shell", val: token.receiver }, payload: [...payload] } });
const turn = (effects: readonly unknown[], status: string) => ({ uiPatches: [], effects, nextWake: null, status: { tag: status } } as never);

describe("🗞️ wgpu multi-frame reply: app frames and typed-operation pages on one endpoint", () => {
  it("reports the exact framing failure a missing discrimination produces", () => {
    expect(TYPED_OPERATION_PAGE_MAGIC[0]).toBe(lanes.page.pageMagicFirstByte);
    expect(() => decodeAppFrame(encodePage(lanes.terminalTag, "{}"))).toThrowError(`decodeAppFrame: unknown tag ${lanes.page.pageMagicFirstByte}`);
  });

  it("splits one interactive job's reply into decodable frames and owed acknowledgements", () => {
    const done = encodeAppFrame({ Done: { in_reply_to: 1 } });
    const effects = [
      shellMessage(encodePage(13, '{"meshes":3}')),
      shellMessage(done),
      shellMessage(encodePage(12, '{"ratio":0.5}')),
      shellMessage(encodePage(lanes.terminalTag, "{}")),
    ];
    const drive = new WgpuTypedOperationDrive(token.receiver);
    const one = turn(effects, "more-work");
    drive.observe([one]);
    const hostEffects = drive.hostEffects([one]);
    const frames = hostEffects.map((effect) => shellFrameBytes(effect, token.receiver)).filter((frame): frame is Uint8Array => frame !== null);
    expect(frames.length).toBe(1);
    expect(decodeAppFrame(frames[0]!)).toEqual({ Done: { in_reply_to: 1 } });
    const owed = drive.takeAcknowledgements();
    expect(owed.length).toBe(3);
    expect(owed.every((ack) => ack.payload.source.val === String(token.receiver))).toBe(true);
    expect(drive.takeAcknowledgements().length).toBe(0);
    expect(drive.owesASettle(one)).toBe(true);
    expect(drive.owesASettle(turn([], "idle"))).toBe(false);
  });

  it("never observes the same turn twice, so one page is acknowledged exactly once", () => {
    const one = turn([shellMessage(encodePage(lanes.terminalTag, "{}"))], "idle");
    const drive = new WgpuTypedOperationDrive(token.receiver);
    drive.observe([one]);
    drive.observe([one]);
    expect(drive.takeAcknowledgements().length).toBe(1);
    expect(drive.hostEffects([one]).length).toBe(0);
  });

  it("agrees with the shared scanner about the fixture's own declared stream", () => {
    const effects = stream.messages.map((message) =>
      message.kind === "page" ? shellMessage(encodePage(message.lane!, message.payload ?? "")) : { tag: "send-message", val: { target: { tag: "shell", val: message.instanceId ?? stream.instanceId }, payload: [message.appFrameTag!, 0] } },
    );
    const scan = scanTypedOperationPages(effects);
    expect(scan.pages.length).toBe(stream.expect.pages);
    expect(scan.terminal).toBe(stream.expect.terminal);
    expect(scan.kept.map((effect) => shellFrameBytes(effect, stream.instanceId)).filter((frame) => frame !== null).length).toBe(stream.expect.appFrames);
  });

  // 🧯️ The hop after the framing one: the reply the guest finally produced carried an
  // `invokeExtension` whose `req` is a u64, and `JSON.stringify` refuses a bigint outright.
  it("projects a bigint-bearing reply onto the JSON the program bridge parses", () => {
    const response = { output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics: [], events: [], requestedEffects: [{ invokeExtension: { req: 7n, extensionId: "flow-extension-brep", capability: "evaluate", requestJson: "{}" } }] } as never;
    expect(() => JSON.stringify(response)).toThrow();
    expect(JSON.parse(invocationResponseJson(response)).requestedEffects[0].invokeExtension.req).toBe(7);
  });

  it("declares finite settle, drain and retained-effect authorities", () => {
    expect(WGPU_TYPED_OPERATION_SETTLE_LIMIT).toBeGreaterThan(0);
    expect(WGPU_TYPED_OPERATION_DRAIN_BUDGET).toBeGreaterThan(WGPU_TYPED_OPERATION_SETTLE_LIMIT);
    expect(WGPU_TYPED_OPERATION_EFFECT_CAPACITY).toBeGreaterThan(0);
  });
});
