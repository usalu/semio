/** 🌐️ The UI i18n port on its own: the domain-neutral chrome bundles (English and German), the shared
 * i18next-backed {@link uiI18n}, product bundle registration, per-shell instances and locale resolution —
 * importable without the rest of the React target as `@semio-tech/ui-react/i18n`. i18next and react-i18next stay
 * behind the owned {@link UiI18nPort}; the React target barrel re-exports everything here unchanged.
 *
 * @see ../../../🧱️elements/📚️I18n/🟦️.tsx — the port interface, label shapes and key types
 * @see ../🟦️.tsx — the barrel that re-exports this module
 */

import i18next from "i18next";
import { initReactI18next } from "react-i18next";
import { createBrowserStoragePort, ephemeralSet, type ShellLocale, type StoragePort } from "@semio-tech/framework";
import {
  resolveUiLabel,
  type DeepUiTranslationKeys,
  type UiI18nPort,
  type UiLabelPair,
  type UiLabelValue,
  type UiLocale,
  type UiRegisteredTranslationKey,
  type UiRibbonParentEntries,
  type UiTranslateFn,
  type UiTranslationKey,
  type UiTranslationSchema,
} from "../../../🧱️elements/📚️I18n/🟦️.tsx";

export { resolveUiLabel };
export type { DeepUiTranslationKeys, UiI18nPort, UiLabelPair, UiLabelValue, UiLocale, UiRegisteredTranslationKey, UiTranslateFn, UiTranslationKey, UiTranslationSchema };

// #region 🌐️LocaleStorage
/** @emoji 🌐️ Storage key for the active UI locale. */
export const UI_CHROME_LOCALE_STORAGE_KEY = "ui.chrome.locale";

/** @emoji 🌐️ Reads the persisted UI locale from the given shell's storage, if any. */
export function readStoredUiChromeLocale(storage: StoragePort): UiLocale | null {
  const raw = storage.get(UI_CHROME_LOCALE_STORAGE_KEY);
  return raw === "en" || raw === "de" ? raw : null;
}

/** @emoji 🌐️ Persists the active UI locale to the given shell's storage. */
export function writeStoredUiChromeLocale(storage: StoragePort, locale: UiLocale): void {
  storage.set(UI_CHROME_LOCALE_STORAGE_KEY, locale);
}
// #endregion 🌐️LocaleStorage

// #region 🇩️🇪️ German Bundle
// German (`de`) translation bundle: ribbon-parent labels here, plus the nested `de` translation tree further below
// inside {@link uiChromeTranslationBundles} (kept as one object so both locales satisfy the same schema).

const uiRibbonParentDe: UiRibbonParentEntries = {
  history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
  hand: { label: { normal: "Hand", beginner: "Hand" } },
  selection: { label: { normal: "Auswahl", beginner: "Auswahl" } },
  lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
  filter: { label: { normal: "Filter", beginner: "Filter" } },
  open: { label: { normal: "Öffnen", beginner: "Öffnen" } },
  save: { label: { normal: "Speichern", beginner: "Speichern" } },
  transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
  transform: { label: { normal: "Transformieren", beginner: "Transformieren" } },
  create: { label: { normal: "Erstellen", beginner: "Erstellen" } },
  view: { label: { normal: "Ansicht", beginner: "Ansicht" } },
  actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
  settings: { label: { normal: "Einstellungen", beginner: "Einstellungen" } },
  methods: { label: { normal: "Methoden", beginner: "Methoden" } },
  mode: { label: { normal: "Modus", beginner: "Modus" } },
  targets: { label: { normal: "Ziele", beginner: "Ziele" } },
  export: { label: { normal: "Export", beginner: "Export" } },
  tools: { label: { normal: "Werkzeuge", beginner: "Werkzeuge" } },
  utilities: { label: { normal: "Hilfsmittel", beginner: "Hilfsmittel" } },
  sync: { label: { normal: "Sync", beginner: "Sync" } },
};

// #endregion 🇩️🇪️ German Bundle

// #region 🇬️🇧️ English Bundle
// English (`en`) translation bundle: ribbon-parent labels here, plus the nested `en` translation tree further below
// inside {@link uiChromeTranslationBundles} (kept as one object so both locales satisfy the same schema).

const uiRibbonParentEn: UiRibbonParentEntries = {
  history: { label: { normal: "History", beginner: "History" } },
  hand: { label: { normal: "Hand", beginner: "Hand" } },
  selection: { label: { normal: "Selection", beginner: "Selection" } },
  lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
  filter: { label: { normal: "Filter", beginner: "Filter" } },
  open: { label: { normal: "Open", beginner: "Open" } },
  save: { label: { normal: "Save", beginner: "Save" } },
  transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
  transform: { label: { normal: "Transform", beginner: "Transform" } },
  create: { label: { normal: "Create", beginner: "Create" } },
  view: { label: { normal: "View", beginner: "View" } },
  actions: { label: { normal: "Actions", beginner: "Actions" } },
  settings: { label: { normal: "Settings", beginner: "Settings" } },
  methods: { label: { normal: "Methods", beginner: "Methods" } },
  mode: { label: { normal: "Mode", beginner: "Mode" } },
  targets: { label: { normal: "Targets", beginner: "Targets" } },
  export: { label: { normal: "Export", beginner: "Export" } },
  tools: { label: { normal: "Tools", beginner: "Tools" } },
  utilities: { label: { normal: "Utilities", beginner: "Utilities" } },
  sync: { label: { normal: "Sync", beginner: "Sync" } },
};

// #endregion 🇬️🇧️ English Bundle

