import * as React from "react";
import { ephemeralSet } from "@semio-tech/framework-transient";
import type { StoragePort } from "@semio-tech/ui/modules/storage-port";
import { shellScopeStorageOrBrowserFallback, useShellScopeOptional } from "@semio-tech/ui/elements/ShellScope";

/** 🗣️ Id of the always-available default terminology (no term substitutions). */
export const UI_TERMINOLOGY_NATIVE = "native";

/** 🗣️ Storage key for the active app terminology id. */
export const UI_CHROME_TERMINOLOGY_STORAGE_KEY = "ui.chrome.terminology";

/** 🗣️ Reads the persisted terminology id from the given shell's storage, defaulting to native. */
export function readStoredUiChromeTerminology(storage: StoragePort): string {
  return storage.get(UI_CHROME_TERMINOLOGY_STORAGE_KEY) || UI_TERMINOLOGY_NATIVE;
}

/** 🗣️ Persists the active terminology id to the given shell's storage. */
export function writeStoredUiChromeTerminology(storage: StoragePort, id: string): void {
  storage.set(UI_CHROME_TERMINOLOGY_STORAGE_KEY, id);
}

const uiTerminologyChangeListeners = ephemeralSet<() => void>("framework.modules.ui.packages.typescript.targets.react.index.tsx.uiTerminologyChangeListeners");

/** 🗣️ React hook giving TS-native products (no Rust `AppDefinition`) read/write access to the shared `ui.chrome.terminology` contract — the same localStorage key the shell's Settings terminology dropdown drives — without depending on `os-shell` state or any Rust type. */
export function useUiTerminology(): { readonly terminology: string; readonly setTerminology: (id: string) => void } {
  const storage = shellScopeStorageOrBrowserFallback(useShellScopeOptional());
  const [terminology, setTerminologyState] = React.useState<string>(() => readStoredUiChromeTerminology(storage));
  React.useEffect(() => {
    const onStorage = (event: StorageEvent) => {
      if (event.key === UI_CHROME_TERMINOLOGY_STORAGE_KEY) setTerminologyState(readStoredUiChromeTerminology(storage));
    };
    const onLocalChange = () => setTerminologyState(readStoredUiChromeTerminology(storage));
    uiTerminologyChangeListeners.add(onLocalChange);
    if (typeof window !== "undefined") window.addEventListener("storage", onStorage);
    return () => {
      uiTerminologyChangeListeners.delete(onLocalChange);
      if (typeof window !== "undefined") window.removeEventListener("storage", onStorage);
    };
  }, [storage]);
  const setTerminology = React.useCallback(
    (id: string) => {
      writeStoredUiChromeTerminology(storage, id);
      for (const listener of uiTerminologyChangeListeners) listener();
    },
    [storage],
  );
  return { terminology, setTerminology };
}
