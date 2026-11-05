import type { LocalCatalog, LocalCatalogConfigMutation } from "@semio-tech/framework/config/mutations";

export type VerifiedReadbackListenerV1<T> = Readonly<{ status: (persisted: boolean) => void; archive: (value: T) => void }>;
export type LocalCatalogAcknowledgmentV1 = VerifiedReadbackListenerV1<LocalCatalog>;

/** 🗃️ Checks the requested catalog effect without excluding unrelated concurrent admissions. */
export function localCatalogMutationReflectedV1(catalog: LocalCatalog, mutation: LocalCatalogConfigMutation): boolean {
  const document = catalog.documents.find((entry) => entry.documentId === mutation.documentId);
  if (mutation.mutation === "retireLocalDocument") return document === undefined;
  return document !== undefined && document.schema === mutation.schema && document.name === mutation.name && document.storage === mutation.storage && document.target === mutation.target && document.admittedAtMs === mutation.admittedAtMs
    && document.surface?.pluginId === mutation.surface?.pluginId && document.surface?.appId === mutation.surface?.appId;
}

/** 💾️ A submitted operation is acknowledged only by accepted content in a fresh readback. */
export function awaitVerifiedReadbackV1<T>(port: Readonly<{
  accepts: (value: T) => boolean;
  signal: AbortSignal;
  current: () => boolean;
  timeoutMs: number;
  subscribe: (listener: VerifiedReadbackListenerV1<T>) => () => void;
  submit: () => void;
  probe: () => void;
}>): Promise<T | null> {
  if (port.signal.aborted || !port.current()) return Promise.resolve(null);
  return new Promise((resolve) => {
    let unsubscribe = (): void => undefined;
    let settled = false, submitted = false;
    const finish = (catalog: T | null): void => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      port.signal.removeEventListener("abort", abort);
      unsubscribe();
      resolve(catalog);
    };
    const abort = (): void => finish(null);
    const timer = setTimeout(abort, port.timeoutMs);
    const probe = (): void => {
      if (!port.current()) return finish(null);
      try { port.probe(); } catch { finish(null); }
    };
    port.signal.addEventListener("abort", abort, { once: true });
    try {
      unsubscribe = port.subscribe({
        status: (persisted) => { if (submitted && persisted && !settled) probe(); },
        archive: (catalog) => {
          if (!port.current()) return finish(null);
          if (submitted && !settled && port.accepts(catalog)) finish(catalog);
        },
      });
      if (settled) return unsubscribe();
      submitted = true;
      port.submit();
      if (!settled) probe();
    } catch { finish(null); }
  });
}

/** 🗃️ A catalog mutation requires its exact effect in fresh disk content. */
export function awaitLocalCatalogMutationV1(port: Readonly<{
  mutation: LocalCatalogConfigMutation;
  signal: AbortSignal;
  current: () => boolean;
  timeoutMs: number;
  subscribe: (listener: LocalCatalogAcknowledgmentV1) => () => void;
  submit: () => void;
  probe: () => void;
}>): Promise<LocalCatalog | null> {
  return awaitVerifiedReadbackV1<LocalCatalog>({ ...port, accepts: (catalog) => localCatalogMutationReflectedV1(catalog, port.mutation) });
}
