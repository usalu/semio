//! 👥️ Presence binary representation implements the original semantic peer types.
import type { ArtifactPresenceDomain, ArtifactPresenceHistoryEdit, ArtifactPresenceInteraction, ArtifactPresencePeer, ArtifactPresencePrincipalKind, ArtifactPresenceToolRun, ArtifactPresenceTyping, ArtifactPresenceUi, ArtifactPresenceViewKind, ArtifactPresenceWindowView } from "../../../🟦️.ts";
import { PRESENCE_HISTORY_EDIT_STAGES, PRESENCE_PRINCIPAL_KINDS, PRESENCE_TOOL_RUN_STATES, PRESENCE_TYPING_EXCERPT_BYTES } from "../../../🟦️.ts";
import { writeVarintU64, readVarintU64, writeStr, readStr, writeBytes, writeBool, writeF64 } from "../🟦️.ts";
import { writeVecStr, readVecStr, writeOptStr } from "../🔗️causal/🟦️.ts";

/** 🎯️ `actor str | flags varint_u64 | connected_at_ms varint | fields present per bitmask, strictly
 * in bit order (label str? | presence_pack bytes? | user_id str? | role str? | drag_ghost_json str? |
 * interaction? | color u8? | surface str? | views? | ui? | tool_run?)` — the TS twin of Rust
 * `encode_presence_peer` (`📡️wire/🦀️.rs`). This is what `ClientFrame::Presence.peer`/
 * `ServerFrame::Presence.peers[]` actually carry — real binary, not JSON bytes. `flags` is a varint
 * (not a single byte) now that bit 9 exceeds a byte's range. */
/** 🕳️ Presence-flag guard. A field is "present" only when it is neither `undefined` **nor
 * `null`**: these peers are reconstructed from JSON view state, where an absent optional arrives as
 * `null`, and `null !== undefined` is true — so a bare `!== undefined` check set the flag and then
 * handed `null` to `writeStr`/`writeBytes`, throwing `Cannot read properties of null (reading
 * 'length')` on every heartbeat and wedging the plugin instance. */
function presencePresent<T>(value: T | null | undefined): value is T {
  return value !== undefined && value !== null;
}

export function encodePresencePeer(peer: ArtifactPresencePeer): number[] {
  const out: number[] = [];
  writeStr(out, peer.actor);
  let flags = 0;
  if (presencePresent(peer.label)) flags |= 1 << 0;
  if (presencePresent(peer.presencePack)) flags |= 1 << 1;
  if (presencePresent(peer.userId)) flags |= 1 << 2;
  if (presencePresent(peer.role)) flags |= 1 << 3;
  if (presencePresent(peer.dragGhostJson)) flags |= 1 << 4;
  if (presencePresent(peer.interaction)) flags |= 1 << 5;
  if (presencePresent(peer.color)) flags |= 1 << 6;
  if (presencePresent(peer.surface)) flags |= 1 << 7;
  if (peer.views.length > 0) flags |= 1 << 8;
  if (presencePresent(peer.ui)) flags |= 1 << 9;
  if (presencePresent(peer.toolRun)) flags |= 1 << 10;
  if (presencePresent(peer.principalKind)) flags |= 1 << 11;
  if (presencePresent(peer.activeTool)) flags |= 1 << 12;
  if (presencePresent(peer.historyEdit)) flags |= 1 << 13;
  if ((peer.typing?.length ?? 0) > 0) flags |= 1 << 14;
  writeVarintU64(out, flags);
  writeVarintU64(out, peer.connectedAtMs ?? 0);
  if (presencePresent(peer.label)) writeStr(out, peer.label);
  if (presencePresent(peer.presencePack)) writeBytes(out, peer.presencePack);
  if (presencePresent(peer.userId)) writeStr(out, peer.userId);
  if (presencePresent(peer.role)) writeStr(out, peer.role);
  if (presencePresent(peer.dragGhostJson)) writeStr(out, peer.dragGhostJson);
  if (presencePresent(peer.interaction)) writePresenceInteraction(out, peer.interaction);
  if (presencePresent(peer.color)) out.push(peer.color);
  if (presencePresent(peer.surface)) writeStr(out, peer.surface);
  if (peer.views.length > 0) writeVecPresenceWindowView(out, peer.views);
  if (presencePresent(peer.ui)) writePresenceUi(out, peer.ui);
  if (presencePresent(peer.toolRun)) writePresenceToolRun(out, peer.toolRun);
  if (presencePresent(peer.principalKind)) {
    const tag = PRESENCE_PRINCIPAL_KINDS.indexOf(peer.principalKind);
    if (tag < 0) throw new Error(`presence peer principal kind: unknown ${peer.principalKind}`);
    out.push(tag);
  }
  if (presencePresent(peer.activeTool)) writeStr(out, peer.activeTool);
  if (presencePresent(peer.historyEdit)) writePresenceHistoryEdit(out, peer.historyEdit);
  if (peer.typing !== undefined && peer.typing.length > 0) {
    writeVarintU64(out, peer.typing.length);
    for (const typing of peer.typing) {
      writeStr(out, typing.windowId);
      writeStr(out, typing.deleted);
      writeStr(out, typing.insert);
    }
  }
  return out;
}