export const uiChromeTranslationBundles = {
  // #region 🇩️🇪️ German Bundle
  de: {
    translation: {
      ui: {
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
        settings: {
          tab: {
            general: { label: { normal: "Allgemein", beginner: "Allgemein" } },
            driver: { label: { normal: "Treiber", beginner: "Treiber" } },
            app: { label: { normal: "App", beginner: "App" } },
            appearance: { label: { normal: "Design", beginner: "Design" } },
            layout: { label: { normal: "Layout", beginner: "Layout" } },
            language: { label: { normal: "Sprache", beginner: "Sprache" } },
            terminology: { label: { normal: "Terminologie", beginner: "Terminologie" } },
            theme: { label: { normal: "Thema", beginner: "Thema" } },
            keybindings: { label: { normal: "Tastenkürzel", beginner: "Tastenkürzel" } },
          },
          appearance: {
            light: { label: { normal: "Hell", beginner: "Hell" } },
            dark: { label: { normal: "Dunkel", beginner: "Dunkel" } },
            system: { label: { normal: "System", beginner: "System" } },
          },
          language: {
            en: { label: { normal: "English", beginner: "English" } },
            de: { label: { normal: "Deutsch", beginner: "Deutsch" } },
          },
          terminology: {
            native: { label: { normal: "Nativ", beginner: "Nativ" } },
            reuse: { label: { normal: "Wiederverwendung", beginner: "Wiederverwendung" } },
          },
          app: {
            name: { label: { normal: "Name", beginner: "Name" } },
            id: { label: { normal: "App-ID", beginner: "App-ID" } },
            controller: { label: { normal: "Controller", beginner: "Controller" } },
            plugin: { label: { normal: "Plugin", beginner: "Plugin" } },
          },
          theme: {
            select: { label: { normal: "Thema", beginner: "Thema" } },
            save: { label: { normal: "Speichern unter", beginner: "Speichern unter" } },
            savePlaceholder: { label: { normal: "Themenname", beginner: "Themenname" } },
            reset: { label: { normal: "Zurücksetzen", beginner: "Zurücksetzen" } },
            export: { label: { normal: "Exportieren", beginner: "Exportieren" } },
            import: { label: { normal: "Importieren", beginner: "Importieren" } },
            delete: { label: { normal: "Löschen", beginner: "Löschen" } },
            colors: { label: { normal: "Farben", beginner: "Farben" } },
            spacing: { label: { normal: "Abstand", beginner: "Abstand" } },
            fonts: { label: { normal: "Schriftarten", beginner: "Schriftarten" } },
            strokes: { label: { normal: "Strichstärken", beginner: "Strichstärken" } },
            radii: { label: { normal: "Rundungen", beginner: "Rundungen" } },
            opacities: { label: { normal: "Deckkraft", beginner: "Deckkraft" } },
            metrics: { label: { normal: "Masse", beginner: "Masse" } },
            appearances: { label: { normal: "Erscheinungsbilder", beginner: "Erscheinungsbilder" } },
            dirty: { label: { normal: "Nicht gespeichert", beginner: "Nicht gespeichert" } },
            appearance: {
              light: { label: { normal: "Hell", beginner: "Hell" } },
              dark: { label: { normal: "Dunkel", beginner: "Dunkel" } },
            },
            group: {
              board: { label: { normal: "Board", beginner: "Board" } },
              map: { label: { normal: "Karte", beginner: "Karte" } },
              canvas: { label: { normal: "Leinwand", beginner: "Leinwand" } },
              chrome: { label: { normal: "Oberfläche", beginner: "Oberfläche" } },
              outcome: { label: { normal: "Ergebnis", beginner: "Farben für Fehler und Erfolg" } },
              diagram: { label: { normal: "Diagramm", beginner: "Farben für Diagramme" } },
            },
            contrast: {
              label: { label: { normal: "Kontrast", beginner: "Lesbarkeit des Textes" } },
              aaa: { label: { normal: "AAA", beginner: "Sehr gut lesbar" } },
              aa: { label: { normal: "AA", beginner: "Gut lesbar" } },
              aaLarge: { label: { normal: "AA nur für grossen Text", beginner: "Nur für grosse Schrift lesbar" } },
              fail: { label: { normal: "Unter AA — zu geringer Kontrast", beginner: "Zu schwacher Kontrast, schwer lesbar" } },
              warning: { label: { normal: "Geringer Kontrast {{ratio}}:1 zu {{counterpart}} — WCAG AA verlangt mindestens {{minimum}}:1", beginner: "Schwer lesbar zusammen mit {{counterpart}} ({{ratio}}:1, nötig sind {{minimum}}:1)" } },
            },
          },
          unavailable: { label: { normal: "Einstellungen nicht verfügbar", beginner: "Einstellungen nicht verfügbar" } },
          resetDock: { label: { normal: "Panels zurücksetzen", beginner: "Panels zurücksetzen" } },
        },
        plugins: {
          status: {
            available: { label: { normal: "Verfügbar", beginner: "Verfügbar" } },
            installing: { label: { normal: "Wird installiert…", beginner: "Wird installiert…" } },
            loaded: { label: { normal: "Geladen", beginner: "Geladen" } },
            failed: { label: { normal: "Fehlgeschlagen", beginner: "Fehlgeschlagen" } },
            reloading: { label: { normal: "Wird neu geladen…", beginner: "Wird neu geladen…" } },
          },
          action: {
            install: { label: { normal: "Installieren", beginner: "Installieren" } },
            uninstall: { label: { normal: "Deinstallieren", beginner: "Deinstallieren" } },
            reload: { label: { normal: "Neu laden", beginner: "Neu laden" } },
          },
          waitingForHost: { label: { normal: "Warte auf Host-Programm…", beginner: "Warte auf Host-Programm…" } },
          unavailable: { label: { normal: "Plugins nicht verfügbar", beginner: "Plugins nicht verfügbar" } },
          source: { label: { normal: "Quelle", beginner: "Quelle" } },
          marketplace: { label: { normal: "Marktplatz", beginner: "Marktplatz" } },
          marketplaceUnavailable: { label: { normal: "Marktplatz nicht verfügbar", beginner: "Marktplatz nicht verfügbar" } },
          extension: {
            enabled: { label: { normal: "aktiviert", beginner: "an" } },
            disabled: { label: { normal: "deaktiviert", beginner: "aus" } },
            enable: { label: { normal: "Aktivieren", beginner: "Einschalten" } },
            disable: { label: { normal: "Deaktivieren", beginner: "Ausschalten" } },
            install: { label: { normal: "Erweiterung installieren", beginner: "Erweiterung hinzufügen" } },
            fromUrl: { label: { normal: "Von URL", beginner: "Aus dem Internet" } },
            installFromUrl: { label: { normal: "Von URL installieren", beginner: "Aus dem Internet hinzufügen" } },
            urlPrompt: { label: { normal: "URL des Erweiterungspakets", beginner: "Adresse der Erweiterung" } },
            fromFile: { label: { normal: "Aus Datei", beginner: "Aus einer Datei" } },
            installFromFile: { label: { normal: "Aus Datei installieren", beginner: "Aus einer Datei hinzufügen" } },
          },
          recovery: {
            title: { label: { normal: "Plugin-Wiederherstellung", beginner: "Programm reparieren" } },
            crashed: { label: { normal: "Dieses Programm ist abgestürzt.", beginner: "Dieses Programm ist abgestürzt." } },
            quarantined: { label: { normal: "Dieses Programm wurde nach wiederholten Abstürzen isoliert.", beginner: "Dieses Programm ist mehrmals abgestürzt und wurde angehalten." } },
            restartApp: { label: { normal: "App neu starten", beginner: "Neu starten" } },
            disablePlugin: { label: { normal: "Plugin deaktivieren", beginner: "Programm ausschalten" } },
          },
        },
        command: {
          introduceApp: { label: { normal: "App vorstellen", beginner: "App vorstellen" } },
          playTutorial: { label: { normal: "Tutorial abspielen", beginner: "Tutorial abspielen" } },
          recordTutorial: { label: { normal: "Tutorial aufnehmen", beginner: "Tutorial aufnehmen" } },
          setAppearance: { label: { normal: "Erscheinungsbild festlegen", beginner: "Erscheinungsbild festlegen" } },
          setTheme: { label: { normal: "Thema festlegen", beginner: "Thema festlegen" } },
          setLayout: { label: { normal: "Layout festlegen", beginner: "Layout festlegen" } },
          setLocale: { label: { normal: "Sprache festlegen", beginner: "Sprache festlegen" } },
          setTerminology: { label: { normal: "Terminologie festlegen", beginner: "Terminologie festlegen" } },
          setDriver: { label: { normal: "Treiber festlegen", beginner: "Treiber festlegen" } },
          openTaskManager: { label: { normal: "Aufgaben öffnen", beginner: "Aufgaben öffnen" } },
          openHub: { label: { normal: "Hub und Bereiche öffnen", beginner: "Mit anderen arbeiten" } },
          exportDocument: { label: { normal: "Dokument exportieren", beginner: "Dokument als Datei sichern" } },
          importDocument: { label: { normal: "Dokument importieren…", beginner: "Dokument aus Datei öffnen…" } },
        },
        shellCommand: {
          dockMove: { label: { normal: "Panel-Tab verschieben", beginner: "Panel-Tab verschieben" } },
          windowResize: { label: { normal: "Fenster skalieren", beginner: "Fenster skalieren" } },
          windowMove: { label: { normal: "Fenster neu anordnen", beginner: "Fenster neu anordnen" } },
          windowActivate: { label: { normal: "Fenster aktivieren", beginner: "Fenster aktivieren" } },
          windowClose: { label: { normal: "Fenster schließen", beginner: "Fenster schließen" } },
          windowSplit: { label: { normal: "Fenster teilen", beginner: "Fenster teilen" } },
          windowOpenInNewWindow: { label: { normal: "In neuem Fenster öffnen", beginner: "In neuem Fenster öffnen" } },
          panelToggle: { label: { normal: "Panel umschalten", beginner: "Panel umschalten" } },
          panelTab: { label: { normal: "Panel-Tab wechseln", beginner: "Panel-Tab wechseln" } },
        },
        ribbon: {
          group: {
            parent: {
              label: {
                normal: "Hilfsmittel",
                beginner: "Hilfsmittel",
              },
            },
          },
          parent: uiRibbonParentDe,
        },
        selection: {
          method: { label: { normal: "Methode", beginner: "Methode" } },
          mode: { label: { normal: "Modus", beginner: "Modus" } },
          rectangle: { label: { normal: "Rechteck", beginner: "Rechteck" } },
          lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
          selective: { label: { normal: "Selektiv", beginner: "Selektiv" } },
          additive: { label: { normal: "Additiv", beginner: "Additiv" } },
          subtractive: { label: { normal: "Subtraktiv", beginner: "Subtraktiv" } },
          invertive: { label: { normal: "Invertierend", beginner: "Invertierend" } },
        },
        windowFault: {
          title: { label: { normal: "Fenster reagiert nicht", beginner: "Fenster reagiert nicht" } },
          abiMismatch: { label: { normal: "Plugin-Modul passt nicht zur Host-Schnittstelle", beginner: "Das Plugin ist veraltet und passt nicht mehr zum Programm" } },
          interactiveCeiling: { label: { normal: "Plugin-Schritt hat die interaktive Zeitgrenze überschritten", beginner: "Das Plugin hat für einen Schritt zu lange gebraucht" } },
          clock: { label: { normal: "Keine monotone Uhr verfügbar", beginner: "Die Zeitmessung des Systems steht nicht zur Verfügung" } },
          pluginInternal: { label: { normal: "Interner Plugin-Laufzeitfehler", beginner: "Im Plugin ist ein interner Fehler aufgetreten" } },
          installFailed: { label: { normal: "Plugin konnte nicht installiert werden", beginner: "Das Plugin liess sich nicht laden" } },
          unknown: { label: { normal: "Unbekannte Fehlerursache", beginner: "Die Ursache ist unbekannt" } },
        },
        common: {
          routeNotFound: { label: { normal: "Route nicht gefunden: {{path}}", beginner: "Diese Seite gibt es nicht: {{path}}" } },
          mixedValues: {
            label: {
              normal: "Gemischt",
              beginner: "Gemischt",
            },
          },
          name: { label: { normal: "Name", beginner: "Name" } },
          save: { label: { normal: "Speichern", beginner: "Speichern" } },
          delete: { label: { normal: "Löschen", beginner: "Löschen" } },
          loading: { label: { normal: "Lädt…", beginner: "Lädt…" } },
          loadingPlugins: { label: { normal: "Plugins werden geladen…", beginner: "Plugins werden geladen…" } },
          renderError: { label: { normal: "Renderfehler", beginner: "Renderfehler" } },
          noPluginsLoaded: { label: { normal: "Keine Plugins geladen", beginner: "Keine Plugins geladen" } },
          workerLost: { label: { normal: "Die Sitzung wurde beendet — bitte neu laden", beginner: "Die Sitzung wurde beendet — bitte die Seite neu laden" } },
          missingWindow: { label: { normal: "Fehlendes Fenster", beginner: "Fehlendes Fenster" } },
          home: { label: { normal: "Startseite", beginner: "Startseite" } },
          backToWorkflow: { label: { normal: "Zurück zum Workflow", beginner: "Zurück zum Workflow" } },
          execute: { label: { normal: "Ausführen", beginner: "Ausführen" } },
          reset: { label: { normal: "Zurücksetzen", beginner: "Zurücksetzen" } },
          windowOptions: { label: { normal: "Fensteroptionen", beginner: "Fensteroptionen" } },
          focus: { label: { normal: "Fokussieren", beginner: "Fokussieren" } },
          unfocus: { label: { normal: "Fokus aufheben", beginner: "Fokus aufheben" } },
          example: { label: { normal: "Beispiel", beginner: "Beispiel" } },
          noExample: { label: { normal: "Kein Beispiel", beginner: "Kein Beispiel" } },
          loadingSurface: { label: { normal: "Oberfläche wird geladen…", beginner: "Oberfläche wird geladen…" } },
          unknownComponent: { label: { normal: "Unbekannte Komponente", beginner: "Unbekannte Komponente" } },
          select: { label: { normal: "Auswählen", beginner: "Auswählen" } },
          commandPalette: { label: { normal: "Befehlspalette", beginner: "Befehlspalette" } },
          searchForCommand: { label: { normal: "Nach einem Befehl suchen…", beginner: "Nach einem Befehl suchen…" } },
          find: { label: { normal: "Finden…", beginner: "Finden…" } },
          noData: { label: { normal: "Keine Daten", beginner: "Keine Daten" } },
          noFileSystemNodes: { label: { normal: "Keine Dateisystemknoten", beginner: "Keine Dateisystemknoten" } },
          selectTarget: { label: { normal: "Ziel auswählen", beginner: "Ziel auswählen" } },
          selectOption: { label: { normal: "Option auswählen…", beginner: "Option auswählen…" } },
          noOptionsFound: { label: { normal: "Keine Optionen gefunden.", beginner: "Keine Optionen gefunden." } },
          close: { label: { normal: "Schließen", beginner: "Schließen" } },
          newWindow: { label: { normal: "Neues Fenster", beginner: "Neues Fenster" } },
          minimize: { label: { normal: "Minimieren", beginner: "Minimieren" } },
          maximize: { label: { normal: "Maximieren", beginner: "Maximieren" } },
          action: { label: { normal: "Aktion", beginner: "Aktion" } },
          actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
          utilities: { label: { normal: "Hilfsmittel", beginner: "Hilfsmittel" } },
          retry: { label: { normal: "Erneut versuchen", beginner: "Erneut versuchen" } },
          somethingWentWrong: { label: { normal: "Etwas ist schiefgelaufen", beginner: "Etwas ist schiefgelaufen" } },
          doubleClickToEdit: { label: { normal: "Zum Bearbeiten doppelklicken", beginner: "Zum Bearbeiten doppelklicken" } },
          importFile: { label: { normal: "Datei importieren…", beginner: "Datei importieren…" } },
          clear: { label: { normal: "Leeren", beginner: "Leeren" } },
          collapse: { label: { normal: "Einklappen", beginner: "Einklappen" } },
          expand: { label: { normal: "Ausklappen", beginner: "Ausklappen" } },
          cancel: { label: { normal: "Abbrechen", beginner: "Abbrechen" } },
          error: { label: { normal: "Fehler", beginner: "Fehler" } },
        },
        window: {
          close: { label: { normal: "Schließen", beginner: "Schließen" } },
          focus: { label: { normal: "Fokussieren", beginner: "Fokussieren" } },
          unfocus: { label: { normal: "Fokus aufheben", beginner: "Fokus aufheben" } },
          newWindow: { label: { normal: "Neues Fenster", beginner: "Neues Fenster" } },
          tabs: { label: { normal: "Fensterreiter", beginner: "Fensterreiter" } },
        },
        contextMenu: {
          more: { label: { normal: "Mehr", beginner: "Mehr" } },
          select: { label: { normal: "Auswählen", beginner: "Auswählen" } },
          deselect: { label: { normal: "Abwählen", beginner: "Abwählen" } },
          selectAll: { label: { normal: "Alles auswählen", beginner: "Alles auswählen" } },
          clearSelection: { label: { normal: "Auswahl aufheben", beginner: "Auswahl aufheben" } },
          selectSameKind: { label: { normal: "Gleiche Art auswählen", beginner: "Gleiche Art auswählen" } },
          duplicate: { label: { normal: "Duplizieren", beginner: "Duplizieren" } },
          delete: { label: { normal: "Löschen", beginner: "Löschen" } },
          zoomToSelection: { label: { normal: "Auf Auswahl zoomen", beginner: "Auf Auswahl zoomen" } },
          focusZoom: { label: { normal: "Fokus / Zoom darauf", beginner: "Fokus / Zoom darauf" } },
          openSource: { label: { normal: "Quelle öffnen", beginner: "Quelle öffnen" } },
          fitWorld: { label: { normal: "Welt einpassen", beginner: "Welt einpassen" } },
          cut: { label: { normal: "Ausschneiden", beginner: "Ausschneiden" } },
          copy: { label: { normal: "Kopieren", beginner: "Kopieren" } },
          paste: { label: { normal: "Einfügen", beginner: "Einfügen" } },
          rename: { label: { normal: "Umbenennen", beginner: "Umbenennen" } },
          formatDocument: { label: { normal: "Dokument formatieren", beginner: "Dokument formatieren" } },
          lintDocument: { label: { normal: "Dokument prüfen", beginner: "Dokument prüfen" } },
          suggestCompletions: { label: { normal: "Vervollständigungen vorschlagen", beginner: "Vervollständigungen vorschlagen" } },
          selectToken: { label: { normal: "Token auswählen", beginner: "Token auswählen" } },
          selectLine: { label: { normal: "Zeile auswählen", beginner: "Zeile auswählen" } },
          hide: { label: { normal: "Ausblenden", beginner: "Ausblenden" } },
          show: { label: { normal: "Einblenden", beginner: "Einblenden" } },
          lock: { label: { normal: "Sperren", beginner: "Sperren" } },
          unlock: { label: { normal: "Entsperren", beginner: "Entsperren" } },
        },
        diagram: {
          label: { label: { normal: "Knotengraph", beginner: "Diagramm aus Knoten und Verbindungen" } },
          roleDescription: { label: { normal: "Knotengraph-Editor", beginner: "Editor für Knoten und Verbindungen" } },
          keyboardHelp: {
            label: {
              normal: "Pfeiltasten: Knoten wechseln · Eingabe: auswählen · Umschalt+Eingabe: zur Auswahl hinzufügen · Esc: Auswahl aufheben",
              beginner: "Mit den Pfeiltasten von Knoten zu Knoten springen, mit der Eingabetaste auswählen, mit Esc die Auswahl aufheben",
            },
          },
          nodes: { label: { normal: "Knoten", beginner: "Knoten" } },
          edges: { label: { normal: "Verbindungen", beginner: "Verbindungen" } },
          empty: { label: { normal: "Leerer Graph", beginner: "Noch keine Knoten vorhanden" } },
          focusedNode: { label: { normal: "{{node}}, {{position}} von {{count}}", beginner: "Knoten {{node}}, Nummer {{position}} von {{count}}" } },
          selectedNode: { label: { normal: "{{node}} ausgewählt, {{count}} in der Auswahl", beginner: "{{node}} ist jetzt ausgewählt ({{count}} ausgewählt)" } },
          deselectedNode: { label: { normal: "{{node}} aus der Auswahl entfernt, {{count}} in der Auswahl", beginner: "{{node}} ist nicht mehr ausgewählt ({{count}} ausgewählt)" } },
          selectionCleared: { label: { normal: "Auswahl aufgehoben", beginner: "Nichts mehr ausgewählt" } },
        },
        host: {
          emptyScene: { label: { normal: "Keine Szene", beginner: "Keine Szene" } },
          tableRowRange: { label: { normal: "Zeilen {{from}}–{{to}} von {{total}}", beginner: "Zeilen {{from}} bis {{to}} von {{total}}" } },
          preview: { label: { normal: "Vorschau", beginner: "Vorschau" } },
          sourceAvailable: { label: { normal: "Quelle verfügbar", beginner: "Quelle verfügbar" } },
          blockImage: { label: { normal: "Bild", beginner: "Bild" } },
          blockTable: { label: { normal: "Tabelle", beginner: "Tabelle" } },
          blockMath: { label: { normal: "Mathe", beginner: "Mathe" } },
          blockInk: { label: { normal: "Tinte", beginner: "Tinte" } },
          blockGroup: { label: { normal: "Gruppe", beginner: "Gruppe" } },
          blockText: { label: { normal: "Text", beginner: "Text" } },
          checkingPlacement: { label: { normal: "Prüfe kollisionsfreie Platzierungen…", beginner: "Prüfe kollisionsfreie Platzierungen…" } },
          noPlacement: { label: { normal: "Keine kollisionsfreie Platzierung an diesem Verbinder", beginner: "Keine kollisionsfreie Platzierung an diesem Verbinder" } },
          canvasUnavailable: { label: { normal: "Leinwand nicht verfügbar", beginner: "Leinwand nicht verfügbar" } },
          rendering: { label: { normal: "Wird gerendert…", beginner: "Wird gerendert…" } },
          iconRenderFailed: { label: { normal: "Symbol konnte nicht gerendert werden", beginner: "Symbol konnte nicht gerendert werden" } },
          documentPlaceholder: { label: { normal: "Dokument", beginner: "Dokument" } },
          languageDocument: { label: { normal: "{{language}}-Dokument", beginner: "{{language}}-Dokument" } },
          iconShot: { label: { normal: "Symbolbild", beginner: "Symbolbild" } },
          projection: { label: { normal: "Projektion", beginner: "Projektion" } },
          frameVisible: { label: { normal: "Sichtbares einpassen", beginner: "Sichtbares einpassen" } },
          perspective: { label: { normal: "Perspektivisch", beginner: "Perspektivisch" } },
          orthographic: { label: { normal: "Orthografisch", beginner: "Orthografisch" } },
        },
        blockList: {
          steps: { label: { normal: "Schritte", beginner: "Schritte" } },
          addStep: { label: { normal: "Schritt hinzufügen", beginner: "Schritt hinzufügen" } },
        },
        tableStepper: {
          decrement: { label: { normal: "Verringern", beginner: "Weniger" } },
          increment: { label: { normal: "Erhöhen", beginner: "Mehr" } },
          value: { label: { normal: "Wert", beginner: "Wert" } },
        },
        docs: {
          navigation: {
            previous: {
              label: {
                normal: "Zurück",
                beginner: "Zurück",
              },
            },
            next: {
              label: {
                normal: "Weiter",
                beginner: "Weiter",
              },
            },
          },
        },
        ring: {
          demo: {
            label: {
              normal: "Ring",
              beginner: "Ring",
            },
          },
        },
        iconSelector: {
          mode: {
            url: { label: { normal: "URL", beginner: "URL" } },
            shortcode: { label: { normal: "Kurzcode", beginner: "Kurzcode" } },
            math: { label: { normal: "Mathe / Typst", beginner: "Mathe / Typst" } },
            data: { label: { normal: "Daten-URL", beginner: "Daten-URL" } },
            emoji: { label: { normal: "Emoji", beginner: "Emoji" } },
            text: { label: { normal: "Text", beginner: "Text" } },
            vector: { label: { normal: "Katalog / SVG", beginner: "Katalog / SVG" } },
          },
        },
        stepper: {
          demo: {
            label: {
              normal: "Wert",
              beginner: "Wert",
            },
          },
        },
        engagement: {
          actions: {
            label: {
              normal: "Aktionen",
              beginner: "Schnellaktionen für den aktuellen Schritt",
            },
          },
          viewport: {
            label: {
              normal: "Ansicht",
              beginner: "Ansicht",
            },
          },
        },
        windowSearch: {
          title: {
            label: {
              normal: "Suche",
              beginner: "Suche",
            },
          },
          action: {
            label: {
              normal: "Aktion",
              beginner: "Aktion eingeben oder aus der Liste wählen",
            },
          },
          actionActive: {
            label: {
              normal: "Aktion oder Wert",
              beginner: "Aktion oder Zahl für den aktuellen Schritt",
            },
          },
          suggestions: {
            label: {
              normal: "Vorschläge",
              beginner: "Liste der passenden Aktionen öffnen",
            },
          },
          noMatches: {
            label: {
              normal: "Keine Treffer",
              beginner: "Keine passenden Aktionen",
            },
          },
        },
        flowSpotlight: {
          typeToAdd: { label: { normal: "Zum Hinzufügen tippen…", beginner: "Zum Hinzufügen tippen…" } },
          collapseSuggestions: { label: { normal: "Vorschläge einklappen", beginner: "Vorschläge einklappen" } },
          showAllSuggestions: { label: { normal: "Alle Vorschläge anzeigen", beginner: "Alle Vorschläge anzeigen" } },
        },
        nodeGraph: {
          fitGraph: { label: { normal: "Graph einpassen", beginner: "Ganzen Graph zeigen" } },
          incompatiblePorts: { label: { normal: "{{source}} führt {{sourceType}}, {{target}} nimmt {{targetType}}", beginner: "Diese beiden Anschlüsse passen nicht zusammen: {{source}} führt {{sourceType}}, {{target}} nimmt {{targetType}}." } },
          portType: {
            geometry: { label: { normal: "Geometrie", beginner: "Geometrie" } },
            vector: { label: { normal: "Vektor", beginner: "Vektor" } },
            point: { label: { normal: "Punkt", beginner: "Punkt" } },
            number: { label: { normal: "Zahl", beginner: "Zahl" } },
            text: { label: { normal: "Text", beginner: "Text" } },
            boolean: { label: { normal: "Ja/Nein", beginner: "Ja/Nein" } },
            list: { label: { normal: "Liste", beginner: "Liste" } },
          },
        },
        sync: {
          attach: { label: { normal: "Verbinden", beginner: "Verbinden" } },
          detach: { label: { normal: "Trennen", beginner: "Trennen" } },
          browse: { label: { normal: "Durchsuchen", beginner: "Ordner wählen" } },
          statusLabel: { label: { normal: "Synchronisierungsstatus", beginner: "Synchronisierungsstatus" } },
          live: { label: { normal: "verbunden", beginner: "verbunden" } },
          connecting: { label: { normal: "verbinde…", beginner: "verbinde…" } },
          reconnecting: { label: { normal: "verbinde erneut…", beginner: "verbinde erneut…" } },
          offline: { label: { normal: "offline", beginner: "nicht verbunden" } },
          signedOut: { label: { normal: "abgemeldet", beginner: "nicht angemeldet" } },
          peerOne: { label: { normal: "{{count}} Mitwirkender", beginner: "{{count}} Mitwirkender" } },
          peerMany: { label: { normal: "{{count}} Mitwirkende", beginner: "{{count}} Mitwirkende" } },
          saved: { label: { normal: "gespeichert", beginner: "gespeichert" } },
          unsaved: { label: { normal: "ungespeichert", beginner: "nicht gespeichert" } },
          pending: { label: { normal: "{{count}} ausstehend", beginner: "{{count}} ausstehend" } },
          hubLabel: { label: { normal: "Hub-Verbindung", beginner: "Hub-Verbindung" } },
          hubSignIn: { label: { normal: "Anmelden", beginner: "Anmelden" } },
          online: { label: { normal: "online", beginner: "mit dem Hub verbunden" } },
          localOnly: { label: { normal: "nur lokal", beginner: "nur auf diesem Gerät" } },
          backboneFile: { label: { normal: "Dateisynchronisierung", beginner: "Mit einer Datei synchronisieren" } },
          backboneFolder: { label: { normal: "Ordnersynchronisierung", beginner: "Mit einem Ordner synchronisieren" } },
          backboneRemote: { label: { normal: "Hub-Synchronisierung", beginner: "Mit dem Hub synchronisieren" } },
          documentUnidentified: { label: { normal: "Dieses Programm hat kein Dokument zum Verbinden", beginner: "Das gewählte Programm hat kein eigenes Dokument, das mit einem Ordner, einer Datei oder einem Hub verbunden werden kann." } },
          reconnect: {
            label: { label: { normal: "Ordner dieses Dokuments", beginner: "Ordner, mit dem dieses Dokument auf diesem Gerät verbunden war" } },
            message: { label: { normal: "Dieses Dokument war mit dem Ordner „{{folder}}“ verbunden.", beginner: "Dieses Dokument war auf diesem Gerät mit dem Ordner „{{folder}}“ verbunden. Verbinde ihn wieder, um den gespeicherten Stand und Verlauf zu laden." } },
            attach: { label: { normal: "Ordner wieder verbinden", beginner: "Ordner wieder verbinden und gespeicherten Stand laden" } },
            forget: { label: { normal: "Ordner vergessen", beginner: "Diesen Ordner für dieses Dokument vergessen" } },
          },
        },
        ink: {
          link: { label: { normal: "Link", beginner: "Link" } },
          linkUrlPrompt: { label: { normal: "Link-URL", beginner: "Link-URL" } },
        },
        surfaceContextMenu: {
          architecture: { label: { normal: "Architekturmenü", beginner: "Architekturmenü" } },
          attraction: { label: { normal: "Anziehungsmenü", beginner: "Anziehungsmenü" } },
          block: { label: { normal: "Blockmenü", beginner: "Blockmenü" } },
          edge: { label: { normal: "Kantenmenü", beginner: "Kantenmenü" } },
          entry: { label: { normal: "Eintragsmenü", beginner: "Eintragsmenü" } },
          feature: { label: { normal: "Elementmenü", beginner: "Elementmenü" } },
          group: { label: { normal: "Gruppenmenü", beginner: "Gruppenmenü" } },
          handle: { label: { normal: "Griffmenü", beginner: "Griffmenü" } },
          layer: { label: { normal: "Ebenenmenü", beginner: "Ebenenmenü" } },
          object: { label: { normal: "Objektmenü", beginner: "Objektmenü" } },
          part: { label: { normal: "Teilmenü", beginner: "Teilmenü" } },
          path: { label: { normal: "Pfadmenü", beginner: "Pfadmenü" } },
          pixel: { label: { normal: "Pixelmenü", beginner: "Pixelmenü" } },
          position: { label: { normal: "Positionsmenü", beginner: "Positionsmenü" } },
          reference: { label: { normal: "Referenzmenü", beginner: "Referenzmenü" } },
          route: { label: { normal: "Routenmenü", beginner: "Routenmenü" } },
          slider: { label: { normal: "Reglermenü", beginner: "Reglermenü" } },
          vortex: { label: { normal: "Vortexmenü", beginner: "Vortexmenü" } },
          file: { label: { normal: "Dateimenü", beginner: "Dateimenü" } },
          workspace: { label: { normal: "Arbeitsbereichsmenü", beginner: "Arbeitsbereichsmenü" } },
          canvas: { label: { normal: "Canvas-Menü", beginner: "Canvas-Menü" } },
          scene: { label: { normal: "Szenenmenü", beginner: "Szenenmenü" } },
          placementSuggestions: { label: { normal: "Platzierungsvorschläge", beginner: "Platzierungsvorschläge" } },
          node: { label: { normal: "Knotenmenü", beginner: "Knotenmenü" } },
          flow: { label: { normal: "Flow-Menü", beginner: "Flow-Menü" } },
          row: { label: { normal: "Zeilenmenü", beginner: "Zeilenmenü" } },
          paint: { label: { normal: "Malmenü", beginner: "Malmenü" } },
          board: { label: { normal: "Board-Menü", beginner: "Board-Menü" } },
          ink: { label: { normal: "Tintenmenü", beginner: "Tintenmenü" } },
          history: { label: { normal: "Verlaufsmenü", beginner: "Verlaufsmenü" } },
          step: { label: { normal: "Schrittmenü", beginner: "Schrittmenü" } },
          diff: { label: { normal: "Vergleichsmenü", beginner: "Vergleichsmenü" } },
          event: { label: { normal: "Ereignismenü", beginner: "Ereignismenü" } },
          editor: { label: { normal: "Editormenü", beginner: "Editormenü" } },
          map: { label: { normal: "Kartenmenü", beginner: "Kartenmenü" } },
        },
        mutation: {
          level: {
            info: { label: { normal: "Info", beginner: "Info" } },
            warning: { label: { normal: "Warnung", beginner: "Warnung" } },
            error: { label: { normal: "Fehler", beginner: "Fehler" } },
            fatal: { label: { normal: "Kritisch", beginner: "Kritischer Fehler" } },
          },
          code: {
            targetMissing: { label: { normal: "Ziel fehlt", beginner: "Das Ziel dieser Änderung existiert nicht mehr." } },
            targetReferenced: { label: { normal: "Ziel wird noch referenziert", beginner: "Das Ziel wird noch von anderen Elementen verwendet — löse diese Verweise zuerst." } },
            targetMismatch: { label: { normal: "Widerspricht dem Ziel", beginner: "Die Änderung passt nicht zum aktuellen Zustand ihres Ziels." } },
            noOp: { label: { normal: "Keine Änderung", beginner: "Der Zustand war bereits so — nichts wurde geändert." } },
            partial: { label: { normal: "Teilweise angewendet", beginner: "Nur ein Teil der Änderung konnte angewendet werden." } },
            clamped: { label: { normal: "Begrenzt", beginner: "Ein Wert wurde auf den zulässigen Bereich begrenzt." } },
            duplicateId: { label: { normal: "ID bereits vergeben", beginner: "Es existiert bereits ein Element mit dieser ID." } },
            invariant: { label: { normal: "Ungültiger Zustand", beginner: "Diese Änderung würde einen ungültigen Zustand erzeugen." } },
            cascade: { label: { normal: "Folgeänderung", beginner: "Diese Änderung hat weitere Änderungen ausgelöst." } },
            apply: { label: { normal: "Nicht anwendbar", beginner: "Die Änderung ließ sich nicht auf das aktuelle Dokument anwenden." } },
          },
          history: {
            foreignTransition: { label: { normal: "Änderung einer anderen Person", beginner: "Du kannst nur eigene Änderungen rückgängig machen oder wiederherstellen." } },
          },
          policy: {
            laissezFaire: {
              label: { label: { normal: "Laissez-faire", beginner: "Laissez-faire" } },
              description: { label: { normal: "Nimmt jede Änderung an, außer sie ist kritisch.", beginner: "Nimmt jede Änderung an, solange sie nicht kritisch ist." } },
            },
            normal: {
              label: { label: { normal: "Normal", beginner: "Normal" } },
              description: { label: { normal: "Lehnt fehlerhafte Änderungen ab, erlaubt Warnungen.", beginner: "Lehnt Änderungen mit Fehlern ab, lässt Warnungen aber zu." } },
            },
            vigilant: {
              label: { label: { normal: "Wachsam", beginner: "Wachsam" } },
              description: { label: { normal: "Lehnt bereits Änderungen mit Warnungen ab.", beginner: "Am strengsten: lehnt schon Änderungen mit Warnungen ab." } },
            },
            setting: {
              label: { label: { normal: "Merge-Richtlinie", beginner: "Merge-Richtlinie" } },
            },
          },
          rejected: {
            title: { label: { normal: "Änderung abgelehnt", beginner: "Änderung abgelehnt" } },
            body: { label: { normal: "Diese Änderung konnte nicht angewendet werden.", beginner: "Diese Änderung konnte nicht angewendet werden." } },
          },
        },
        conflict: {
          panel: { label: { normal: "Konflikte", beginner: "Konflikte" } },
          accept: { label: { normal: "Übernehmen", beginner: "Übernehmen" } },
          discard: { label: { normal: "Verwerfen", beginner: "Verwerfen" } },
          quarantined: { label: { normal: "Zurückgehalten", beginner: "Eingehende Änderungen werden zurückgehalten, bis du entscheidest." } },
          degraded: { label: { normal: "Beeinträchtigt", beginner: "Übernommen, aber mit Warnungen." } },
          hubRejected: { label: { normal: "Änderung vom Hub abgelehnt", beginner: "Der Hub hat deine Änderung nicht angenommen; sie wurde zurückgenommen." } },
          hubTransformed: { label: { normal: "Änderung angepasst", beginner: "Eine gleichzeitige Änderung hatte Vorrang; deine Änderung wurde angepasst übernommen." } },
          hubConcurrentEdit: { label: { normal: "Jemand anderes hat gleichzeitig dieselbe Stelle geändert", beginner: "Jemand anderes hat gleichzeitig dieselbe Stelle geändert; deine Änderung wurde nicht übernommen." } },
          hubConcurrentInvariant: { label: { normal: "Widerspricht einer gleichzeitigen Änderung", beginner: "Deine Änderung widerspricht einer gleichzeitigen Änderung einer anderen Person und wurde nicht übernommen." } },
          local: {
            readOnly: { label: { normal: "Änderung nicht übernommen: Dieses Dokument ist hier schreibgeschützt", beginner: "Du kannst dieses Dokument hier nur ansehen, deshalb wurde deine Änderung nicht übernommen." } },
            queueFull: { label: { normal: "Änderung nicht übernommen: Zu viele Änderungen warten aufs Speichern", beginner: "Zu viele deiner Änderungen warten noch aufs Speichern; diese wurde nicht übernommen. Versuche es gleich noch einmal." } },
            duplicate: { label: { normal: "Änderung nicht übernommen: Sie wartet bereits aufs Speichern", beginner: "Diese Änderung wartet bereits aufs Speichern, deshalb wurde sie nicht ein zweites Mal übernommen." } },
            notReady: { label: { normal: "Änderung nicht übernommen: Das Dokument ist noch nicht bereit", beginner: "Das Dokument wird noch vorbereitet, deshalb wurde deine Änderung nicht übernommen. Versuche es gleich noch einmal." } },
            foreignDocument: { label: { normal: "Änderung nicht übernommen: Sie gehört zu einem anderen Dokument", beginner: "Deine Änderung nennt ein anderes Dokument als das hier geöffnete, deshalb wurde sie nicht übernommen." } },
            unreadable: { label: { normal: "Änderung nicht übernommen: Sie konnte nicht gelesen werden", beginner: "Deine Änderung konnte nicht gelesen werden, deshalb wurde sie nicht übernommen." } },
            tooLarge: { label: { normal: "Änderung nicht übernommen: Sie ist zu groß zum Senden", beginner: "Deine Änderung ist zu groß, um sie in einem Stück zu senden, deshalb wurde sie nicht übernommen." } },
          },
        },
        presence: {
          roster: { label: { normal: "Anwesende", beginner: "Anwesende" } },
          empty: { label: { normal: "Niemand sonst ist hier", beginner: "Niemand sonst ist hier" } },
          overflow: { label: { normal: "+{{count}} weitere", beginner: "+{{count}} weitere" } },
          role: {
            author: { label: { normal: "Bearbeitet", beginner: "Bearbeitet" } },
            spectator: { label: { normal: "Betrachtet", beginner: "Betrachtet" } },
          },
          kind: {
            agent: { label: { normal: "KI-Agent", beginner: "Ein KI-Agent, dem jemand Zugriff erteilt hat" } },
          },
        },
        timeTravel: {
          band: { label: { normal: "Zeitreise", beginner: "Zeitreise: Du bearbeitest den Verlauf" } },
          indicator: { label: { normal: "Zeitreise", beginner: "Zeitreise" } },
          indicatorTarget: { label: { normal: "Zeitreise: Dokument vor {{target}}", beginner: "Zeitreise: Dieses Fenster zeigt das Dokument vor {{target}}" } },
          stage: {
            editing: { label: { normal: "Mutation wird bearbeitet", beginner: "Mutation wird bearbeitet" } },
            replaying: { label: { normal: "Spätere Mutationen werden neu angewendet", beginner: "Spätere Mutationen werden neu angewendet" } },
            reviewing: { label: { normal: "Bearbeiteter Verlauf wird geprüft", beginner: "Bearbeiteter Verlauf wird geprüft" } },
            choosing: { label: { normal: "Art des Abschlusses wählen", beginner: "Art des Abschlusses wählen" } },
            finalizing: { label: { normal: "Verlaufsbearbeitung wird abgeschlossen", beginner: "Verlaufsbearbeitung wird abgeschlossen" } },
          },
          target: { label: { normal: "Bearbeitet: {{target}}", beginner: "Bearbeitete Mutation: {{target}}" } },
          progress: { label: { normal: "{{done}} von {{total}} Mutationen werden neu angewendet", beginner: "{{done}} von {{total}} Mutationen werden neu angewendet" } },
          worst: { label: { normal: "Schwerstes Ergebnis: {{level}}", beginner: "Schwerstes Ergebnis der neu angewendeten Mutationen: {{level}}" } },
          review: {
            noChanges: { label: { normal: "Keine Änderungen: aktueller Verlauf wird angezeigt", beginner: "Keine Änderungen: aktueller Verlauf wird angezeigt" } },
            needsReplay: { label: { normal: "Neu anwenden nötig: spätere Mutationen sind noch nicht geprüft", beginner: "Neu anwenden nötig: spätere Mutationen sind noch nicht geprüft" } },
            blocked: { label: { normal: "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden", beginner: "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden" } },
            ready: { label: { normal: "Bereit zum Abschließen", beginner: "Bereit zum Abschließen" } },
          },
          fault: { label: { normal: "Neuanwendung fehlgeschlagen ({{code}})", beginner: "Die Neuanwendung ist fehlgeschlagen ({{code}})" } },
          accepted: { label: { normal: "Übernommene Änderungen: {{count}}", beginner: "Übernommene Änderungen: {{count}}" } },
          accept: { label: { normal: "Entwurf übernehmen", beginner: "Entwurf übernehmen und spätere Mutationen neu anwenden" } },
          discard: { label: { normal: "Entwurf verwerfen", beginner: "Entwurf verwerfen" } },
          exit: { label: { normal: "Zeitreise beenden", beginner: "Zeitreise beenden und alle Entwürfe verwerfen" } },
          finalize: { label: { normal: "Abschließen…", beginner: "Bearbeiteten Verlauf abschließen…" } },
          back: { label: { normal: "Zurück", beginner: "Zurück zur Prüfung" } },
          cancelReplay: { label: { normal: "Neuanwendung abbrechen", beginner: "Neuanwendung abbrechen" } },
          rerun: { label: { normal: "Erneut anwenden", beginner: "Erneut anwenden" } },
          peer: {
            editingRow: { label: { normal: "{{name}} bearbeitet dies in der Zeitreise", beginner: "{{name}} bearbeitet dies gerade in der Zeitreise" } },
            editingTarget: { label: { normal: "{{name}} bearbeitet {{target}} in der Zeitreise", beginner: "{{name}} bearbeitet gerade {{target}} in der Zeitreise" } },
            editingHistory: { label: { normal: "{{name}} bearbeitet den Verlauf in der Zeitreise", beginner: "{{name}} bearbeitet gerade den Verlauf in der Zeitreise" } },
          },
          refusal: {
            frozen: { label: { normal: "Bearbeiten ist pausiert, solange der Verlauf bearbeitet wird", beginner: "Bearbeiten ist pausiert, solange der Verlauf bearbeitet wird" } },
            illegal: { label: { normal: "Derzeit nicht möglich", beginner: "Derzeit nicht möglich" } },
            stale: { label: { normal: "Veraltete Anfrage ignoriert", beginner: "Veraltete Anfrage ignoriert" } },
            blocked: { label: { normal: "Blockiert: zuerst die offene Änderung oder die Fehler auflösen", beginner: "Blockiert: zuerst die offene Änderung oder die Fehler auflösen" } },
            empty: { label: { normal: "Nichts abzuschließen: keine übernommenen Änderungen", beginner: "Nichts abzuschließen: keine übernommenen Änderungen" } },
            cancelled: { label: { normal: "Neu anwenden abgebrochen", beginner: "Neu anwenden abgebrochen" } },
            nameInvalid: { label: { normal: "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden", beginner: "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden" } },
            busy: { label: { normal: "Verlaufsbearbeitung beschäftigt: zuerst das laufende Werkzeug oder die andere Verlaufsbearbeitung abschließen", beginner: "Verlaufsbearbeitung beschäftigt: zuerst das laufende Werkzeug oder die andere Verlaufsbearbeitung abschließen" } },
            unknownMutation: { label: { normal: "Diese Mutation ist nicht mehr im Verlauf", beginner: "Diese Mutation ist nicht mehr im Verlauf" } },
            notEditable: { label: { normal: "Die Eingaben dieser Mutation können nicht bearbeitet werden", beginner: "Die Eingaben dieser Mutation können nicht bearbeitet werden" } },
            unknownInput: { label: { normal: "Diese Eingabe gibt es in der Mutation nicht", beginner: "Diese Eingabe gibt es in der Mutation nicht" } },
            invalidInput: { label: { normal: "Ungültiger Wert: Die Eingabe behält ihren bisherigen Wert", beginner: "Ungültiger Wert: Die Eingabe behält ihren bisherigen Wert" } },
            noSelection: { label: { normal: "Für diese Eingabe ist nichts Passendes ausgewählt", beginner: "Für diese Eingabe ist nichts Passendes ausgewählt" } },
            nameRequired: { label: { normal: "Einen Namen für die neue Alternative eingeben", beginner: "Einen Namen für die neue Alternative eingeben" } },
            schemaUnavailable: { label: { normal: "Das Eingabeschema dieser Mutation ist nicht verfügbar", beginner: "Das Eingabeschema dieser Mutation ist nicht verfügbar" } },
            replayFaulted: { label: { normal: "Erneutes Anwenden fehlgeschlagen: Spätere Mutationen konnten nicht geprüft werden", beginner: "Erneutes Anwenden fehlgeschlagen: Spätere Mutationen konnten nicht geprüft werden" } },
            commitFailed: { label: { normal: "Abschließen fehlgeschlagen: Der Verlauf ist unverändert", beginner: "Abschließen fehlgeschlagen: Der Verlauf ist unverändert" } },
            memberGone: { label: { normal: "Der Teil, den diese Verlaufsbearbeitung betrifft, wurde geschlossen", beginner: "Der Teil, den diese Verlaufsbearbeitung betrifft, wurde geschlossen" } },
          },
        },
        history: {
          refusal: {
            malformedTransition: { label: { normal: "Verlaufsbearbeitung abgelehnt: Die Änderung konnte nicht gelesen werden.", beginner: "Verlaufsbearbeitung abgelehnt: Die Änderung konnte nicht gelesen werden." } },
            unknownTarget: { label: { normal: "Verlaufsbearbeitung abgelehnt: Die bearbeitete Mutation existiert nicht mehr.", beginner: "Verlaufsbearbeitung abgelehnt: Die bearbeitete Mutation existiert nicht mehr." } },
            transitionRefused: { label: { normal: "Der Hub hat eine Verlaufsbearbeitung abgelehnt; der Schritt wurde zurückgenommen.", beginner: "Der Hub hat eine Verlaufsbearbeitung abgelehnt; der Schritt wurde zurückgenommen." } },
          },
        },
        referenceList: {
          useSelection: { label: { normal: "Aktuelle Auswahl verwenden", beginner: "Aktuelle Auswahl übernehmen" } },
          remove: { label: { normal: "{{item}} entfernen", beginner: "{{item}} entfernen" } },
          empty: { label: { normal: "Nichts ausgewählt", beginner: "Noch nichts ausgewählt" } },
        },
        colorInput: {
          hex: { label: { normal: "Hex", beginner: "Hex-Farbcode" } },
          alpha: { label: { normal: "Deckkraft", beginner: "Deckkraft (0 bis 1)" } },
        },
        nullableInput: {
          clear: { label: { normal: "Leeren", beginner: "Wert leeren" } },
        },
      },
      settings: {
        layout: {
          desktop: {
            label: {
              normal: "Desktop-Layout",
              beginner: "Verwendet das Standard-Layout, optimiert für Maus und Tastatur.",
            },
          },
          tablet: {
            label: {
              normal: "Tablet-Layout",
              beginner: "Verwendet das Tablet-Layout mit größeren, touch-freundlichen Bedienelementen.",
            },
          },
          mobile: {
            label: {
              normal: "Mobil-Layout",
              beginner: "Verwendet das Mobil-Layout, automatisch aktiv auf kleinen Bildschirmen.",
            },
          },
        },
        driver: {
          select: { label: { normal: "Treiber", beginner: "Treiber" } },
          default: { label: { normal: "Standard", beginner: "Standard" } },
          compact: { label: { normal: "Kompakt", beginner: "Kompakt" } },
          labels: { label: { normal: "Beschriftungen", beginner: "Beschriftungen" } },
          labelsOption: {
            full: { label: { normal: "Voll", beginner: "Symbol und Beschriftung" } },
            icons: { label: { normal: "Nur Symbole", beginner: "Nur Symbole" } },
          },
          labelTier: { label: { normal: "Beschriftungsstufe", beginner: "Beschriftungsstufe" } },
          labelTierOption: {
            beginner: { label: { normal: "Anfänger", beginner: "Ausführliche Beschriftungen" } },
            normal: { label: { normal: "Normal", beginner: "Kurze Beschriftungen" } },
          },
          drag: { label: { normal: "Ziehen", beginner: "Ziehen" } },
          dragOption: {
            handle: { label: { normal: "Griff", beginner: "Eigener Ziehgriff" } },
            surface: { label: { normal: "Fläche", beginner: "Ganzes Element ziehbar" } },
          },
          chrome: { label: { normal: "Oberflächenanzeige", beginner: "Oberflächenanzeige" } },
          chromeOption: {
            always: { label: { normal: "Immer", beginner: "Immer sichtbar" } },
            hover: { label: { normal: "Bei Hover", beginner: "Nur bei Mauszeiger sichtbar" } },
          },
          gumball: { label: { normal: "Gumball-Anzeige", beginner: "Gumball-Anzeige" } },
          gumballOption: {
            always: { label: { normal: "Immer", beginner: "Immer sichtbar" } },
            hover: { label: { normal: "Bei Hover", beginner: "Nur bei Mauszeiger sichtbar" } },
          },
          tooltips: { label: { normal: "Tooltips", beginner: "Tooltips" } },
          tooltipsOption: {
            full: { label: { normal: "Voll", beginner: "Mit Handbuch- und Tutorial-Links" } },
            minimal: { label: { normal: "Minimal", beginner: "Nur Name und Tastenkürzel" } },
            none: { label: { normal: "Keine", beginner: "Keine Tooltips" } },
          },
          hotkeys: { label: { normal: "Tastenkürzel", beginner: "Tastenkürzel" } },
          hotkeysOption: {
            inline: { label: { normal: "Inline", beginner: "Am Steuerelement" } },
            tooltip: { label: { normal: "Tooltip", beginner: "Nur im Tooltip" } },
            none: { label: { normal: "Keine", beginner: "Ausgeblendet" } },
          },
          save: { label: { normal: "Speichern unter", beginner: "Speichern unter" } },
          savePlaceholder: { label: { normal: "Treibername", beginner: "Treibername" } },
          delete: { label: { normal: "Löschen", beginner: "Löschen" } },
          dirty: { label: { normal: "Nicht gespeichert", beginner: "Nicht gespeichert" } },
        },
        keybindings: {
          capture: { label: { normal: "Aufnehmen", beginner: "Aufnehmen" } },
          reset: { label: { normal: "Zurücksetzen", beginner: "Zurücksetzen" } },
          conflict: { label: { normal: "Belegt", beginner: "Bereits vergeben" } },
          pressKeys: { label: { normal: "Tasten drücken…", beginner: "Tasten drücken…" } },
        },
      },
      tooltip: {
        manual: {
          label: {
            normal: "Handbuch",
            beginner: "Handbuch",
          },
        },
        tutorial: {
          label: {
            normal: "Tutorial",
            beginner: "Tutorial",
          },
        },
      },
      introduction: {
        skip: { label: { normal: "Überspringen", beginner: "Überspringen" } },
        back: { label: { normal: "Zurück", beginner: "Zurück" } },
        next: { label: { normal: "Weiter", beginner: "Weiter" } },
        done: { label: { normal: "Fertig", beginner: "Fertig" } },
      },
      tutorial: {
        play: { label: { normal: "Abspielen", beginner: "Abspielen" } },
        pause: { label: { normal: "Pause", beginner: "Pause" } },
        stop: { label: { normal: "Tutorial beenden", beginner: "Tutorial beenden" } },
        rate: { label: { normal: "Geschwindigkeit", beginner: "Geschwindigkeit" } },
        mute: { label: { normal: "Ton aus", beginner: "Ton aus" } },
        captions: { label: { normal: "Untertitel", beginner: "Untertitel" } },
        record: { label: { normal: "Aufnehmen", beginner: "Aufnehmen" } },
        recording: { label: { normal: "Aufnahme läuft", beginner: "Aufnahme läuft" } },
        addChapter: { label: { normal: "Kapitel setzen", beginner: "Kapitel setzen" } },
        chapter: { label: { normal: "Kapitel", beginner: "Kapitel" } },
      },
    } satisfies UiTranslationSchema,
  },
  // #endregion 🇩️🇪️ German Bundle

  // #region 🇬️🇧️ English Bundle
  en: {
    translation: {
      ui: {
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
        settings: {
          tab: {
            general: { label: { normal: "General", beginner: "General" } },
            driver: { label: { normal: "Driver", beginner: "Driver" } },
            app: { label: { normal: "App", beginner: "App" } },
            appearance: { label: { normal: "Appearance", beginner: "Appearance" } },
            layout: { label: { normal: "Layout", beginner: "Layout" } },
            language: { label: { normal: "Language", beginner: "Language" } },
            terminology: { label: { normal: "Terminology", beginner: "Terminology" } },
            theme: { label: { normal: "Theme", beginner: "Theme" } },
            keybindings: { label: { normal: "Hotkeys", beginner: "Hotkeys" } },
          },
          appearance: {
            light: { label: { normal: "Light", beginner: "Light" } },
            dark: { label: { normal: "Dark", beginner: "Dark" } },
            system: { label: { normal: "System", beginner: "System" } },
          },
          language: {
            en: { label: { normal: "English", beginner: "English" } },
            de: { label: { normal: "Deutsch", beginner: "Deutsch" } },
          },
          terminology: {
            native: { label: { normal: "Native", beginner: "Native" } },
            reuse: { label: { normal: "Reuse", beginner: "Reuse" } },
          },
          app: {
            name: { label: { normal: "Name", beginner: "Name" } },
            id: { label: { normal: "App id", beginner: "App id" } },
            controller: { label: { normal: "Controller", beginner: "Controller" } },
            plugin: { label: { normal: "Plugin", beginner: "Plugin" } },
          },
          theme: {
            select: { label: { normal: "Theme", beginner: "Theme" } },
            save: { label: { normal: "Save as", beginner: "Save as" } },
            savePlaceholder: { label: { normal: "Theme name", beginner: "Theme name" } },
            reset: { label: { normal: "Reset", beginner: "Reset" } },
            export: { label: { normal: "Export", beginner: "Export" } },
            import: { label: { normal: "Import", beginner: "Import" } },
            delete: { label: { normal: "Delete", beginner: "Delete" } },
            colors: { label: { normal: "Colors", beginner: "Colors" } },
            spacing: { label: { normal: "Spacing", beginner: "Spacing" } },
            fonts: { label: { normal: "Fonts", beginner: "Fonts" } },
            strokes: { label: { normal: "Strokes", beginner: "Strokes" } },
            radii: { label: { normal: "Radii", beginner: "Radii" } },
            opacities: { label: { normal: "Opacities", beginner: "Opacities" } },
            metrics: { label: { normal: "Metrics", beginner: "Metrics" } },
            appearances: { label: { normal: "Appearances", beginner: "Appearances" } },
            dirty: { label: { normal: "Unsaved", beginner: "Unsaved" } },
            appearance: {
              light: { label: { normal: "Light", beginner: "Light" } },
              dark: { label: { normal: "Dark", beginner: "Dark" } },
            },
            group: {
              board: { label: { normal: "Board", beginner: "Board" } },
              map: { label: { normal: "Map", beginner: "Map" } },
              canvas: { label: { normal: "Canvas", beginner: "Canvas" } },
              chrome: { label: { normal: "Chrome", beginner: "Chrome" } },
              outcome: { label: { normal: "Outcome", beginner: "Colors for errors and success" } },
              diagram: { label: { normal: "Diagram", beginner: "Colors for diagrams" } },
            },
            contrast: {
              label: { label: { normal: "Contrast", beginner: "How readable the text is" } },
              aaa: { label: { normal: "AAA", beginner: "Very easy to read" } },
              aa: { label: { normal: "AA", beginner: "Easy to read" } },
              aaLarge: { label: { normal: "AA for large text only", beginner: "Readable at large sizes only" } },
              fail: { label: { normal: "Below AA — contrast too low", beginner: "Contrast too low, hard to read" } },
              warning: { label: { normal: "Low contrast {{ratio}}:1 with {{counterpart}} — WCAG AA needs at least {{minimum}}:1", beginner: "Hard to read together with {{counterpart}} ({{ratio}}:1, needs {{minimum}}:1)" } },
            },
          },
          unavailable: { label: { normal: "Settings unavailable", beginner: "Settings unavailable" } },
          resetDock: { label: { normal: "Reset panels", beginner: "Reset panels" } },
        },
        plugins: {
          status: {
            available: { label: { normal: "Available", beginner: "Available" } },
            installing: { label: { normal: "Installing…", beginner: "Installing…" } },
            loaded: { label: { normal: "Loaded", beginner: "Loaded" } },
            failed: { label: { normal: "Failed", beginner: "Failed" } },
            reloading: { label: { normal: "Reloading…", beginner: "Reloading…" } },
          },
          action: {
            install: { label: { normal: "Install", beginner: "Install" } },
            uninstall: { label: { normal: "Uninstall", beginner: "Uninstall" } },
            reload: { label: { normal: "Reload", beginner: "Reload" } },
          },
          waitingForHost: { label: { normal: "Waiting for host program…", beginner: "Waiting for host program…" } },
          unavailable: { label: { normal: "Plugins unavailable", beginner: "Plugins unavailable" } },
          source: { label: { normal: "Source", beginner: "Source" } },
          marketplace: { label: { normal: "Marketplace", beginner: "Marketplace" } },
          marketplaceUnavailable: { label: { normal: "Marketplace unavailable", beginner: "Marketplace unavailable" } },
          extension: {
            enabled: { label: { normal: "enabled", beginner: "on" } },
            disabled: { label: { normal: "disabled", beginner: "off" } },
            enable: { label: { normal: "Enable", beginner: "Turn on" } },
            disable: { label: { normal: "Disable", beginner: "Turn off" } },
            install: { label: { normal: "Install extension", beginner: "Add an extension" } },
            fromUrl: { label: { normal: "From URL", beginner: "From the web" } },
            installFromUrl: { label: { normal: "Install from URL", beginner: "Add from the web" } },
            urlPrompt: { label: { normal: "Extension package URL", beginner: "Address of the extension" } },
            fromFile: { label: { normal: "From file", beginner: "From a file" } },
            installFromFile: { label: { normal: "Install from file", beginner: "Add from a file" } },
          },
          recovery: {
            title: { label: { normal: "Plugin Recovery", beginner: "Repair program" } },
            crashed: { label: { normal: "This program crashed.", beginner: "This program crashed." } },
            quarantined: { label: { normal: "This program was quarantined after repeated crashes.", beginner: "This program crashed several times and was stopped." } },
            restartApp: { label: { normal: "Restart App", beginner: "Restart" } },
            disablePlugin: { label: { normal: "Disable Plugin", beginner: "Turn program off" } },
          },
        },
        command: {
          introduceApp: { label: { normal: "Introduce App", beginner: "Introduce App" } },
          playTutorial: { label: { normal: "Play Tutorial", beginner: "Play Tutorial" } },
          recordTutorial: { label: { normal: "Record Tutorial", beginner: "Record Tutorial" } },
          setAppearance: { label: { normal: "Set Appearance", beginner: "Set Appearance" } },
          setTheme: { label: { normal: "Set Theme", beginner: "Set Theme" } },
          setLayout: { label: { normal: "Set Layout", beginner: "Set Layout" } },
          setLocale: { label: { normal: "Set Locale", beginner: "Set Locale" } },
          setTerminology: { label: { normal: "Set Terminology", beginner: "Set Terminology" } },
          setDriver: { label: { normal: "Set Driver", beginner: "Set Driver" } },
          openTaskManager: { label: { normal: "Open Tasks", beginner: "Open Tasks" } },
          openHub: { label: { normal: "Open Hub and Spaces", beginner: "Work With Others" } },
          exportDocument: { label: { normal: "Export Document", beginner: "Save Document as File" } },
          importDocument: { label: { normal: "Import Document…", beginner: "Open Document from File…" } },
        },
        shellCommand: {
          dockMove: { label: { normal: "Move Panel Tab", beginner: "Move Panel Tab" } },
          windowResize: { label: { normal: "Resize Window", beginner: "Resize Window" } },
          windowMove: { label: { normal: "Rearrange Windows", beginner: "Rearrange Windows" } },
          windowActivate: { label: { normal: "Activate Window", beginner: "Activate Window" } },
          windowClose: { label: { normal: "Close Window", beginner: "Close Window" } },
          windowSplit: { label: { normal: "Split Window", beginner: "Split Window" } },
          windowOpenInNewWindow: { label: { normal: "Open in New Window", beginner: "Open in New Window" } },
          panelToggle: { label: { normal: "Toggle Panel", beginner: "Toggle Panel" } },
          panelTab: { label: { normal: "Switch Panel Tab", beginner: "Switch Panel Tab" } },
        },
        ribbon: {
          group: {
            parent: {
              label: {
                normal: "Utility",
                beginner: "Utility",
              },
            },
          },
          parent: uiRibbonParentEn,
        },
        selection: {
          method: { label: { normal: "Method", beginner: "Method" } },
          mode: { label: { normal: "Mode", beginner: "Mode" } },
          rectangle: { label: { normal: "Rectangle", beginner: "Rectangle" } },
          lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
          selective: { label: { normal: "Selective", beginner: "Selective" } },
          additive: { label: { normal: "Additive", beginner: "Additive" } },
          subtractive: { label: { normal: "Subtractive", beginner: "Subtractive" } },
          invertive: { label: { normal: "Invertive", beginner: "Invertive" } },
        },
        windowFault: {
          title: { label: { normal: "Window is not responding", beginner: "Window is not responding" } },
          abiMismatch: { label: { normal: "Plugin module does not match the host interface", beginner: "This plugin is out of date and no longer fits the program" } },
          interactiveCeiling: { label: { normal: "Plugin step overran the interactive time ceiling", beginner: "The plugin took too long for one step" } },
          clock: { label: { normal: "No monotonic clock available", beginner: "The system clock reading is unavailable" } },
          pluginInternal: { label: { normal: "Internal plugin runtime fault", beginner: "Something went wrong inside the plugin" } },
          installFailed: { label: { normal: "Plugin failed to install", beginner: "The plugin could not be loaded" } },
          unknown: { label: { normal: "Unknown fault cause", beginner: "The cause is unknown" } },
        },
        common: {
          routeNotFound: { label: { normal: "Route not found: {{path}}", beginner: "This page does not exist: {{path}}" } },
          mixedValues: {
            label: {
              normal: "Mixed",
              beginner: "Mixed",
            },
          },
          name: { label: { normal: "Name", beginner: "Name" } },
          save: { label: { normal: "Save", beginner: "Save" } },
          delete: { label: { normal: "Delete", beginner: "Delete" } },
          loading: { label: { normal: "Loading…", beginner: "Loading…" } },
          loadingPlugins: { label: { normal: "Loading plugins…", beginner: "Loading plugins…" } },
          renderError: { label: { normal: "Render error", beginner: "Render error" } },
          noPluginsLoaded: { label: { normal: "No plugins loaded", beginner: "No plugins loaded" } },
          workerLost: { label: { normal: "The session was terminated — please reload", beginner: "The session was terminated — please reload the page" } },
          missingWindow: { label: { normal: "Missing window", beginner: "Missing window" } },
          home: { label: { normal: "Home", beginner: "Home" } },
          backToWorkflow: { label: { normal: "Back to Workflow", beginner: "Back to Workflow" } },
          execute: { label: { normal: "Execute", beginner: "Execute" } },
          reset: { label: { normal: "Reset", beginner: "Reset" } },
          windowOptions: { label: { normal: "Window Options", beginner: "Window Options" } },
          focus: { label: { normal: "Focus", beginner: "Focus" } },
          unfocus: { label: { normal: "Unfocus", beginner: "Unfocus" } },
          example: { label: { normal: "Example", beginner: "Example" } },
          noExample: { label: { normal: "No example", beginner: "No example" } },
          loadingSurface: { label: { normal: "Loading surface…", beginner: "Loading surface…" } },
          unknownComponent: { label: { normal: "Unknown component", beginner: "Unknown component" } },
          select: { label: { normal: "Select", beginner: "Select" } },
          commandPalette: { label: { normal: "Command Palette", beginner: "Command Palette" } },
          searchForCommand: { label: { normal: "Search for a command to run…", beginner: "Search for a command to run…" } },
          find: { label: { normal: "Find…", beginner: "Find…" } },
          noData: { label: { normal: "No data", beginner: "No data" } },
          noFileSystemNodes: { label: { normal: "No file system nodes", beginner: "No file system nodes" } },
          selectTarget: { label: { normal: "Select target", beginner: "Select target" } },
          selectOption: { label: { normal: "Select option…", beginner: "Select option…" } },
          noOptionsFound: { label: { normal: "No options found.", beginner: "No options found." } },
          close: { label: { normal: "Close", beginner: "Close" } },
          newWindow: { label: { normal: "New Window", beginner: "New Window" } },
          minimize: { label: { normal: "Minimize", beginner: "Minimize" } },
          maximize: { label: { normal: "Maximize", beginner: "Maximize" } },
          action: { label: { normal: "Action", beginner: "Action" } },
          actions: { label: { normal: "Actions", beginner: "Actions" } },
          utilities: { label: { normal: "Utilities", beginner: "Utilities" } },
          retry: { label: { normal: "Retry", beginner: "Retry" } },
          somethingWentWrong: { label: { normal: "Something went wrong", beginner: "Something went wrong" } },
          doubleClickToEdit: { label: { normal: "Double-click to edit", beginner: "Double-click to edit" } },
          importFile: { label: { normal: "Import file…", beginner: "Import file…" } },
          clear: { label: { normal: "Clear", beginner: "Clear" } },
          collapse: { label: { normal: "Collapse", beginner: "Collapse" } },
          expand: { label: { normal: "Expand", beginner: "Expand" } },
          cancel: { label: { normal: "Cancel", beginner: "Cancel" } },
          error: { label: { normal: "Error", beginner: "Error" } },
        },
        window: {
          close: { label: { normal: "Close", beginner: "Close" } },
          focus: { label: { normal: "Focus", beginner: "Focus" } },
          unfocus: { label: { normal: "Unfocus", beginner: "Unfocus" } },
          newWindow: { label: { normal: "New Window", beginner: "New Window" } },
          tabs: { label: { normal: "Window tabs", beginner: "Window tabs" } },
        },
        contextMenu: {
          more: { label: { normal: "More", beginner: "More" } },
          select: { label: { normal: "Select", beginner: "Select" } },
          deselect: { label: { normal: "Deselect", beginner: "Deselect" } },
          selectAll: { label: { normal: "Select all", beginner: "Select all" } },
          clearSelection: { label: { normal: "Clear selection", beginner: "Clear selection" } },
          selectSameKind: { label: { normal: "Select same kind", beginner: "Select same kind" } },
          duplicate: { label: { normal: "Duplicate", beginner: "Duplicate" } },
          delete: { label: { normal: "Delete", beginner: "Delete" } },
          zoomToSelection: { label: { normal: "Zoom to selection", beginner: "Zoom to selection" } },
          focusZoom: { label: { normal: "Focus / zoom to", beginner: "Focus / zoom to" } },
          openSource: { label: { normal: "Open source", beginner: "Open source" } },
          fitWorld: { label: { normal: "Fit world", beginner: "Fit world" } },
          cut: { label: { normal: "Cut", beginner: "Cut" } },
          copy: { label: { normal: "Copy", beginner: "Copy" } },
          paste: { label: { normal: "Paste", beginner: "Paste" } },
          rename: { label: { normal: "Rename", beginner: "Rename" } },
          formatDocument: { label: { normal: "Format document", beginner: "Format document" } },
          lintDocument: { label: { normal: "Lint document", beginner: "Lint document" } },
          suggestCompletions: { label: { normal: "Suggest completions", beginner: "Suggest completions" } },
          selectToken: { label: { normal: "Select token", beginner: "Select token" } },
          selectLine: { label: { normal: "Select line", beginner: "Select line" } },
          hide: { label: { normal: "Hide", beginner: "Hide" } },
          show: { label: { normal: "Show", beginner: "Show" } },
          lock: { label: { normal: "Lock", beginner: "Lock" } },
          unlock: { label: { normal: "Unlock", beginner: "Unlock" } },
        },
        diagram: {
          label: { label: { normal: "Node graph", beginner: "Diagram of nodes and connections" } },
          roleDescription: { label: { normal: "Node graph editor", beginner: "Editor for nodes and connections" } },
          keyboardHelp: {
            label: {
              normal: "Arrow keys: move between nodes · Enter: select · Shift+Enter: add to selection · Esc: clear selection",
              beginner: "Use the arrow keys to move from node to node, Enter to select one, Esc to clear the selection",
            },
          },
          nodes: { label: { normal: "Nodes", beginner: "Nodes" } },
          edges: { label: { normal: "Connections", beginner: "Connections" } },
          empty: { label: { normal: "Empty graph", beginner: "No nodes yet" } },
          focusedNode: { label: { normal: "{{node}}, {{position}} of {{count}}", beginner: "Node {{node}}, number {{position}} of {{count}}" } },
          selectedNode: { label: { normal: "{{node}} selected, {{count}} in selection", beginner: "{{node}} is now selected ({{count}} selected)" } },
          deselectedNode: { label: { normal: "{{node}} removed from selection, {{count}} in selection", beginner: "{{node}} is no longer selected ({{count}} selected)" } },
          selectionCleared: { label: { normal: "Selection cleared", beginner: "Nothing is selected any more" } },
        },
        host: {
          emptyScene: { label: { normal: "No scene", beginner: "No scene" } },
          tableRowRange: { label: { normal: "Rows {{from}}–{{to}} of {{total}}", beginner: "Rows {{from}} to {{to}} of {{total}}" } },
          preview: { label: { normal: "Preview", beginner: "Preview" } },
          sourceAvailable: { label: { normal: "Source available", beginner: "Source available" } },
          blockImage: { label: { normal: "Image", beginner: "Image" } },
          blockTable: { label: { normal: "Table", beginner: "Table" } },
          blockMath: { label: { normal: "Math", beginner: "Math" } },
          blockInk: { label: { normal: "Ink", beginner: "Ink" } },
          blockGroup: { label: { normal: "Group", beginner: "Group" } },
          blockText: { label: { normal: "Text", beginner: "Text" } },
          checkingPlacement: { label: { normal: "Checking collision-free placements…", beginner: "Checking collision-free placements…" } },
          noPlacement: { label: { normal: "No collision-free placement at this connector", beginner: "No collision-free placement at this connector" } },
          canvasUnavailable: { label: { normal: "Canvas unavailable", beginner: "Canvas unavailable" } },
          rendering: { label: { normal: "Rendering…", beginner: "Rendering…" } },
          iconRenderFailed: { label: { normal: "Icon rendering failed", beginner: "Icon rendering failed" } },
          documentPlaceholder: { label: { normal: "Artifact", beginner: "Artifact" } },
          languageDocument: { label: { normal: "{{language}} document", beginner: "{{language}} document" } },
          iconShot: { label: { normal: "Icon shot", beginner: "Icon shot" } },
          projection: { label: { normal: "Projection", beginner: "Projection" } },
          frameVisible: { label: { normal: "Frame visible", beginner: "Frame visible" } },
          perspective: { label: { normal: "Perspective", beginner: "Perspective" } },
          orthographic: { label: { normal: "Orthographic", beginner: "Orthographic" } },
        },
        blockList: {
          steps: { label: { normal: "Steps", beginner: "Steps" } },
          addStep: { label: { normal: "Add Step", beginner: "Add Step" } },
        },
        tableStepper: {
          decrement: { label: { normal: "Decrease", beginner: "Less" } },
          increment: { label: { normal: "Increase", beginner: "More" } },
          value: { label: { normal: "Value", beginner: "Value" } },
        },
        docs: {
          navigation: {
            previous: {
              label: {
                normal: "Previous",
                beginner: "Previous",
              },
            },
            next: {
              label: {
                normal: "Next",
                beginner: "Next",
              },
            },
          },
        },
        ring: {
          demo: {
            label: {
              normal: "Ring",
              beginner: "Ring",
            },
          },
        },
        iconSelector: {
          mode: {
            url: { label: { normal: "URL", beginner: "URL" } },
            shortcode: { label: { normal: "Shortcode", beginner: "Shortcode" } },
            math: { label: { normal: "Math / Typst", beginner: "Math / Typst" } },
            data: { label: { normal: "Data URL", beginner: "Data URL" } },
            emoji: { label: { normal: "Emoji", beginner: "Emoji" } },
            text: { label: { normal: "Text", beginner: "Text" } },
            vector: { label: { normal: "Catalog / SVG", beginner: "Catalog / SVG" } },
          },
        },
        stepper: {
          demo: {
            label: {
              normal: "Value",
              beginner: "Value",
            },
          },
        },
        engagement: {
          actions: {
            label: {
              normal: "Actions",
              beginner: "Quick actions for the current step",
            },
          },
          viewport: {
            label: {
              normal: "Viewport",
              beginner: "Viewport",
            },
          },
        },
        windowSearch: {
          title: {
            label: {
              normal: "Search",
              beginner: "Search",
            },
          },
          action: {
            label: {
              normal: "Action",
              beginner: "Type an action or pick one from the list",
            },
          },
          actionActive: {
            label: {
              normal: "Action or value",
              beginner: "Action or number for the current step",
            },
          },
          suggestions: {
            label: {
              normal: "Suggestions",
              beginner: "Open the list of matching actions",
            },
          },
          noMatches: {
            label: {
              normal: "No matches",
              beginner: "No matching actions",
            },
          },
        },
        flowSpotlight: {
          typeToAdd: { label: { normal: "Type to add…", beginner: "Type to add…" } },
          collapseSuggestions: { label: { normal: "Collapse suggestions", beginner: "Collapse suggestions" } },
          showAllSuggestions: { label: { normal: "Show all suggestions", beginner: "Show all suggestions" } },
        },
        nodeGraph: {
          fitGraph: { label: { normal: "Fit graph", beginner: "Show the whole graph" } },
          incompatiblePorts: { label: { normal: "{{source}} carries {{sourceType}}, {{target}} takes {{targetType}}", beginner: "These two ports do not fit: {{source}} carries {{sourceType}}, {{target}} takes {{targetType}}." } },
          portType: {
            geometry: { label: { normal: "geometry", beginner: "geometry" } },
            vector: { label: { normal: "vector", beginner: "vector" } },
            point: { label: { normal: "point", beginner: "point" } },
            number: { label: { normal: "number", beginner: "number" } },
            text: { label: { normal: "text", beginner: "text" } },
            boolean: { label: { normal: "yes/no", beginner: "yes/no" } },
            list: { label: { normal: "list", beginner: "list" } },
          },
        },
        sync: {
          attach: { label: { normal: "Attach", beginner: "Attach" } },
          detach: { label: { normal: "Detach", beginner: "Detach" } },
          browse: { label: { normal: "Browse", beginner: "Pick a folder" } },
          statusLabel: { label: { normal: "Sync status", beginner: "Sync status" } },
          live: { label: { normal: "live", beginner: "live" } },
          connecting: { label: { normal: "connecting…", beginner: "connecting…" } },
          reconnecting: { label: { normal: "reconnecting…", beginner: "reconnecting…" } },
          offline: { label: { normal: "offline", beginner: "not connected" } },
          signedOut: { label: { normal: "signed out", beginner: "not signed in" } },
          peerOne: { label: { normal: "{{count}} peer", beginner: "{{count}} collaborator" } },
          peerMany: { label: { normal: "{{count}} peers", beginner: "{{count}} collaborators" } },
          saved: { label: { normal: "saved", beginner: "saved" } },
          unsaved: { label: { normal: "unsaved", beginner: "not saved" } },
          pending: { label: { normal: "{{count}} pending", beginner: "{{count}} pending" } },
          hubLabel: { label: { normal: "Hub connection", beginner: "Hub connection" } },
          hubSignIn: { label: { normal: "Sign in", beginner: "Sign in" } },
          online: { label: { normal: "online", beginner: "connected to the hub" } },
          localOnly: { label: { normal: "local only", beginner: "only on this device" } },
          backboneFile: { label: { normal: "File sync", beginner: "Sync with a file" } },
          backboneFolder: { label: { normal: "Folder sync", beginner: "Sync with a folder" } },
          backboneRemote: { label: { normal: "Hub sync", beginner: "Sync with the hub" } },
          documentUnidentified: { label: { normal: "This program has no document to attach", beginner: "The selected program has no document of its own that could be attached to a folder, a file or a hub." } },
          reconnect: {
            label: { label: { normal: "Folder of this document", beginner: "The folder this document was attached to on this device" } },
            message: { label: { normal: "This document was attached to the folder “{{folder}}”.", beginner: "This document was attached to the folder “{{folder}}” on this device. Reconnect it to load the saved state and history." } },
            attach: { label: { normal: "Reconnect folder", beginner: "Reconnect folder and load the saved state" } },
            forget: { label: { normal: "Forget folder", beginner: "Forget this folder for this document" } },
          },
        },
        ink: {
          link: { label: { normal: "Link", beginner: "Link" } },
          linkUrlPrompt: { label: { normal: "Link URL", beginner: "Link URL" } },
        },
        surfaceContextMenu: {
          architecture: { label: { normal: "Architecture Menu", beginner: "Architecture Menu" } },
          attraction: { label: { normal: "Attraction Menu", beginner: "Attraction Menu" } },
          block: { label: { normal: "Block Menu", beginner: "Block Menu" } },
          edge: { label: { normal: "Edge Menu", beginner: "Edge Menu" } },
          entry: { label: { normal: "Entry Menu", beginner: "Entry Menu" } },
          feature: { label: { normal: "Feature Menu", beginner: "Feature Menu" } },
          group: { label: { normal: "Group Menu", beginner: "Group Menu" } },
          handle: { label: { normal: "Handle Menu", beginner: "Handle Menu" } },
          layer: { label: { normal: "Layer Menu", beginner: "Layer Menu" } },
          object: { label: { normal: "Object Menu", beginner: "Object Menu" } },
          part: { label: { normal: "Part Menu", beginner: "Part Menu" } },
          path: { label: { normal: "Path Menu", beginner: "Path Menu" } },
          pixel: { label: { normal: "Pixel Menu", beginner: "Pixel Menu" } },
          position: { label: { normal: "Position Menu", beginner: "Position Menu" } },
          reference: { label: { normal: "Reference Menu", beginner: "Reference Menu" } },
          route: { label: { normal: "Route Menu", beginner: "Route Menu" } },
          slider: { label: { normal: "Slider Menu", beginner: "Slider Menu" } },
          vortex: { label: { normal: "Vortex Menu", beginner: "Vortex Menu" } },
          file: { label: { normal: "File Menu", beginner: "File Menu" } },
          workspace: { label: { normal: "Workspace Menu", beginner: "Workspace Menu" } },
          canvas: { label: { normal: "Canvas Menu", beginner: "Canvas Menu" } },
          scene: { label: { normal: "Scene Menu", beginner: "Scene Menu" } },
          placementSuggestions: { label: { normal: "Placement suggestions", beginner: "Placement suggestions" } },
          node: { label: { normal: "Node Menu", beginner: "Node Menu" } },
          flow: { label: { normal: "Flow Menu", beginner: "Flow Menu" } },
          row: { label: { normal: "Row Menu", beginner: "Row Menu" } },
          paint: { label: { normal: "Paint Menu", beginner: "Paint Menu" } },
          board: { label: { normal: "Board Menu", beginner: "Board Menu" } },
          ink: { label: { normal: "Ink Menu", beginner: "Ink Menu" } },
          history: { label: { normal: "History Menu", beginner: "History Menu" } },
          step: { label: { normal: "Step Menu", beginner: "Step Menu" } },
          diff: { label: { normal: "Diff Menu", beginner: "Diff Menu" } },
          event: { label: { normal: "Event Menu", beginner: "Event Menu" } },
          editor: { label: { normal: "Editor Menu", beginner: "Editor Menu" } },
          map: { label: { normal: "Map Menu", beginner: "Map Menu" } },
        },
        mutation: {
          level: {
            info: { label: { normal: "Info", beginner: "Info" } },
            warning: { label: { normal: "Warning", beginner: "Warning" } },
            error: { label: { normal: "Error", beginner: "Error" } },
            fatal: { label: { normal: "Fatal", beginner: "Fatal error" } },
          },
          code: {
            targetMissing: { label: { normal: "Target missing", beginner: "The target of this change no longer exists." } },
            targetReferenced: { label: { normal: "Target still referenced", beginner: "Other elements still use the target — resolve those references first." } },
            targetMismatch: { label: { normal: "Inconsistent with the target", beginner: "The change does not fit the current state of its target." } },
            noOp: { label: { normal: "No change", beginner: "Nothing changed — the state already matched." } },
            partial: { label: { normal: "Partially applied", beginner: "Only part of the change could be applied." } },
            clamped: { label: { normal: "Clamped", beginner: "A value was clamped to its valid range." } },
            duplicateId: { label: { normal: "Duplicate id", beginner: "An element with this id already exists." } },
            invariant: { label: { normal: "Invalid state", beginner: "This change would leave the document in an invalid state." } },
            cascade: { label: { normal: "Cascaded", beginner: "This change triggered further changes." } },
            apply: { label: { normal: "Could not apply", beginner: "The change could not be applied to the current document." } },
          },
          history: {
            foreignTransition: { label: { normal: "Change belongs to another author", beginner: "You can only undo or redo your own changes." } },
          },
          policy: {
            laissezFaire: {
              label: { label: { normal: "Laissez-faire", beginner: "Laissez-faire" } },
              description: { label: { normal: "Accepts every change unless it is fatal.", beginner: "Accepts every change as long as it isn't fatal." } },
            },
            normal: {
              label: { label: { normal: "Normal", beginner: "Normal" } },
              description: { label: { normal: "Rejects changes with errors, allows warnings.", beginner: "Rejects any change with an error, but allows warnings through." } },
            },
            vigilant: {
              label: { label: { normal: "Vigilant", beginner: "Vigilant" } },
              description: { label: { normal: "Rejects changes with warnings too.", beginner: "Strictest: rejects a change as soon as it carries a warning." } },
            },
            setting: {
              label: { label: { normal: "Merge policy", beginner: "Merge policy" } },
            },
          },
          rejected: {
            title: { label: { normal: "Change rejected", beginner: "Change rejected" } },
            body: { label: { normal: "This change could not be applied.", beginner: "This change could not be applied." } },
          },
        },
        conflict: {
          panel: { label: { normal: "Conflicts", beginner: "Conflicts" } },
          accept: { label: { normal: "Accept", beginner: "Accept" } },
          discard: { label: { normal: "Discard", beginner: "Discard" } },
          quarantined: { label: { normal: "Held back", beginner: "Incoming changes are held back until you decide." } },
          degraded: { label: { normal: "Degraded", beginner: "Applied, but with warnings." } },
          hubRejected: { label: { normal: "Change refused by the hub", beginner: "The hub did not accept your change; it was rolled back." } },
          hubTransformed: { label: { normal: "Change adjusted", beginner: "A concurrent change won; your change was applied in adjusted form." } },
          hubConcurrentEdit: { label: { normal: "Someone else changed the same part at the same time", beginner: "Someone else changed the same part at the same time; your change was not applied." } },
          hubConcurrentInvariant: { label: { normal: "Conflicts with a simultaneous change", beginner: "Your change conflicts with someone else's simultaneous change and was not applied." } },
          local: {
            readOnly: { label: { normal: "Change not applied: this document is read-only here", beginner: "You can only view this document here, so your change was not applied." } },
            queueFull: { label: { normal: "Change not applied: too many changes are waiting to be saved", beginner: "Too many of your changes are still waiting to be saved; this one was not applied. Try again in a moment." } },
            duplicate: { label: { normal: "Change not applied: it is already waiting to be saved", beginner: "This change is already waiting to be saved, so it was not applied a second time." } },
            notReady: { label: { normal: "Change not applied: the document is not ready yet", beginner: "The document is still being prepared, so your change was not applied. Try again in a moment." } },
            foreignDocument: { label: { normal: "Change not applied: it belongs to another document", beginner: "Your change names a different document than the one open here, so it was not applied." } },
            unreadable: { label: { normal: "Change not applied: it could not be read", beginner: "Your change could not be read, so it was not applied." } },
            tooLarge: { label: { normal: "Change not applied: it is too large to send", beginner: "Your change is too large to send in one piece, so it was not applied." } },
          },
        },
        presence: {
          roster: { label: { normal: "People here", beginner: "People here" } },
          empty: { label: { normal: "No one else is here", beginner: "No one else is here" } },
          overflow: { label: { normal: "+{{count}} more", beginner: "+{{count}} more" } },
          role: {
            author: { label: { normal: "Editing", beginner: "Editing" } },
            spectator: { label: { normal: "Viewing", beginner: "Viewing" } },
          },
          kind: {
            agent: { label: { normal: "AI agent", beginner: "An AI agent someone gave access to" } },
          },
        },
        timeTravel: {
          band: { label: { normal: "Time travel", beginner: "Time travel: you are editing the history" } },
          indicator: { label: { normal: "Time travel", beginner: "Time travel" } },
          indicatorTarget: { label: { normal: "Time travel: document before {{target}}", beginner: "Time travel: this window shows the document before {{target}}" } },
          stage: {
            editing: { label: { normal: "Editing a mutation", beginner: "Editing a mutation" } },
            replaying: { label: { normal: "Replaying later mutations", beginner: "Replaying later mutations" } },
            reviewing: { label: { normal: "Reviewing the edited history", beginner: "Reviewing the edited history" } },
            choosing: { label: { normal: "Choose how to finalize", beginner: "Choose how to finalize" } },
            finalizing: { label: { normal: "Finalizing the history edit", beginner: "Finalizing the history edit" } },
          },
          target: { label: { normal: "Editing: {{target}}", beginner: "Edited mutation: {{target}}" } },
          progress: { label: { normal: "Replaying {{done}} of {{total}} mutations", beginner: "Replaying {{done}} of {{total}} mutations" } },
          worst: { label: { normal: "Worst outcome: {{level}}", beginner: "Worst outcome of the replayed mutations: {{level}}" } },
          review: {
            noChanges: { label: { normal: "No changes: showing the current history", beginner: "No changes: showing the current history" } },
            needsReplay: { label: { normal: "Replay needed: later mutations are not checked yet", beginner: "Replay needed: later mutations are not checked yet" } },
            blocked: { label: { normal: "Errors must be fixed or withdrawn before finalizing", beginner: "Errors must be fixed or withdrawn before finalizing" } },
            ready: { label: { normal: "Ready to finalize", beginner: "Ready to finalize" } },
          },
          fault: { label: { normal: "Replay failed ({{code}})", beginner: "The replay failed ({{code}})" } },
          accepted: { label: { normal: "Accepted changes: {{count}}", beginner: "Accepted changes: {{count}}" } },
          accept: { label: { normal: "Accept draft", beginner: "Accept the draft and replay later mutations" } },
          discard: { label: { normal: "Discard draft", beginner: "Discard draft" } },
          exit: { label: { normal: "Exit time travel", beginner: "Exit time travel and discard every draft" } },
          finalize: { label: { normal: "Finalize…", beginner: "Finalize the edited history…" } },
          back: { label: { normal: "Back", beginner: "Back to reviewing" } },
          cancelReplay: { label: { normal: "Cancel replay", beginner: "Cancel replay" } },
          rerun: { label: { normal: "Replay again", beginner: "Replay again" } },
          peer: {
            editingRow: { label: { normal: "{{name}} is editing this in time travel", beginner: "{{name}} is editing this right now in time travel" } },
            editingTarget: { label: { normal: "{{name}} is editing {{target}} in time travel", beginner: "{{name}} is editing {{target}} right now in time travel" } },
            editingHistory: { label: { normal: "{{name}} is editing the history in time travel", beginner: "{{name}} is editing the history right now in time travel" } },
          },
          refusal: {
            frozen: { label: { normal: "Editing is paused while history is being edited", beginner: "Editing is paused while history is being edited" } },
            illegal: { label: { normal: "Not possible right now", beginner: "Not possible right now" } },
            stale: { label: { normal: "Outdated request ignored", beginner: "Outdated request ignored" } },
            blocked: { label: { normal: "Blocked: resolve the pending change or the errors first", beginner: "Blocked: resolve the pending change or the errors first" } },
            empty: { label: { normal: "Nothing to finalize: no accepted changes", beginner: "Nothing to finalize: no accepted changes" } },
            cancelled: { label: { normal: "Replay cancelled", beginner: "Replay cancelled" } },
            nameInvalid: { label: { normal: "Invalid alternative name: use 1 to 256 characters", beginner: "Invalid alternative name: use 1 to 256 characters" } },
            busy: { label: { normal: "History editing is busy: finish the running tool or the other history edit first", beginner: "History editing is busy: finish the running tool or the other history edit first" } },
            unknownMutation: { label: { normal: "This mutation is no longer in the history", beginner: "This mutation is no longer in the history" } },
            notEditable: { label: { normal: "The inputs of this mutation cannot be edited", beginner: "The inputs of this mutation cannot be edited" } },
            unknownInput: { label: { normal: "This input does not exist in the mutation", beginner: "This input does not exist in the mutation" } },
            invalidInput: { label: { normal: "Invalid value: the input keeps its previous value", beginner: "Invalid value: the input keeps its previous value" } },
            noSelection: { label: { normal: "Nothing suitable is selected for this input", beginner: "Nothing suitable is selected for this input" } },
            nameRequired: { label: { normal: "Name the new alternative", beginner: "Name the new alternative" } },
            schemaUnavailable: { label: { normal: "The input schema of this mutation is unavailable", beginner: "The input schema of this mutation is unavailable" } },
            replayFaulted: { label: { normal: "Replay failed: later mutations could not be checked", beginner: "Replay failed: later mutations could not be checked" } },
            commitFailed: { label: { normal: "Finalizing failed: the history is unchanged", beginner: "Finalizing failed: the history is unchanged" } },
            memberGone: { label: { normal: "The part this history edit targets was closed", beginner: "The part this history edit targets was closed" } },
          },
        },
        history: {
          refusal: {
            malformedTransition: { label: { normal: "History edit refused: the change could not be read.", beginner: "History edit refused: the change could not be read." } },
            unknownTarget: { label: { normal: "History edit refused: the edited mutation no longer exists.", beginner: "History edit refused: the edited mutation no longer exists." } },
            transitionRefused: { label: { normal: "The hub refused a history edit; the step was withdrawn.", beginner: "The hub refused a history edit; the step was withdrawn." } },
          },
        },
        referenceList: {
          useSelection: { label: { normal: "Use current selection", beginner: "Use the current selection" } },
          remove: { label: { normal: "Remove {{item}}", beginner: "Remove {{item}}" } },
          empty: { label: { normal: "Nothing selected", beginner: "Nothing selected yet" } },
        },
        colorInput: {
          hex: { label: { normal: "Hex", beginner: "Hex colour code" } },
          alpha: { label: { normal: "Opacity", beginner: "Opacity (0 to 1)" } },
        },
        nullableInput: {
          clear: { label: { normal: "Clear", beginner: "Clear the value" } },
        },
      },
      settings: {
        layout: {
          desktop: {
            label: {
              normal: "Desktop layout",
              beginner: "Use the standard layout optimized for mouse and keyboard.",
            },
          },
          tablet: {
            label: {
              normal: "Tablet layout",
              beginner: "Use the tablet layout with larger, touch-friendly controls.",
            },
          },
          mobile: {
            label: {
              normal: "Mobile layout",
              beginner: "Uses the mobile layout automatically on small screens.",
            },
          },
        },
        driver: {
          select: { label: { normal: "Driver", beginner: "Driver" } },
          default: { label: { normal: "Default", beginner: "Default" } },
          compact: { label: { normal: "Compact", beginner: "Compact" } },
          labels: { label: { normal: "Labels", beginner: "Labels" } },
          labelsOption: {
            full: { label: { normal: "Full", beginner: "Icon and label" } },
            icons: { label: { normal: "Icons only", beginner: "Icons only" } },
          },
          labelTier: { label: { normal: "Label Tier", beginner: "Label Tier" } },
          labelTierOption: {
            beginner: { label: { normal: "Beginner", beginner: "Verbose labels" } },
            normal: { label: { normal: "Normal", beginner: "Short labels" } },
          },
          drag: { label: { normal: "Drag", beginner: "Drag" } },
          dragOption: {
            handle: { label: { normal: "Handle", beginner: "Dedicated grip handle" } },
            surface: { label: { normal: "Surface", beginner: "Whole element draggable" } },
          },
          chrome: { label: { normal: "Chrome Reveal", beginner: "Chrome Reveal" } },
          chromeOption: {
            always: { label: { normal: "Always", beginner: "Always visible" } },
            hover: { label: { normal: "On Hover", beginner: "Visible only near the cursor" } },
          },
          gumball: { label: { normal: "Gumball Reveal", beginner: "Gumball Reveal" } },
          gumballOption: {
            always: { label: { normal: "Always", beginner: "Always visible" } },
            hover: { label: { normal: "On Hover", beginner: "Visible only near the cursor" } },
          },
          tooltips: { label: { normal: "Tooltips", beginner: "Tooltips" } },
          tooltipsOption: {
            full: { label: { normal: "Full", beginner: "With manual and tutorial links" } },
            minimal: { label: { normal: "Minimal", beginner: "Name and hotkey only" } },
            none: { label: { normal: "None", beginner: "No tooltips" } },
          },
          hotkeys: { label: { normal: "Hotkeys", beginner: "Hotkeys" } },
          hotkeysOption: {
            inline: { label: { normal: "Inline", beginner: "On the control" } },
            tooltip: { label: { normal: "Tooltip", beginner: "In tooltip only" } },
            none: { label: { normal: "None", beginner: "Hidden" } },
          },
          save: { label: { normal: "Save As", beginner: "Save As" } },
          savePlaceholder: { label: { normal: "Driver name", beginner: "Driver name" } },
          delete: { label: { normal: "Delete", beginner: "Delete" } },
          dirty: { label: { normal: "Unsaved", beginner: "Unsaved" } },
        },
        keybindings: {
          capture: { label: { normal: "Record", beginner: "Record" } },
          reset: { label: { normal: "Reset", beginner: "Reset" } },
          conflict: { label: { normal: "Conflict", beginner: "Already assigned" } },
          pressKeys: { label: { normal: "Press keys…", beginner: "Press keys…" } },
        },
      },
      tooltip: {
        manual: {
          label: {
            normal: "Manual",
            beginner: "Manual",
          },
        },
        tutorial: {
          label: {
            normal: "Tutorial",
            beginner: "Tutorial",
          },
        },
      },
      introduction: {
        skip: { label: { normal: "Skip", beginner: "Skip" } },
        back: { label: { normal: "Back", beginner: "Back" } },
        next: { label: { normal: "Next", beginner: "Next" } },
        done: { label: { normal: "Done", beginner: "Done" } },
      },
      tutorial: {
        play: { label: { normal: "Play", beginner: "Play" } },
        pause: { label: { normal: "Pause", beginner: "Pause" } },
        stop: { label: { normal: "Stop Tutorial", beginner: "Stop Tutorial" } },
        rate: { label: { normal: "Speed", beginner: "Speed" } },
        mute: { label: { normal: "Mute", beginner: "Mute" } },
        captions: { label: { normal: "Captions", beginner: "Captions" } },
        record: { label: { normal: "Record", beginner: "Record" } },
        recording: { label: { normal: "Recording", beginner: "Recording" } },
        addChapter: { label: { normal: "Add Chapter", beginner: "Add Chapter" } },
        chapter: { label: { normal: "Chapter", beginner: "Chapter" } },
      },
    } satisfies UiTranslationSchema,
  },
  // #endregion 🇬️🇧️ English Bundle
} satisfies Record<UiLocale, { readonly translation: UiTranslationSchema }>;

