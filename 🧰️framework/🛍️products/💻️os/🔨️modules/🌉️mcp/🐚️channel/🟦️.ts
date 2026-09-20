/** 🐚️ The shell bank of the live artifact route's payload codec — the TypeScript twin of
 * `🐚️channel/🦀️.rs`.
 *
 * The bridge frame envelope (`🧵️bridge/🟦️.ts`) carries `AppCommand.command` and each
 * `AppFrames.frames[i]` as opaque bytes. THIS module is what those bytes mean: the versioned JSON
 * contract the gateway and the shell must agree on for an MCP client's `action_invoke`,
 * `history_undo`, `transaction_*`, `artifact_snapshot` and `artifact_export` to execute inside the
 * shell the human is looking at instead of inside the gateway's own headless interpreter.
 *
 * Both banks are tested against the SAME fixture file (`🐚️channel/🧫️fixtures/🗿️app-payloads.json`),
 * the way `🧵️bridge`'s own 23 frame fixtures already pin the envelope: a drift in either direction
 * is a red test here and there, never a silently-dropped mutation at runtime. */

//#region 🔖️Version
/** 🏷️ The payload contract version. A payload declaring anything else is refused by name — the two
 * banks ship in one repository but not necessarily in one build (a staged gateway binary can be
 * older than the served shell; that exact skew is what `📓️m7`'s §5.1(i) cost a session). */
export const SHELL_CHANNEL_PAYLOAD_VERSION = 1;
//#endregion 🔖️Version

//#region 🔖️Base64
const BASE64_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/** 🔤️ Standard base64 with padding — byte-identical to `🐚️channel/🦀️.rs`'s `encode_base64`, and
 * written out rather than delegated to `btoa` so the two banks share one definition that also works
 * outside a browser (the shell runs under vitest and under a headless worker too). */
export function encodeChannelBase64(bytes: Uint8Array): string {
  let out = "";
  for (let at = 0; at < bytes.length; at += 3) {
    const b0 = bytes[at] ?? 0;
    const b1 = bytes[at + 1] ?? 0;
    const b2 = bytes[at + 2] ?? 0;
    const remaining = bytes.length - at;
    out += BASE64_ALPHABET[b0 >> 2];
    out += BASE64_ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)];
    out += remaining > 1 ? BASE64_ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] : "=";
    out += remaining > 2 ? BASE64_ALPHABET[b2 & 0x3f] : "=";
  }
  return out;
}

/** 🔡️ Total inverse of {@link encodeChannelBase64} — `null` for anything malformed, never a partial
 * decode: these bytes come from another process. */
export function decodeChannelBase64(text: string): Uint8Array | null {
  if (text.length % 4 !== 0) return null;
  const out: number[] = [];
  for (let at = 0; at < text.length; at += 4) {
    const quad = [0, 0, 0, 0];
    let padding = 0;
    for (let index = 0; index < 4; index += 1) {
      const symbol = text[at + index] ?? "";
      if (symbol === "=") {
        if (index < 2) return null;
        padding += 1;
        continue;
      }
      if (padding > 0) return null;
      const position = BASE64_ALPHABET.indexOf(symbol);
      if (position < 0) return null;
      quad[index] = position;
    }
    const triple = (quad[0] << 18) | (quad[1] << 12) | (quad[2] << 6) | quad[3];
    out.push((triple >> 16) & 0xff);
    if (padding < 2) out.push((triple >> 8) & 0xff);
    if (padding < 1) out.push(triple & 0xff);
  }
  return Uint8Array.from(out);
}
//#endregion 🔖️Base64

//#region 🔖️Types
/** 📦️ One op payload set split by store lane — the shell never inspects an individual op, it
 * carries them back verbatim, exactly as the gateway's own `PreparedOps` contract requires. */
export type ShellPreparedOpsV1 = {
  readonly document: readonly Uint8Array[];
  readonly config: readonly Uint8Array[];
  readonly draft: readonly Uint8Array[];
};

export const EMPTY_PREPARED_OPS: ShellPreparedOpsV1 = { document: [], config: [], draft: [] };

/** ✍️ Who is mutating. The shell stamps the agent as its own actor so the edit is attributable in
 * the human's history rather than looking like something they did themselves. */
