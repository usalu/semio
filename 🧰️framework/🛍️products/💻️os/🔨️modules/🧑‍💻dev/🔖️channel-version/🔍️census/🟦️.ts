import type { ChannelVersionConsumerV1 } from "../📣️contributions/🟦️.ts";

/** 📌️ The pin that owns the number, relative to the repository root. */
export const CHANNEL_VERSION_PIN_PATH = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json";

/** 🔎️ Every shape a channel version literal takes in Rust, TypeScript and JSON; group 1 is the number. */
export const CHANNEL_VERSION_LITERAL_PATTERNS: readonly RegExp[] = [
  /appChannelVersion"?\s*:\s*(?:\{\s*"const"\s*:\s*)?(\d+)/gu,
  /"channelVersion"\s*:\s*(?:\{\s*"const"\s*:\s*)?(\d+)/gu,
  /appChannelVersion\s*===\s*(\d+)/gu,
  /APP_CHANNEL_VERSION\w*\s*=\s*(\d+)/gu,
  /CHANNEL_VERSION: u32 = (\d+)/gu,
  /app_channel_version:\s*(\d+)/gu,
  /"executionProtocol"\s*:\s*(\d+)/gu,
];

/** 🧬️ The version inside a hex-encoded first-party Pack value (a descriptor fixture's bytes): the key `appChannelVersion`
 * followed by its f64 value (tag `0x05`, eight little-endian bytes = group 1). The census counts it like a literal; the
 * generator never rewrites it — only a `derived` consumer may hold one, re-derived by its owner through the Pack codec. */
export const CHANNEL_VERSION_PACK_HEX_PATTERN = /6170704368616e6e656c56657273696f6e05([0-9a-f]{16})/gu;

/** 🧮️ One version literal found in a file: its byte offset and value. */
export type ChannelVersionLiteralV1 = Readonly<{ index: number; length: number; valueStart: number; value: number; encoded?: true }>;

/** 🔎️ Every version literal in `text`, in file order, with the exact span of its number. */
export function channelVersionLiterals(text: string): readonly ChannelVersionLiteralV1[] {
  const found: ChannelVersionLiteralV1[] = [];
  for (const pattern of CHANNEL_VERSION_LITERAL_PATTERNS) {
    for (const match of text.matchAll(new RegExp(pattern.source, pattern.flags))) {
      const number = match[1] ?? "";
      found.push({ index: match.index, length: match[0].length, valueStart: match.index + match[0].length - number.length, value: Number(number) });
    }
  }
  for (const match of text.matchAll(new RegExp(CHANNEL_VERSION_PACK_HEX_PATTERN.source, CHANNEL_VERSION_PACK_HEX_PATTERN.flags))) {
    const bytes = match[1] ?? "";
    found.push({ index: match.index, length: match[0].length, valueStart: match.index + match[0].length - bytes.length, value: Buffer.from(bytes, "hex").readDoubleLE(0), encoded: true });
  }
  return found.sort((left, right) => left.valueStart - right.valueStart).filter((literal, position, all) => position === 0 || all[position - 1]!.valueStart !== literal.valueStart);
}

export type ChannelVersionSourceViewV1 = Readonly<{
  pin: number;
  consumers: readonly ChannelVersionConsumerV1[];
  candidates: readonly string[];
  readText: (path: string) => string;
}>;

/** 🗺️Derives census roots from supplied source owners and declared consumers. */
export function channelVersionCensusRoots(owners: readonly { ownerRel: string }[], consumers: readonly ChannelVersionConsumerV1[]): readonly string[] {
  return [...new Set([CHANNEL_VERSION_PIN_PATH, ...consumers.map(consumer => consumer.path), ...owners.map(owner => owner.ownerRel)].map(path => path.split("/")[0]!))].sort();
}

/** 🧾️ One census finding: a file that states the version where it must not, or states it wrongly. */
export type ChannelVersionFindingV1 = Readonly<{ path: string; problem: "unregistered" | "missing" | "occurrences" | "hostile" | "drift"; detail: string }>;

/** 🔎️ The census: every candidate file is a registered consumer, holds exactly its declared literals and exactly its declared
 * hostile ones (a stale pin that equals a hostile value is otherwise invisible), and every other literal equals the pin. */
export function channelVersionCensus(source: ChannelVersionSourceViewV1): Readonly<{ pin: number; findings: readonly ChannelVersionFindingV1[] }> {
  const pin = source.pin;
  const findings: ChannelVersionFindingV1[] = [];
  const registered = new Map(source.consumers.map((consumer) => [consumer.path, consumer]));
  for (const path of source.candidates) {
    const literals = channelVersionLiterals(source.readText(path));
    if (literals.length > 0 && !registered.has(path)) findings.push({ path, problem: "unregistered", detail: `${literals.length} hand-written version literal(s): ${literals.map((literal) => literal.value).join(", ")}` });
  }
  for (const consumer of source.consumers) {
    let text: string;
    try {
      text = source.readText(consumer.path);
    } catch {
      findings.push({ path: consumer.path, problem: "missing", detail: "a registered consumer no longer exists" });
      continue;
    }
    const literals = channelVersionLiterals(text);
    if (literals.length !== consumer.occurrences) findings.push({ path: consumer.path, problem: "occurrences", detail: `declares ${consumer.occurrences} version literal(s), holds ${literals.length}` });
    if (consumer.arbitrary) continue;
    const hostile = literals.filter((literal) => (consumer.hostileValues ?? []).includes(literal.value)).length;
    if (hostile !== (consumer.hostileOccurrences ?? 0)) findings.push({ path: consumer.path, problem: "hostile", detail: `declares ${consumer.hostileOccurrences ?? 0} hostile literal(s), holds ${hostile}: a literal equal to a hostile value is a stale pin or an undeclared vector` });
    const drifted = literals.filter((literal) => literal.value !== pin && !(consumer.hostileValues ?? []).includes(literal.value));
    if (drifted.length > 0) findings.push({ path: consumer.path, problem: "drift", detail: `${drifted.length} literal(s) at ${[...new Set(drifted.map((literal) => literal.value))].join(", ")}, the pin is ${pin}${consumer.derived ? `; derived values to recompute by the owner: ${consumer.derived}` : ""}` });
  }
  return { pin, findings };
}

/** ✍️ Writes the pin into every drifted literal of every consumer the caller may rewrite: never an arbitrary or hostile
 * literal, never a file whose hostile literal count differs from its declaration, a guest-linked consumer only with `guest`, and never a consumer with derived values (those are reported). */
export function writeChannelVersionConsumers(source: ChannelVersionSourceViewV1, options: Readonly<{ guest: boolean; writeText: (path: string, text: string) => void }>): Readonly<{ written: readonly string[]; refused: readonly string[] }> {
  const pin = source.pin;
  const written: string[] = [];
  const refused: string[] = [];
  for (const consumer of source.consumers) {
    if (consumer.arbitrary) continue;
    const text = source.readText(consumer.path);
    const literals = channelVersionLiterals(text);
    if (literals.filter((literal) => (consumer.hostileValues ?? []).includes(literal.value)).length !== (consumer.hostileOccurrences ?? 0)) {
      refused.push(`${consumer.path}: holds another number of hostile literals than declared, set the stale pin literal by hand`);
      continue;
    }
    const drifted = literals.filter((literal) => literal.value !== pin && !(consumer.hostileValues ?? []).includes(literal.value));
    if (drifted.length === 0) continue;
    if (consumer.derived || drifted.some((literal) => literal.encoded) || (consumer.guest && !options.guest)) {
      refused.push(`${consumer.path}: ${consumer.derived ? `derived values (${consumer.derived}) must be recomputed by its owner` : drifted.some((literal) => literal.encoded) ? "Pack-encoded versions must be re-derived by its owner" : "guest-linked, rewrite inside a landing window with --guest"}`);
      continue;
    }
    let next = text;
    for (const literal of [...drifted].reverse()) {
      const digits = String(literal.value).length;
      next = `${next.slice(0, literal.valueStart)}${pin}${next.slice(literal.valueStart + digits)}`;
    }
    options.writeText(consumer.path, next);
    written.push(consumer.path);
  }
  return { written, refused };
}

