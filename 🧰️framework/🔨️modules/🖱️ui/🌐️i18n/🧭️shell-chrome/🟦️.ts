import type { UiTranslationSchema } from "../../🧱️elements/📚️I18n/🟦️.tsx";

/** 🧭️ Shell navigation, search, panel and display chrome labels (`de`). */
export const shellChromeDe = {
  nav: {
    back: {
      label: {
        normal: "Zurück",
        beginner: "Zurück",
      },
    },
    forward: {
      label: {
        normal: "Vorwärts",
        beginner: "Vorwärts",
      },
    },
    up: {
      label: {
        normal: "Eine Ebene hoch",
        beginner: "Eine Ebene hoch",
      },
    },
  },
  search: {
    toggle: {
      label: {
        normal: "Suche",
        beginner: "Suche",
      },
    },
    close: {
      label: {
        normal: "Suche schließen",
        beginner: "Suche schließen",
      },
    },
    title: {
      label: {
        normal: "Suche",
        beginner: "Suche",
      },
    },
    description: {
      label: {
        normal: "Nach Elementen suchen",
        beginner: "Nach Elementen suchen",
      },
    },
    placeholder: {
      label: {
        normal: "Suchen...",
        beginner: "Suchen...",
      },
    },
    empty: {
      label: {
        normal: "Keine Ergebnisse gefunden.",
        beginner: "Keine Ergebnisse gefunden.",
      },
    },
    category: {
      panels: { label: { normal: "Panels", beginner: "Panels" } },
      windows: { label: { normal: "Fenster", beginner: "Fenster" } },
      catalogue: { label: { normal: "Katalog", beginner: "Katalog" } },
      // 🏠️ "Space" here is a deliberate, deferred duplicate of the host plugin's own manifest label
      // (`App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space"))`,
      // ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs:869) — reading it from there via
      // `resolveManifestLabel(hostApp.label, …)` (the same pattern `appWindowLabel` already uses)
      // is the correct fix, EXCEPT `ShellHost/🟦️.tsx`'s `hostApp` lookup (line ~1132,
      // `manifest.apps.find(app => app.id === hostConfig?.hostAppId)`) is ALREADY always
      // `undefined`: `hostConfig.hostAppId` is the raw Cargo.toml `host = { shell = "studio" }`
      // alias, never the real dialect-derived `AppDefinition.id`
      // (`s.space.studio@1/*#editor`) — a pre-existing bug the same file's own w4-h comment
      // (lines 4112-4121) already documents and declines to fix. Wiring this label to
      // `hostApp?.label` today would render an EMPTY category header, not "Space" — a regression.
      // Fix plan (needs a new field, not a string-matching workaround): add
      // `AppDefinition.host_role: Option<HostRole>` (`Landing`/`Host`) in
      // `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (~3034), a `.host_role(...)` builder
      // method on `AppBuilder`/forwarded by `EditorBuilder`/`ViewerBuilder`
      // (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), set it in
      // `create_home_app()`/`create_space_app()`, regenerate the TS mirror, then have
      // `ShellHost/🟦️.tsx:1132-1133` match on `hostRole` instead of the broken id
      // comparison. See
      // `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/STUBS-AND-PLACEHOLDERS-COMPLETION/📓️hostapp-label-layering.md`.
      hostApp: { label: { normal: "Space", beginner: "Space" } },
      navigation: { label: { normal: "Navigation", beginner: "Navigation" } },
    },
  },
  palette: {
    undo: { label: { normal: "Rückgängig", beginner: "Rückgängig" } },
    redo: { label: { normal: "Wiederholen", beginner: "Wiederholen" } },
    goHome: { label: { normal: "Nach Hause", beginner: "Nach Hause" } },
    spawnPrefix: { label: { normal: "Erzeugen", beginner: "Erzeugen" } },
  },
  panel: {
    artifact: { label: { normal: "Dokument", beginner: "Dokument" } },
    catalogue: { label: { normal: "Katalog", beginner: "Katalog" } },
    inspection: { label: { normal: "Inspektion", beginner: "Inspektion" } },
    parameters: { label: { normal: "Parameter", beginner: "Parameter" } },
    artifactEmpty: { label: { normal: "—", beginner: "—" } },
    spawnedAppsSuffix: { label: { normal: "gestartete App(s)", beginner: "gestartete App(s)" } },
    sync: { label: { normal: "Synchronisierung", beginner: "Synchronisierung" } },
    actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
    history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
  },
  tree: {
    drag: {
      sort: { label: { normal: "Sortieren", beginner: "Zeile sortieren" } },
      sortTarget: { label: { normal: "Linksklick gedrückt halten, um {{target}} zu ziehen", beginner: "Linksklick gedrückt halten, um {{target}} zu ziehen" } },
      transfer: { label: { normal: "Verschieben", beginner: "In ein Fenster ziehen" } },
      transferTarget: { label: { normal: "Linksklick gedrückt halten, um {{target}} zu ziehen", beginner: "Linksklick gedrückt halten, um {{target}} zu ziehen" } },
    },
  },
  find: {
    toggle: {
      label: {
        normal: "Finden",
        beginner: "Im aktuellen Kontext finden",
      },
    },
    title: {
      label: {
        normal: "Finden",
        beginner: "Finden",
      },
    },
    description: {
      label: {
        normal: "Elemente in dieser Ansicht finden",
        beginner: "Elemente in dieser Ansicht finden",
      },
    },
    placeholder: {
      label: {
        normal: "Finden...",
        beginner: "Finden...",
      },
    },
    empty: {
      label: {
        normal: "Keine Ergebnisse gefunden.",
        beginner: "Keine Ergebnisse gefunden.",
      },
    },
  },
  fullscreen: {
    toggle: {
      label: {
        normal: "Vollbild",
        beginner: "Vollbild",
      },
    },
    exit: {
      label: {
        normal: "Vollbild beenden",
        beginner: "Vollbild beenden",
      },
    },
  },
  mobilePanel: {
    toggle: {
      label: {
        normal: "Panel",
        beginner: "Panel",
      },
    },
    app: {
      label: {
        normal: "App",
        beginner: "App",
      },
    },
  },
  panelToggle: {
    topLeft: {
      label: {
        normal: "Oben links",
        beginner: "Oben links",
      },
    },
    topRight: {
      label: {
        normal: "Oben rechts",
        beginner: "Oben rechts",
      },
    },
    bottomLeft: {
      label: {
        normal: "Unten links",
        beginner: "Unten links",
      },
    },
    bottomRight: {
      label: {
        normal: "Unten rechts",
        beginner: "Unten rechts",
      },
    },
    display: {
      label: {
        normal: "Anzeige",
        beginner: "Anzeige",
      },
    },
    command: {
      label: {
        normal: "Befehl",
        beginner: "Befehl",
      },
    },
    tool: {
      label: {
        normal: "Werkzeug",
        beginner: "Werkzeug",
      },
    },
    overview: {
      label: {
        normal: "Übersicht",
        beginner: "Übersicht",
      },
    },
    workbench: {
      label: {
        normal: "Arbeitsbereich",
        beginner: "Arbeitsbereich",
      },
    },
    details: {
      label: {
        normal: "Details",
        beginner: "Details",
      },
    },
    settings: {
      label: {
        normal: "Einstellungen",
        beginner: "Einstellungen",
      },
    },
    chat: {
      label: {
        normal: "Chat",
        beginner: "Chat",
      },
    },
    plugins: {
      label: {
        normal: "Plugins",
        beginner: "Plugins",
      },
    },
    taskManager: {
      label: {
        normal: "Aufgaben",
        beginner: "Aufgaben",
      },
    },
  },
  display: {
    tab: {
      windows: { label: { normal: "Fenster", beginner: "Fenster" } },
      layout: { label: { normal: "Layout", beginner: "Layout" } },
    },
    saveLayout: { label: { normal: "Layout speichern", beginner: "Layout speichern" } },
    saveLayoutPlaceholder: { label: { normal: "Layoutname", beginner: "Layoutname" } },
    saveCurrentLayout: { label: { normal: "Aktuelles Layout speichern", beginner: "Aktuelles Layout speichern" } },
    deleteLayout: { label: { normal: "Löschen", beginner: "Löschen" } },
    emptyShell: {
      label: {
        normal: "Fenster aus Anzeige in der Navigationsleiste hierher ziehen oder ein gespeichertes Layout wiederherstellen.",
        beginner: "Fenster aus Anzeige in der Navigationsleiste hierher ziehen oder ein gespeichertes Layout wiederherstellen.",
      },
    },
    layouts: { label: { normal: "Layouts", beginner: "Layouts" } },
    saved: { label: { normal: "Gespeichert", beginner: "Gespeichert" } },
    unavailable: { label: { normal: "Anzeige nicht verfügbar", beginner: "Anzeige nicht verfügbar" } },
  },
} satisfies Pick<UiTranslationSchema["ui"], "nav" | "search" | "palette" | "panel" | "tree" | "find" | "fullscreen" | "mobilePanel" | "panelToggle" | "display">;

