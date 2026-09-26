/** 🔀️ The navbar's two switching axes as laws over ONE language-neutral fixture
 * (`🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`).
 *
 * 🏁️ What these encode: `http://127.0.0.1:6018/?plugin=generation3d` always booted the editor and the
 * playground had no control to reach `s.procedural.generation3d@1/*#viewer` at all — the viewer window
 * was unreachable by any in-app action, and mode switching was mouse-only
 * (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 2, §4 P1 item 5). The fix is three units: the
 * `?role=` boot axis, a role-aware primary-app resolution that lets the role win over the playground's
 * app pin, and an in-place session switch that RETIRES its predecessor.
 *
 * ⚖️ Every row runs at least twice: through the shipped modules and through an independent in-file
 * oracle; the pure resolution laws additionally run in a bare `node` process over the shipped source
 * text (the twin) — no vitest, no bundler alias, no jsdom. */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "vitest";
import Ajv from "ajv";
import { SHELL_KEYBINDINGS, ariaKeyshortcutsText, composeControlKeybindings, formatKeybindingShortcut } from "@semio-tech/ui-react";
import { keyboardEventMatchesOwnedHotkey, parseOwnedHotkeyChords } from "../../../../../../../🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import type { AppRole, ArtifactDialect } from "@semio-tech/framework";
import { resolveBootQueryAppRole } from "../../../../🧑‍💻dev/🔗️boot-query/🟦️.ts";
import {
  MODE_STEP_CONTROL_IDS,
  SEALED_INSTANCE_DROP_CODE,
  SURFACE_ROLE_CONTROL_IDS,
  SURFACE_ROLE_ORDER,
  createSealedInstanceLedgerV1,
  createSessionAppSwitchGateV1,
  createSessionWorkLedgerV1,
  createShellSessionLaneV1,
  shellIdentityResolutionV1,
  shellRouteAdmissionTextV1,
  shellRouteAdmissionV1,
  shellRouteIsOverlayV1,
  shellSessionRouteV1,
  quiesceSessionWorkV1,
  resolveBootPrimaryAppV1,
  roleSwitchTargetV1,
  runSessionAppSwitchV1,
  sealedInstanceDropTextV1,
  sealedInstanceDropV1,
  sessionInstanceKeyV1,
  stepModeIdV1,
  surfaceRoleAppsV1,
  surfaceSwitchBusyTextV1,
  type RoleSurfaceAppV1,
  type ShellIdentityResolutionV1,
  type ShellRouteAdmissionV1,
  type SessionAppSwitchQuiesceV1,
  type SessionAppSwitchSessionV1,
  type SessionAppSwitchStatusV1,
  type SessionAppSwitchStepV1,
  type SessionWorkKindV1,
} from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import fixtureJson from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json";
import sessionLaneFixtureJson from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🧭️session-lane/🔣️.json";
import pLimit from "p-limit";
import { match } from "path-to-regexp";
import { createActor, setup } from "xstate";

type SurfaceSwitchFixture = {
  readonly note: string;
  readonly dialects: Readonly<Record<string, ArtifactDialect>>;
  readonly manifests: Readonly<Record<string, readonly { readonly id: string; readonly role: AppRole; readonly dialect?: string }[]>>;
  readonly boot: readonly { readonly id: string; readonly manifest: string; readonly search: string; readonly envRole: AppRole; readonly defaultAppId: string | null; readonly pinnedAppId: string | null; readonly expectedRole: AppRole; readonly expectedAppId: string | null }[];
  readonly group: readonly { readonly id: string; readonly manifest: string; readonly dialect: string | null; readonly expected: { readonly editor: string; readonly viewer: string } | null }[];
  readonly roleTargets: readonly { readonly id: string; readonly manifest: string; readonly dialect: string; readonly currentRole: AppRole; readonly requested: AppRole; readonly expected: string | null }[];
  readonly switch: readonly {
    readonly id: string;
    readonly manifest: string;
    readonly session: { readonly pluginId: string; readonly instanceId: number; readonly appId: string } | null;
    readonly request: { readonly pluginId: string; readonly appId: string; readonly viewState: boolean };
    readonly quiesce: SessionAppSwitchQuiesceV1;
    readonly createFails: boolean;
    readonly retireMode: SwitchRetireModeV1;
    readonly expected: {
      readonly status: SessionAppSwitchStatusV1 | "create-failed";
      readonly steps: readonly string[];
      readonly trace: readonly SessionAppSwitchStepV1[];
      readonly sessionAppId: string | null;
      readonly instanceId: number | null;
      readonly created: number;
      readonly retired: readonly number[];
      readonly pending: number;
      readonly retireFailures: number;
    };
  }[];
  readonly gate: {
    readonly note: string;
    readonly first: { readonly appId: string; readonly status: SessionAppSwitchStatusV1 };
    readonly second: { readonly appId: string; readonly status: SessionAppSwitchStatusV1 };
    readonly expectedRetired: readonly number[];
    readonly expectedCreated: number;
  };
  readonly work: readonly {
    readonly id: string;
    readonly begin: readonly (readonly [string, number, SessionWorkKindV1])[];
    readonly release: readonly number[];
    readonly pending: readonly (readonly [string, number, number])[];
    readonly total: number;
  }[];
  readonly quiesce: readonly {
    readonly id: string;
    readonly pending: readonly number[];
    readonly budgetMs: number;
    readonly pollMs: number;
    readonly expected: { readonly settled: boolean; readonly pending: number; readonly sleeps: number };
  }[];
  readonly sealed: {
    readonly slots: number;
    readonly seal: readonly (readonly [string, number])[];
    readonly expectedSize: number;
    readonly expectedSealed: readonly (readonly [string, number, boolean])[];
    readonly unseal: { readonly note: string; readonly target: readonly [string, number]; readonly sealedAfterUnseal: boolean; readonly sizeAfterUnseal: number };
    readonly drop: { readonly pluginId: string; readonly instanceId: number; readonly what: string; readonly detail: string; readonly code: string; readonly text: string };
    readonly dropWithoutDetail: { readonly pluginId: string; readonly instanceId: number; readonly what: string; readonly text: string };
  };
  readonly busyLabel: readonly { readonly locale: string; readonly text: string }[];
  readonly modeSteps: readonly { readonly id: string; readonly modeIds: readonly string[]; readonly activeModeId: string; readonly step: 1 | -1; readonly expected: string | null }[];
  readonly keybindings: readonly {
    readonly controlId: string;
    readonly chord: string;
    readonly aria: string;
    readonly ariaApple: string;
    readonly badge: string;
    readonly badgeApple: string;
    readonly event: { readonly key: string; readonly ctrlKey: boolean; readonly altKey: boolean; readonly metaKey: boolean; readonly shiftKey: boolean };
    readonly dispatches: { readonly requested: AppRole; readonly currentRole: AppRole; readonly expectedAppId: string } | null;
  }[];
  readonly keybindingOverride: { readonly controlId: string; readonly keys: string; readonly aria: string; readonly ariaApple: string; readonly badge: string; readonly badgeApple: string };
};

