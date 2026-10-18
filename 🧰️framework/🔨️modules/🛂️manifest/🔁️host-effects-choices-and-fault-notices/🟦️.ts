/** 🔁️ The host effect invocation, the artifact kind and surface app choices and the fault notice definitions with their validation and text. */
import { type ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import type { AppRole, AppRef } from "../🧬️schema/🟦️.ts";
import { SHELL_LOCALES, SHELL_TERMINOLOGIES, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "../🤖️generated/🎚️ui-axes/🟦️.ts";
import type { ActionInvocation } from "../🎬️action-types-and-semantics/🟦️.ts";
import type { CommandInvocation } from "../🔘️definitions-action-ids-and-shell-brand/🟦️.ts";
import type { PluginManifest } from "../🗣️contributions-and-examples/🟦️.ts";

// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
//#region 🔁️HostEffectInvocation
/** 🔁️ The scope a host effect's re-dispatch is addressed in — the fields both invocation shapes need,
 * read off whatever view state the host holds. */
export type HostEffectDispatchScope = {
  readonly pluginId: string;
  readonly appId: string;
  readonly modeId: string;
  readonly windowKindId: string;
  readonly windowInstanceId: string;
};

/** 🔁️ The ONE rule that decides which channel a guest's `dispatchAction` host effect re-enters, and
 * the invocation it re-enters with.
 *
 * ⚖️ An id the app DECLARES as a command re-enters the typed command channel; everything else — every
 * framework-reserved verb, every window action — re-enters the scoped ACTION channel. Addressing a
 * reserved verb as an app-owned command is refused by the guest's own ownership gate
 * (`dispatch_command`, `💻️os/🔨️modules/🔌️plugin/🦀️.rs`: `command '<id>' is not owned by app <app>`), and
 * a refused re-arm is a dropped continuation: the wgpu bridge addressed EVERY re-arm as an app
 * command, so `toolRunStart` was refused at boot and the preview evaluation's `ToolRunView` never
 * reached the guest's own surface — leaving `cancellable: false` and no Cancel affordance for the
 * whole of a computation React offers one for at t=5.3 s
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`).
 *
 * Fixture: `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔁️host-effect-invocation/🔣️.json`. */
export function hostEffectInvocationV1(
  scope: HostEffectDispatchScope,
  appCommandIds: readonly string[],
  action: string,
  args?: Record<string, unknown>,
): { readonly kind: "command"; readonly invocation: CommandInvocation } | { readonly kind: "action"; readonly invocation: ActionInvocation } {
  if (appCommandIds.includes(action)) {
    return { kind: "command", invocation: { address: { owner: { app: { pluginId: scope.pluginId, appId: scope.appId } }, commandId: action }, arguments: { ...args } } as CommandInvocation };
  }
  const invocation = {
    address: { pluginId: scope.pluginId, appId: scope.appId, modeId: scope.modeId, windowKindId: scope.windowKindId, windowInstanceId: scope.windowInstanceId, actionId: action },
    arguments: { ...args, windowId: scope.windowInstanceId },
  } as ActionInvocation;
  return { kind: "action", invocation };
}

/** 🔁️ Every command id one app of `manifest` declares — the left-hand side of
 * {@link hostEffectInvocationV1}'s rule, read off the manifest a host already holds. */
export function appCommandIdsV1(manifest: PluginManifest, appId: string): readonly string[] {
  const app = manifest.apps.find((entry) => (entry as { readonly id?: unknown }).id === appId) as { readonly commands?: readonly { readonly id?: unknown }[] } | undefined;
  return (app?.commands ?? []).map((command) => String(command.id ?? "")).filter((id) => id.length > 0);
}

//#endregion 🔁️HostEffectInvocation
//#region 🔖️HostResolvedArgs
/** 🗂️ TS twin of Rust `ArtifactKindChoice` — one artifact-kind choice offered by an
 * `ActionArgControl.artifactKind` dialog field, resolved by the host from its live plugin catalogue
 * (`artifactKindChoices`) into a plain `select` control right before the dialog renders. Round-trips
 * through `ActionArgOption.value` as JSON via `encodeArtifactKindChoice`/`decodeArtifactKindChoice` —
 * the frozen wire shape (contract §C8.1): `{"kindId":"s.draw.draw","schema":"draw.document","dialect":
 * {"artifactKind":"s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}`.
 * Rust twin: `ArtifactKindChoice` (`🦀️.rs`) — both codecs must agree byte-for-byte over the
 * pinned fixtures. */
export type ArtifactKindChoice = {
  readonly kindId: string;
  readonly schema: string;
  readonly dialect: ArtifactDialect;
  readonly label: { readonly en: string; readonly de: string };
};

/** 🎭️ TS twin of Rust `SurfaceAppChoice` — one `(pluginId, appId, role)` choice offered by an
 * `ActionArgControl.surfaceApp` dialog field, resolved by the host against the dialect coordinate
 * found in the dialog's seed argument named `dialectArg`. Round-trips through `ActionArgOption.value`
 * as JSON via `encodeSurfaceAppChoice`/`decodeSurfaceAppChoice`. Rust twin: `SurfaceAppChoice`
 * (`🦀️.rs`). */
export type SurfaceAppChoice = {
  readonly app: AppRef;
  readonly role: AppRole;
};

/** 🧵️ Encodes an `ArtifactKindChoice` into the frozen `ActionArgOption.value` JSON shape — key order
 * (kindId, schema, dialect, label.en, label.de) matches Rust `encode_artifact_kind_choice`'s
 * `serde_json::json!` insertion order byte-for-byte. */
export function encodeArtifactKindChoice(choice: ArtifactKindChoice): string {
  return JSON.stringify({
    kindId: choice.kindId,
    schema: choice.schema,
    dialect: choice.dialect,
    label: { en: choice.label.en, de: choice.label.de },
  });
}

/** 🧵️ Inverse of {@link encodeArtifactKindChoice}. Throws with a message naming the missing/malformed
 * field, mirroring Rust `decode_artifact_kind_choice`'s `Result<_, String>` messages. */
export function decodeArtifactKindChoice(value: string): ArtifactKindChoice {
  const json = JSON.parse(value) as Record<string, unknown>;
  if (typeof json.kindId !== "string") throw new Error("artifact kind choice missing string field kindId");
  if (typeof json.schema !== "string") throw new Error("artifact kind choice missing string field schema");
  const dialect = json.dialect as Partial<ArtifactDialect> | undefined;
  if (typeof dialect?.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") {
    throw new Error("artifact kind choice missing field dialect");
  }
  const label = json.label as { readonly en?: unknown; readonly de?: unknown } | undefined;
  if (typeof label?.en !== "string") throw new Error("artifact kind choice missing string field label.en");
  if (typeof label.de !== "string") throw new Error("artifact kind choice missing string field label.de");
  return { kindId: json.kindId, schema: json.schema, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, label: { en: label.en, de: label.de } };
}

/** 🧵️ Encodes a `SurfaceAppChoice` into its frozen `ActionArgOption.value` JSON shape — must agree
 * byte-for-byte with Rust `encode_surface_app_choice`. */
export function encodeSurfaceAppChoice(choice: SurfaceAppChoice): string {
  return JSON.stringify({ pluginId: choice.app.pluginId, appId: choice.app.appId, role: choice.role });
}

/** 🧵️ Inverse of {@link encodeSurfaceAppChoice}. */
export function decodeSurfaceAppChoice(value: string): SurfaceAppChoice {
  const json = JSON.parse(value) as Record<string, unknown>;
  if (typeof json.pluginId !== "string") throw new Error("surface app choice missing string field pluginId");
  if (typeof json.appId !== "string") throw new Error("surface app choice missing string field appId");
  if (json.role !== "editor" && json.role !== "viewer") throw new Error("surface app choice missing string field role");
  return { app: { pluginId: json.pluginId, appId: json.appId }, role: json.role };
}

/** 🗺️ Resolves an app's manifest `label` field's native (terminology-invariant) cell — the wire shape
 * is `{ native: { en, de }, reuse: { en, de } }` (Rust `LocalizedLabel`'s `Serialize`, see
 * `🛠️ShellHelpers/🏷️labels/🟦️.tsx`'s `resolveManifestLabel` for the full terminology-aware resolver
 * used at render time). `artifactKindChoices` only ever needs the native cell, matching Rust
 * `encode_artifact_kind_choice` resolving under `Terminology::Native`. Takes `unknown` because
 * `AppDefinition.label` is `unknown` on the generated type (no owned schema mirror for `LocalizedLabel` yet). */
function resolveNativeLabel(label: unknown): { readonly en: string; readonly de: string } {
  const native = (label as { readonly native?: { readonly en?: string; readonly de?: string } } | undefined)?.native;
  return { en: native?.en ?? "", de: native?.de ?? "" };
}

/** 🗂️ Every artifact-kind choice for the given `roles` — TS twin of Rust `artifact_kind_choices`.
 * Every app across `manifests` whose `role` is in `roles` and whose `io.artifactSchema` is non-empty
 * and whose package declares an artifact kind of that schema (on any of its apps — a viewer shares its
 * editor's kind — or on the manifest) contributes one choice per dialect coordinate, labelled with that
 * KIND's own label, never its app's label (every editor app is labelled "Editor"). An app whose schema
 * names no declared kind (a per-user Home, a studio) is not a creatable kind. Deduped by dialect
 * coordinate (first manifest/app wins — callers pass owner manifests first so the owner's label wins
 * over a later contributor's), sorted by coordinate for determinism — the pure resolver behind
 * `ActionArgControl.artifactKind`. */
export function artifactKindChoices(manifests: readonly { readonly apps: readonly unknown[]; readonly artifactKinds?: readonly unknown[] }[], roles: readonly AppRole[]): ArtifactKindChoice[] {
  const byCoordinate = new Map<string, ArtifactKindChoice>();
  for (const manifest of manifests) {
    for (const raw of manifest.apps) {
      const app = raw as unknown as { readonly role: AppRole; readonly dialect: ArtifactDialect; readonly label: unknown; readonly io: { readonly artifactSchema: string }; readonly artifactKinds?: readonly unknown[] };
      if (!roles.includes(app.role) || app.io.artifactSchema === "") continue;
      const coordinate = `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`;
      if (byCoordinate.has(coordinate)) continue;
      const kinds = [...manifest.apps.flatMap((other) => (other as { readonly artifactKinds?: readonly unknown[] }).artifactKinds ?? []), ...(manifest.artifactKinds ?? [])] as readonly { readonly schema?: string; readonly label?: unknown }[];
      const kind = kinds.find((candidate) => candidate.schema === app.io.artifactSchema);
      if (kind === undefined) continue;
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.artifactSchema, dialect: app.dialect, label: resolveNativeLabel(kind.label) });
    }
  }
  return [...byCoordinate.keys()].sort().map((coordinate) => byCoordinate.get(coordinate)!);
}

