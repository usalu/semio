/** 🟠️ The shell uri phase of useDirectoryAndRoute. */
import { useCallback, useEffect, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { DockLayoutStore, DockUiStateStore, expandPluginRegistry, resolvePluginRegistryId } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { SHELL_HUB_ROUTE, shellRouteAdmissionV1, shellRouteIsOverlayV1 } from "../../🔀️surface-switch/🟦️.ts";
import { parseShellRoute } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { useUIHistory } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { isPluginInstanceRetiredV1 } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { openDirectoryHomeOwnerV1, directoryHomeOwnerAppV1 } from "../../📇️directory-bootstrap/🟦️.tsx";
import type { useCatalogState } from "../🔶️catalog/🟦️.ts";

/** 🟡 Builds shellUri, canGoBack, canGoForward, canGoUp of useDirectoryAndRoute. */
export function useShellUriState({ identity, verifiedSessionAuthority, hubEnv, hostPlugin, landingApp, session, ensureBackboneWorker, directoryBootstrapEpochRef, directoryHomeOpeningRef, sessionRef, uiLocaleRef, uiTerminologyRef, refreshDirectoryHomeRef, retireDirectoryHomeOwner, directoryHomeOwnerRef, setDirectoryBootstrapUi, presenceCursorRef, hostMode, scope, identityResolutionRef, shellSessionLaneRef, shellRouteLedgerRef, plugins, pluginFilter, catalog }: Pick<ReturnType<typeof useCatalogState>, "identity" | "verifiedSessionAuthority" | "hubEnv" | "hostPlugin" | "landingApp" | "session" | "ensureBackboneWorker" | "directoryBootstrapEpochRef" | "directoryHomeOpeningRef" | "sessionRef" | "uiLocaleRef" | "uiTerminologyRef" | "refreshDirectoryHomeRef" | "retireDirectoryHomeOwner" | "directoryHomeOwnerRef" | "setDirectoryBootstrapUi" | "presenceCursorRef" | "hostMode" | "scope" | "identityResolutionRef" | "shellSessionLaneRef" | "shellRouteLedgerRef" | "plugins" | "pluginFilter" | "catalog">) {
  useEffect(() => {
    if (!identity || !verifiedSessionAuthority || verifiedSessionAuthority.userId !== identity.userId || !hubEnv || !hostPlugin || !landingApp || !session || session.pluginId !== hostPlugin.handle.pluginId) return;
    const ownerApp = directoryHomeOwnerAppV1(landingApp, session.app);
    if (!ownerApp) return;
    const worker = ensureBackboneWorker();
    const bootstrapEpoch = directoryBootstrapEpochRef.current + 1;
    directoryBootstrapEpochRef.current = bootstrapEpoch;
    const openingAbort = new AbortController();
    const visibleSession = session;
    const opening = directoryHomeOpeningRef.current.catch(() => {}).then(async () => {
      if (openingAbort.signal.aborted) return;
      const current = sessionRef.current;
      if (!current || current.pluginId !== visibleSession.pluginId || current.app.id !== visibleSession.app.id || current.instanceId !== visibleSession.instanceId) return;
      const owner = await openDirectoryHomeOwnerV1({
        plugin: hostPlugin.handle,
        app: ownerApp,
        identity: { userId: identity.userId, displayName: identity.displayName },
        instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState },
        baseUrl: identity.hubBaseUrl,
        bootstrapEpoch,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        signal: openingAbort.signal,
        beforeBootstrap: async (openingOwner) => {
          const active = sessionRef.current;
          if (!active || active.pluginId !== openingOwner.plugin.pluginId || active.app.id !== openingOwner.app.id || active.instanceId !== openingOwner.instanceId) throw new Error("directory-bootstrap.visible-owner-stale");
          await refreshDirectoryHomeRef.current(active);
        },
        post: (request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }),
      });
      if (openingAbort.signal.aborted || directoryBootstrapEpochRef.current !== bootstrapEpoch) {
        await retireDirectoryHomeOwner(owner, worker);
        return;
      }
      directoryHomeOwnerRef.current = owner;
      setDirectoryBootstrapUi({ kind: "idle" });
    });
    directoryHomeOpeningRef.current = opening.catch(() => {});
    void opening.catch((ownerError) => {
      if (!openingAbort.signal.aborted && directoryBootstrapEpochRef.current === bootstrapEpoch) setDirectoryBootstrapUi({ kind: "fault", code: ownerError instanceof Error ? ownerError.message : "directory-bootstrap.owner-open-failed" });
    });
    return () => {
      openingAbort.abort("directory-bootstrap-owner-replaced");
      const owner = directoryHomeOwnerRef.current;
      if (!owner || owner.bootstrapEpoch !== bootstrapEpoch) return;
      directoryHomeOwnerRef.current = null;
      void retireDirectoryHomeOwner(owner, worker);
    };
  }, [ensureBackboneWorker, hostPlugin?.handle, identity?.displayName, identity?.hubBaseUrl, identity?.userId, landingApp?.id, retireDirectoryHomeOwner, session?.app.id, session?.instanceId, session?.pluginId, verifiedSessionAuthority?.authorizationGeneration, verifiedSessionAuthority?.sessionBindingSha256]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    const trackPointer = (event: PointerEvent) => {
      presenceCursorRef.current = { x: event.clientX, y: event.clientY };
    };
    window.addEventListener("pointermove", trackPointer, { passive: true });
    return () => window.removeEventListener("pointermove", trackPointer);
  }, []);

  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", hostMode && scope.ownsPage);
  const shellRoute = useMemo(() => parseShellRoute(shellUri.split("?")[0] ?? "/"), [shellUri]);
  /** 🧭️ The path the shell renders as not found — never the hub overlay route, which renders over the underlying session. */
  const notFoundPath = shellRoute.kind === "notFound" && shellRoute.path !== SHELL_HUB_ROUTE ? shellRoute.path : null;
  const shellUriRef = useRef(shellUri);
  shellUriRef.current = shellUri;
  /** 🚦️ A navigation a human or a guest asks for always takes effect: a session-replacing route is invalidated, a new URI
   * goes through history (the route effect applies it), and the URI already shown is applied on the spot — back to a
   * space's index from one of its documents, whose URI is the space's own ({@link createShellRouteLedgerV1}). */
  /** 🪟️ The route the session stands at ({@link shellSessionRouteV1}): the shell URI, or the route an open overlay sits
   * over — kept so a session replaced under `/hub` returns to it. The ref is what the route code reads mid-flight, the
   * state what a render reads. */
  const sessionRouteRef = useRef<string | null>(null);
  const [sessionRoute, setSessionRoute] = useState<string | null>(null);
  /** 🚪️ Hands a session route to the lane unless its admission holds it ({@link shellRouteAdmissionV1}); a held route is
   * applied by the route effect once the identity it waits for resolves. */
  const requestSessionRoute = useCallback((uri: string): void => {
    const admission = shellRouteAdmissionV1(uri, identityResolutionRef.current);
    if (admission === "await-identity" || admission === "await-sign-in") return;
    void shellSessionLaneRef.current.route({ uri }).catch((uriError: unknown) => {
      if (!isPluginInstanceRetiredV1(uriError)) console.warn(`[os-shell] route ${uri} was not applied`, uriError);
    });
  }, []);
  const navigateShellUri = useCallback((uri: string) => {
    const overlay = shellRouteIsOverlayV1(uri);
    if (!overlay) shellRouteLedgerRef.current.invalidate();
    if (uri !== shellUriRef.current) {
      navigateHistory(uri);
      return;
    }
    if (!hostMode || overlay) return;
    requestSessionRoute(uri);
  }, [hostMode, navigateHistory, requestSessionRoute]);

  const shellStorage = scope.storage;
  const dockLayoutStore = useMemo(() => new DockLayoutStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);
  const dockUiStateStore = useMemo(() => new DockUiStateStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);

  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter ? resolvePluginRegistryId(catalog, pluginFilter) : undefined, hostMode);
    if (hostMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [catalog, pluginFilter, plugins, hostMode]);
  return { shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigateHistory, notFoundPath, sessionRouteRef, sessionRoute, setSessionRoute, requestSessionRoute, navigateShellUri, dockLayoutStore, dockUiStateStore, registry };
}