function writePresenceHistoryEdit(out: number[], historyEdit: ArtifactPresenceHistoryEdit): void {
  const tag = PRESENCE_HISTORY_EDIT_STAGES.indexOf(historyEdit.stage);
  if (tag < 0) throw new Error(`presence history edit stage: unknown ${historyEdit.stage}`);
  if (!Number.isSafeInteger(historyEdit.drafts) || historyEdit.drafts < 0 || historyEdit.drafts > 0xffffffff) throw new Error("presence history edit drafts: limit exceeded");
  writeStr(out, historyEdit.mutationId);
  out.push(tag);
  writeVarintU64(out, historyEdit.drafts);
}

function writePresenceToolRun(out: number[], toolRun: ArtifactPresenceToolRun): void {
  const tag = PRESENCE_TOOL_RUN_STATES.indexOf(toolRun.state);
  if (tag < 0) throw new Error(`presence tool run state: unknown ${toolRun.state}`);
  writeStr(out, toolRun.toolId);
  out.push(tag);
  writeVarintU64(out, toolRun.stage);
  writeVarintU64(out, toolRun.completed);
  out.push(presencePresent(toolRun.total) ? 1 : 0);
  if (presencePresent(toolRun.total)) writeVarintU64(out, toolRun.total);
}

/** 🛡️ Fixed hostile-input ceilings shared byte-for-byte with Rust. */
export const PRESENCE_PEER_WIRE_LIMITS_V1 = Object.freeze({
  maximumEntryBytes: 4_096,
  maximumTextBytes: 1_024,
  maximumPresencePackBytes: 2_048,
  maximumViews: 16,
  maximumInteractionDomains: 16,
  maximumDomainIds: 64,
  maximumConnectedAtMs: Number.MAX_SAFE_INTEGER,
  maximumToolRunUnits: Number.MAX_SAFE_INTEGER,
  maximumTypingRuns: 8,
});

class PresencePeerReader {
  readonly bytes: Uint8Array;
  position: number;

  constructor(bytes: Uint8Array, position: number) {
    this.bytes = bytes;
    this.position = position;
  }

  fail(what: string, detail: string): never {
    throw new Error(`${what} at ${this.position}: ${detail}`);
  }