//#endregion 🔖️HostResolvedArgs
//#region 🔖️FaultNotices
/** 📢️ One app-declared fault notice (design §20.12) — TS twin of Rust `FaultNoticeDefinition`: the code a guest refusal carries
 * (`<app>.<area>.<name>`) and its text in every terminology × locale cell, `{name}` placeholders filled from `Fault.params`. */
export type FaultNoticeDefinition = { readonly code: string; readonly label: LocalizedLabel };

/** 🚫️ One refusal of a fault notice table — TS twin of Rust `FaultNoticeError` (corpus `🧫️fixtures/🧫️fault-notices`). */
export type FaultNoticeError =
  | { readonly rule: "syntax" | "duplicate"; readonly code: string }
  | { readonly rule: "emptyText" | "placeholder"; readonly code: string; readonly terminology: ShellTerminology; readonly locale: ShellLocale };

const FAULT_NOTICE_CODE = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*(?:\.[a-z][a-z0-9]*(?:-[a-z0-9]+)*){2,}$/u;
const FAULT_PARAM_NAME = /^[a-z][A-Za-z0-9]*$/u;

/** 🔤️ Whether `code` is an app notice code (three or more kebab segments) — TS twin of Rust `is_fault_notice_code`. */
export function isFaultNoticeCode(code: string): boolean {
  return FAULT_NOTICE_CODE.test(code);
}