const fixture = fixtureJson as unknown as SurfaceSwitchFixture;

const FIXTURE_SCHEMA: Record<string, unknown> = {
  type: "object",
  additionalProperties: false,
  required: ["note", "dialects", "manifests", "boot", "group", "roleTargets", "switch", "gate", "work", "quiesce", "sealed", "busyLabel", "modeSteps", "keybindings", "keybindingOverride"],
  properties: {
    note: { type: "string", minLength: 1 },
    dialects: { type: "object", additionalProperties: { type: "object", additionalProperties: false, required: ["artifactKind", "standard", "subset"], properties: { artifactKind: { type: "string" }, standard: { type: "string" }, subset: { type: "string" } } } },
    manifests: {
      type: "object",
      additionalProperties: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "role"], properties: { id: { type: "string" }, role: { enum: ["editor", "viewer"] }, dialect: { type: "string" } } } },
    },
    boot: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "manifest", "search", "envRole", "defaultAppId", "pinnedAppId", "expectedRole", "expectedAppId"],
        properties: { id: { type: "string" }, manifest: { type: "string" }, search: { type: "string" }, envRole: { enum: ["editor", "viewer"] }, defaultAppId: { type: ["string", "null"] }, pinnedAppId: { type: ["string", "null"] }, expectedRole: { enum: ["editor", "viewer"] }, expectedAppId: { type: ["string", "null"] } },
      },
    },
    group: {
      type: "array",
      minItems: 1,
      items: { type: "object", additionalProperties: false, required: ["id", "manifest", "dialect", "expected"], properties: { id: { type: "string" }, manifest: { type: "string" }, dialect: { type: ["string", "null"] }, expected: { type: ["object", "null"] } } },
    },
    roleTargets: {
      type: "array",
      minItems: 1,
      items: { type: "object", additionalProperties: false, required: ["id", "manifest", "dialect", "currentRole", "requested", "expected"], properties: { id: { type: "string" }, manifest: { type: "string" }, dialect: { type: "string" }, currentRole: { enum: ["editor", "viewer"] }, requested: { enum: ["editor", "viewer"] }, expected: { type: ["string", "null"] } } },
    },
    switch: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "manifest", "session", "request", "quiesce", "createFails", "retireMode", "expected"],
        properties: {
          id: { type: "string" },
          manifest: { type: "string" },
          session: { type: ["object", "null"] },
          request: { type: "object" },
          quiesce: { type: "object", additionalProperties: false, required: ["settled", "pending"], properties: { settled: { type: "boolean" }, pending: { type: "integer", minimum: 0 } } },
          createFails: { type: "boolean" },
          retireMode: { enum: ["resolve", "reject", "never"] },
          expected: {
            type: "object",
            additionalProperties: false,
            required: ["status", "steps", "trace", "sessionAppId", "instanceId", "created", "retired", "pending", "retireFailures"],
            properties: {
              status: { enum: ["switched", "mounted", "republished", "unchanged", "unresolvable", "busy", "draining", "create-failed"] },
              steps: { type: "array", items: { type: "string" } },
              trace: { type: "array", items: { enum: ["quiesce", "seal", "unseal", "create", "create-failed", "retire-started", "retire", "retire-failed", "publish", "seed", "refresh"] } },
              sessionAppId: { type: ["string", "null"] },
              instanceId: { type: ["integer", "null"] },
              created: { type: "integer", minimum: 0 },
              retired: { type: "array", items: { type: "integer" } },
              pending: { type: "integer", minimum: 0 },
              retireFailures: { type: "integer", minimum: 0 },
            },
          },
        },
      },
    },
    gate: {
      type: "object",
      additionalProperties: false,
      required: ["note", "first", "second", "expectedRetired", "expectedCreated"],
      properties: {
        note: { type: "string", minLength: 1 },
        first: { type: "object", additionalProperties: false, required: ["appId", "status"], properties: { appId: { type: "string" }, status: { type: "string" } } },
        second: { type: "object", additionalProperties: false, required: ["appId", "status"], properties: { appId: { type: "string" }, status: { type: "string" } } },
        expectedRetired: { type: "array", items: { type: "integer" } },
        expectedCreated: { type: "integer", minimum: 0 },
      },
    },
    work: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "begin", "release", "pending", "total"],
        properties: {
          id: { type: "string" },
          begin: { type: "array", items: { type: "array", minItems: 3, maxItems: 3 } },
          release: { type: "array", items: { type: "integer", minimum: 0 } },
          pending: { type: "array", items: { type: "array", minItems: 3, maxItems: 3 } },
          total: { type: "integer", minimum: 0 },
        },
      },
    },
    quiesce: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "pending", "budgetMs", "pollMs", "expected"],
        properties: {
          id: { type: "string" },
          pending: { type: "array", minItems: 1, items: { type: "integer", minimum: 0 } },
          budgetMs: { type: "integer", minimum: 0 },
          pollMs: { type: "integer", minimum: 1 },
          expected: { type: "object", additionalProperties: false, required: ["settled", "pending", "sleeps"], properties: { settled: { type: "boolean" }, pending: { type: "integer", minimum: 0 }, sleeps: { type: "integer", minimum: 0 } } },
        },
      },
    },
    sealed: {
      type: "object",
      additionalProperties: false,
      required: ["slots", "seal", "expectedSize", "expectedSealed", "unseal", "drop", "dropWithoutDetail"],
      properties: {
        slots: { type: "integer", minimum: 1 },
        seal: { type: "array", minItems: 1, items: { type: "array", minItems: 2, maxItems: 2 } },
        expectedSize: { type: "integer", minimum: 0 },
        expectedSealed: { type: "array", minItems: 1, items: { type: "array", minItems: 3, maxItems: 3 } },
        unseal: { type: "object", additionalProperties: false, required: ["note", "target", "sealedAfterUnseal", "sizeAfterUnseal"], properties: { note: { type: "string", minLength: 1 }, target: { type: "array", minItems: 2, maxItems: 2 }, sealedAfterUnseal: { type: "boolean" }, sizeAfterUnseal: { type: "integer", minimum: 0 } } },
        drop: { type: "object", additionalProperties: false, required: ["pluginId", "instanceId", "what", "detail", "code", "text"], properties: { pluginId: { type: "string" }, instanceId: { type: "integer" }, what: { type: "string" }, detail: { type: "string" }, code: { type: "string" }, text: { type: "string" } } },
        dropWithoutDetail: { type: "object", additionalProperties: false, required: ["pluginId", "instanceId", "what", "text"], properties: { pluginId: { type: "string" }, instanceId: { type: "integer" }, what: { type: "string" }, text: { type: "string" } } },
      },
    },
    busyLabel: {
      type: "array",
      minItems: 2,
      items: { type: "object", additionalProperties: false, required: ["locale", "text"], properties: { locale: { type: "string" }, text: { type: "string", minLength: 1 } } },
    },
    modeSteps: {
      type: "array",
      minItems: 1,
      items: { type: "object", additionalProperties: false, required: ["id", "modeIds", "activeModeId", "step", "expected"], properties: { id: { type: "string" }, modeIds: { type: "array", items: { type: "string" } }, activeModeId: { type: "string" }, step: { enum: [1, -1] }, expected: { type: ["string", "null"] } } },
    },
    keybindings: {
      type: "array",
      minItems: 1,
      items: { type: "object", additionalProperties: false, required: ["controlId", "chord", "aria", "ariaApple", "badge", "badgeApple", "event", "dispatches"], properties: { controlId: { type: "string" }, chord: { type: "string" }, aria: { type: "string" }, ariaApple: { type: "string" }, badge: { type: "string" }, badgeApple: { type: "string" }, event: { type: "object" }, dispatches: { type: ["object", "null"] } } },
    },
    keybindingOverride: { type: "object", additionalProperties: false, required: ["controlId", "keys", "aria", "ariaApple", "badge", "badgeApple"], properties: { controlId: { type: "string" }, keys: { type: "string" }, aria: { type: "string" }, ariaApple: { type: "string" }, badge: { type: "string" }, badgeApple: { type: "string" } } },
  },
};