  varint(what: string): number {
    const start = this.position;
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      const byte = this.bytes[this.position];
      if (byte === undefined) this.fail(what, "truncated varint");
      this.position += 1;
      if (index === 9 && byte > 1) this.fail(what, "varint exceeds u64");
      value |= BigInt(byte & 0x7f) << BigInt(index * 7);
      if ((byte & 0x80) === 0) {
        if (this.position - start > 1 && byte === 0) this.fail(what, "noncanonical varint");
        if (value > BigInt(Number.MAX_SAFE_INTEGER)) this.fail(what, "integer exceeds exact TypeScript range");
        return Number(value);
      }
    }
    return this.fail(what, "overlong varint");
  }

  length(maximum: number, what: string): number {
    const length = this.varint(what);
    if (length > maximum) this.fail(what, "limit exceeded");
    if (length > this.bytes.length - this.position) this.fail(what, "truncated field");
    return length;
  }

  count(maximum: number, what: string): number {
    return this.length(maximum, what);
  }

  text(what: string): string {
    const length = this.length(PRESENCE_PEER_WIRE_LIMITS_V1.maximumTextBytes, what);
    const end = this.position + length;
    let value: string;
    try { value = new TextDecoder("utf-8", { fatal: true }).decode(this.bytes.subarray(this.position, end)); }
    catch { return this.fail(what, "invalid utf8"); }
    this.position = end;
    return value;
  }

  blob(what: string): number[] {
    const length = this.length(PRESENCE_PEER_WIRE_LIMITS_V1.maximumPresencePackBytes, what);
    const end = this.position + length;
    const value = Array.from(this.bytes.subarray(this.position, end));
    this.position = end;
    return value;
  }

  byte(what: string): number {
    const value = this.bytes[this.position];
    if (value === undefined) this.fail(what, "truncated byte");
    this.position += 1;
    return value;
  }

  boolean(what: string): boolean {
    const value = this.byte(what);
    if (value !== 0 && value !== 1) this.fail(what, "boolean must be zero or one");
    return value === 1;
  }

  number(what: string): number {
    const end = this.position + 8;
    if (end > this.bytes.length) this.fail(what, "truncated f64");
    const value = new DataView(this.bytes.buffer, this.bytes.byteOffset + this.position, 8).getFloat64(0, true);
    if (!Number.isFinite(value)) this.fail(what, "non-finite f64");
    this.position = end;
    return value;
  }

  triple(what: string): readonly [number, number, number] {
    return [this.number(what), this.number(what), this.number(what)];
  }

  strings(what: string): string[] {
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumDomainIds, what);
    const values: string[] = [];
    for (let index = 0; index < count; index += 1) values.push(this.text(what));
    return values;
  }

  interaction(): ArtifactPresenceInteraction {
    const app_id = this.text("presence interaction app id");
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumInteractionDomains, "presence interaction domains");
    const domains: ArtifactPresenceDomain[] = [];
    for (let index = 0; index < count; index += 1) domains.push({ domain: this.text("presence interaction domain"), granularity: this.text("presence interaction granularity"), selected: this.strings("presence interaction selected"), hovered: this.strings("presence interaction hovered") });
    return { app_id, domains };
  }

  viewKind(): ArtifactPresenceViewKind {
    const tag = this.byte("presence view kind");
    if (tag === 0) return { kind: "canvas", x: this.number("presence canvas x"), y: this.number("presence canvas y"), zoom: this.number("presence canvas zoom") };
    if (tag === 1) return { kind: "orbit", position: this.triple("presence orbit position"), target: this.triple("presence orbit target"), up: this.triple("presence orbit up"), fov: this.number("presence orbit fov") };
    if (tag === 2) return { kind: "geo", lng: this.number("presence geo longitude"), lat: this.number("presence geo latitude"), zoom: this.number("presence geo zoom"), bearing: this.number("presence geo bearing"), pitch: this.number("presence geo pitch") };
    return this.fail("presence view kind", `unknown tag ${tag}`);
  }

  views(): ArtifactPresenceWindowView[] {
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumViews, "presence views");
    const views: ArtifactPresenceWindowView[] = [];
    for (let index = 0; index < count; index += 1) {
      const windowId = this.text("presence view window id");
      const space = this.text("presence view space");
      const kind = this.viewKind();
      const size: readonly [number, number] = [this.number("presence view width"), this.number("presence view height")];
      const pointer = this.boolean("presence view pointer") ? this.triple("presence view pointer") : undefined;
      const rayOrigin = this.boolean("presence view ray origin") ? this.triple("presence view ray origin") : undefined;
      views.push({ windowId, space, kind, size, pointer, rayOrigin });
    }
    return views;
  }

  optionalText(what: string): string | undefined {
    return this.boolean(what) ? this.text(what) : undefined;
  }

  toolRunUnits(what: string): number {
    const value = this.varint(what);
    if (value > PRESENCE_PEER_WIRE_LIMITS_V1.maximumToolRunUnits) this.fail(what, "limit exceeded");
    return value;
  }

  toolRun(): ArtifactPresenceToolRun {
    const toolId = this.text("presence tool run id");
    const tag = this.byte("presence tool run state");
    const state = PRESENCE_TOOL_RUN_STATES[tag];
    if (state === undefined) this.fail("presence tool run state", `unknown tag ${tag}`);
    const stage = this.varint("presence tool run stage");
    if (stage > 0xffff) this.fail("presence tool run stage", "limit exceeded");
    const completed = this.toolRunUnits("presence tool run completed");
    const total = this.boolean("presence tool run total") ? this.toolRunUnits("presence tool run total") : undefined;
    if (total !== undefined && completed > total) this.fail("presence tool run completed", "completed exceeds total");
    return { toolId, state, stage, completed, total };
  }

  ui(): ArtifactPresenceUi {
    return { hoveredPath: this.optionalText("presence ui hovered path"), focusedPath: this.optionalText("presence ui focused path"), pressedPath: this.optionalText("presence ui pressed path") };
  }

  historyEdit(): ArtifactPresenceHistoryEdit {
    const mutationId = this.text("presence history edit mutation");
    const tag = this.byte("presence history edit stage");
    const stage = PRESENCE_HISTORY_EDIT_STAGES[tag];
    if (stage === undefined) this.fail("presence history edit stage", `unknown tag ${tag}`);
    const drafts = this.varint("presence history edit drafts");
    if (drafts > 0xffffffff) this.fail("presence history edit drafts", "limit exceeded");
    return { mutationId, stage, drafts };
  }

  typing(): ArtifactPresenceTyping[] {
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumTypingRuns, "presence typing runs");
    if (count === 0) this.fail("presence typing runs", "an empty typing list is never flagged");
    const runs: ArtifactPresenceTyping[] = [];
    for (let index = 0; index < count; index += 1) {
      const windowId = this.text("presence typing window");
      const deleted = this.text("presence typing deleted");
      const insert = this.text("presence typing insert");
      if (new TextEncoder().encode(deleted).length > PRESENCE_TYPING_EXCERPT_BYTES || new TextEncoder().encode(insert).length > PRESENCE_TYPING_EXCERPT_BYTES) this.fail("presence typing excerpt", "limit exceeded");
      runs.push({ windowId, deleted, insert });
    }
    return runs;
  }

  principalKind(): ArtifactPresencePrincipalKind {
    const tag = this.byte("presence peer principal kind");
    const kind = PRESENCE_PRINCIPAL_KINDS[tag];
    if (kind === undefined) this.fail("presence peer principal kind", `unknown principal kind tag ${tag}`);
    return kind;
  }
}