// #region 🔌️I18n Port
// i18n "port"/wiring glue: registration functions, locale resolvers, the i18next module augmentation, and the shared port instance.

export type UiTranslationLocaleCode = UiLocale;

export type UiTranslationBundlesInput = {
  readonly [L in UiLocale]: { readonly translation: Record<string, unknown> };
};

declare module "i18next" {
  interface CustomTypeOptions {
    defaultNS: "translation";
    resources: {
      readonly en: { readonly translation: UiTranslationSchema };
      readonly de: { readonly translation: UiTranslationSchema };
    };
  }
}

// UiRegisteredTranslationKey imported from core I18n above/with schema import

/** @emoji 🪁️ Merges additional locale bundles into the shared UI i18n instance, requiring every
 * {@link UiLocale} to register the exact same schema `S` (a compile error otherwise — the same
 * both-locales-or-nothing guarantee the domain-neutral chrome bundle gets from `satisfies
 * UiTranslationSchema`). Returns a caster from `S`'s own dot-path key union to {@link UiRegisteredTranslationKey}
 * — the only way callers should obtain a key for their registered strings; passing an unregistered
 * string does not type-check. */
export function registerUiTranslationBundles<S extends Record<string, unknown>>(bundles: { readonly [L in UiLocale]: { readonly translation: S } }): <K extends DeepUiTranslationKeys<S>>(key: K) => UiRegisteredTranslationKey {
  applyUiTranslationBundleTo(i18next, bundles);
  registeredUiTranslationBundles.push(bundles);
  for (const instance of liveShellI18nInstances) applyUiTranslationBundleTo(instance, bundles);
  return (key) => key as UiRegisteredTranslationKey;
}

