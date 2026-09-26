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

function intlOracle(value: HostTemporalValueV1, nowMs: number, locale: string, timeZone: string): string {
  const date = new Date(value.source.kind === "epochMs" ? value.source.timestampMs : value.source.iso);
  if (Number.isNaN(date.valueOf())) return value.source.kind === "iso" ? value.source.iso : "Invalid Date";
  if (value.format === "relative") return relativeOracle(date, nowMs, locale);
  if (value.format === "time") return new Intl.DateTimeFormat(locale, { ...HOST_TEMPORAL_TIME_OPTIONS, timeZone }).format(date);
  return new Intl.DateTimeFormat(locale, { ...(value.format === "date" ? HOST_TEMPORAL_DATE_OPTIONS : HOST_TEMPORAL_DATE_TIME_OPTIONS), timeZone }).format(date);
}

describe("WGPU shared temporal host door", () => {
  it("matches browser Intl for EventFeed time and VFS date, datetime, relative, and invalid ISO values", () => {
    expect(new Ajv2020({ strict: true }).compile(schema)(fixture)).toBe(true);
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
