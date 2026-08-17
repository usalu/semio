// #region Header
// framework/ui/elements/core/📚️I18n/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

// #region Adapters
import { type ShellLocale, type ShellTerminology } from "@semio-tech/framework";
// #endregion Adapters

// #region I18n
export type UiLocale = ShellLocale;

/** @emoji 🪁️ Label pair resolved by the active driver's `labelTier` axis. */
export type UiLabelPair = { readonly normal: string; readonly beginner: string };

/** @emoji 🪁️ Translation leaf with optional manual and tutorial metadata. */
export type UiLabelValue = {
  readonly label: UiLabelPair;
  readonly manual?: string;
  readonly tutorial?: string;
};

/** @emoji 🪁️ Ribbon collection ids for ribbon collection toggles. */
export type UiRibbonParentCategory =
  | "history"
  | "hand"
  | "selection"
  | "lasso"
  | "filter"
  | "open"
  | "save"
  | "transfer"
  | "transform"
  | "create"
  | "view"
  | "actions"
  | "settings"
  | "methods"
  | "mode"
  | "targets"
  | "export"
  | "tools"
  | "utilities"
  | "sync";

/** @emoji 🪁️ Runtime enumeration of {@link UiRibbonParentCategory} in taxonomy order — for grouping/sorting menu rows by category at runtime. */
export const UI_RIBBON_PARENT_CATEGORIES: readonly UiRibbonParentCategory[] = [
  "history",
  "hand",
  "selection",
  "lasso",
  "filter",
  "open",
  "save",
  "transfer",
  "transform",
  "create",
  "view",
  "actions",
  "settings",
  "methods",
  "mode",
  "targets",
  "export",
  "tools",
  "utilities",
  "sync",
];

export type UiRibbonParentEntries = { readonly [K in UiRibbonParentCategory]: UiLabelValue };

type DeepUiTranslationKeys<T, Prefix extends string = ""> = T extends UiLabelValue
  ? Prefix extends ""
    ? never
    : Prefix
  : T extends string
    ? Prefix extends ""
      ? never
      : Prefix
    : T extends number | boolean | null | undefined
      ? never
      : T extends readonly unknown[]
        ? never
        : {
            [K in keyof T & string]: DeepUiTranslationKeys<T[K], Prefix extends "" ? K : `${Prefix}.${K}`>;
          }[keyof T & string];