export type ShellMutationOriginV1 = { readonly kind: "agent"; readonly principal: string; readonly invocationId: string };

/** 📤️ Every command the gateway may send down the live artifact route. */
export type ShellAppCommandV1 =
  | { readonly kind: "readHistory" }
  | { readonly kind: "readArtifact" }
  | { readonly kind: "pureCommand"; readonly capabilityId: string; readonly input: unknown }
  | { readonly kind: "transactionPrepare"; readonly txnId: string; readonly ops: ShellPreparedOpsV1; readonly label: string; readonly origin: ShellMutationOriginV1 }
  | { readonly kind: "transactionCommit"; readonly txnId: string }
  | { readonly kind: "transactionRollback"; readonly txnId: string }
  | { readonly kind: "transactionUndo"; readonly groupId: string }
  | { readonly kind: "transactionRedo"; readonly groupId: string }
  | { readonly kind: "exportMedia"; readonly port: string; readonly document: Uint8Array; readonly documentSpr: Uint8Array }
  /** 🛑️ The gateway gave up on `cancelSeq` (its own wall budget expired). Cooperative: the shell
   * stops working on that command rather than committing an edit nobody is waiting for. */
  | { readonly kind: "cancel"; readonly cancelSeq: bigint };

/** 📥️ Every reply the shell may answer with. `error` is a COMMAND-level refusal carrying the
 * gateway's own fault-code vocabulary; there is no "no answer" — silence is the one outcome this
 * protocol never permits, because it costs the agent its whole wall budget. */
export type ShellAppFrameV1 =
  | { readonly kind: "historySnapshot"; readonly artifactId: string; readonly headEditId: string; readonly cursor: string }
  | { readonly kind: "emit"; readonly ops: ShellPreparedOpsV1; readonly warnings: readonly string[] }
  | { readonly kind: "transactionPrepared"; readonly txnId: string }
  | { readonly kind: "transactionCommitted"; readonly txnId: string; readonly editId: string }
  | { readonly kind: "transactionRolledBack"; readonly txnId: string }
  | { readonly kind: "transactionUndone"; readonly groupId: string }
  | { readonly kind: "transactionRedone"; readonly groupId: string }
  | { readonly kind: "artifact"; readonly pack: Uint8Array; readonly spr: Uint8Array }
  | { readonly kind: "exported"; readonly port: string; readonly descriptor: Uint8Array; readonly data: Uint8Array }
  | { readonly kind: "error"; readonly code: string; readonly message: string };

/** ⚠️ The fault codes the gateway's own `map_fault` recognises. Anything else becomes an opaque
 * `INTERNAL` on the agent's side, so a shell that wants to be understood answers one of these. */
export type ShellAppFaultCode = "viewer.read-only" | "capability-denied" | "mutation.rejected" | "transaction.generation-mismatch" | "transaction.instance-busy" | "budget.exceeded" | "capability.not-found" | "plugin.unavailable" | "channel.not-wired";

export function shellAppFault(code: ShellAppFaultCode, message: string): ShellAppFrameV1 {
  return { kind: "error", code, message };
}
//#endregion 🔖️Types

//#region 🔖️Codec
function lanes(value: unknown): readonly Uint8Array[] {
  if (value === undefined || value === null) return [];
  if (!Array.isArray(value)) throw new Error("prepared-ops lane is not an array");
  return value.map((entry) => {
    const decoded = typeof entry === "string" ? decodeChannelBase64(entry) : null;
    if (decoded === null) throw new Error("prepared-ops lane entry is not base64");
    return decoded;
  });
}

function requireString(record: Record<string, unknown>, key: string): string {
  const value = record[key];
  if (typeof value !== "string") throw new Error(`shell AppCommand payload is missing the string field \`${key}\``);
  return value;
}

function requireBytes(record: Record<string, unknown>, key: string): Uint8Array {
  const value = record[key];
  const decoded = typeof value === "string" ? decodeChannelBase64(value) : null;
  if (decoded === null) throw new Error(`shell AppCommand payload field \`${key}\` is not base64`);
  return decoded;
}

/** 📥️ Decodes one `GatewayToShell.appCommand` payload. Throws with a named reason on anything
 * malformed — the caller turns that into an `error` frame so the agent learns WHY rather than
 * timing out. */
