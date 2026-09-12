// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/surface-switch/component.ts
/** @emoji 🔀️ The navbar's two switching axes — surface ROLE (`…#editor` ⇄ `…#viewer`, a different app
 * instance) and app MODE (a different layout of the same instance) — plus the in-place session switch
 * both share, as an OWNED unit: no React, no shell imports, so a law can drive it without pulling the
 * shell's element graph (the same separation `🛠️ShellHelpers/🧩️contributions` keeps).
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
// #endregion 🧲️Header

import { dialectCoordinate, type AppRole, type ArtifactDialect } from "@semio-tech/framework";

/** 👁️✏️ The manifest fields surface-role resolution reads — narrower than `AppDefinition` so every law
 * here is fixture-drivable without building a whole manifest. */
export type RoleSurfaceAppV1 = { readonly id: string; readonly role: AppRole; readonly dialect?: ArtifactDialect };

/** 👁️✏️ Focus order of the navbar role group — editor first, viewer second, matching the order the
 * surface-role chip and the default-apps rows already present the two roles in. */
export const SURFACE_ROLE_ORDER: readonly AppRole[] = ["editor", "viewer"];

/** 👁️✏️ DOM ids of the navbar role buttons, which double as their keybinding control ids (see
 * `SHELL_KEYBINDINGS`) so each button renders its own chord badge, tooltip and Settings row. */
export const SURFACE_ROLE_CONTROL_IDS: Readonly<Record<AppRole, string>> = {
  editor: "playground.navbar.roles.editor",
  viewer: "playground.navbar.roles.viewer",
};

/** 🎛️ Control ids of the two framework mode-cycling verbs (`SHELL_KEYBINDINGS`), republished by the
 * navbar mode group's `aria-keyshortcuts`. Positional because mode ids are plugin-authored while that
 * table is static. */
export const MODE_STEP_CONTROL_IDS = { next: "ui.shell.mode.next", previous: "ui.shell.mode.previous" } as const;

/** 👁️✏️ Both surfaces one plugin declares for ONE dialect, or `null` when it declares fewer than two —
 * which is exactly when the navbar role group must not render at all. */
export function surfaceRoleAppsV1<T extends RoleSurfaceAppV1>(apps: readonly T[], dialect: ArtifactDialect | undefined): Readonly<Record<AppRole, T>> | null {
  if (dialect === undefined) return null;
  const coordinate = dialectCoordinate(dialect);
  const editor = apps.find((app) => app.role === "editor" && app.dialect !== undefined && dialectCoordinate(app.dialect) === coordinate);
  const viewer = apps.find((app) => app.role === "viewer" && app.dialect !== undefined && dialectCoordinate(app.dialect) === coordinate);
  return editor !== undefined && viewer !== undefined ? { editor, viewer } : null;
}

/** 👁️✏️ The app a role switch would open, or `null` when the request is a no-op (already that role) or
 * unserviceable (no sibling declared for the open document's dialect). */
export function roleSwitchTargetV1<T extends RoleSurfaceAppV1>(apps: readonly T[], dialect: ArtifactDialect | undefined, currentRole: AppRole, requested: AppRole): T | null {
  if (currentRole === requested) return null;
  return surfaceRoleAppsV1(apps, dialect)?.[requested] ?? null;
}

/** 👁️✏️ Picks the app a non-host session boots into from the three boot axes, role LAST-WINS over the
 * app-id axis.
 *
 * `pinnedAppId` (`VITE_SEMIO_APP_ID`) and `defaultAppId` (the playground registry's `app` column) both
 * name ONE artifact surface, and a surface app id is `<dialect>#<role>` by construction
 * (`surfaceAppId`) — so an anchor that names the wrong role is an anchor that named the right
 * DIALECT. Projecting it onto `appRole` is what makes `?role=viewer` reach
 * `s.procedural.generation3d@1/*#viewer` even though the `generation3d` playground pins `…#editor`;
 * without the projection the pin silently wins and the viewer stays unreachable
 * (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 2).
 *
 * Returns `undefined` only when `pinnedAppId` names no declared app at all — the caller raises that as
 * a boot error. A plugin that declares no sibling for the requested role keeps the anchor rather than
 * failing: asking for a viewer of an editor-only artifact is a downgrade to the surface that exists,
 * never a dead boot. An absent `appRole` (an embed that never states one) disables the projection. */