/** @emoji 🪁️ Domain-neutral chrome translation tree (settings, tooltip, `ui.*`). */
export type UiTranslationSchema = {
  readonly ui: {
    readonly nav: {
      readonly back: UiLabelValue;
      readonly forward: UiLabelValue;
      readonly up: UiLabelValue;
    };
    readonly search: {
      readonly toggle: UiLabelValue;
      readonly close: UiLabelValue;
      readonly title: UiLabelValue;
      readonly description: UiLabelValue;
      readonly placeholder: UiLabelValue;
      readonly empty: UiLabelValue;
      readonly category: {
        readonly panels: UiLabelValue;
        readonly windows: UiLabelValue;
        readonly catalogue: UiLabelValue;
        /** 🏠️ Host-app-neutral label slot for the search category naming the embedding host application
         * (was `studio`, hardcoded to the s-plugin's "Space" identity — see ticket
         * `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`). */
        readonly hostApp: UiLabelValue;
        readonly navigation: UiLabelValue;
      };
    };
    readonly palette: {
      readonly undo: UiLabelValue;
      readonly redo: UiLabelValue;
      readonly goHome: UiLabelValue;
      readonly spawnPrefix: UiLabelValue;
    };
    readonly panel: {
      readonly artifact: UiLabelValue;
      readonly catalogue: UiLabelValue;
      readonly inspection: UiLabelValue;
      readonly parameters: UiLabelValue;
      readonly artifactEmpty: UiLabelValue;
      readonly spawnedAppsSuffix: UiLabelValue;
      readonly sync: UiLabelValue;
      readonly actions: UiLabelValue;
      readonly history: UiLabelValue;
    };
    readonly tree: {
      readonly drag: {
        readonly sort: UiLabelValue;
        readonly sortTarget: UiLabelValue;
        readonly transfer: UiLabelValue;
        readonly transferTarget: UiLabelValue;
      };
    };
    readonly find: {
      readonly toggle: UiLabelValue;
      readonly title: UiLabelValue;
      readonly description: UiLabelValue;
      readonly placeholder: UiLabelValue;
      readonly empty: UiLabelValue;
    };
    readonly fullscreen: {
      readonly toggle: UiLabelValue;
      readonly exit: UiLabelValue;
    };
    readonly mobilePanel: {
      readonly toggle: UiLabelValue;
      readonly app: UiLabelValue;
    };
    readonly panelToggle: {
      readonly topLeft: UiLabelValue;
      readonly topRight: UiLabelValue;
      readonly bottomLeft: UiLabelValue;
      readonly bottomRight: UiLabelValue;
      readonly display: UiLabelValue;
      readonly command: UiLabelValue;
      readonly tool: UiLabelValue;
      readonly overview: UiLabelValue;
      readonly workbench: UiLabelValue;
      readonly details: UiLabelValue;
      readonly settings: UiLabelValue;
      readonly chat: UiLabelValue;
      readonly plugins: UiLabelValue;
    };
    readonly display: {
      readonly tab: {
        readonly windows: UiLabelValue;
        readonly layout: UiLabelValue;
      };
      readonly saveLayout: UiLabelValue;
      readonly saveLayoutPlaceholder: UiLabelValue;
      readonly saveCurrentLayout: UiLabelValue;
      readonly deleteLayout: UiLabelValue;
      readonly emptyShell: UiLabelValue;
      readonly layouts: UiLabelValue;
      readonly saved: UiLabelValue;
      readonly unavailable: UiLabelValue;
    };
    readonly settings: {
      readonly tab: {
        readonly general: UiLabelValue;
        readonly driver: UiLabelValue;
        readonly app: UiLabelValue;
        readonly appearance: UiLabelValue;
        readonly layout: UiLabelValue;
        readonly language: UiLabelValue;
        readonly terminology: UiLabelValue;
        readonly theme: UiLabelValue;
        readonly keybindings: UiLabelValue;
      };
      readonly appearance: {
        readonly light: UiLabelValue;
        readonly dark: UiLabelValue;
        readonly system: UiLabelValue;
      };
      readonly language: {
        readonly en: UiLabelValue;
        readonly de: UiLabelValue;
      };
      readonly terminology: {
        readonly native: UiLabelValue;
        readonly reuse: UiLabelValue;
      };
      readonly app: {
        readonly name: UiLabelValue;
        readonly id: UiLabelValue;
        readonly controller: UiLabelValue;
        readonly plugin: UiLabelValue;
      };
      readonly theme: {
        readonly select: UiLabelValue;
        readonly save: UiLabelValue;
        readonly savePlaceholder: UiLabelValue;
        readonly reset: UiLabelValue;
        readonly export: UiLabelValue;
        readonly import: UiLabelValue;
        readonly delete: UiLabelValue;
        readonly colors: UiLabelValue;
        readonly spacing: UiLabelValue;
        readonly fonts: UiLabelValue;
        readonly strokes: UiLabelValue;
        readonly radii: UiLabelValue;
        readonly opacities: UiLabelValue;
        readonly metrics: UiLabelValue;
        readonly appearances: UiLabelValue;
        readonly dirty: UiLabelValue;
        readonly appearance: {
          readonly light: UiLabelValue;
          readonly dark: UiLabelValue;
        };
        readonly group: {
          readonly board: UiLabelValue;
          readonly map: UiLabelValue;
          readonly canvas: UiLabelValue;
          readonly chrome: UiLabelValue;
        };
      };
      readonly unavailable: UiLabelValue;
      readonly resetDock: UiLabelValue;
    };
    /** 🔌️ Plugin panel (bottom-right dock, next to Settings/Theme): install/uninstall/reload for the
     * dev `PluginSource`'s registry — see `createFrameworkPluginsPanelTabs`. */
    readonly plugins: {
      readonly status: {
        readonly available: UiLabelValue;
        readonly installing: UiLabelValue;
        readonly loaded: UiLabelValue;
        readonly failed: UiLabelValue;
        readonly reloading: UiLabelValue;
      };
      readonly action: {
        readonly install: UiLabelValue;
        readonly uninstall: UiLabelValue;
        readonly reload: UiLabelValue;
      };
      readonly waitingForHost: UiLabelValue;
      readonly unavailable: UiLabelValue;
      readonly source: UiLabelValue;
    };
    readonly command: {
      readonly introduceApp: UiLabelValue;
      readonly playTutorial: UiLabelValue;
      readonly recordTutorial: UiLabelValue;
      readonly setAppearance: UiLabelValue;
      readonly setTheme: UiLabelValue;
      readonly setLayout: UiLabelValue;
      readonly setLocale: UiLabelValue;
      readonly setTerminology: UiLabelValue;
      readonly setDriver: UiLabelValue;
    };
    /** @emoji 🧭️ Labels for `noteShellCommand`'s shell-chrome commandIds (dock drag, window resize/rearrange/
     * activate/close/split, panel toggle/tab) — logged into the plugin's session-only command-history panel. */
    readonly shellCommand: {
      readonly dockMove: UiLabelValue;
      readonly windowResize: UiLabelValue;
      readonly windowMove: UiLabelValue;
      readonly windowActivate: UiLabelValue;
      readonly windowClose: UiLabelValue;
      readonly windowSplit: UiLabelValue;
      readonly panelToggle: UiLabelValue;
      readonly panelTab: UiLabelValue;
    };
    readonly ribbon: {
      readonly group: {
        readonly parent: UiLabelValue;
      };
      readonly parent: UiRibbonParentEntries;
    };
    readonly selection: {
      readonly method: UiLabelValue;
      readonly mode: UiLabelValue;
      readonly rectangle: UiLabelValue;
      readonly lasso: UiLabelValue;
      readonly selective: UiLabelValue;
      readonly additive: UiLabelValue;
      readonly subtractive: UiLabelValue;
      readonly invertive: UiLabelValue;
    };
    readonly common: {
      readonly mixedValues: UiLabelValue;
      readonly name: UiLabelValue;
      readonly save: UiLabelValue;
      readonly loading: UiLabelValue;
      readonly loadingPlugins: UiLabelValue;
      readonly renderError: UiLabelValue;
      readonly noPluginsLoaded: UiLabelValue;
      readonly missingWindow: UiLabelValue;
      readonly home: UiLabelValue;
      readonly backToWorkflow: UiLabelValue;
      readonly execute: UiLabelValue;
      readonly reset: UiLabelValue;
      readonly windowOptions: UiLabelValue;
      readonly focus: UiLabelValue;
      readonly unfocus: UiLabelValue;
      readonly example: UiLabelValue;
      readonly noExample: UiLabelValue;
      readonly loadingSurface: UiLabelValue;
      readonly unknownComponent: UiLabelValue;
      readonly select: UiLabelValue;
      readonly commandPalette: UiLabelValue;
      readonly searchForCommand: UiLabelValue;
      readonly find: UiLabelValue;
      readonly noData: UiLabelValue;
      readonly noFileSystemNodes: UiLabelValue;
      readonly selectTarget: UiLabelValue;
      readonly selectOption: UiLabelValue;
      readonly noOptionsFound: UiLabelValue;
      readonly close: UiLabelValue;
      readonly newWindow: UiLabelValue;
      readonly minimize: UiLabelValue;
      readonly maximize: UiLabelValue;
      readonly action: UiLabelValue;
      readonly actions: UiLabelValue;
      readonly utilities: UiLabelValue;
      readonly retry: UiLabelValue;
      readonly somethingWentWrong: UiLabelValue;
      readonly doubleClickToEdit: UiLabelValue;
      readonly importFile: UiLabelValue;
      readonly clear: UiLabelValue;
      readonly collapse: UiLabelValue;
      readonly expand: UiLabelValue;
      readonly cancel: UiLabelValue;
      readonly error: UiLabelValue;
    };
    readonly contextMenu: {
      readonly select: UiLabelValue;
      readonly deselect: UiLabelValue;
      readonly selectAll: UiLabelValue;
      readonly clearSelection: UiLabelValue;
      readonly selectSameKind: UiLabelValue;
      readonly duplicate: UiLabelValue;
      readonly delete: UiLabelValue;
      readonly zoomToSelection: UiLabelValue;
      readonly focusZoom: UiLabelValue;
      readonly openSource: UiLabelValue;
      readonly fitWorld: UiLabelValue;
      readonly cut: UiLabelValue;
      readonly copy: UiLabelValue;
      readonly paste: UiLabelValue;
      readonly rename: UiLabelValue;
      readonly formatDocument: UiLabelValue;
      readonly lintDocument: UiLabelValue;
      readonly suggestCompletions: UiLabelValue;
      readonly selectToken: UiLabelValue;
      readonly selectLine: UiLabelValue;
      readonly hide: UiLabelValue;
      readonly show: UiLabelValue;
      readonly lock: UiLabelValue;
      readonly unlock: UiLabelValue;
    };
    readonly host: {
      readonly emptyScene: UiLabelValue;
      readonly preview: UiLabelValue;
      readonly sourceAvailable: UiLabelValue;
      readonly blockImage: UiLabelValue;
      readonly blockTable: UiLabelValue;
      readonly blockMath: UiLabelValue;
      readonly blockInk: UiLabelValue;
      readonly blockGroup: UiLabelValue;
      readonly blockText: UiLabelValue;
      readonly checkingPlacement: UiLabelValue;
      readonly noPlacement: UiLabelValue;
      readonly canvasUnavailable: UiLabelValue;
      readonly rendering: UiLabelValue;
      readonly documentPlaceholder: UiLabelValue;
      readonly languageDocument: UiLabelValue;
      readonly iconShot: UiLabelValue;
      readonly projection: UiLabelValue;
      readonly perspective: UiLabelValue;
      readonly orthographic: UiLabelValue;
    };
    readonly chat: {
      readonly readyFor: UiLabelValue;
      readonly localOnly: UiLabelValue;
      readonly instructions: UiLabelValue;
      readonly placeholder: UiLabelValue;
      readonly savedLocally: UiLabelValue;
      readonly send: UiLabelValue;
    };
    readonly docs: {
      readonly navigation: {
        readonly previous: UiLabelValue;
        readonly next: UiLabelValue;
      };
    };
    readonly blockList: {
      readonly steps: UiLabelValue;
      readonly addStep: UiLabelValue;
    };
    readonly ring: {
      readonly demo: UiLabelValue;
    };
    readonly iconSelector: {
      readonly mode: {
        readonly url: UiLabelValue;
        readonly shortcode: UiLabelValue;
        readonly math: UiLabelValue;
        readonly data: UiLabelValue;
        readonly emoji: UiLabelValue;
        readonly text: UiLabelValue;
        readonly vector: UiLabelValue;
      };
    };
    readonly stepper: {
      readonly demo: UiLabelValue;
    };
    readonly engagement: {
      readonly actions: UiLabelValue;
      readonly viewport: UiLabelValue;
    };
    readonly windowSearch: {
      readonly title: UiLabelValue;
      readonly action: UiLabelValue;
      readonly actionActive: UiLabelValue;
      readonly suggestions: UiLabelValue;
      readonly noMatches: UiLabelValue;
    };
    readonly flowSpotlight: {
      readonly typeToAdd: UiLabelValue;
      readonly collapseSuggestions: UiLabelValue;
      readonly showAllSuggestions: UiLabelValue;
    };
    readonly sync: {
      readonly attach: UiLabelValue;
      readonly detach: UiLabelValue;
    };
    readonly ink: {
      readonly link: UiLabelValue;
      readonly linkUrlPrompt: UiLabelValue;
    };
    readonly surfaceContextMenu: {
      readonly file: UiLabelValue;
      readonly workspace: UiLabelValue;
      readonly canvas: UiLabelValue;
      readonly scene: UiLabelValue;
      readonly placementSuggestions: UiLabelValue;
      readonly node: UiLabelValue;
      readonly flow: UiLabelValue;
      readonly row: UiLabelValue;
      readonly paint: UiLabelValue;
      readonly board: UiLabelValue;
      readonly ink: UiLabelValue;
      readonly history: UiLabelValue;
      readonly step: UiLabelValue;
      readonly diff: UiLabelValue;
      readonly event: UiLabelValue;
      readonly editor: UiLabelValue;
      readonly map: UiLabelValue;
    };
    /** ⚖️ Mutation-outcome vocabulary (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-
     * AND-FIRST-CLASS-CONFLICTS` §C2/§C3/§C9) — `level.*` mirrors `Severity`, `code.*` the frozen
     * seven `mutation.*` codes (UI localizes by code, never by parsing the English `message` prose),
     * `policy.*` the three `MergePolicy` choices plus the settings row's own label, `rejected.*` the
     * `ShellHost` toast for a rejected local dispatch. */
    readonly mutation: {
      readonly level: {
        readonly info: UiLabelValue;
        readonly warning: UiLabelValue;
        readonly error: UiLabelValue;
        readonly fatal: UiLabelValue;
      };
      readonly code: {
        readonly targetMissing: UiLabelValue;
        readonly noOp: UiLabelValue;
        readonly partial: UiLabelValue;
        readonly clamped: UiLabelValue;
        readonly duplicateId: UiLabelValue;
        readonly invariant: UiLabelValue;
        readonly cascade: UiLabelValue;
      };
      readonly policy: {
        readonly laissezFaire: { readonly label: UiLabelValue; readonly description: UiLabelValue };
        readonly normal: { readonly label: UiLabelValue; readonly description: UiLabelValue };
        readonly vigilant: { readonly label: UiLabelValue; readonly description: UiLabelValue };
        readonly setting: { readonly label: UiLabelValue };
      };
      readonly rejected: {
        readonly title: UiLabelValue;
        readonly body: UiLabelValue;
      };
    };
    /** ⚔️ First-class conflict vocabulary (contract freeze §C5/§C9) — the `ChromePanels` Conflicts
     * panel's own chrome strings; per-conflict `MutationMessage`s reuse `ui.mutation.*` above. */
    readonly conflict: {
      readonly panel: UiLabelValue;
      readonly accept: UiLabelValue;
      readonly discard: UiLabelValue;
      readonly quarantined: UiLabelValue;
      readonly degraded: UiLabelValue;
    };
    /** 👥️ `PresenceBar` roster chrome (ticket `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`
     * lane 2-F) — the `(space, document, surface)` peer list's own aria strings; per-peer display names are
     * runtime data, never a translation key. */
    readonly presence: {
      readonly roster: UiLabelValue;
      readonly empty: UiLabelValue;
      readonly overflow: UiLabelValue;
      readonly role: {
        readonly author: UiLabelValue;
        readonly spectator: UiLabelValue;
      };
    };
  };
  readonly settings: {
    readonly layout: {
      readonly desktop: UiLabelValue;
      readonly tablet: UiLabelValue;
      readonly mobile: UiLabelValue;
    };
    readonly driver: {
      readonly select: UiLabelValue;
      readonly default: UiLabelValue;
      readonly compact: UiLabelValue;
      readonly labels: UiLabelValue;
      readonly labelsOption: {
        readonly full: UiLabelValue;
        readonly icons: UiLabelValue;
      };
      readonly labelTier: UiLabelValue;
      readonly labelTierOption: {
        readonly beginner: UiLabelValue;
        readonly normal: UiLabelValue;
      };
      readonly drag: UiLabelValue;
      readonly dragOption: {
        readonly handle: UiLabelValue;
        readonly surface: UiLabelValue;
      };
      readonly chrome: UiLabelValue;
      readonly chromeOption: {
        readonly always: UiLabelValue;
        readonly hover: UiLabelValue;
      };
      readonly gumball: UiLabelValue;
      readonly gumballOption: {
        readonly always: UiLabelValue;
        readonly hover: UiLabelValue;
      };
      readonly tooltips: UiLabelValue;
      readonly tooltipsOption: {
        readonly full: UiLabelValue;
        readonly minimal: UiLabelValue;
        readonly none: UiLabelValue;
      };
      readonly hotkeys: UiLabelValue;
      readonly hotkeysOption: {
        readonly inline: UiLabelValue;
        readonly tooltip: UiLabelValue;
        readonly none: UiLabelValue;
      };
      readonly save: UiLabelValue;
      readonly savePlaceholder: UiLabelValue;
      readonly delete: UiLabelValue;
      readonly dirty: UiLabelValue;
    };
    readonly keybindings: {
      readonly capture: UiLabelValue;
      readonly reset: UiLabelValue;
      readonly conflict: UiLabelValue;
      readonly pressKeys: UiLabelValue;
    };
  };
  readonly tooltip: {
    readonly manual: UiLabelValue;
    readonly tutorial: UiLabelValue;
  };
  readonly introduction: {
    readonly skip: UiLabelValue;
    readonly back: UiLabelValue;
    readonly next: UiLabelValue;
    readonly done: UiLabelValue;
  };
  readonly tutorial: {
    readonly play: UiLabelValue;
    readonly pause: UiLabelValue;
    readonly stop: UiLabelValue;
    readonly rate: UiLabelValue;
    readonly mute: UiLabelValue;
    readonly captions: UiLabelValue;
    readonly record: UiLabelValue;
    readonly recording: UiLabelValue;
    readonly addChapter: UiLabelValue;
    readonly chapter: UiLabelValue;
  };
};