/** 🎯️ Exact, allocation-bounded inverse of {@link encodePresencePeer}. */
export function decodePresencePeer(bytes: Uint8Array, pos: [number]): ArtifactPresencePeer {
  if (!Number.isSafeInteger(pos[0]) || pos[0] < 0 || pos[0] > bytes.length) throw new Error("presence peer position: invalid");
  if (bytes.length - pos[0] > PRESENCE_PEER_WIRE_LIMITS_V1.maximumEntryBytes) throw new Error("presence peer entry bytes: limit exceeded");
  const reader = new PresencePeerReader(bytes, pos[0]);
  const actor = reader.text("presence peer actor");
  const flags = reader.varint("presence peer flags");
  if (flags > 0x7fff) reader.fail("presence peer flags", `unknown flag bits set: ${flags.toString(16)}`);
  const connectedAtMs = reader.varint("presence peer connected at");
  if (connectedAtMs > PRESENCE_PEER_WIRE_LIMITS_V1.maximumConnectedAtMs) reader.fail("presence peer connected at", "limit exceeded");
  const label = flags & (1 << 0) ? reader.text("presence peer label") : undefined;
  const presencePack = flags & (1 << 1) ? reader.blob("presence peer pack") : undefined;
  const userId = flags & (1 << 2) ? reader.text("presence peer user id") : undefined;
  const role = flags & (1 << 3) ? reader.text("presence peer role") : undefined;
  const dragGhostJson = flags & (1 << 4) ? reader.text("presence peer drag ghost") : undefined;
  const interaction = flags & (1 << 5) ? reader.interaction() : undefined;
  const color = flags & (1 << 6) ? reader.byte("presence peer color") : undefined;
  const surface = flags & (1 << 7) ? reader.text("presence peer surface") : undefined;
  const views = flags & (1 << 8) ? reader.views() : [];
  const ui = flags & (1 << 9) ? reader.ui() : undefined;
  const toolRun = flags & (1 << 10) ? reader.toolRun() : undefined;
  const principalKind = flags & (1 << 11) ? reader.principalKind() : undefined;
  const activeTool = flags & (1 << 12) ? reader.text("presence peer active tool") : undefined;
  const historyEdit = flags & (1 << 13) ? reader.historyEdit() : undefined;
  const typing = flags & (1 << 14) ? reader.typing() : undefined;
  if (reader.position !== bytes.length) reader.fail("presence peer", "trailing bytes");
  pos[0] = reader.position;
  return { actor, connectedAtMs, label, presencePack, userId, role, dragGhostJson, interaction, color, surface, views, ui, toolRun, principalKind, activeTool, historyEdit, ...(typing === undefined ? {} : { typing }) };
}