export function resolveBootPrimaryAppV1<T extends RoleSurfaceAppV1>(apps: readonly T[], pinnedAppId: string | undefined, defaultAppId: string | undefined, appRole: AppRole | undefined): T | undefined {
  const anchorId = pinnedAppId ?? defaultAppId;
  const anchor = anchorId === undefined ? undefined : apps.find((app) => app.id === anchorId);
  if (pinnedAppId !== undefined && anchor === undefined) return undefined;
  if (anchor === undefined) return apps.find((app) => app.role === appRole) ?? apps[0];
  if (appRole === undefined || anchor.role === appRole) return anchor;
  const anchorDialect = anchor.dialect;
  const sibling = anchorDialect === undefined ? undefined : apps.find((app) => app.role === appRole && app.dialect !== undefined && dialectCoordinate(app.dialect) === dialectCoordinate(anchorDialect));
  return sibling ?? anchor;
}

/** 🪪️ The mounted session as this unit sees it — structurally `ActiveSession`, restated locally so the
 * module keeps zero shell imports. */
export type SessionAppSwitchSessionV1<TApp, TViewState> = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly app: TApp;
  readonly viewState: TViewState;
};

/** 🧊️ One `(pluginId, instanceId)` coordinate as a single comparable key — what the retired ledger
 * stores and what every late effect is matched against. Instance ids are per plugin handle, so the
 * plugin id is part of the identity. */
export function sessionInstanceKeyV1(pluginId: string, instanceId: number): string {
  return `${pluginId}#${instanceId}`;
}

/** 🪦️ The set of instances a switch has SEALED — sealed means "this instance is being torn down; every
 * effect, completion, refresh and action still addressed to it is stale". Sealing happens BEFORE the
 * close ladder runs, which is the whole point: the ladder's own first step (closing document sessions)
 * already revokes the guest actor, so a completion frame that arrives between that revocation and the
 * ladder's last step would otherwise reach a dead actor and surface as
 * `plugin-ui.intake-rejected:intake:actor-activation.revoked` / `no actor for instance N` — measured on
 * 6018 as a page error plus one failure per later user action
 * (`🗑️generated/journey-3/console.txt`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * Bounded: the newest {@link SEALED_INSTANCE_LEDGER_SLOTS} keys are retained and older ones fall out —
 * a stale effect for an instance retired that long ago cannot exist, and an unbounded set in a
 * long-lived shell is a leak. */
export const SEALED_INSTANCE_LEDGER_SLOTS = 64;

export type SealedInstanceLedgerV1 = {
  readonly seal: (pluginId: string, instanceId: number) => void;
  readonly unseal: (pluginId: string, instanceId: number) => void;
  readonly sealed: (pluginId: string, instanceId: number) => boolean;
  readonly size: () => number;
};

export function createSealedInstanceLedgerV1(slots: number = SEALED_INSTANCE_LEDGER_SLOTS): SealedInstanceLedgerV1 {
  const keys: string[] = [];
  const live = new Set<string>();
  return {
    seal: (pluginId, instanceId) => {
      const key = sessionInstanceKeyV1(pluginId, instanceId);
      if (live.has(key)) return;
      live.add(key);
      keys.push(key);
      while (keys.length > slots) {
        const dropped = keys.shift();
        if (dropped !== undefined) live.delete(dropped);
      }
    },
    // 🔄️ The ONLY caller is a switch whose successor could not be created: the predecessor is being
    // KEPT, so its seal has to come off or every later action on the surface the user still sees would
    // be dropped forever. Leaves the key in the eviction ring — a slot is cheap, and re-sealing the
    // same instance is idempotent anyway.
    unseal: (pluginId, instanceId) => {
      live.delete(sessionInstanceKeyV1(pluginId, instanceId));
    },
    sealed: (pluginId, instanceId) => live.has(sessionInstanceKeyV1(pluginId, instanceId)),
    size: () => live.size,
  };
}

/** 🔇️ The ONE typed diagnostic a sealed instance's late work is allowed to produce. Everything a
 * retired instance is still addressed by — a queued action, a completion's host-effect pass, an
 * extension answer landing after the round trip — resolves to this and stops, instead of reaching
 * `requireActorId` and surfacing as `no actor for instance N` with a stack (twenty of those, plus one
 * `pageerror`, is what the 6018 journey measured after one role chord). `code` is the stable half:
 * laws and log greps match on it, never on the prose. */
export const SEALED_INSTANCE_DROP_CODE = "shell.surface-switch.sealed-instance";