// #region 🐚️ShellI18n
/** 🐚️ Every bundle ever registered — the chrome's own domain-neutral one plus every product's, in
 * registration order — replayed onto each new per-shell i18next instance at creation time so an
 * embedded shell has the same translations as the page-owning singleton from its very first render.
 * Seeded with `uiChromeTranslationBundles` itself since that one is loaded directly via `.init({resources})`
 * rather than through `registerUiTranslationBundles`. */
const registeredUiTranslationBundles: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }[] = [uiChromeTranslationBundles];

/** 🐚️ Every currently-mounted shell's own i18next instance — {@link registerUiTranslationBundles}
 * replays a late-registering bundle (e.g. a lazily-loaded product module importing after some shells
 * already mounted) into each of these too, not just the shared singleton. */
const liveShellI18nInstances = ephemeralSet<typeof i18next>("framework.modules.ui.packages.typescript.targets.react.index.tsx.liveShellI18nInstances");

function applyUiTranslationBundleTo(instance: typeof i18next, bundle: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }): void {
  for (const [language, resource] of Object.entries(bundle)) {
    instance.addResourceBundle(language, "translation", resource.translation, true, true);
  }
}

/** 🐚️ Creates and synchronously initializes a fresh i18next instance for one shell, pre-loaded with
 * every bundle registered so far — mirrors {@link initializeUiI18n}'s own synchronous init (`initImmediate:
 * false`) so an embedded shell never flashes untranslated chrome on its first paint either. `react-i18next`
 * resolves the *nearest* `I18nextProvider` ancestor via context, so wrapping a shell's subtree in one
 * (see `FrameworkOsShell`) is the only wiring `useUiTranslation`/`useLabel` call sites need — none of
 * their many call sites throughout this file change. */