/** 🧩️ The sorted, unique `{name}` placeholders of one notice text, `null` for an unbalanced brace or an invalid name — TS twin of
 * Rust `fault_notice_placeholders`. */
export function faultNoticePlaceholders(text: string): readonly string[] | null {
  const names = new Set<string>();
  let rest = text;
  for (let open = rest.search(/[{}]/u); open >= 0; open = rest.search(/[{}]/u)) {
    if (rest[open] === "}") return null;
    const tail = rest.slice(open + 1);
    const close = tail.search(/[{}]/u);
    if (close < 0 || tail[close] !== "}" || !FAULT_PARAM_NAME.test(tail.slice(0, close))) return null;
    names.add(tail.slice(0, close));
    rest = tail.slice(close + 1);
  }
  return [...names].sort();
}

/** ✅️ Every refusal of a published fault notice table, in table order — TS twin of Rust `validate_fault_notices`: code grammar,
 * unique codes, a non-blank text in every cell, one placeholder set across a notice's cells. Empty = valid. */
export function validateFaultNotices(notices: readonly FaultNoticeDefinition[]): FaultNoticeError[] {
  const errors: FaultNoticeError[] = [];
  notices.forEach((notice, index) => {
    if (!isFaultNoticeCode(notice.code)) errors.push({ rule: "syntax", code: notice.code });
    if (notices.slice(0, index).some((earlier) => earlier.code === notice.code)) errors.push({ rule: "duplicate", code: notice.code });
    const first = faultNoticePlaceholders(notice.label[SHELL_TERMINOLOGIES[0]][SHELL_LOCALES[0]]);
    for (const terminology of SHELL_TERMINOLOGIES) {
      for (const locale of SHELL_LOCALES) {
        const text = notice.label[terminology][locale];
        if (text.trim() === "") errors.push({ rule: "emptyText", code: notice.code, terminology, locale });
        else if (first === null || faultNoticePlaceholders(text)?.join("\u0000") !== first.join("\u0000")) errors.push({ rule: "placeholder", code: notice.code, terminology, locale });
      }
    }
  });
  return errors;
}