export type SealedInstanceDropV1 = { readonly code: typeof SEALED_INSTANCE_DROP_CODE; readonly instance: string; readonly what: string; readonly detail: string | null };

export function sealedInstanceDropV1(pluginId: string, instanceId: number, what: string, detail?: string): SealedInstanceDropV1 {
  return { code: SEALED_INSTANCE_DROP_CODE, instance: sessionInstanceKeyV1(pluginId, instanceId), what, detail: detail ?? null };
}

/** 🔇️ One line, one drop — the exact text the shell prints and the probe greps. */
export function sealedInstanceDropTextV1(drop: SealedInstanceDropV1): string {
  return `${drop.code} dropped ${drop.what} for ${drop.instance}${drop.detail === null ? "" : ` (${drop.detail})`}`;
}

/** 🚦️ The refusal the user SEES when a switch is declined because the mounted surface is still busy
 * (`draining`) or another switch is already running (`busy`). Authored here, in the unit that decides
 * the refusal, rather than in the shell's i18n dictionary: no shell import, so the fixture asserts both
 * languages, and there is no default language — `locale` picks, and nothing falls back to English
 * except an unknown locale, which is the only case where a choice has not been made. */
export type SurfaceSwitchLabelV1 = { readonly en: string; readonly de: string };
export const SURFACE_SWITCH_BUSY_LABEL: SurfaceSwitchLabelV1 = { en: "Finishing the current operation…", de: "Laufender Vorgang wird abgeschlossen…" };
export function surfaceSwitchBusyTextV1(locale: string): string {
  return locale === "de" ? SURFACE_SWITCH_BUSY_LABEL.de : SURFACE_SWITCH_BUSY_LABEL.en;
}

/** ⏳️ What the host's quiesce pass answers: whether the predecessor went quiet, and how much work was
 * still outstanding when the pass gave up. `settled === false` REFUSES the switch — a switch over a
 * busy instance is exactly the mid-chain tear-down this unit exists to prevent. */
export type SessionAppSwitchQuiesceV1 = { readonly settled: boolean; readonly pending: number };

/** ⏳️ The two kinds of guest-bound work a switch has to outlive. A typed operation is admitted on the
 * guest's first reactor turn and finishes turns later (`OperationCompleted`); an extension invocation
 * is a host-brokered round trip (`invokeExtension` → the extension's own turn → `Completed` back into
 * the requester). BOTH hold a captured actor activation for the whole round trip, which is why
 * retiring the instance underneath either one surfaces as
 * `plugin-ui.intake-rejected:intake:actor-activation.revoked` rather than a clean cancel. */
export type SessionWorkKindV1 = "typed-operation" | "extension-invocation";

/** ⏳️ Counts the guest-bound work outstanding per `(pluginId, instanceId)`. Deliberately a COUNTER and
 * not a promise set: the shell starts this work from a dozen call sites that already own their own
 * error handling, and a ledger that had to own their promises would have to re-own their failures too.
 * `begin` answers a release that is idempotent, so a `finally` that runs twice cannot drive the count
 * negative. */
export type SessionWorkLedgerV1 = {
  readonly begin: (pluginId: string, instanceId: number, kind: SessionWorkKindV1) => () => void;
  readonly pending: (pluginId: string, instanceId: number) => number;
  readonly outstanding: (pluginId: string, instanceId: number) => string;
  readonly total: () => number;
};

/** ⏳️ Keyed by instance AND kind, so a refused switch can SAY what it was waiting on
 * ({@link SessionWorkLedgerV1.outstanding}) instead of only how much — the difference between a
 * diagnosable `draining` and an unexplained one. `pending` is the sum, which is what the quiesce pass
 * polls. */
function sessionWorkKeyV1(pluginId: string, instanceId: number, kind: SessionWorkKindV1): string {
  return `${sessionInstanceKeyV1(pluginId, instanceId)}|${kind}`;
}

export function createSessionWorkLedgerV1(): SessionWorkLedgerV1 {
  const counts = new Map<string, number>();
  const kinds: readonly SessionWorkKindV1[] = ["typed-operation", "extension-invocation"];
  const at = (pluginId: string, instanceId: number, kind: SessionWorkKindV1): number => counts.get(sessionWorkKeyV1(pluginId, instanceId, kind)) ?? 0;
  return {
    begin: (pluginId, instanceId, kind) => {
      const key = sessionWorkKeyV1(pluginId, instanceId, kind);
      counts.set(key, (counts.get(key) ?? 0) + 1);
      let released = false;
      return () => {
        if (released) return;
        released = true;
        const next = (counts.get(key) ?? 1) - 1;
        if (next <= 0) counts.delete(key);
        else counts.set(key, next);
      };
    },
    pending: (pluginId, instanceId) => kinds.reduce((sum, kind) => sum + at(pluginId, instanceId, kind), 0),
    outstanding: (pluginId, instanceId) => kinds.map((kind) => `${kind}=${at(pluginId, instanceId, kind)}`).join(" "),
    total: () => [...counts.values()].reduce((sum, count) => sum + count, 0),
  };
}

