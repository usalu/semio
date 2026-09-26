/** 🕰️ Host-owned temporal presentation shared by renderer surfaces. */
export const HOST_TEMPORAL_FORMAT_MAX_VALUES = 64;

export type HostHourCycleV1 = "h11" | "h12" | "h23" | "h24";
export type HostTemporalFormatV1 = "time" | "date" | "dateTime" | "relative";
export type HostTemporalSourceV1 =
  | { readonly kind: "epochMs"; readonly timestampMs: number }
  | { readonly kind: "iso"; readonly iso: string };
export interface HostTemporalValueV1 {
  readonly id: string;
  readonly source: HostTemporalSourceV1;
  readonly format: HostTemporalFormatV1;
}
export interface HostTemporalFormatRequestV1 {
  readonly nowMs: number;
  readonly values: readonly HostTemporalValueV1[];
}
export interface HostTemporalProfileV1 {
  readonly locale: string;
  readonly timeZone: string;
  readonly hourCycle: HostHourCycleV1;
  readonly profileRevision: number;
}
export interface HostTemporalLabelV1 {
  readonly id: string;
  readonly text: string;
}
export interface HostTemporalFormatReplyV1 {
  readonly profile: HostTemporalProfileV1;
  readonly labels: readonly HostTemporalLabelV1[];
}
export interface HostTemporalFormatPageStateV1 {
  profileRevision: number;
  profileSignature: string;
}

export const HOST_TEMPORAL_TIME_OPTIONS = { hour: "2-digit", minute: "2-digit", second: "2-digit" } as const;
export const HOST_TEMPORAL_DATE_OPTIONS = { year: "numeric", month: "2-digit", day: "2-digit" } as const;
export const HOST_TEMPORAL_DATE_TIME_OPTIONS = { ...HOST_TEMPORAL_DATE_OPTIONS, hour: "2-digit", minute: "2-digit" } as const;

function temporalValuesV1(request: HostTemporalFormatRequestV1): readonly HostTemporalValueV1[] {
  if (!Number.isSafeInteger(request.nowMs)) throw new Error("host-temporal-format.now-invalid");
  if (request.values.length === 0 || request.values.length > HOST_TEMPORAL_FORMAT_MAX_VALUES) throw new Error("host-temporal-format.values-capacity");
  const ids = new Set<string>();
  for (const value of request.values) {
    if (!value.id || value.id.length > 256 || ids.has(value.id)) throw new Error("host-temporal-format.id-invalid");
    ids.add(value.id);
    if (!["time", "date", "dateTime", "relative"].includes(value.format)) throw new Error("host-temporal-format.format-invalid");
    if (value.source.kind === "epochMs") {
      if (!Number.isSafeInteger(value.source.timestampMs)) throw new Error("host-temporal-format.epoch-invalid");
    } else if (value.source.kind === "iso") {
      if (!value.source.iso || value.source.iso.length > 256) throw new Error("host-temporal-format.iso-invalid");
    } else {
      throw new Error("host-temporal-format.source-invalid");
    }
  }
  return request.values;
}

const HOST_TEMPORAL_CANONICAL_ISO = /^(\d{4})-(\d{2})-(\d{2})(?:$|T(\d{2}):(\d{2})(?::(\d{2})(?:\.(\d+))?)?(Z|[+-](\d{2}):(\d{2})))$/;

/** 🧭 Accepts only the cross-platform ISO subset owned by the host temporal contract. */
export function isHostTemporalIsoV1(value: string): boolean {
  const match = HOST_TEMPORAL_CANONICAL_ISO.exec(value);
  if (!match) return false;
  const year = Number(match[1]);
  const month = Number(match[2]);
  const day = Number(match[3]);
  const hour = match[4] === undefined ? 0 : Number(match[4]);
  const minute = match[5] === undefined ? 0 : Number(match[5]);
  const second = match[6] === undefined ? 0 : Number(match[6]);
  const offsetHour = match[9] === undefined ? 0 : Number(match[9]);
  const offsetMinute = match[10] === undefined ? 0 : Number(match[10]);
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const monthDays = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  return month >= 1 && month <= 12 && day >= 1 && day <= monthDays[month - 1]! && hour <= 23 && minute <= 59 && second <= 59 && offsetHour <= 23 && offsetMinute <= 59;
}

function sourceDate(source: HostTemporalSourceV1): Date {
  if (source.kind === "iso" && !isHostTemporalIsoV1(source.iso)) return new Date(Number.NaN);
  return new Date(source.kind === "epochMs" ? source.timestampMs : source.iso);
}