export function decodeShellAppCommand(bytes: Uint8Array): ShellAppCommandV1 {
  const parsed: unknown = JSON.parse(new TextDecoder("utf-8").decode(bytes));
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) throw new Error("shell AppCommand payload is not a JSON object");
  const record = parsed as Record<string, unknown>;
  const version = record.version;
  if (version !== SHELL_CHANNEL_PAYLOAD_VERSION) throw new Error(`shell AppCommand payload declares codec version ${String(version)}, this shell speaks ${SHELL_CHANNEL_PAYLOAD_VERSION}`);
  const kind = record.kind;
  switch (kind) {
    case "readHistory":
      return { kind: "readHistory" };
    case "readArtifact":
      return { kind: "readArtifact" };
    case "pureCommand":
      return { kind: "pureCommand", capabilityId: requireString(record, "capabilityId"), input: record.input };
    case "transactionPrepare": {
      const ops = (record.ops ?? {}) as Record<string, unknown>;
      const origin = (record.origin ?? {}) as Record<string, unknown>;
      return {
        kind: "transactionPrepare",
        txnId: requireString(record, "txnId"),
        ops: { document: lanes(ops.document), config: lanes(ops.config), draft: lanes(ops.draft) },
        label: requireString(record, "label"),
        origin: { kind: "agent", principal: requireString(origin, "principal"), invocationId: requireString(origin, "invocationId") },
      };
    }
    case "transactionCommit":
      return { kind: "transactionCommit", txnId: requireString(record, "txnId") };
    case "transactionRollback":
      return { kind: "transactionRollback", txnId: requireString(record, "txnId") };
    case "transactionUndo":
      return { kind: "transactionUndo", groupId: requireString(record, "groupId") };
    case "transactionRedo":
      return { kind: "transactionRedo", groupId: requireString(record, "groupId") };
    case "exportMedia":
      return { kind: "exportMedia", port: requireString(record, "port"), document: requireBytes(record, "document"), documentSpr: requireBytes(record, "documentSpr") };
    case "cancel":
      return { kind: "cancel", cancelSeq: BigInt(String(record.cancelSeq ?? "0")) };
    default:
      throw new Error(`shell AppCommand payload declares unknown kind \`${String(kind)}\``);
  }
}

function encodeLanes(ops: ShellPreparedOpsV1): Record<string, string[]> {
  return {
    document: ops.document.map((lane) => encodeChannelBase64(lane)),
    config: ops.config.map((lane) => encodeChannelBase64(lane)),
    draft: ops.draft.map((lane) => encodeChannelBase64(lane)),
  };
}

/** 📤️ Renders one reply frame as this codec's JSON value — the shape the shared fixture pins. */
export function shellAppFrameToJson(frame: ShellAppFrameV1): Record<string, unknown> {
  const version = SHELL_CHANNEL_PAYLOAD_VERSION;
  switch (frame.kind) {
    case "historySnapshot":
      return { version, kind: frame.kind, artifactId: frame.artifactId, headEditId: frame.headEditId, cursor: frame.cursor };
    case "emit":
      return { version, kind: frame.kind, ops: encodeLanes(frame.ops), warnings: [...frame.warnings] };
    case "transactionPrepared":
    case "transactionRolledBack":
      return { version, kind: frame.kind, txnId: frame.txnId };
    case "transactionCommitted":
      return { version, kind: frame.kind, txnId: frame.txnId, editId: frame.editId };
    case "transactionUndone":
    case "transactionRedone":
      return { version, kind: frame.kind, groupId: frame.groupId };
    case "artifact":
      return { version, kind: frame.kind, pack: encodeChannelBase64(frame.pack), spr: encodeChannelBase64(frame.spr) };
    case "exported":
      return { version, kind: frame.kind, port: frame.port, descriptor: encodeChannelBase64(frame.descriptor), data: encodeChannelBase64(frame.data) };
    case "error":
      return { version, kind: frame.kind, code: frame.code, message: frame.message };
  }
}

/** 📦️ One reply frame as the bytes a `ShellToGateway.appFrames` entry carries. */
export function encodeShellAppFrame(frame: ShellAppFrameV1): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(shellAppFrameToJson(frame)));
}
//#endregion 🔖️Codec