/** ⏳️ How long a switch waits for its predecessor to go quiet before refusing. Sized off the measured
 * worst case on 6018: one `interactionSelect` whose continuation turns plus a `flow-extension-brep`
 * `evaluate` round trip took 4.2 s end to end (`🗑️generated/journey-3/console.txt`, 347.2 s → 351.4 s).
 * A budget under that turns every switch pressed during an evaluation into a refusal, which is the
 * WRONG answer for a chord the user just pressed; a budget far over it makes an actually wedged
 * instance feel like a hang. */
export const SESSION_SWITCH_QUIESCE_BUDGET_MS = 12_000;
export const SESSION_SWITCH_QUIESCE_POLL_MS = 40;

/** ⏳️ Polls `pending` until it reads zero or the budget runs out. Pure over its clock and its sleeper
 * so a law can drive it with a fake one — the shipped caller passes `setTimeout`/`Date.now`.
 *
 * Answering `{ settled: false }` is not a failure to be retried inside this function: the caller turns
 * it into a `draining` outcome, which leaves the predecessor mounted and untouched. That is the entire
 * point — a predecessor that will not go quiet must keep its instance, not lose it mid-turn. */
export async function quiesceSessionWorkV1(
  pending: () => number,
  options?: { readonly budgetMs?: number; readonly pollMs?: number; readonly now?: () => number; readonly sleep?: (ms: number) => Promise<void> },
): Promise<SessionAppSwitchQuiesceV1> {
  const budgetMs = options?.budgetMs ?? SESSION_SWITCH_QUIESCE_BUDGET_MS;
  const pollMs = options?.pollMs ?? SESSION_SWITCH_QUIESCE_POLL_MS;
  const now = options?.now ?? (() => Date.now());
  const sleep = options?.sleep ?? ((ms: number) => new Promise<void>((resolve) => { setTimeout(resolve, ms); }));
  const deadline = now() + budgetMs;
  let outstanding = pending();
  while (outstanding > 0 && now() < deadline) {
    await sleep(pollMs);
    outstanding = pending();
  }
  return { settled: outstanding === 0, pending: outstanding };
}

/** 🔭️ Every ordered step of one switch, as a typed diagnostic rather than free-text logging — the
 * fixture asserts this exact vocabulary, and the shell prints it, so the browser console and the law
 * read the same word for the same step. */
export type SessionAppSwitchStepV1 = "quiesce" | "seal" | "unseal" | "create" | "create-failed" | "retire-started" | "retire" | "retire-failed" | "publish" | "seed" | "refresh";

/** 🚦️ How a switch request ended.
 * - `switched` — a predecessor was quiesced, sealed, retired, and the successor is mounted.
 * - `mounted` — there was no predecessor (a cold shell); the successor is mounted.
 * - `republished` / `unchanged` — the request named the app that is already mounted.
 * - `unresolvable` — `(pluginId, appId)` names no declared app; nothing was created or retired.
 * - `busy` — another switch is still running (the gate refused this one).
 * - `draining` — the predecessor did not go quiet; nothing was created or retired. */
export type SessionAppSwitchStatusV1 = "switched" | "mounted" | "republished" | "unchanged" | "unresolvable" | "busy" | "draining";

export type SessionAppSwitchOutcomeV1<TApp, TViewState> = {
  readonly status: SessionAppSwitchStatusV1;
  readonly session: SessionAppSwitchSessionV1<TApp, TViewState> | null;
  readonly pending: number;
};

/** 🔌️ Everything one in-place app switch needs from its host. `quiesce` drains (or cancels) whatever
 * the predecessor still has in flight; `seal` marks it stale for every late effect; `retire` is the
 * caller's own close ladder; `seedLayout` is where host-only bookkeeping lives, behind the caller's
 * `hostMode` guard. */
