import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, it } from "vitest";
import schema from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🕰️host-temporal-format/🧬️schema/🔣️.json";
import fixture from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🕰️host-temporal-format/🧫️fixtures/🔣️.json";
import {
  HOST_TEMPORAL_DATE_OPTIONS,
  HOST_TEMPORAL_DATE_TIME_OPTIONS,
  HOST_TEMPORAL_TIME_OPTIONS,
  formatHostTemporalValuesV1,
  hostTemporalFormatReplyMatchesV1,
  isHostTemporalIsoV1,
  type HostTemporalFormatReplyV1,
  type HostTemporalValueV1,
} from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🕰️host-temporal-format/🟦️.ts";
import { createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";

function relativeOracle(date: Date, nowMs: number, locale: string): string {
  const seconds = (date.valueOf() - nowMs) / 1_000;
  const absolute = Math.abs(seconds);
  const [amount, unit]: readonly [number, Intl.RelativeTimeFormatUnit] =
    absolute < 60
      ? [Math.round(seconds), "second"]
      : absolute < 3_600
        ? [Math.round(seconds / 60), "minute"]
        : absolute < 86_400
          ? [Math.round(seconds / 3_600), "hour"]
          : absolute < 2_592_000
            ? [Math.round(seconds / 86_400), "day"]
            : absolute < 31_536_000
              ? [Math.round(seconds / 2_592_000), "month"]
              : [Math.round(seconds / 31_536_000), "year"];
  return new Intl.RelativeTimeFormat(locale, { numeric: "always" }).format(amount, unit);
}

function canonicalIsoOracle(value: string): boolean {
  const structural = new RegExp(schema.$defs.CanonicalIso.pattern).exec(value);
  if (!structural) return false;
  const [year, month, day] = value.slice(0, 10).split("-").map(Number) as [number, number, number];
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const monthDays = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  if (month < 1 || month > 12 || day < 1 || day > monthDays[month - 1]!) return false;
  const time = /T(\d{2}):(\d{2})(?::(\d{2}))?(?:\.\d+)?(?:Z|[+-](\d{2}):(\d{2}))$/.exec(value);
  return !time || Number(time[1]) <= 23 && Number(time[2]) <= 59 && Number(time[3] ?? 0) <= 59 && Number(time[4] ?? 0) <= 23 && Number(time[5] ?? 0) <= 59;
}

function intlOracle(value: HostTemporalValueV1, nowMs: number, locale: string, timeZone: string): string {
  if (value.source.kind === "iso" && !canonicalIsoOracle(value.source.iso)) return value.source.iso;
  const date = new Date(value.source.kind === "epochMs" ? value.source.timestampMs : value.source.iso);
  if (Number.isNaN(date.valueOf())) return value.source.kind === "iso" ? value.source.iso : "Invalid Date";
  if (value.format === "relative") return relativeOracle(date, nowMs, locale);
  if (value.format === "time") return new Intl.DateTimeFormat(locale, { ...HOST_TEMPORAL_TIME_OPTIONS, timeZone }).format(date);
  return new Intl.DateTimeFormat(locale, { ...(value.format === "date" ? HOST_TEMPORAL_DATE_OPTIONS : HOST_TEMPORAL_DATE_TIME_OPTIONS), timeZone }).format(date);
}

describe("WGPU shared temporal host door", () => {
  it("matches browser Intl for EventFeed time and VFS date, datetime, relative, and invalid ISO values", () => {
    expect(new Ajv2020({ strict: true }).compile(schema)(fixture)).toBe(true);
    for (const value of fixture.isoGrammar.valid) {
      expect(canonicalIsoOracle(value)).toBe(true);
      expect(isHostTemporalIsoV1(value)).toBe(true);
    }
    for (const value of fixture.isoGrammar.invalid) {
      expect(canonicalIsoOracle(value)).toBe(false);
      expect(isHostTemporalIsoV1(value)).toBe(false);
    }
    for (const row of fixture.cases) {
      const request = { nowMs: row.nowMs, values: row.values as readonly HostTemporalValueV1[] };
      const reply = formatHostTemporalValuesV1(request, { profileRevision: 0, profileSignature: "" }, row.locales, row.timeZone);
      const oracle = request.values.map((value) => intlOracle(value, row.nowMs, row.locales[0]!, row.timeZone));
      expect(oracle).toEqual(row.expected);
      expect(reply.labels.map((label) => label.text)).toEqual(row.expected);
      expect(reply.profile.timeZone).toBe(row.timeZone);
      expect(hostTemporalFormatReplyMatchesV1(request, reply)).toBe(true);
    }
  });

  it("answers one strict generic batch through the page host door", async () => {
    const row = fixture.cases[0]!;
    const request = { op: "format-temporal-values" as const, nowMs: row.nowMs, values: row.values as readonly HostTemporalValueV1[] };
    const reply = JSON.parse(await createWgpuPageHostIo()(JSON.stringify(request), null)) as HostTemporalFormatReplyV1;
    const ajv = new Ajv2020({ strict: true }).addSchema(schema);
    expect(ajv.getSchema(schema.$id + "#/$defs/HostRequest")!(request)).toBe(true);
    expect(ajv.getSchema(schema.$id + "#/$defs/Reply")!(reply)).toBe(true);
    expect(hostTemporalFormatReplyMatchesV1(request, reply)).toBe(true);
  });

  it("keeps one revision while the resolved profile is unchanged", async () => {
    const hostIo = createWgpuPageHostIo();
    const row = fixture.cases[0]!;
    const request = JSON.stringify({ op: "format-temporal-values", nowMs: row.nowMs, values: [row.values[0]!] });
    const first = JSON.parse(await hostIo(request, null)) as HostTemporalFormatReplyV1;
    const second = JSON.parse(await hostIo(request, null)) as HostTemporalFormatReplyV1;
    expect(second.profile).toEqual(first.profile);
  });

  it("rejects partial and duplicate replies before publication", () => {
    const row = fixture.cases[0]!;
    const request = { nowMs: row.nowMs, values: row.values.slice(0, 2) as readonly HostTemporalValueV1[] };
    const reply = formatHostTemporalValuesV1(request, { profileRevision: 0, profileSignature: "" }, ["en-US"], "UTC");
    expect(hostTemporalFormatReplyMatchesV1(request, { ...reply, labels: reply.labels.slice(0, 1) })).toBe(false);
    expect(hostTemporalFormatReplyMatchesV1(request, { ...reply, labels: [reply.labels[0]!, reply.labels[0]!] })).toBe(false);
  });
});
