/** 🗞️ The TS twin of the native renderer's shell-message demux law
 * (`📺️renderer/🧑‍🎨engine/🧪️tests/🗞️typed-result-page/🦀️.rs`), on the SAME neutral oracle
 * `🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json`.
 *
 * ⚖️ `route_exchange_output` multiplexes a mounted typed operation's result pages onto the very
 * endpoint that carries `AppFrame` replies, so a host that claims every `Shell{instance}` payload as
 * a frame hands the page magic to an `AppFrame` decoder and reads its `'s'` as a frame tag
 * (`decodeAppFrame: unknown tag 115`). These laws pin the discrimination, the acknowledgement every
 * page owes, and the lane ceiling the guest's own mesh publication lane sits under. */
type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const {
    scanTypedOperationPages,
    shellFrameBytes,
    shellMessageKind,
    typedOperationAcknowledgements,
    typedOperationResult,
    TYPED_OPERATION_ACK_MAGIC,
    TYPED_OPERATION_LANE_FAULT,
    TYPED_OPERATION_LANE_TERMINAL,
    TYPED_OPERATION_PAGE_HEADER_BYTES,
    TYPED_OPERATION_PAGE_MAGIC,
    TYPED_OPERATION_RESULT_LANE_MAX,
    TYPED_OPERATION_RESULT_PAGE_BYTES,
    TYPED_OPERATION_TOKEN_BYTES,
  } = dependencies;
  const { describe, expect, it } = vitest;
  const { readFileSync } = await import("node:fs");
  const { fileURLToPath } = await import("node:url");
  const { dirname, join } = await import("node:path");

  type Token = { readonly receiver: number; readonly operation: number; readonly generation: number; readonly sequence: number; readonly attempt: number };
  type Message = { readonly name: string; readonly kind: "page" | "appFrame"; readonly lane?: number; readonly payload?: string; readonly appFrameTag?: number; readonly instanceId?: number };
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json"), "utf8")) as {
    readonly page: { readonly pageMagic: string; readonly ackMagic: string; readonly headerBytes: number; readonly tokenBytes: number; readonly laneOffset: number; readonly lengthOffset: number; readonly maxPayloadBytes: number; readonly pageMagicFirstByte: number };
    readonly lanes: readonly { readonly name: string; readonly tag: number }[];
    readonly invalidTags: readonly number[];
    readonly terminalTag: number;
    readonly faultTag: number;
    readonly shellMessageStream: {
      readonly instanceId: number;
      readonly token: Token;
      readonly messages: readonly Message[];
      readonly expect: { readonly appFrames: number; readonly pages: number; readonly acknowledgements: number; readonly terminal: boolean; readonly faults: number };
    };
  };

  const encodePage = (token: Token, lane: number, payload: string): Uint8Array => {
    const body = new TextEncoder().encode(payload);
    const bytes = new Uint8Array(TYPED_OPERATION_PAGE_MAGIC.length + fixture.page.headerBytes + body.length);
    bytes.set(TYPED_OPERATION_PAGE_MAGIC);
    const header = new DataView(bytes.buffer, TYPED_OPERATION_PAGE_MAGIC.length, fixture.page.headerBytes);
    header.setUint32(0, token.receiver, true);
    header.setBigUint64(4, BigInt(token.operation), true);
    header.setBigUint64(12, BigInt(token.generation), true);
    header.setUint32(20, token.sequence, true);
    header.setUint8(24, token.attempt);
    header.setUint8(fixture.page.laneOffset, lane);
    header.setUint32(fixture.page.lengthOffset, body.length, true);
    bytes.set(body, TYPED_OPERATION_PAGE_MAGIC.length + fixture.page.headerBytes);
    return bytes;
  };
  const shellMessage = (instanceId: number, payload: Uint8Array) => ({ tag: "send-message", val: { target: { tag: "shell", val: instanceId }, payload: [...payload] } });

  describe("🗞️ one shell endpoint, two wire languages", () => {
    it("declares the production page layout the guest writes", () => {
      expect(new TextDecoder().decode(TYPED_OPERATION_PAGE_MAGIC)).toBe(fixture.page.pageMagic);
      expect(new TextDecoder().decode(TYPED_OPERATION_ACK_MAGIC)).toBe(fixture.page.ackMagic);
      expect(TYPED_OPERATION_PAGE_MAGIC[0]).toBe(fixture.page.pageMagicFirstByte);
      expect(TYPED_OPERATION_PAGE_HEADER_BYTES).toBe(fixture.page.headerBytes);
      expect(TYPED_OPERATION_TOKEN_BYTES).toBe(fixture.page.tokenBytes);
      expect(TYPED_OPERATION_RESULT_PAGE_BYTES).toBe(fixture.page.maxPayloadBytes);
      expect(TYPED_OPERATION_LANE_TERMINAL).toBe(fixture.terminalTag);
      expect(TYPED_OPERATION_LANE_FAULT).toBe(fixture.faultTag);
      expect(TYPED_OPERATION_RESULT_LANE_MAX).toBe(Math.max(...fixture.lanes.map((lane) => lane.tag)));
      expect(fixture.invalidTags.every((tag) => tag > TYPED_OPERATION_RESULT_LANE_MAX)).toBe(true);
    });

    it("reads every declared lane and refuses every declared invalid one", () => {
      const token = fixture.shellMessageStream.token;
      for (const lane of fixture.lanes) {
        const effect = shellMessage(token.receiver, encodePage(token, lane.tag, "{}"));
        const page = typedOperationResult(effect);
        expect(page, `${lane.name} must decode`).not.toBeNull();
        expect(page.lane).toBe(lane.tag);
        expect(page.token.operation).toBe(BigInt(token.operation));
        expect(new TextDecoder().decode(page.payload)).toBe("{}");
      }
      for (const tag of fixture.invalidTags) {
        expect(() => typedOperationResult(shellMessage(token.receiver, encodePage(token, tag, "{}")))).toThrow();
      }
    });

    // 🧯️ The defect itself: the page must never reach an `AppFrame` decoder.
    it("never hands a typed-operation page or ack to the app-frame decoder", () => {
      const token = fixture.shellMessageStream.token;
      const page = shellMessage(token.receiver, encodePage(token, fixture.terminalTag, "{}"));
      expect(shellFrameBytes(page, token.receiver)).toBeNull();
      expect(shellMessageKind(page, token.receiver)).toBe("typed-operation-page");
      const ack = typedOperationResult(page).acknowledgement.payload.payload as readonly number[];
      expect(shellFrameBytes(shellMessage(token.receiver, new Uint8Array(ack)), token.receiver)).toBeNull();
      expect(shellMessageKind(shellMessage(token.receiver, new Uint8Array(ack)), token.receiver)).toBe("typed-operation-ack");
      expect(ack.length).toBe(TYPED_OPERATION_ACK_MAGIC.length + fixture.page.tokenBytes);
    });

    it("splits the declared stream into exactly its app frames, pages and acknowledgements", () => {
      const stream = fixture.shellMessageStream;
      const effects = stream.messages.map((message) =>
        message.kind === "page"
          ? shellMessage(message.instanceId ?? stream.instanceId, encodePage(stream.token, message.lane!, message.payload ?? ""))
          : shellMessage(message.instanceId ?? stream.instanceId, new Uint8Array([message.appFrameTag!, 0])),
      );
      const scan = scanTypedOperationPages(effects);
      const frames = scan.kept.map((effect: unknown) => shellFrameBytes(effect, stream.instanceId)).filter((frame: Uint8Array | null) => frame !== null);
      expect(frames.length).toBe(stream.expect.appFrames);
      expect(scan.pages.length).toBe(stream.expect.pages);
      expect(scan.acknowledgements.length).toBe(stream.expect.acknowledgements);
      expect(scan.terminal).toBe(stream.expect.terminal);
      expect(scan.faults.length).toBe(stream.expect.faults);
      expect(typedOperationAcknowledgements({ effects }).length).toBe(stream.expect.acknowledgements);
      expect(stream.messages.some((message) => message.lane === 13)).toBe(true);
      for (const ack of scan.acknowledgements) {
        expect(ack.kind).toBe("message");
        expect(ack.payload.source).toEqual({ tag: "shell", val: String(stream.token.receiver) });
      }
    });

    it("names a fault page without choosing a policy for it", () => {
      const token = fixture.shellMessageStream.token;
      const scan = scanTypedOperationPages([shellMessage(token.receiver, encodePage(token, fixture.faultTag, "extension.missing"))]);
      expect(scan.faults).toEqual(["extension.missing"]);
      expect(scan.kept.length).toBe(0);
      expect(scan.acknowledgements.length).toBe(1);
    });
  });
}