/** 🧵️ `text` with each `{name}` replaced by the value `params` names; `null` when one is missing or the text is malformed — TS twin
 * of Rust `fill_fault_notice` (a notice is never shown half-filled). */
export function fillFaultNotice(text: string, params: Readonly<Record<string, string>> | undefined): string | null {
  let filled = "";
  let rest = text;
  for (let open = rest.indexOf("{"); open >= 0; open = rest.indexOf("{")) {
    const close = rest.indexOf("}", open);
    if (close < 0) return null;
    const value = params !== undefined && Object.hasOwn(params, rest.slice(open + 1, close)) ? params[rest.slice(open + 1, close)] : undefined;
    if (value === undefined) return null;
    filled += rest.slice(0, open) + value;
    rest = rest.slice(close + 1);
  }
  return filled + rest;
}

/** 🗣️ The notice text `code` names in `notices` for one shell axis pair, filled from `params`; `null` when undeclared or unfillable —
 * TS twin of Rust `fault_notice_text`. */
export function faultNoticeText(notices: readonly FaultNoticeDefinition[], code: string, params: Readonly<Record<string, string>> | undefined, terminology: ShellTerminology, locale: ShellLocale): string | null {
  const notice = notices.find((candidate) => candidate.code === code);
  return notice === undefined ? null : fillFaultNotice(notice.label[terminology][locale], params);
}
//#endregion 🔖️FaultNotices
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