export function createShellI18nInstance(initialLocale: UiLocale): typeof i18next {
  const instance = i18next.createInstance();
  instance.use(initReactI18next);
  void instance.init({
    resources: {},
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: initialLocale,
    showSupportNotice: false,
    returnObjects: true,
    initImmediate: false,
    interpolation: { escapeValue: false },
    react: { useSuspense: false, bindI18n: "languageChanged", bindI18nStore: "added removed" },
  });
  for (const bundle of registeredUiTranslationBundles) applyUiTranslationBundleTo(instance, bundle);
  liveShellI18nInstances.add(instance);
  return instance;
}

/** 🐚️ Releases a shell's i18next instance on unmount — stops it receiving future
 * {@link registerUiTranslationBundles} replays. */
export function disposeShellI18nInstance(instance: typeof i18next): void {
  liveShellI18nInstances.delete(instance);
}
// #endregion 🐚️ShellI18n

function normalizeUiLocale(language?: string): UiTranslationLocaleCode {
  return language?.toLowerCase().startsWith("de") ? "de" : "en";
}

/** @emoji 🧭️ Maps a BCP47 tag (e.g. `navigator.language`, `"de-AT"`) onto a {@link ShellLocale};
 * defaults to `"en"`. Same rule the chrome's own locale detector uses — exposed so boot code
 * (renderer/demonstrator) can resolve a default before any brand lock is known. */