export type SessionAppSwitchPortsV1<TApp, TViewState> = {
  readonly session: SessionAppSwitchSessionV1<TApp, TViewState> | null;
  readonly resolveApp: (pluginId: string, appId: string) => TApp | null;
  readonly appId: (app: TApp) => string;
  readonly quiesce: (session: SessionAppSwitchSessionV1<TApp, TViewState>) => Promise<SessionAppSwitchQuiesceV1>;
  readonly seal: (session: SessionAppSwitchSessionV1<TApp, TViewState>) => void;
  readonly unseal: (session: SessionAppSwitchSessionV1<TApp, TViewState>) => void;
  readonly createInstance: (pluginId: string, app: TApp) => Promise<number>;
  readonly retire: (session: SessionAppSwitchSessionV1<TApp, TViewState>) => Promise<void>;
  readonly defaultViewState: (app: TApp) => TViewState;
  readonly publish: (session: SessionAppSwitchSessionV1<TApp, TViewState>) => void;
  readonly seedLayout: (app: TApp) => void;
  readonly refresh: (session: SessionAppSwitchSessionV1<TApp, TViewState>) => Promise<void>;
  readonly trace?: (step: SessionAppSwitchStepV1, detail: string) => void;
};

/** 🪦️ One TRANSACTIONAL switch of the mounted session to `(pluginId, appId)`.
 *
 * The order is the law, and it is the fix for the mid-chain tear-down measured on 6018: pressing the
 * role chord while a typed operation and an extension invocation were still in flight retired the
 * predecessor underneath them, so the completion pass landed on a revoked actor and every later action
 * failed with `no actor for instance 1` while the shell still showed the OLD surface
 * (`🗑️generated/journey-3/console.txt`, `📓️role-switch-runtime-2026-09-12.md` §2).
 *
 * 1. `quiesce(current)` — the predecessor must be QUIET first. A predecessor that will not go quiet
 *    refuses the switch (`draining`) instead of being torn down under its own in-flight work; the
 *    caller shows that as a busy state and the user's next press succeeds.
 * 2. `seal(current)` — from here every late effect addressed to the predecessor is dropped by the
 *    caller's ledger with one typed diagnostic. It seals BEFORE `createInstance`, not after, because
 *    `createApp` itself revokes the predecessor's captured activation: measured on 6018 as four
 *    `actor-activation.revoked` failures (a completion pass, a history-snapshot read and two scheduled
 *    `flowEvalTick` dispatches) landing 26 ms after the `create` trace and 0 ms after the old `seal`
 *    position — i.e. the window this step exists to close was open for exactly one `createApp`.
 * 3. `createInstance` — before the retire, so a successor that cannot be created leaves the session
 *    exactly as it was rather than blanking the shell. It publishes nothing, so no effect can flow to
 *    it yet. A create that throws `unseal`s the predecessor and rethrows: the surface the user is still
 *    looking at must not stay sealed.
 * 4. `retire(current)` — the caller's close ladder, STARTED but never awaited. A sealed predecessor
 *    owns nothing the successor needs: it publishes nothing, answers nothing and is dropped by the
 *    ledger, so the only thing awaiting its ladder buys is latency. Measured on 6018
 *    (`🗑️generated/journey-5/console.txt`, `🗑️generated/role-switch-runtime/switch-3`): the ladder ran
 *    62–87 s after a long editor session and then threw
 *    `plugin-ui.lifecycle-close-budget-exhausted`, and the successor was published only afterwards —
 *    so the user's ⌘⌥V sat on the old surface for over a minute and then landed on a viewer whose
 *    predecessor had never finished. A retirement that fails is reported through `onRetireFailed`
 *    when it finishes; it can no longer delay, and never blocks, the successor.
 * 5. `publish` → `seedLayout` → `refresh` — the successor's session starts immediately, while the
 *    predecessor retires behind it. No render observes two live instances because the predecessor is
 *    sealed and unpublished from the moment step 2 ran.
 *
 * Switching to the app that is already mounted creates and retires nothing: with a `viewState` it is a
 * view-state republish, without one it is a no-op that answers the live session. */