/** @emoji 🪁️ Dot-path union of keys in {@link UiTranslationSchema}. */
export type UiTranslationKey = DeepUiTranslationKeys<UiTranslationSchema>;

/** @emoji 🪁️ Compile-time check that ribbon collection ids have chrome translation keys. */
export type AssertUiRibbonParentKeysCovered<Categories extends string> = {
  readonly [K in Categories]: `ui.ribbon.parent.${K}` extends UiTranslationKey ? true : false;
}[Categories] extends true
  ? true
  : false;

/** @emoji 🪁️ Compile-time check that every {@link UiLocale} has a settings-dropdown label key. */
export type AssertUiSettingsLanguageKeysCovered<Locales extends string> = {
  readonly [L in Locales]: `ui.settings.language.${L}` extends UiTranslationKey ? true : false;
}[Locales] extends true
  ? true
  : false;

/** @emoji 🗣️ Chrome-known terminology ids — single source `@semio-tech/framework`'s `ShellTerminology`; app-declared ids beyond this set fall back to their raw id in the dropdown. */
export type UiChromeTerminologyId = ShellTerminology;

/** @emoji 🗣️ Compile-time check that every {@link UiChromeTerminologyId} has a settings-dropdown label key. */
export type AssertUiSettingsTerminologyKeysCovered<Ids extends string> = {
  readonly [I in Ids]: `ui.settings.terminology.${I}` extends UiTranslationKey ? true : false;
}[Ids] extends true
  ? true
  : false;

/** @emoji 🪁️ Typed translate function for domain-neutral chrome keys. */
export type UiTranslateFn = <K extends UiTranslationKey>(key: K, options?: Record<string, unknown>) => unknown;

/** @emoji 🪁️ Shared UI i18n port (wraps i18next; do not import i18next outside this bundle). */
export interface UiI18nPort {
  readonly t: UiTranslateFn;
  changeLanguage(locale: UiLocale): Promise<unknown>;
  readonly language: string | undefined;
  readonly resolvedLanguage: string | undefined;
  readonly isInitialized: boolean;
}

declare const uiRegisteredTranslationKeyBrand: unique symbol;
/** @emoji 🪁️ Key branded by {@link registerUiTranslationBundles} — only obtainable from the caster it
 * returns, so a value of this type provably exists in every {@link UiLocale} bundle registered together
 * with it. Products (coda, compose, …) hold this instead of hand-rolling their own translation-key union
 * and casting into `useLabel`. */
export type UiRegisteredTranslationKey = string & { readonly [uiRegisteredTranslationKeyBrand]: true };
// #endregion I18n