function dialectOf(key: string | null | undefined): ArtifactDialect | undefined {
  if (key === null || key === undefined) return undefined;
  const dialect = fixture.dialects[key];
  assert.ok(dialect, `fixture names an undeclared dialect ${key}`);
  return dialect;
}

function appsOf(manifest: string): readonly RoleSurfaceAppV1[] {
  const apps = fixture.manifests[manifest];
  assert.ok(apps, `fixture names an undeclared manifest ${manifest}`);
  return apps.map((app) => ({ id: app.id, role: app.role, dialect: dialectOf(app.dialect) }));
}

/** 🔮️ Independent oracle for the boot axes — rebuilds the wanted surface id from `<dialect>#<role>`
 * instead of scanning the manifest for a sibling, so it agrees with `resolveBootPrimaryAppV1` only if
 * the projection law itself is right. */
function oracleBootApp(apps: readonly RoleSurfaceAppV1[], pinnedAppId: string | null, defaultAppId: string | null, role: AppRole): string | null {
  const anchorId = pinnedAppId ?? defaultAppId;
  if (anchorId === null) return (apps.find((app) => app.role === role) ?? apps[0])?.id ?? null;
  const anchor = apps.find((app) => app.id === anchorId);
  if (anchor === undefined) return pinnedAppId !== null ? null : ((apps.find((app) => app.role === role) ?? apps[0])?.id ?? null);
  if (anchor.role === role) return anchor.id;
  const projected = anchor.id.replace(/#(editor|viewer)$/u, `#${role}`);
  return apps.some((app) => app.id === projected) ? projected : anchor.id;
}

/** 🔮️ Independent oracle for the role-group render gate — counts the surfaces sharing one dialect
 * coordinate instead of looking each role up by name. */
function oracleGroupRoles(apps: readonly RoleSurfaceAppV1[], dialect: ArtifactDialect | undefined): readonly AppRole[] {
  if (dialect === undefined) return [];
  const coordinate = `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}`;
  const roles = apps.filter((app) => app.dialect !== undefined && `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}` === coordinate).map((app) => app.role);
  return SURFACE_ROLE_ORDER.filter((role) => roles.includes(role));
}

type SwitchApp = { readonly id: string };
type SwitchSession = SessionAppSwitchSessionV1<SwitchApp, string>;

/** 🧪️ A recording host for {@link runSessionAppSwitchV1}: every port appends one step, so the ORDER the
 * unit runs them in is the assertion rather than a side effect nobody observes. `trace` is recorded
 * separately because it is the vocabulary the SHELL prints — a law that only checked the port calls
 * would let the diagnostic and the behaviour drift apart. */
function recordingSwitchHost(
  apps: readonly RoleSurfaceAppV1[],
  session: SwitchSession | null,
  nextInstanceId: number,
  quiesce: SessionAppSwitchQuiesceV1 = { settled: true, pending: 0 },
  createFails = false,
  retireMode: SwitchRetireModeV1 = "resolve",
) {
  const steps: string[] = [];
  const trace: SessionAppSwitchStepV1[] = [];
  const retired: number[] = [];
  const sealed: string[] = [];
  const unsealed: string[] = [];
  const retireFailures: number[] = [];
  let created = 0;
  return {
    steps,
    trace,
    retired,
    sealed,
    unsealed,
    retireFailures,
    createdCount: (): number => created,
    ports: {
      session,
      resolveApp: (_pluginId: string, appId: string): SwitchApp | null => apps.find((app) => app.id === appId) ?? null,
      appId: (app: SwitchApp): string => app.id,
      quiesce: async (draining: SwitchSession): Promise<SessionAppSwitchQuiesceV1> => {
        steps.push(`quiesce:${draining.instanceId}`);
        return quiesce;
      },
      seal: (sealing: SwitchSession): void => {
        sealed.push(sessionInstanceKeyV1(sealing.pluginId, sealing.instanceId));
        steps.push(`seal:${sealing.instanceId}`);
      },
      unseal: (kept: SwitchSession): void => {
        unsealed.push(sessionInstanceKeyV1(kept.pluginId, kept.instanceId));
        steps.push(`unseal:${kept.instanceId}`);
      },
      createInstance: async (_pluginId: string, app: SwitchApp): Promise<number> => {
        if (createFails) throw new Error(`createApp refused for ${app.id}`);
        created += 1;
        steps.push(`create:${app.id}`);
        return nextInstanceId;
      },
      // 🚪️ `never` models the close ladder measured on 6018 — 62–87 s and then
      // `plugin-ui.lifecycle-close-budget-exhausted`. A switch that awaited THIS is a switch the user
      // waits a minute for, which is the whole defect these rows encode.
      retire: (retiring: SwitchSession): Promise<void> => {
        retired.push(retiring.instanceId);
        steps.push(`retire:${retiring.instanceId}`);
        if (retireMode === "never") return new Promise<void>(() => {});
        if (retireMode === "reject") return Promise.reject(new Error(`plugin-ui.lifecycle-close-budget-exhausted:${retiring.instanceId}`));
        return Promise.resolve();
      },
      defaultViewState: (app: SwitchApp): string => `default:${app.id}`,
      publish: (next: SwitchSession): void => {
        steps.push(`publish:${next.app.id}`);
      },
      seedLayout: (app: SwitchApp): void => {
        steps.push(`seed:${app.id}`);
      },
      refresh: async (next: SwitchSession): Promise<void> => {
        steps.push(`refresh:${next.app.id}`);
      },
      trace: (step: SessionAppSwitchStepV1): void => {
        trace.push(step);
      },
    },
    onRetireFailed: (failed: SwitchSession): void => {
      retireFailures.push(failed.instanceId);
    },
  };
}

/** 🚪️ How the fixture's predecessor close ladder behaves: it completes, it fails, or it never
 * finishes at all. */
type SwitchRetireModeV1 = "resolve" | "reject" | "never";

/** ⏳️ Lets every already-queued microtask (and one macrotask hop) run, so a law can observe what a
 * BACKGROUND retirement did after the switch's own promise already resolved. A retirement that has
 * not reported by here is one the successor genuinely did not wait for. */
async function settleBackgroundWork(): Promise<void> {
  await new Promise<void>((resolve) => { setTimeout(resolve, 0); });
  await Promise.resolve();
}

/** 🔮️ Independent oracle for the switch order: derives the steps a CORRECT transactional switch owes
 * from the request alone — no reference to the unit's own control flow — so it agrees with
 * `runSessionAppSwitchV1` only if that order is actually the law. The whole defect this encodes is a
 * `retire` that ran BEFORE `quiesce` answered and before the successor existed. */
function oracleSwitchTrace(sessionAppId: string | null, requestAppId: string, declared: boolean, withViewState: boolean, settled: boolean, createFails: boolean): readonly SessionAppSwitchStepV1[] {
  if (!declared) return [];
  if (sessionAppId === requestAppId) return withViewState ? ["publish", "refresh"] : [];
  if (sessionAppId === null) return createFails ? ["create-failed"] : ["create", "publish", "seed", "refresh"];
  if (!settled) return ["quiesce"];
  if (createFails) return ["quiesce", "seal", "create-failed", "unseal"];
  return ["quiesce", "seal", "create", "retire-started", "publish", "seed", "refresh"];
}

/** 🔮️ The same oracle once the BACKGROUND retirement has had its turn: a ladder that resolves reports
 * `retire`, one that rejects reports `retire-failed`, and one that never finishes reports neither —
 * and in all three cases the successor was already published. */
function oracleSettledTrace(resolved: readonly SessionAppSwitchStepV1[], retireMode: SwitchRetireModeV1): readonly SessionAppSwitchStepV1[] {
  if (!resolved.includes("retire-started")) return resolved;
  if (retireMode === "resolve") return [...resolved, "retire"];
  if (retireMode === "reject") return [...resolved, "retire-failed"];
  return resolved;
}

type TwinRow = { readonly id: string; readonly appId: string | null; readonly roles: readonly string[] };

/** 🌱️ Runs the SHIPPED source text of the pure resolution laws in a bare `node` process against the
 * same fixture. A law that only holds inside this repo's test harness is not a law. */
function nodeTwin(): readonly TwinRow[] {
  const require = createRequire(import.meta.url);
  const ts = require("typescript");
  const unit = resolve(dirname(fileURLToPath(import.meta.url)), "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts");
  const text = readFileSync(unit, "utf8");
  const source = ts.createSourceFile(unit, text, ts.ScriptTarget.Latest, true);
  const wanted = new Set([
    "surfaceRoleAppsV1",
    "resolveBootPrimaryAppV1",
    "roleSwitchTargetV1",
    "stepModeIdV1",
    "SURFACE_ROLE_ORDER",
    "sessionInstanceKeyV1",
    "SEALED_INSTANCE_LEDGER_SLOTS",
    "createSealedInstanceLedgerV1",
    "SEALED_INSTANCE_DROP_CODE",
    "sealedInstanceDropV1",
    "sealedInstanceDropTextV1",
    "SURFACE_SWITCH_BUSY_LABEL",
    "surfaceSwitchBusyTextV1",
  ]);
  const statements = source.statements.filter((node: any) =>
    ts.isFunctionDeclaration(node) ? wanted.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration: any) => wanted.has(declaration.name?.text ?? "")),
  );
  assert.equal(statements.length, wanted.size, "the twin must find every shipped law by name");
  const runtime = ts.transpileModule(statements.map((node: any) => node.getText(source)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText;
  const program = `function dialectCoordinate(dialect) { return \`\${dialect.artifactKind}@\${dialect.standard}/\${dialect.subset}\`; }
${runtime}
const fixture = ${JSON.stringify(fixture)};
const dialectOf = (key) => (key == null ? undefined : fixture.dialects[key]);
const appsOf = (name) => fixture.manifests[name].map((app) => ({ id: app.id, role: app.role, dialect: dialectOf(app.dialect) }));
const rows = [];
for (const row of fixture.boot) {
  const raw = new URLSearchParams(row.search).get("role");
  const role = raw === "viewer" ? "viewer" : raw === "editor" ? "editor" : row.envRole;
  const app = exports.resolveBootPrimaryAppV1(appsOf(row.manifest), row.pinnedAppId ?? undefined, row.defaultAppId ?? undefined, role);
  rows.push({ id: row.id, appId: app ? app.id : null, roles: [role] });
}
for (const row of fixture.group) {
  const pair = exports.surfaceRoleAppsV1(appsOf(row.manifest), dialectOf(row.dialect));
  rows.push({ id: row.id, appId: pair ? pair.editor.id : null, roles: pair ? Array.from(exports.SURFACE_ROLE_ORDER) : [] });
}
for (const row of fixture.roleTargets) {
  const target = exports.roleSwitchTargetV1(appsOf(row.manifest), dialectOf(row.dialect), row.currentRole, row.requested);
  rows.push({ id: row.id, appId: target ? target.id : null, roles: [] });
}
for (const row of fixture.modeSteps) rows.push({ id: row.id, appId: exports.stepModeIdV1(row.modeIds, row.activeModeId, row.step), roles: [] });
const ledger = exports.createSealedInstanceLedgerV1(fixture.sealed.slots);
for (const [pluginId, instanceId] of fixture.sealed.seal) ledger.seal(pluginId, instanceId);
for (const [pluginId, instanceId] of fixture.sealed.expectedSealed) rows.push({ id: "sealed:" + exports.sessionInstanceKeyV1(pluginId, instanceId), appId: ledger.sealed(pluginId, instanceId) ? "sealed" : null, roles: [] });
rows.push({ id: "sealed-drop", appId: exports.sealedInstanceDropTextV1(exports.sealedInstanceDropV1(fixture.sealed.drop.pluginId, fixture.sealed.drop.instanceId, fixture.sealed.drop.what, fixture.sealed.drop.detail)), roles: [] });
rows.push({ id: "sealed-drop-bare", appId: exports.sealedInstanceDropTextV1(exports.sealedInstanceDropV1(fixture.sealed.dropWithoutDetail.pluginId, fixture.sealed.dropWithoutDetail.instanceId, fixture.sealed.dropWithoutDetail.what)), roles: [] });
for (const row of fixture.busyLabel) rows.push({ id: "busy:" + row.locale, appId: exports.surfaceSwitchBusyTextV1(row.locale), roles: [] });
console.log(JSON.stringify(rows));`;
  const result = spawnSync("node", ["-e", program], { encoding: "utf8", timeout: 20000 });
  assert.equal(result.status, 0, result.stderr);
  return JSON.parse(result.stdout) as readonly TwinRow[];
}

export async function testSurfaceSwitch(): Promise<void> {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(FIXTURE_SCHEMA);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));

  for (const row of fixture.boot) {
    const role = resolveBootQueryAppRole(row.search, row.envRole);
    assert.equal(role, row.expectedRole, `${row.id}: role`);
    const apps = appsOf(row.manifest);
    assert.equal(resolveBootPrimaryAppV1(apps, row.pinnedAppId ?? undefined, row.defaultAppId ?? undefined, role)?.id ?? null, row.expectedAppId, `${row.id}: app`);
    assert.equal(oracleBootApp(apps, row.pinnedAppId, row.defaultAppId, role), row.expectedAppId, `${row.id}: oracle`);
  }

  for (const row of fixture.group) {
    const apps = appsOf(row.manifest);
    const pair = surfaceRoleAppsV1(apps, dialectOf(row.dialect));
    assert.deepEqual(pair === null ? null : { editor: pair.editor.id, viewer: pair.viewer.id }, row.expected, `${row.id}: group`);
    assert.equal(oracleGroupRoles(apps, dialectOf(row.dialect)).length === SURFACE_ROLE_ORDER.length, row.expected !== null, `${row.id}: oracle`);
  }

  for (const row of fixture.roleTargets) {
    assert.equal(roleSwitchTargetV1(appsOf(row.manifest), dialectOf(row.dialect), row.currentRole, row.requested)?.id ?? null, row.expected, `${row.id}: target`);
  }

  for (const row of fixture.switch) {
    const apps = appsOf(row.manifest);
    const session: SwitchSession | null = row.session === null ? null : { pluginId: row.session.pluginId, instanceId: row.session.instanceId, app: { id: row.session.appId }, viewState: "mounted" };
    const host = recordingSwitchHost(apps, session, row.expected.instanceId ?? 0, row.quiesce, row.createFails, row.retireMode);
    const request = { pluginId: row.request.pluginId, appId: row.request.appId, viewState: row.request.viewState ? "requested" : undefined };
    const outcome = row.createFails
      ? await runSessionAppSwitchV1<SwitchApp, string>(host.ports, request, host.onRetireFailed).then(
          () => { throw new Error(`${row.id}: a refused createApp must reject, not answer an outcome`); },
          (): null => null,
        )
      : await runSessionAppSwitchV1<SwitchApp, string>(host.ports, request, host.onRetireFailed);
    assert.deepEqual(host.steps, row.expected.steps, `${row.id}: steps`);
    // 🚪️ Read at the moment the switch's OWN promise resolved: the successor is already published,
    // whatever the predecessor's close ladder is still doing.
    const publishedAtResolve = host.trace.includes("publish");
    await settleBackgroundWork();
    assert.deepEqual(host.trace, [...row.expected.trace], `${row.id}: trace vocabulary`);
    if (outcome !== null) {
      assert.equal(outcome.status, row.expected.status, `${row.id}: status`);
      assert.equal(outcome.pending, row.expected.pending, `${row.id}: pending`);
      assert.equal(outcome.session?.app.id ?? null, row.expected.sessionAppId, `${row.id}: session app`);
      assert.equal(outcome.session?.instanceId ?? null, row.expected.instanceId, `${row.id}: instance`);
    }
    assert.equal(host.createdCount(), row.expected.created, `${row.id}: created`);
    assert.deepEqual(host.retired, [...row.expected.retired], `${row.id}: retired`);
    assert.deepEqual(host.sealed, row.expected.retired.map((instanceId) => sessionInstanceKeyV1(row.request.pluginId, instanceId)).concat(row.createFails ? [sessionInstanceKeyV1(row.request.pluginId, row.session?.instanceId ?? 0)] : []), `${row.id}: every retired instance was sealed first`);
    assert.deepEqual(host.unsealed, row.createFails ? [sessionInstanceKeyV1(row.request.pluginId, row.session?.instanceId ?? 0)] : [], `${row.id}: only a kept predecessor is unsealed`);
    assert.equal(host.createdCount() - host.retired.length, row.session === null ? row.expected.created : 0, `${row.id}: no leaked instance`);
    // ⚖️ Second reading: the independent oracle, plus the orderings that ARE the defect — a predecessor
    // may never be retired before it went quiet, and it must be sealed before `createApp` runs, because
    // `createApp` itself revokes its activation.
    assert.deepEqual(host.trace, oracleSettledTrace(oracleSwitchTrace(row.session?.appId ?? null, row.request.appId, apps.some((app) => app.id === row.request.appId), row.request.viewState, row.quiesce.settled, row.createFails), row.retireMode), `${row.id}: oracle`);
    const retireAt = host.trace.indexOf("retire-started");
    if (retireAt !== -1) {
      assert.ok(host.trace.indexOf("quiesce") !== -1 && host.trace.indexOf("quiesce") < retireAt, `${row.id}: the predecessor is quiesced before it is retired`);
      assert.ok(host.trace.indexOf("seal") !== -1 && host.trace.indexOf("seal") < host.trace.indexOf("create"), `${row.id}: the predecessor is sealed before the successor is created`);
      assert.ok(host.trace.indexOf("create") !== -1 && host.trace.indexOf("create") < retireAt, `${row.id}: the successor exists before the predecessor is retired`);
      assert.ok(host.trace.indexOf("publish") > retireAt, `${row.id}: the successor is published only after the predecessor was sealed and its ladder started`);
      // 🚪️ THE law of this lane: the successor is MOUNTED by the time the switch answers, whatever the
      // predecessor's close ladder is still doing. A ladder that fails, and one that never finishes at
      // all, both leave the successor published and the outcome `switched` — before the fix the shell
      // awaited the ladder, so a 62–87 s close (measured on 6018, `🗑️generated/journey-5/console.txt`)
      // was 62–87 s of the user still looking at the predecessor.
      assert.ok(publishedAtResolve, `${row.id}: the successor is published before the switch answers`);
      const reportedAt = Math.max(host.trace.indexOf("retire"), host.trace.indexOf("retire-failed"));
      assert.ok(reportedAt === -1 || reportedAt > host.trace.indexOf("refresh"), `${row.id}: the close ladder reports after the successor's session started, never before`);
      assert.equal(outcome?.status ?? null, row.expected.status, `${row.id}: the successor mounts while the predecessor retires`);
    }
    if (!row.quiesce.settled) assert.deepEqual(host.retired, [], `${row.id}: a refused switch retires nothing`);
    assert.deepEqual(host.trace, oracleSettledTrace(oracleSwitchTrace(row.session?.appId ?? null, row.request.appId, apps.some((app) => app.id === row.request.appId), row.request.viewState, row.quiesce.settled, row.createFails), row.retireMode), `${row.id}: settled oracle`);
    assert.equal(host.retireFailures.length, row.expected.retireFailures, `${row.id}: retirement failures reported`);
  }

  // 🚦️ Mid-chain law: the second chord arrives while the first switch is still awaiting its quiesce
  // pass — exactly the `⌘️⌥️V` pressed twice during an in-flight evaluation — and is REFUSED, not
  // queued, so only one tear-down ever runs.
  {
    const gate = createSessionAppSwitchGateV1<SwitchApp, string>();
    const apps = appsOf("procedural");
    const mounted: SwitchSession = { pluginId: "procedural", instanceId: 7, app: { id: "s.procedural.generation3d@1/*#editor" }, viewState: "mounted" };
    const host = recordingSwitchHost(apps, mounted, 8);
    let releaseQuiesce = (): void => {};
    const held = new Promise<void>((resolve) => { releaseQuiesce = resolve; });
    const ports = { ...host.ports, quiesce: async (draining: SwitchSession): Promise<SessionAppSwitchQuiesceV1> => { host.steps.push(`quiesce:${draining.instanceId}`); await held; return { settled: true, pending: 0 }; } };
    const first = gate.run(ports, { pluginId: "procedural", appId: fixture.gate.first.appId });
    assert.equal(gate.busy(), true, "gate: a running switch reports busy");
    const second = await gate.run(ports, { pluginId: "procedural", appId: fixture.gate.second.appId });
    assert.equal(second.status, fixture.gate.second.status, "gate: the overlapping switch is refused");
    assert.equal(second.session?.instanceId ?? null, mounted.instanceId, "gate: the refused switch keeps the mounted session");
    releaseQuiesce();
    const settled = await first;
    assert.equal(settled.status, fixture.gate.first.status, "gate: the first switch still completes");
    assert.equal(gate.busy(), false, "gate: the gate reopens once the switch settles");
    assert.deepEqual(host.retired, [...fixture.gate.expectedRetired], "gate: exactly one tear-down ran");
    assert.equal(host.createdCount(), fixture.gate.expectedCreated, "gate: exactly one instance was created");
  }

  for (const row of fixture.work) {
    const ledger = createSessionWorkLedgerV1();
    const releases = row.begin.map(([pluginId, instanceId, kind]) => ledger.begin(pluginId, instanceId, kind as SessionWorkKindV1));
    for (const index of row.release) releases[index]!();
    for (const [pluginId, instanceId, expected] of row.pending) {
      assert.equal(ledger.pending(pluginId, instanceId), expected, `${row.id}: pending ${pluginId}#${instanceId}`);
      // 🔎️ The split a refusal prints must add up to the number the quiesce pass polls — a diagnostic
      // that disagreed with the gate would be worse than none.
      const split = ledger.outstanding(pluginId, instanceId);
      assert.equal([...split.matchAll(/=(\d+)/gu)].reduce((sum, match) => sum + Number(match[1]), 0), expected, `${row.id}: outstanding ${pluginId}#${instanceId} = ${split}`);
    }
    assert.equal(ledger.total(), row.total, `${row.id}: total`);
    // ⚖️ Oracle: the count is begins minus DISTINCT releases, which is only true because a release is
    // idempotent — the property the shell's `finally` blocks depend on.
    assert.equal(ledger.total(), row.begin.length - new Set(row.release).size, `${row.id}: oracle`);
  }

  for (const row of fixture.quiesce) {
    let reads = 0;
    let sleeps = 0;
    let clock = 0;
    const answer = await quiesceSessionWorkV1(() => row.pending[Math.min(reads++, row.pending.length - 1)]!, {
      budgetMs: row.budgetMs,
      pollMs: row.pollMs,
      now: () => clock,
      sleep: async (ms: number) => { sleeps += 1; clock += ms; },
    });
    assert.equal(answer.settled, row.expected.settled, `${row.id}: settled`);
    assert.equal(answer.pending, row.expected.pending, `${row.id}: pending`);
    assert.equal(sleeps, row.expected.sleeps, `${row.id}: sleeps`);
  }

  {
    const ledger = createSealedInstanceLedgerV1(fixture.sealed.slots);
    for (const [pluginId, instanceId] of fixture.sealed.seal) ledger.seal(pluginId, instanceId);
    assert.equal(ledger.size(), fixture.sealed.expectedSize, "sealed: the ledger is bounded");
    for (const [pluginId, instanceId, expected] of fixture.sealed.expectedSealed) assert.equal(ledger.sealed(pluginId, instanceId), expected, `sealed: ${pluginId}#${instanceId}`);
    const [unsealPlugin, unsealInstance] = fixture.sealed.unseal.target;
    ledger.unseal(unsealPlugin, unsealInstance);
    assert.equal(ledger.sealed(unsealPlugin, unsealInstance), fixture.sealed.unseal.sealedAfterUnseal, "sealed: a kept predecessor comes back out of the ledger");
    assert.equal(ledger.size(), fixture.sealed.unseal.sizeAfterUnseal, "sealed: unsealing removes exactly one");
    const drop = sealedInstanceDropV1(fixture.sealed.drop.pluginId, fixture.sealed.drop.instanceId, fixture.sealed.drop.what, fixture.sealed.drop.detail);
    assert.equal(drop.code, fixture.sealed.drop.code, "sealed: the drop carries the stable code");
    assert.equal(drop.code, SEALED_INSTANCE_DROP_CODE, "sealed: the fixture names the shipped code");
    assert.equal(sealedInstanceDropTextV1(drop), fixture.sealed.drop.text, "sealed: one line per drop");
    const bare = fixture.sealed.dropWithoutDetail;
    assert.equal(sealedInstanceDropTextV1(sealedInstanceDropV1(bare.pluginId, bare.instanceId, bare.what)), bare.text, "sealed: a drop without a detail still reads as one line");
  }

  for (const row of fixture.busyLabel) assert.equal(surfaceSwitchBusyTextV1(row.locale), row.text, `busy label: ${row.locale}`);

  for (const row of fixture.modeSteps) {
    assert.equal(stepModeIdV1(row.modeIds, row.activeModeId, row.step), row.expected, `${row.id}: mode step`);
  }

  const composed = composeControlKeybindings(new Map(), {});
  for (const row of fixture.keybindings) {
    assert.equal(SHELL_KEYBINDINGS[row.controlId], row.chord, `${row.controlId}: declared chord`);
    assert.equal(composed.get(row.controlId), row.chord, `${row.controlId}: reaches the settings registry`);
    assert.equal(ariaKeyshortcutsText(row.chord, "Win32"), row.aria, `${row.controlId}: aria-keyshortcuts`);
    // 🍎 The chord a screen reader is told must be the chord that FIRES: `mod` is Command on Apple, so
    // `aria-keyshortcuts` and the visual badge resolve it through one predicate
    // (`keybindingPlatformUsesMetaV1`). Publishing `Control+Alt+V` beside a rendered `⌘️⌥️V` told a
    // macOS screen-reader user a chord that does nothing.
    assert.equal(ariaKeyshortcutsText(row.chord, "MacIntel"), row.ariaApple, `${row.controlId}: aria-keyshortcuts on Apple`);
    assert.equal(formatKeybindingShortcut(row.chord, "MacIntel"), row.badgeApple, `${row.controlId}: badge on Apple`);
    assert.equal(formatKeybindingShortcut(row.chord, "Win32"), row.badge, `${row.controlId}: badge off Apple`);
    assert.ok(row.ariaApple.startsWith("Meta+") && row.badgeApple.startsWith("⌘️"), `${row.controlId}: badge and aria name the same physical key on Apple`);
    assert.ok(row.aria.startsWith("Control+") && row.badge.startsWith("Ctrl+"), `${row.controlId}: badge and aria name the same physical key off Apple`);
    const chords = parseOwnedHotkeyChords(row.chord, false);
    assert.equal(chords.length, 1, `${row.controlId}: one chord`);
    assert.equal(keyboardEventMatchesOwnedHotkey(new KeyboardEvent("keydown", row.event), chords[0]!), true, `${row.controlId}: matches its own event`);
    const dispatches = row.dispatches;
    if (dispatches === null) continue;
    const apps = appsOf("procedural");
    const target = roleSwitchTargetV1(apps, dialectOf("generation3d"), dispatches.currentRole, dispatches.requested);
    assert.equal(target?.id ?? null, dispatches.expectedAppId, `${row.controlId}: dispatch target`);
    const mountedId = surfaceRoleAppsV1(apps, dialectOf("generation3d"))?.[dispatches.currentRole].id;
    assert.ok(mountedId, `${row.controlId}: fixture must declare the mounted surface`);
    const host = recordingSwitchHost(apps, { pluginId: "procedural", instanceId: 7, app: { id: mountedId }, viewState: "mounted" }, 8);
    const outcome = await runSessionAppSwitchV1<SwitchApp, string>(host.ports, { pluginId: "procedural", appId: target!.id });
    assert.equal(outcome.status, "switched", `${row.controlId}: the chord's switch completes`);
    assert.equal(outcome.session?.app.id, dispatches.expectedAppId, `${row.controlId}: the chord dispatches the switch`);
    assert.deepEqual(host.retired, [7], `${row.controlId}: the dispatched switch retires the predecessor`);
    assert.deepEqual(host.sealed, [sessionInstanceKeyV1("procedural", 7)], `${row.controlId}: the chord's switch seals before it retires`);
  }

  const override = fixture.keybindingOverride;
  const overridden = composeControlKeybindings(new Map(), { [override.controlId]: override.keys });
  assert.equal(overridden.get(override.controlId), override.keys, "a user override replaces the framework chord");
  assert.equal(ariaKeyshortcutsText(overridden.get(override.controlId), "Win32"), override.aria, "the overridden chord is what aria-keyshortcuts republishes");
  assert.equal(ariaKeyshortcutsText(overridden.get(override.controlId), "MacIntel"), override.ariaApple, "a user override is resolved by the same platform rule");
  assert.equal(formatKeybindingShortcut(overridden.get(override.controlId)!, "MacIntel"), override.badgeApple, "a user override's badge is resolved by the same platform rule");
  for (const role of SURFACE_ROLE_ORDER) assert.equal(SURFACE_ROLE_CONTROL_IDS[role], `playground.navbar.roles.${role}`, "role button ids double as keybinding control ids");
  for (const controlId of Object.values(MODE_STEP_CONTROL_IDS)) assert.ok(SHELL_KEYBINDINGS[controlId], `${controlId} must be a declared framework verb`);

  const twin = nodeTwin();
  const expectedTwin: readonly TwinRow[] = [
    ...fixture.boot.map((row) => ({ id: row.id, appId: row.expectedAppId, roles: [row.expectedRole] })),
    ...fixture.group.map((row) => ({ id: row.id, appId: row.expected === null ? null : row.expected.editor, roles: row.expected === null ? [] : [...SURFACE_ROLE_ORDER] })),
    ...fixture.roleTargets.map((row) => ({ id: row.id, appId: row.expected, roles: [] })),
    ...fixture.modeSteps.map((row) => ({ id: row.id, appId: row.expected, roles: [] })),
    ...fixture.sealed.expectedSealed.map(([pluginId, instanceId, expected]) => ({ id: `sealed:${sessionInstanceKeyV1(pluginId, instanceId)}`, appId: expected ? "sealed" : null, roles: [] })),
    { id: "sealed-drop", appId: fixture.sealed.drop.text, roles: [] },
    { id: "sealed-drop-bare", appId: fixture.sealed.dropWithoutDetail.text, roles: [] },
    ...fixture.busyLabel.map((row) => ({ id: `busy:${row.locale}`, appId: row.text, roles: [] })),
  ];
  assert.deepEqual(twin, expectedTwin, "the node twin must answer the fixture identically");

  console.log(`[DEBUG] surface-switch boot=${fixture.boot.length} group=${fixture.group.length} roleTargets=${fixture.roleTargets.length} switch=${fixture.switch.length} work=${fixture.work.length} quiesce=${fixture.quiesce.length} sealed=${fixture.sealed.seal.length} busyLabel=${fixture.busyLabel.length} modeSteps=${fixture.modeSteps.length} keybindings=${fixture.keybindings.length} twin=${twin.length} PASS`);
}

