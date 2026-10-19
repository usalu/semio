/** 🔁️ The lag rebootstrap control and loopback port tests. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { createServer } from "node:http";
import { join } from "node:path";
import Ajv from "ajv";
import { type ServerFrame, decodeServerFrame, encodeServerFrame } from "@semio-tech/framework-replication";
import { expect, it } from "vitest";
import { findFreePort, getWorkspaceRoot, resolveHubBinaryPath, waitForHttpReady } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { cargoTargetDirectory } from "@semio-tech/process/execution";

/** 🔌 Registers the typed lag rebootstrap control, the rebindable loopback port and the HTTP listener debug binary tests. */
export function registerLagRebootstrapAndLoopbackPortTests(): void {
  it("validates and independently encodes the typed lag rebootstrap control", () => {
    const root = getWorkspaceRoot();
    const fixture = JSON.parse(readFileSync(join(root, "🌎️hub", "🛰️lag-rebootstrap", "🧪️tests/🧫️fixtures", "🛟️lag-rebootstrap", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: false, discriminator: true });
    ajv.addSchema(schema);
    const validate = ajv.compile({ $ref: `${schema.$id}#/$defs/DirectoryStreamMessage` });
    expect(validate(fixture.control), "validate rejected its own fixture").toBe(true);
    const leaked = structuredClone(fixture.control);
    leaked.control.storageKey = `semio.artifact-cas.manifest/v1/${"ab".repeat(32)}`;
    expect(validate(leaked)).toBe(false);
    expect(fixture.closeCode).toBe(1013);
    expect(fixture.closeReason).toBe("rebootstrap-required");
    expect(fixture.scopeMaximumBytes).toBe(256);
    expect(new TextEncoder().encode(fixture.control.control.scope.spaceId).length).toBeLessThanOrEqual(fixture.scopeMaximumBytes);
    expect(new TextEncoder().encode(fixture.control.control.scope.documentId).length).toBeLessThanOrEqual(fixture.scopeMaximumBytes);
    expect(fixture.inlineMaximumBytes).toBe(4_096);
    expect(fixture.chunkMaximumBytes).toBe(4_096);
    expect(fixture.totalMaximumBytes).toBe(64 * 1024 * 1024);
    expect(fixture.chunkMaximumCount).toBe(16_384);

    const value = fixture.control.control;
    const frame: ServerFrame = { RebootstrapRequired: { control: {
      space_id: value.scope.spaceId,
      document_id: value.scope.documentId,
      checkpoint_id: value.checkpointId,
      descriptor_hash: value.descriptorDigestV1,
      baseline_frontier: {
        document_id: value.baselineFrontier.documentId,
        head_edit_ordinal: value.baselineFrontier.headEditOrdinal,
        head_edit_id: value.baselineFrontier.headEditId,
        last_commit_seq: value.baselineFrontier.lastCommitSeq,
        chain_hash: value.baselineFrontier.chainHash,
      },
    } } };
    const encoded = encodeServerFrame(frame, "command");
    const manual: number[] = [0, 12];
    const varint = (input: number): void => {
      let remaining = input;
      while (remaining >= 128) {
        manual.push((remaining & 0x7f) | 0x80);
        remaining = Math.floor(remaining / 128);
      }
      manual.push(remaining);
    };
    const bytes = (input: readonly number[]): void => { varint(input.length); manual.push(...input); };
    const text = (input: string): void => bytes([...new TextEncoder().encode(input)]);
    text(value.scope.spaceId);
    text(value.scope.documentId);
    manual.push(...value.checkpointId, ...value.descriptorDigestV1);
    text(value.baselineFrontier.documentId);
    varint(value.baselineFrontier.headEditOrdinal);
    text(value.baselineFrontier.headEditId);
    varint(value.baselineFrontier.lastCommitSeq);
    manual.push(...value.baselineFrontier.chainHash);
    expect([...encoded]).toEqual(manual);
    expect(decodeServerFrame(Uint8Array.from(manual)).frame).toEqual(frame);
    expect(createHash("sha256").update(encoded).digest()).toEqual(createHash("sha256").update(Uint8Array.from(manual)).digest());
  });
  it("allocates a released loopback port that can be rebound", async () => {
    const port = await findFreePort();
    expect(Number.isSafeInteger(port) && port > 0 && port <= 65_535).toBe(true);
    const server = createServer();
    await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(port, "127.0.0.1", resolveListen));
    await new Promise<void>((resolveClose, rejectClose) => server.close((error) => error ? rejectClose(error) : resolveClose()));
  });
  it("polls a real HTTP listener and retains the platform debug-binary contract", async () => {
    const server = createServer((_request, response) => response.writeHead(204).end());
    await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(0, "127.0.0.1", resolveListen));
    const address = server.address();
    if (!address || typeof address === "string") throw new Error("quick harness: HTTP listener has no TCP address");
    try {
      await waitForHttpReady(`http://127.0.0.1:${address.port}/ready`, {}, 1_000);
      expect(resolveHubBinaryPath("/repo")).toBe(join(cargoTargetDirectory("/repo"), "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub"));
    } finally {
      await new Promise<void>((resolveClose, rejectClose) => server.close((error) => error ? rejectClose(error) : resolveClose()));
    }
  });
}