function invalidTemporalText(source: HostTemporalSourceV1): string {
  return source.kind === "iso" ? source.iso : "Invalid Date";
}

function relativeValue(timestampMs: number, nowMs: number): readonly [number, Intl.RelativeTimeFormatUnit] {
  const seconds = (timestampMs - nowMs) / 1_000;
  const absoluteSeconds = Math.abs(seconds);
  if (absoluteSeconds < 60) return [Math.round(seconds), "second"];
  if (absoluteSeconds < 3_600) return [Math.round(seconds / 60), "minute"];
  if (absoluteSeconds < 86_400) return [Math.round(seconds / 3_600), "hour"];
  if (absoluteSeconds < 2_592_000) return [Math.round(seconds / 86_400), "day"];
  if (absoluteSeconds < 31_536_000) return [Math.round(seconds / 2_592_000), "month"];
  return [Math.round(seconds / 31_536_000), "year"];
}

/** 🧭 Formats one instant through the same Intl primitives that React uses. */
export function formatHostTemporalValueV1(
  value: HostTemporalValueV1,
  nowMs: number,
  locales: readonly string[] = [],
  timeZone?: string,
): string {
  const date = sourceDate(value.source);
  if (Number.isNaN(date.valueOf())) return invalidTemporalText(value.source);
  const zone = timeZone ? { timeZone } : {};
  try {
    if (value.format === "relative") {
      const [amount, unit] = relativeValue(date.valueOf(), nowMs);
      return new Intl.RelativeTimeFormat(locales as string[], { numeric: "always" }).format(amount, unit);
    }
    if (value.format === "time") return date.toLocaleTimeString(locales as string[], { ...HOST_TEMPORAL_TIME_OPTIONS, ...zone });
    const options = value.format === "date" ? HOST_TEMPORAL_DATE_OPTIONS : HOST_TEMPORAL_DATE_TIME_OPTIONS;
    return new Intl.DateTimeFormat(locales as string[], { ...options, ...zone }).format(date);
  } catch {
    return invalidTemporalText(value.source);
  }
}

/** 🧭 Formats a bounded batch and advances the host profile revision when Intl's resolved profile changes. */
export function formatHostTemporalValuesV1(
  request: HostTemporalFormatRequestV1,
  state: HostTemporalFormatPageStateV1,
  locales: readonly string[] = [],
  timeZone?: string,
): HostTemporalFormatReplyV1 {
  const values = temporalValuesV1(request);
  const resolved = new Intl.DateTimeFormat(locales as string[], { ...HOST_TEMPORAL_TIME_OPTIONS, ...(timeZone ? { timeZone } : {}) }).resolvedOptions();
  const hourCycle = resolved.hourCycle;
  if (hourCycle !== "h11" && hourCycle !== "h12" && hourCycle !== "h23" && hourCycle !== "h24") throw new Error("host-temporal-format.hour-cycle-invalid");
  const profileSignature = resolved.locale + "\u0000" + resolved.timeZone + "\u0000" + hourCycle;
  if (state.profileSignature !== profileSignature) {
    state.profileSignature = profileSignature;
    state.profileRevision = Math.max(1, state.profileRevision + 1);
  }
  return {
    profile: { locale: resolved.locale, timeZone: resolved.timeZone, hourCycle, profileRevision: state.profileRevision },
    labels: values.map((value) => ({ id: value.id, text: formatHostTemporalValueV1(value, request.nowMs, locales, timeZone) })),
  };
}

/** 🛡️ Refuses partial, duplicate, over-capacity, or mismatched replies before a renderer cache can own them. */
export function hostTemporalFormatReplyMatchesV1(request: HostTemporalFormatRequestV1, reply: HostTemporalFormatReplyV1): boolean {
  try {
    const requested = temporalValuesV1(request);
    if (!Number.isSafeInteger(reply.profile.profileRevision) || reply.profile.profileRevision < 1 || !reply.profile.locale || !reply.profile.timeZone) return false;
    if (!["h11", "h12", "h23", "h24"].includes(reply.profile.hourCycle)) return false;
    if (reply.labels.length !== requested.length || reply.labels.some((label) => !label.text || label.text.length > 256)) return false;
    const labels = new Map(reply.labels.map((label) => [label.id, label.text]));
    return labels.size === requested.length && requested.every((value) => labels.has(value.id));
  } catch {
    return false;
  }
}