/** ⏯️ Twin of Rust `encode_presence_tool_run`: the standalone tool run summary body a guest's
 * `AppFrame::Ephemeral.tool_run` carries, byte-identical to a peer's flag-bit-10 section. */
export function encodePresenceToolRun(toolRun: ArtifactPresenceToolRun): number[] {
  const out: number[] = [];
  writePresenceToolRun(out, toolRun);
  return out;
}

/** 🎞️ Twin of Rust `decode_presence_tool_run`: the peer decoder's limits over a standalone body, no trailing bytes. */
export function decodePresenceToolRun(bytes: Uint8Array): ArtifactPresenceToolRun {
  if (bytes.length > PRESENCE_PEER_WIRE_LIMITS_V1.maximumEntryBytes) throw new Error("presence tool run bytes: limit exceeded");
  const reader = new PresencePeerReader(bytes, 0);
  const toolRun = reader.toolRun();
  if (reader.position !== bytes.length) reader.fail("presence tool run", "trailing bytes");
  return toolRun;
}

/** ⏪️ Twin of Rust `encode_presence_history_edit`: the standalone history-edit summary body a guest's
 * `AppFrame::Ephemeral.history_edit` carries, byte-identical to a peer's flag-bit-13 section. */
export function encodePresenceHistoryEdit(historyEdit: ArtifactPresenceHistoryEdit): number[] {
  const out: number[] = [];
  writePresenceHistoryEdit(out, historyEdit);
  return out;
}

/** 🎞️ Twin of Rust `decode_presence_history_edit`: the peer decoder's limits over a standalone body, no trailing bytes. */
export function decodePresenceHistoryEdit(bytes: Uint8Array): ArtifactPresenceHistoryEdit {
  if (bytes.length > PRESENCE_PEER_WIRE_LIMITS_V1.maximumEntryBytes) throw new Error("presence history edit bytes: limit exceeded");
  const reader = new PresencePeerReader(bytes, 0);
  const historyEdit = reader.historyEdit();
  if (reader.position !== bytes.length) reader.fail("presence history edit", "trailing bytes");
  return historyEdit;
}

