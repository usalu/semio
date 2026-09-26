import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

type UtcMinuteFixture = {
  cases: { epochMs: number; en: string; de: string; rfc3339: string }[];
  acceptedRfc3339: { text: string; epochMs: number }[];
  refusedRfc3339: string[];
};

const FIXTURE = "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🕰️utc-minute/🔣️.json";

function utcParts(epochMs: number): Record<string, string> {
  const format = new Intl.DateTimeFormat("en-US", { timeZone: "UTC", year: "numeric", month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit", hourCycle: "h23" });
  return Object.fromEntries(format.formatToParts(new Date(epochMs)).map((part) => [part.type, part.value]));
}

/** 🕰️ Proves the space tables' UTC-minute cases with an independent oracle: `Intl.DateTimeFormat` (ICU) in UTC
 * yields the fixture's en and de texts, and `Date.parse` agrees with every accepted and refused RFC 3339 stamp
 * the Rust `rfc3339_utc_epoch_ms` law reads from the same file. */
export function testUtcMinuteFixtureAgainstIntl(root: string): void {
  const fixture = JSON.parse(readFileSync(join(root, FIXTURE), "utf8")) as UtcMinuteFixture;
  assert(fixture.cases.length > 0);
  for (const entry of fixture.cases) {
    const parts = utcParts(entry.epochMs);
    const year = parts.year!.padStart(4, "0");
    assert.equal(entry.en, `${year}-${parts.month}-${parts.day} ${parts.hour}:${parts.minute} UTC`);
    assert.equal(entry.de, `${parts.day}.${parts.month}.${year}, ${parts.hour}:${parts.minute} UTC`);
    assert.equal(Date.parse(entry.rfc3339), entry.epochMs - (entry.epochMs % 1000));
  }
  for (const accepted of fixture.acceptedRfc3339) assert.equal(Math.floor(Date.parse(accepted.text) / 1000) * 1000, accepted.epochMs);
  for (const refused of fixture.refusedRfc3339) assert(!/^\d{4}-\d{2}-\d{2}T([01]\d|2[0-3]):[0-5]\d:[0-5]\d(\.\d+)?Z$/u.test(refused) || Number.isNaN(Date.parse(refused)), refused);
}
