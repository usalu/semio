// 🧵️ `ShellBridge` frame codec — TypeScript twin of `🦀️.rs` (the Rust SSOT). Same hand-
// rolled binary framing (`tag: u8` + fields in declaration order, little-endian, length-prefixed
// bytes/strings), `BRIDGE_VERSION = 1`. `🧫️fixtures/📨️frames.json` is the anti-drift mechanism proving
// both codecs agree byte-for-byte — see that file's sibling Rust test
// (`bridge::quick::every_fixture_round_trips_through_the_rust_codec`) and this module's own
// `checkFixtureParity` (run via a foreground `bun run` script, since this crate does not yet have a
// vitest package wired — that is `🌉️mcp/📦️packages/🟦️typescript/**`, P5's owned territory, not P1b's).
// The React shell (P10) will import this module directly once it dials the bridge.

//#region 🔖️BridgeVersion
export const BRIDGE_VERSION = 1;
//#endregion 🔖️BridgeVersion

//#region 🔖️Wire
/** 🖊️ A little-endian, length-prefixed binary writer — mirrors the Rust `wire` module's `write_*`
 *  free functions exactly, one growable byte array at a time. */
class Writer {
  private chunks: number[] = [];

  u8(value: number): void {
    this.chunks.push(value & 0xff);
  }

  u16(value: number): void {
    this.chunks.push(value & 0xff, (value >>> 8) & 0xff);
  }

  u32(value: number): void {
    this.chunks.push(value & 0xff, (value >>> 8) & 0xff, (value >>> 16) & 0xff, (value >>> 24) & 0xff);
  }

  u64(value: number | bigint): void {
    let big = BigInt(value);
    for (let i = 0; i < 8; i++) {
      this.chunks.push(Number(big & 0xffn));
      big >>= 8n;
    }
  }

  bool(value: boolean): void {
    this.u8(value ? 1 : 0);
  }

  bytes(value: Uint8Array | number[]): void {
    this.u32(value.length);
    for (const byte of value) this.chunks.push(byte & 0xff);
  }

  string(value: string): void {
    this.bytes(Array.from(new TextEncoder().encode(value)));
  }

  optionString(value: string | null | undefined): void {
    if (value === null || value === undefined) {
      this.bool(false);
    } else {
      this.bool(true);
      this.string(value);
    }
  }

  stringVec(value: readonly string[]): void {
    this.u32(value.length);
    for (const item of value) this.string(item);
  }

  bytesVec(value: readonly (Uint8Array | number[])[]): void {
    this.u32(value.length);
    for (const item of value) this.bytes(item);
  }

  finish(): Uint8Array {
    return new Uint8Array(this.chunks);
  }
}

/** 📖️ A forward-only cursor over a decode buffer — mirrors the Rust `wire::Reader` exactly, including
 *  bounds-checking every read and rejecting trailing bytes via `finish()`. */
class Reader {
  private pos = 0;
  constructor(private readonly data: Uint8Array) {}

  private need(count: number): void {
    if (this.pos + count > this.data.length) {
      throw new Error("bridge frame: unexpected end of buffer");
    }
  }

  u8(): number {
    this.need(1);
    return this.data[this.pos++];
  }

  u16(): number {
    this.need(2);
    const value = this.data[this.pos] | (this.data[this.pos + 1] << 8);
    this.pos += 2;
    return value >>> 0;
  }

  u32(): number {
    this.need(4);
    const value = this.data[this.pos] | (this.data[this.pos + 1] << 8) | (this.data[this.pos + 2] << 16) | (this.data[this.pos + 3] << 24);
    this.pos += 4;
    return value >>> 0;
  }

  u64(): bigint {
    this.need(8);
    let value = 0n;
    for (let i = 7; i >= 0; i--) value = (value << 8n) | BigInt(this.data[this.pos + i]);
    this.pos += 8;
    return value;
  }

  bool(): boolean {
    return this.u8() !== 0;
  }

  bytes(): Uint8Array {
    const length = this.u32();
    this.need(length);
    const value = this.data.slice(this.pos, this.pos + length);
    this.pos += length;
    return value;
  }

  string(): string {
    return new TextDecoder("utf-8", { fatal: true }).decode(this.bytes());
  }

  optionString(): string | null {
    return this.bool() ? this.string() : null;
  }

  stringVec(): string[] {
    const length = this.u32();
    const items: string[] = [];
    for (let i = 0; i < length; i++) items.push(this.string());
    return items;
  }

  bytesVec(): Uint8Array[] {
    const length = this.u32();
    const items: Uint8Array[] = [];
    for (let i = 0; i < length; i++) items.push(this.bytes());
    return items;
  }

  finish(): void {
    if (this.pos !== this.data.length) {
      throw new Error(`bridge frame: ${this.data.length - this.pos} trailing byte(s) after decode`);
    }
  }
}
//#endregion 🔖️Wire