describe("surface switch", () => {
  it("resolves the boot role, gates the role group, retires the predecessor on every switch, and keeps both axes on the keyboard", async () => {
    await testSurfaceSwitch();
  });
});

type SessionLaneStepV1 = { readonly route?: string; readonly job?: string; readonly finish?: string; readonly fail?: string };
type SessionLaneOutcomeV1 = "resolved" | "rejected";
type SessionLaneFixtureV1 = {
  readonly note: string;
  readonly scenarios: readonly { readonly id: string; readonly steps: readonly SessionLaneStepV1[]; readonly expected: { readonly started: readonly string[]; readonly requests: readonly SessionLaneOutcomeV1[] } }[];
  readonly routes: readonly { readonly uri: string; readonly overlay: boolean }[];
  readonly sessionRoutes: readonly { readonly uri: string; readonly underlying: string | null; readonly expected: { readonly overlay: boolean; readonly sessionRoute: string | null } }[];
  readonly identities: readonly { readonly state: Parameters<typeof shellIdentityResolutionV1>[0]; readonly expected: ShellIdentityResolutionV1 }[];
  readonly admissions: readonly { readonly route: string; readonly identity: ShellIdentityResolutionV1; readonly expected: ShellRouteAdmissionV1 }[];
  readonly admissionLabels: readonly { readonly admission: Exclude<ShellRouteAdmissionV1, "apply">; readonly locale: string; readonly text: string; readonly action: string | null }[];
};
type SessionLaneUnderTestV1 = { readonly route: (uri: string) => Promise<void>; readonly run: (job: () => Promise<void>) => Promise<void>; readonly idle: () => boolean };