/** 🖥️ Shell navigation, search, panel and display chrome labels (`en`). */
export const shellChromeEn = {
  nav: {
    back: {
      label: {
        normal: "Go back",
        beginner: "Go back",
      },
    },
    forward: {
      label: {
        normal: "Go forward",
        beginner: "Go forward",
      },
    },
    up: {
      label: {
        normal: "Go up one level",
        beginner: "Go up one level",
      },
    },
  },
  search: {
    toggle: {
      label: {
        normal: "Search",
        beginner: "Search",
      },
    },
    close: {
      label: {
        normal: "Close search",
        beginner: "Close search",
      },
    },
    title: {
      label: {
        normal: "Search",
        beginner: "Search",
      },
    },
    description: {
      label: {
        normal: "Search for items",
        beginner: "Search for items",
      },
    },
    placeholder: {
      label: {
        normal: "Search...",
        beginner: "Search...",
      },
    },
    empty: {
      label: {
        normal: "No results found.",
        beginner: "No results found.",
      },
    },
    category: {
      panels: { label: { normal: "Panels", beginner: "Panels" } },
      windows: { label: { normal: "Windows", beginner: "Windows" } },
      catalogue: { label: { normal: "Catalogue", beginner: "Catalogue" } },
      // 🏠️ "Space" here is a deliberate, deferred duplicate of the host plugin's own manifest label
      // (`App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space"))`,
      // ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs:869) — reading it from there via
      // `resolveManifestLabel(hostApp.label, …)` (the same pattern `appWindowLabel` already uses)
      // is the correct fix, EXCEPT `ShellHost/🟦️.tsx`'s `hostApp` lookup (line ~1132,
      // `manifest.apps.find(app => app.id === hostConfig?.hostAppId)`) is ALREADY always
      // `undefined`: `hostConfig.hostAppId` is the raw Cargo.toml `host = { shell = "studio" }`
      // alias, never the real dialect-derived `AppDefinition.id`
      // (`s.space.studio@1/*#editor`) — a pre-existing bug the same file's own w4-h comment
      // (lines 4112-4121) already documents and declines to fix. Wiring this label to
      // `hostApp?.label` today would render an EMPTY category header, not "Space" — a regression.
      // Fix plan (needs a new field, not a string-matching workaround): add
      // `AppDefinition.host_role: Option<HostRole>` (`Landing`/`Host`) in
      // `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (~3034), a `.host_role(...)` builder
      // method on `AppBuilder`/forwarded by `EditorBuilder`/`ViewerBuilder`
      // (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), set it in
      // `create_home_app()`/`create_space_app()`, regenerate the TS mirror, then have
      // `ShellHost/🟦️.tsx:1132-1133` match on `hostRole` instead of the broken id
      // comparison. See
      // `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/STUBS-AND-PLACEHOLDERS-COMPLETION/📓️hostapp-label-layering.md`.
      hostApp: { label: { normal: "Space", beginner: "Space" } },
      navigation: { label: { normal: "Navigation", beginner: "Navigation" } },
    },
  },
  palette: {
    undo: { label: { normal: "Undo", beginner: "Undo" } },
    redo: { label: { normal: "Redo", beginner: "Redo" } },
    goHome: { label: { normal: "Go Home", beginner: "Go Home" } },
    spawnPrefix: { label: { normal: "Spawn", beginner: "Spawn" } },
  },
  panel: {
    artifact: { label: { normal: "Artifact", beginner: "Artifact" } },
    catalogue: { label: { normal: "Catalogue", beginner: "Catalogue" } },
    inspection: { label: { normal: "Inspection", beginner: "Inspection" } },
    parameters: { label: { normal: "Parameters", beginner: "Parameters" } },
    artifactEmpty: { label: { normal: "—", beginner: "—" } },
    spawnedAppsSuffix: { label: { normal: "spawned app(s)", beginner: "spawned app(s)" } },
    sync: { label: { normal: "Sync", beginner: "Sync" } },
    actions: { label: { normal: "Actions", beginner: "Actions" } },
    history: { label: { normal: "History", beginner: "History" } },
  },
  tree: {
    drag: {
      sort: { label: { normal: "Reorder", beginner: "Reorder row" } },
      sortTarget: { label: { normal: "Click and hold left click to drag {{target}}", beginner: "Click and hold left click to drag {{target}}" } },
      transfer: { label: { normal: "Drag to window", beginner: "Drag into a window" } },
      transferTarget: { label: { normal: "Click and hold left click to drag {{target}}", beginner: "Click and hold left click to drag {{target}}" } },
    },
  },
  find: {
    toggle: {
      label: {
        normal: "Find",
        beginner: "Find in view",
      },
    },
    title: {
      label: {
        normal: "Find",
        beginner: "Find",
      },
    },
    description: {
      label: {
        normal: "Find items in this view",
        beginner: "Find items in this view",
      },
    },
    placeholder: {
      label: {
        normal: "Find...",
        beginner: "Find...",
      },
    },
    empty: {
      label: {
        normal: "No results found.",
        beginner: "No results found.",
      },
    },
  },
  fullscreen: {
    toggle: {
      label: {
        normal: "Fullscreen",
        beginner: "Fullscreen",
      },
    },
    exit: {
      label: {
        normal: "Exit Fullscreen",
        beginner: "Exit Fullscreen",
      },
    },
  },
  mobilePanel: {
    toggle: {
      label: {
        normal: "Panel",
        beginner: "Panel",
      },
    },
    app: {
      label: {
        normal: "App",
        beginner: "App",
      },
    },
  },
  panelToggle: {
    topLeft: {
      label: {
        normal: "Top Left",
        beginner: "Top Left",
      },
    },
    topRight: {
      label: {
        normal: "Top Right",
        beginner: "Top Right",
      },
    },
    bottomLeft: {
      label: {
        normal: "Bottom Left",
        beginner: "Bottom Left",
      },
    },
    bottomRight: {
      label: {
        normal: "Bottom Right",
        beginner: "Bottom Right",
      },
    },
    display: {
      label: {
        normal: "Display",
        beginner: "Display",
      },
    },
    command: {
      label: {
        normal: "Command",
        beginner: "Command",
      },
    },
    tool: {
      label: {
        normal: "Tool",
        beginner: "Tool",
      },
    },
    overview: {
      label: {
        normal: "Overview",
        beginner: "Overview",
      },
    },
    workbench: {
      label: {
        normal: "Workbench",
        beginner: "Workbench",
      },
    },
    details: {
      label: {
        normal: "Details",
        beginner: "Details",
      },
    },
    settings: {
      label: {
        normal: "Settings",
        beginner: "Settings",
      },
    },
    chat: {
      label: {
        normal: "Chat",
        beginner: "Chat",
      },
    },
    plugins: {
      label: {
        normal: "Plugins",
        beginner: "Plugins",
      },
    },
    taskManager: {
      label: {
        normal: "Tasks",
        beginner: "Tasks",
      },
    },
  },
  display: {
    tab: {
      windows: { label: { normal: "Windows", beginner: "Windows" } },
      layout: { label: { normal: "Layout", beginner: "Layout" } },
    },
    saveLayout: { label: { normal: "Save layout", beginner: "Save layout" } },
    saveLayoutPlaceholder: { label: { normal: "Layout name", beginner: "Layout name" } },
    saveCurrentLayout: { label: { normal: "Save current layout", beginner: "Save current layout" } },
    deleteLayout: { label: { normal: "Delete", beginner: "Delete" } },
    emptyShell: {
      label: {
        normal: "Drag windows from Display in the navbar, or restore a saved layout.",
        beginner: "Drag windows from Display in the navbar, or restore a saved layout.",
      },
    },
    layouts: { label: { normal: "Layouts", beginner: "Layouts" } },
    saved: { label: { normal: "Saved", beginner: "Saved" } },
    unavailable: { label: { normal: "Display unavailable", beginner: "Display unavailable" } },
  },
} satisfies Pick<UiTranslationSchema["ui"], "nav" | "search" | "palette" | "panel" | "tree" | "find" | "fullscreen" | "mobilePanel" | "panelToggle" | "display">;