export const detectShellLocale = normalizeUiLocale as (language?: string) => ShellLocale;

function resolveRequestedUiLocale(): UiTranslationLocaleCode {
  // 🐚️ The legacy shared `uiI18n` singleton (kept only for callers not yet wrapped in a `ShellScopeProvider`)
  // is inherently page-global, so a plain browser-backed port is the correct (and only sensible) storage here.
  const storedLocale = readStoredUiChromeLocale(createBrowserStoragePort());
  if (storedLocale) return storedLocale;
  return normalizeUiLocale(i18next.resolvedLanguage || i18next.language || (typeof navigator !== "undefined" ? navigator.language : undefined));
}

function registerUiChromeTranslationBundles() {
  registerUiTranslationBundles(uiChromeTranslationBundles);
}

function createUiI18nPort(instance: typeof i18next): UiI18nPort {
  return {
    t: ((key, options) => instance.t(key as never, options as never)) as UiTranslateFn,
    tIn: (locale, key, options) => instance.getFixedT(locale)(key as never, options as never),
    exists: (key) => instance.exists(key),
    changeLanguage: (locale) => instance.changeLanguage(locale),
    get language() {
      return instance.language;
    },
    get resolvedLanguage() {
      return instance.resolvedLanguage;
    },
    get isInitialized() {
      return instance.isInitialized;
    },
  };
}