/** 🧮️ The third-party oracle for {@link createShellSessionLaneV1}: `p-limit` with concurrency 1 is the serial
 * executor, and only the end-of-lane route absorption is modelled on top of it. */
function pLimitSessionLaneV1(applyRoute: (uri: string) => Promise<void>): SessionLaneUnderTestV1 {
  const limit = pLimit(1);
  const lane: { open: { uri: string; done: Promise<void> } | null } = { open: null };
  return {
    route: (uri) => {
      if (lane.open !== null) {
        lane.open.uri = uri;
        return lane.open.done;
      }
      const entry = { uri, done: Promise.resolve() };
      lane.open = entry;
      entry.done = limit(() => {
        if (lane.open === entry) lane.open = null;
        return applyRoute(entry.uri);
      });
      return entry.done;
    },
    run: (job) => {
      lane.open = null;
      return limit(job);
    },
    idle: () => limit.activeCount === 0 && limit.pendingCount === 0,
  };
}

/** 🎬️ Plays one fixture scenario against a lane: every application blocks until a `finish`/`fail` step settles it. */
async function driveSessionLaneScenarioV1(make: (applyRoute: (uri: string) => Promise<void>) => SessionLaneUnderTestV1, steps: readonly SessionLaneStepV1[]): Promise<{ readonly started: readonly string[]; readonly requests: readonly SessionLaneOutcomeV1[]; readonly idle: boolean }> {
  const started: string[] = [];
  const state: { running: { readonly name: string; readonly resolve: () => void; readonly reject: (error: Error) => void } | null } = { running: null };
  const application = (name: string): Promise<void> => new Promise<void>((resolve, reject) => {
    started.push(name);
    state.running = { name, resolve, reject };
  });
  const lane = make(application);
  const outcomes: Promise<SessionLaneOutcomeV1>[] = [];
  const outcome = (request: Promise<void>): Promise<SessionLaneOutcomeV1> => request.then(() => "resolved" as const, () => "rejected" as const);
  for (const step of steps) {
    if (step.route !== undefined) outcomes.push(outcome(lane.route(step.route)));
    else if (step.job !== undefined) {
      const name = step.job;
      outcomes.push(outcome(lane.run(() => application(name))));
    } else {
      const name = step.finish ?? step.fail;
      const running = state.running;
      assert(running !== null && running.name === name, `step settles ${name}, but ${running?.name ?? "nothing"} is running`);
      state.running = null;
      if (step.finish !== undefined) running.resolve();
      else running.reject(new Error(`${name} failed`));
    }
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  return { started, requests: await Promise.all(outcomes), idle: lane.idle() };
}

describe("shell session lane", () => {
  it("applies every route in request order, coalesces a waiting burst to the latest, keeps a re-establishment in its place and survives failures — equal to the p-limit oracle", async () => {
    const fixture = sessionLaneFixtureJson as SessionLaneFixtureV1;
    assert(fixture.scenarios.length >= 6);
    for (const scenario of fixture.scenarios) {
      const shipped = await driveSessionLaneScenarioV1((applyRoute) => createShellSessionLaneV1<string>(applyRoute), scenario.steps);
      const oracle = await driveSessionLaneScenarioV1(pLimitSessionLaneV1, scenario.steps);
      assert.deepEqual({ started: shipped.started, requests: shipped.requests }, scenario.expected, `${scenario.id}: shipped lane`);
      assert.deepEqual({ started: oracle.started, requests: oracle.requests }, scenario.expected, `${scenario.id}: p-limit oracle`);
      assert.equal(shipped.idle, true, `${scenario.id}: the shipped lane drains`);
      assert.equal(oracle.idle, true, `${scenario.id}: the oracle drains`);
    }
  });

  it("opens the hub overlay on the spot and queues only session routes — equal to a WHATWG URL oracle", () => {
    const fixture = sessionLaneFixtureJson as SessionLaneFixtureV1;
    assert(fixture.routes.length >= 9);
    for (const row of fixture.routes) {
      assert.equal(shellRouteIsOverlayV1(row.uri), row.overlay, row.uri);
      assert.equal(new URL(row.uri, "http://127.0.0.1").pathname === "/hub", row.overlay, `${row.uri}: URL oracle`);
    }
  });

  it("keeps the route an overlay was opened over as the session's route — equal to a WHATWG URL oracle", () => {
    const fixture = sessionLaneFixtureJson as SessionLaneFixtureV1;
    assert(fixture.sessionRoutes.length >= 7);
    for (const row of fixture.sessionRoutes) {
      assert.deepEqual(shellSessionRouteV1(row.uri, row.underlying), row.expected, row.uri);
      const overlay = new URL(row.uri, "http://127.0.0.1").pathname === "/hub";
      assert.deepEqual({ overlay, sessionRoute: overlay ? row.underlying : row.uri }, row.expected, `${row.uri}: URL oracle`);
    }
  });

  it("resolves the hub identity for every shell state — equal to an XState machine of the same rule", () => {
    const fixture = sessionLaneFixtureJson as SessionLaneFixtureV1;
    assert.equal(fixture.identities.length, 32);
    const oracle = setup({ types: { context: {} as Parameters<typeof shellIdentityResolutionV1>[0] } }).createMachine({
      context: ({ input }) => input as Parameters<typeof shellIdentityResolutionV1>[0],
      initial: "deciding",
      states: {
        deciding: {
          always: [
            { guard: ({ context }) => !context.hubConfigured, target: "local-only" },
            { guard: ({ context }) => !context.sessionHeld || context.refused, target: "signed-out" },
            { guard: ({ context }) => context.confirmed, target: "signed-in" },
            { guard: ({ context }) => context.offline, target: "hub-unavailable" },
            { target: "pending" },
          ],
        },
        "local-only": { type: "final" },
        "signed-out": { type: "final" },
        "signed-in": { type: "final" },
        "hub-unavailable": { type: "final" },
        pending: { type: "final" },
      },
    });
    for (const row of fixture.identities) {
      assert.equal(shellIdentityResolutionV1(row.state), row.expected, JSON.stringify(row.state));
      const actor = createActor(oracle, { input: row.state }).start();
      assert.equal(actor.getSnapshot().value, row.expected, `${JSON.stringify(row.state)}: XState oracle`);
      actor.stop();
    }
  });

  it("holds a space route until the human it is opened for is known, opens it offline when the hub does not answer, and says so in both languages — equal to a path-to-regexp oracle", () => {
    const fixture = sessionLaneFixtureJson as SessionLaneFixtureV1;
    assert(fixture.admissions.length >= 45);
    const spaceRoute = match("/spaces/:space{/*rest}");
    for (const row of fixture.admissions) {
      assert.equal(shellRouteAdmissionV1(row.route, row.identity), row.expected, `${row.route} × ${row.identity}`);
      const addressesSpace = spaceRoute(new URL(row.route, "http://127.0.0.1").pathname) !== false;
      const oracle: ShellRouteAdmissionV1 = !addressesSpace || row.identity === "local-only" || row.identity === "signed-in" ? "apply" : row.identity === "hub-unavailable" ? "apply-offline" : row.identity === "pending" ? "await-identity" : "await-sign-in";
      assert.equal(oracle, row.expected, `${row.route} × ${row.identity}: path-to-regexp oracle`);
    }
    assert.equal(fixture.admissionLabels.length, 6);
    for (const row of fixture.admissionLabels) assert.deepEqual(shellRouteAdmissionTextV1(row.admission, row.locale), { text: row.text, action: row.action }, `${row.admission}/${row.locale}`);
  });
});