/** 🎞️ One raw byte — the TS twin of `protocol_core::read_u8`-shaped inline reads. */
export function readU8(bytes: Uint8Array, pos: [number]): number {
  const byte = bytes[pos[0]];
  if (byte === undefined) throw new Error("presence peer color: truncated");
  pos[0] += 1;
  return byte;
}

/** 🕹️ Twin of Rust `encode_presence_interaction` — `pub` (C7.4) so a guest that never enables the
 * kernel's `sync` feature can still call it directly. */
export function encodePresenceInteraction(interaction: ArtifactPresenceInteraction): number[] {
  const out: number[] = [];
  writePresenceInteraction(out, interaction);
  return out;
}

function writePresenceInteraction(out: number[], interaction: ArtifactPresenceInteraction): void {
  writeStr(out, interaction.app_id);
  writeVarintU64(out, interaction.domains.length);
  for (const domain of interaction.domains) {
    writeStr(out, domain.domain);
    writeStr(out, domain.granularity);
    writeVecStr(out, domain.selected);
    writeVecStr(out, domain.hovered);
  }
}

/** 🕹️ Twin of Rust `decode_presence_interaction` — `pub` for the same reason as
 * {@link encodePresenceInteraction}. */
export function decodePresenceInteraction(bytes: Uint8Array, pos: [number]): ArtifactPresenceInteraction {
  return readPresenceInteraction(bytes, pos);
}

/** 🕹️ Twin of Rust `decode_presence_interaction` — app id, then a varint-counted run of domains. */
function readPresenceInteraction(bytes: Uint8Array, pos: [number]): ArtifactPresenceInteraction {
  const app_id = readStr(bytes, pos);
  const count = Number(readVarintU64(bytes, pos));
  const domains: ArtifactPresenceDomain[] = [];
  for (let index = 0; index < count; index += 1) {
    domains.push({ domain: readStr(bytes, pos), granularity: readStr(bytes, pos), selected: readVecStr(bytes, pos), hovered: readVecStr(bytes, pos) });
  }
  return { app_id, domains };
}

//#region 🔖️PresenceView
/** 🎥️ Twin of Rust `encode_presence_view_kind` — discriminant `u8` (0 Canvas, 1 Orbit, 2 Geo) then
 * that variant's `f64` fields in declared order. */
function writePresenceViewKind(out: number[], kind: ArtifactPresenceViewKind): void {
  if (kind.kind === "canvas") {
    out.push(0);
    writeF64(out, kind.x);
    writeF64(out, kind.y);
    writeF64(out, kind.zoom);
  } else if (kind.kind === "orbit") {
    out.push(1);
    for (const value of [...kind.position, ...kind.target, ...kind.up]) writeF64(out, value);
    writeF64(out, kind.fov);
  } else {
    out.push(2);
    writeF64(out, kind.lng);
    writeF64(out, kind.lat);
    writeF64(out, kind.zoom);
    writeF64(out, kind.bearing);
    writeF64(out, kind.pitch);
  }
}

function writePresenceWindowView(out: number[], view: ArtifactPresenceWindowView): void {
  writeStr(out, view.windowId);
  writeStr(out, view.space);
  writePresenceViewKind(out, view.kind);
  writeF64(out, view.size[0]);
  writeF64(out, view.size[1]);
  writeBool(out, presencePresent(view.pointer));
  if (presencePresent(view.pointer)) for (const value of view.pointer) writeF64(out, value);
  writeBool(out, presencePresent(view.rayOrigin));
  if (presencePresent(view.rayOrigin)) for (const value of view.rayOrigin) writeF64(out, value);
}

function writeVecPresenceWindowView(out: number[], values: readonly ArtifactPresenceWindowView[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writePresenceWindowView(out, value);
}

function writePresenceUi(out: number[], ui: ArtifactPresenceUi): void {
  writeOptStr(out, ui.hoveredPath ?? null);
  writeOptStr(out, ui.focusedPath ?? null);
  writeOptStr(out, ui.pressedPath ?? null);
}

//#endregion 🔖️PresenceView