export async function runSessionAppSwitchV1<TApp, TViewState>(
  ports: SessionAppSwitchPortsV1<TApp, TViewState>,
  request: { readonly pluginId: string; readonly appId: string; readonly viewState?: TViewState },
  onRetireFailed?: (session: SessionAppSwitchSessionV1<TApp, TViewState>, error: unknown) => void,
): Promise<SessionAppSwitchOutcomeV1<TApp, TViewState>> {
  const trace = ports.trace ?? (() => {});
  const app = ports.resolveApp(request.pluginId, request.appId);
  if (app === null) return { status: "unresolvable", session: ports.session, pending: 0 };
  const current = ports.session;
  if (current !== null && current.pluginId === request.pluginId && ports.appId(current.app) === request.appId) {
    if (request.viewState === undefined) return { status: "unchanged", session: current, pending: 0 };
    const republished = { ...current, viewState: request.viewState };
    trace("publish", ports.appId(republished.app));
    ports.publish(republished);
    trace("refresh", ports.appId(republished.app));
    await ports.refresh(republished);
    return { status: "republished", session: republished, pending: 0 };
  }
  if (current !== null) {
    const quiet = await ports.quiesce(current);
    trace("quiesce", `${quiet.settled ? "settled" : "pending"}:${quiet.pending}`);
    if (!quiet.settled) return { status: "draining", session: current, pending: quiet.pending };
    trace("seal", `${current.pluginId}#${current.instanceId}`);
    ports.seal(current);
  }
  let instanceId: number;
  try {
    instanceId = await ports.createInstance(request.pluginId, app);
  } catch (createError) {
    trace("create-failed", ports.appId(app));
    if (current !== null) {
      trace("unseal", `${current.pluginId}#${current.instanceId}`);
      ports.unseal(current);
    }
    throw createError;
  }
  trace("create", `${ports.appId(app)}:${instanceId}`);
  if (current !== null) {
    trace("retire-started", `${current.pluginId}#${current.instanceId}`);
    void ports.retire(current).then(
      () => trace("retire", `${current.pluginId}#${current.instanceId}`),
      (retireError: unknown) => {
        trace("retire-failed", `${current.pluginId}#${current.instanceId}`);
        onRetireFailed?.(current, retireError);
      },
    );
  }
  const next: SessionAppSwitchSessionV1<TApp, TViewState> = { pluginId: request.pluginId, instanceId, app, viewState: request.viewState ?? ports.defaultViewState(app) };
  trace("publish", ports.appId(app));
  ports.publish(next);
  trace("seed", ports.appId(app));
  ports.seedLayout(app);
  trace("refresh", ports.appId(app));
  await ports.refresh(next);
  return { status: current === null ? "mounted" : "switched", session: next, pending: 0 };
}

/** 🚦️ Serializes {@link runSessionAppSwitchV1} so two chords (or a chord and a click) can never
 * interleave two tear-downs — the second request is REFUSED with `busy` rather than queued, because a
 * queued role switch is a switch the user has already stopped wanting by the time it runs. `busy()` is
 * what the caller renders as the role group's `aria-busy`/disabled state. */
export type SessionAppSwitchGateV1<TApp, TViewState> = {
  readonly busy: () => boolean;
  readonly run: (
    ports: SessionAppSwitchPortsV1<TApp, TViewState>,
    request: { readonly pluginId: string; readonly appId: string; readonly viewState?: TViewState },
    onRetireFailed?: (session: SessionAppSwitchSessionV1<TApp, TViewState>, error: unknown) => void,
  ) => Promise<SessionAppSwitchOutcomeV1<TApp, TViewState>>;
};

export function createSessionAppSwitchGateV1<TApp, TViewState>(): SessionAppSwitchGateV1<TApp, TViewState> {
  let running = false;
  return {
    busy: () => running,
    run: async (ports, request, onRetireFailed) => {
      if (running) return { status: "busy", session: ports.session, pending: 0 };
      running = true;
      try {
        return await runSessionAppSwitchV1(ports, request, onRetireFailed);
      } finally {
        running = false;
      }
    },
  };
}

/** 🎛️ The mode one step away from `activeModeId` in declaration order, wrapping at both ends, or `null`
 * when the app declares fewer than two modes. The keyboard half of the navbar mode group
 * (`ui.shell.mode.next`/`.previous`): positional rather than one binding per mode id, because mode ids
 * are plugin-authored while `SHELL_KEYBINDINGS` is a static framework table. An unknown
 * `activeModeId` steps from the first mode, so a session whose mode was dropped by a hot swap still
 * has a keyboard path. */
export function stepModeIdV1(modeIds: readonly string[], activeModeId: string | undefined, step: 1 | -1): string | null {
  if (modeIds.length <= 1) return null;
  const activeIndex = modeIds.indexOf(activeModeId ?? "");
  const from = activeIndex < 0 ? 0 : activeIndex;
  return modeIds[(((from + step) % modeIds.length) + modeIds.length) % modeIds.length] ?? null;
}
