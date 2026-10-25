import type { UiTranslationSchema } from "../../🧱️elements/📚️I18n/🟦️.tsx";

/** 🎓️ Settings, tooltip, introduction and tutorial labels (`de`). */
export const guidanceDe = {
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
} satisfies Omit<UiTranslationSchema, "ui">;

/** 🧭️ Settings, tooltip, introduction and tutorial labels (`en`). */
export const guidanceEn = {
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
} satisfies Omit<UiTranslationSchema, "ui">;