function initializeUiI18n(): UiI18nPort {
  const requestedLocale = resolveRequestedUiLocale();

  if (i18next.isInitialized) {
    registerUiChromeTranslationBundles();
    if (i18next.language !== requestedLocale) {
      void i18next.changeLanguage(requestedLocale);
    }
    return createUiI18nPort(i18next);
  }

  i18next.use(initReactI18next);

  void i18next.init({
    resources: uiChromeTranslationBundles,
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: requestedLocale,
    showSupportNotice: false,
    returnObjects: true,
    // 🚀️ Resources are bundled inline above (no backend fetch), so there is nothing to await —
    // forces synchronous readiness instead of deferring to a microtask, which is what let the
    // very first paint render with i18next still uninitialized (the English-chrome flash this
    // ticket fixes; see `initUiLocaleSync`).
    initImmediate: false,
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
      bindI18n: "languageChanged",
      bindI18nStore: "added removed",
    },
  });

  return createUiI18nPort(i18next);
}

/** @emoji 🪁️ Shared UI i18n port (domain-neutral bundles; extend via {@link registerUiTranslationBundles}). */
export const uiI18n = initializeUiI18n();

/** @emoji 🪁️ Sets the active UI locale on the shared i18n port (user-initiated, in-app switch —
 * for boot-time/brand-locked locale resolution, use {@link initUiLocaleSync} instead, which runs
 * before the first render rather than in a post-paint effect). */
export function setUiLocale(locale: UiLocale): Promise<unknown> {
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  return uiI18n.changeLanguage(locale);
}

/** @emoji 🚀️ Resolves the shell's locale synchronously, before the first React render — call this
 * at renderer/demonstrator boot (module scope or before `ReactDOM.createRoot(...).render(...)`),
 * never from a `useEffect`. A `useEffect`-based call runs after the first paint has already
 * committed, which is exactly how a German-locked brand could still flash English chrome
 * ("Skip"/"Back"/"Next"/"Done") on first load. Persists the locale (so a reload's
 * `resolveRequestedUiLocale` agrees) and sets `documentElement.lang` synchronously; also nudges the
 * already-initialized i18next instance in case this runs after `uiI18n`'s own module-load default
 * resolved differently. */
export function initUiLocaleSync(locale: ShellLocale): void {
  // 🐚️ Page-owning boot code only (renderer/demonstrator, before `createRoot(...).render(...)`) — a
  // plain browser-backed port is correct here, same as `resolveRequestedUiLocale`.
  writeStoredUiChromeLocale(createBrowserStoragePort(), locale);
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  if (i18next.language !== locale) void i18next.changeLanguage(locale);
}

// #endregion 🔌️I18n Port