//#region 🔖️SharedTypes
export type ShellKind = "react" | "wgpu-web" | "wgpu-native";
const SHELL_KIND_TO_TAG: Record<ShellKind, number> = { react: 0, "wgpu-web": 1, "wgpu-native": 2 };
const SHELL_KIND_FROM_TAG: ShellKind[] = ["react", "wgpu-web", "wgpu-native"];

export interface BridgeFlags {
  relayAppCommands: boolean;
  sharedBackbone: boolean;
  elicit: boolean;
}
export const NO_BRIDGE_FLAGS: BridgeFlags = { relayAppCommands: false, sharedBackbone: false, elicit: false };
function flagsToBits(flags: BridgeFlags): number {
  return (flags.relayAppCommands ? 1 : 0) | (flags.sharedBackbone ? 2 : 0) | (flags.elicit ? 4 : 0);
}
function flagsFromBits(bits: number): BridgeFlags {
  return { relayAppCommands: (bits & 0b001) !== 0, sharedBackbone: (bits & 0b010) !== 0, elicit: (bits & 0b100) !== 0 };
}

export type ApprovalDecision = "deny" | "once" | "session";
const APPROVAL_DECISION_TO_TAG: Record<ApprovalDecision, number> = { deny: 0, once: 1, session: 2 };
const APPROVAL_DECISION_FROM_TAG: ApprovalDecision[] = ["deny", "once", "session"];

export interface BridgeInstanceRef {
  pluginId: string;
  appId: string;
  instanceId: string;
  artifactRef: string;
  windowIds: string[];
}
function encodeInstanceRef(writer: Writer, value: BridgeInstanceRef): void {
  writer.string(value.pluginId);
  writer.string(value.appId);
  writer.string(value.instanceId);
  writer.string(value.artifactRef);
  writer.stringVec(value.windowIds);
}
function decodeInstanceRef(reader: Reader): BridgeInstanceRef {
  return { pluginId: reader.string(), appId: reader.string(), instanceId: reader.string(), artifactRef: reader.string(), windowIds: reader.stringVec() };
}
//#endregion 🔖️SharedTypes

//#region 🔖️ShellToGateway
export type ShellToGateway =
  | { variant: "hello"; bridgeVersion: number; shellKind: ShellKind; shellSessionId: string; principalActor: string; flags: BridgeFlags }
  | { variant: "shellState"; revision: bigint; state: Uint8Array }
  | { variant: "shellStatePatch"; revision: bigint; baseRevision: bigint; patch: Uint8Array }
  | { variant: "instances"; entries: BridgeInstanceRef[] }
  | { variant: "appFrames"; inReplyTo: bigint; instanceId: string; frames: Uint8Array[] }
  | { variant: "shellCommandResult"; inReplyTo: bigint; ok: boolean; fault: string | null }
  | { variant: "approval"; approvalId: string; decision: ApprovalDecision; note: string | null }
  | { variant: "ping" }
  | { variant: "bye" };

export function encodeShellToGateway(frame: ShellToGateway): Uint8Array {
  const writer = new Writer();
  switch (frame.variant) {
    case "hello":
      writer.u8(0);
      writer.u16(frame.bridgeVersion);
      writer.u8(SHELL_KIND_TO_TAG[frame.shellKind]);
      writer.string(frame.shellSessionId);
      writer.string(frame.principalActor);
      writer.u8(flagsToBits(frame.flags));
      break;
    case "shellState":
      writer.u8(1);
      writer.u64(frame.revision);
      writer.bytes(frame.state);
      break;
    case "shellStatePatch":
      writer.u8(2);
      writer.u64(frame.revision);
      writer.u64(frame.baseRevision);
      writer.bytes(frame.patch);
      break;
    case "instances":
      writer.u8(3);
      writer.u32(frame.entries.length);
      for (const entry of frame.entries) encodeInstanceRef(writer, entry);
      break;
    case "appFrames":
      writer.u8(4);
      writer.u64(frame.inReplyTo);
      writer.string(frame.instanceId);
      writer.bytesVec(frame.frames);
      break;
    case "shellCommandResult":
      writer.u8(5);
      writer.u64(frame.inReplyTo);
      writer.bool(frame.ok);
      writer.optionString(frame.fault);
      break;
    case "approval":
      writer.u8(6);
      writer.string(frame.approvalId);
      writer.u8(APPROVAL_DECISION_TO_TAG[frame.decision]);
      writer.optionString(frame.note);
      break;
    case "ping":
      writer.u8(7);
      break;
    case "bye":
      writer.u8(8);
      break;
  }
  return writer.finish();
}

export function decodeShellToGateway(bytes: Uint8Array): ShellToGateway {
  const reader = new Reader(bytes);
  const tag = reader.u8();
  let frame: ShellToGateway;
  switch (tag) {
    case 0:
      frame = { variant: "hello", bridgeVersion: reader.u16(), shellKind: SHELL_KIND_FROM_TAG[reader.u8()], shellSessionId: reader.string(), principalActor: reader.string(), flags: flagsFromBits(reader.u8()) };
      break;
    case 1:
      frame = { variant: "shellState", revision: reader.u64(), state: reader.bytes() };
      break;
    case 2:
      frame = { variant: "shellStatePatch", revision: reader.u64(), baseRevision: reader.u64(), patch: reader.bytes() };
      break;
    case 3: {
      const length = reader.u32();
      const entries: BridgeInstanceRef[] = [];
      for (let i = 0; i < length; i++) entries.push(decodeInstanceRef(reader));
      frame = { variant: "instances", entries };
      break;
    }
    case 4:
      frame = { variant: "appFrames", inReplyTo: reader.u64(), instanceId: reader.string(), frames: reader.bytesVec() };
      break;
    case 5:
      frame = { variant: "shellCommandResult", inReplyTo: reader.u64(), ok: reader.bool(), fault: reader.optionString() };
      break;
    case 6:
      frame = { variant: "approval", approvalId: reader.string(), decision: APPROVAL_DECISION_FROM_TAG[reader.u8()], note: reader.optionString() };
      break;
    case 7:
      frame = { variant: "ping" };
      break;
    case 8:
      frame = { variant: "bye" };
      break;
    default:
      throw new Error(`bridge frame: unknown ShellToGateway tag ${tag}`);
  }
  reader.finish();
  return frame;
}
//#endregion 🔖️ShellToGateway

//#region 🔖️GatewayToShell
export type GatewayToShell =
  | { variant: "welcome"; bridgeVersion: number; connection: string; principal: string }
  | { variant: "shellCommand"; seq: bigint; command: Uint8Array }
  | { variant: "appCommand"; seq: bigint; instanceId: string; command: Uint8Array }
  | { variant: "approvalRequested"; approvalId: string; summary: string }
  | { variant: "approvalResolved"; approvalId: string; decision: ApprovalDecision }
  | { variant: "agentPresence"; active: boolean; label: string; invocationId: string | null }
  | { variant: "pong" }
  | { variant: "bye"; reason: string };

export function encodeGatewayToShell(frame: GatewayToShell): Uint8Array {
  const writer = new Writer();
  switch (frame.variant) {
    case "welcome":
      writer.u8(0);
      writer.u16(frame.bridgeVersion);
      writer.string(frame.connection);
      writer.string(frame.principal);
      break;
    case "shellCommand":
      writer.u8(1);
      writer.u64(frame.seq);
      writer.bytes(frame.command);
      break;
    case "appCommand":
      writer.u8(2);
      writer.u64(frame.seq);
      writer.string(frame.instanceId);
      writer.bytes(frame.command);
      break;
    case "approvalRequested":
      writer.u8(3);
      writer.string(frame.approvalId);
      writer.string(frame.summary);
      break;
    case "approvalResolved":
      writer.u8(4);
      writer.string(frame.approvalId);
      writer.u8(APPROVAL_DECISION_TO_TAG[frame.decision]);
      break;
    case "agentPresence":
      writer.u8(5);
      writer.bool(frame.active);
      writer.string(frame.label);
      writer.optionString(frame.invocationId);
      break;
    case "pong":
      writer.u8(6);
      break;
    case "bye":
      writer.u8(7);
      writer.string(frame.reason);
      break;
  }
  return writer.finish();
}

export function decodeGatewayToShell(bytes: Uint8Array): GatewayToShell {
  const reader = new Reader(bytes);
  const tag = reader.u8();
  let frame: GatewayToShell;
  switch (tag) {
    case 0:
      frame = { variant: "welcome", bridgeVersion: reader.u16(), connection: reader.string(), principal: reader.string() };
      break;
    case 1:
      frame = { variant: "shellCommand", seq: reader.u64(), command: reader.bytes() };
      break;
    case 2:
      frame = { variant: "appCommand", seq: reader.u64(), instanceId: reader.string(), command: reader.bytes() };
      break;
    case 3:
      frame = { variant: "approvalRequested", approvalId: reader.string(), summary: reader.string() };
      break;
    case 4:
      frame = { variant: "approvalResolved", approvalId: reader.string(), decision: APPROVAL_DECISION_FROM_TAG[reader.u8()] };
      break;
    case 5:
      frame = { variant: "agentPresence", active: reader.bool(), label: reader.string(), invocationId: reader.optionString() };
      break;
    case 6:
      frame = { variant: "pong" };
      break;
    case 7:
      frame = { variant: "bye", reason: reader.string() };
      break;
    default:
      throw new Error(`bridge frame: unknown GatewayToShell tag ${tag}`);
  }
  reader.finish();
  return frame;
}
//#endregion 🔖️GatewayToShell

//#region 🔖️Hex
export function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

export function hexToBytes(hex: string): Uint8Array {
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < bytes.length; i++) bytes[i] = parseInt(hex.substring(i * 2, i * 2 + 2), 16);
  return bytes;
}
//#endregion 🔖️Hex
