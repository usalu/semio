import type { UiTranslationSchema } from "../../../../🧱️elements/📚️I18n/🟦️.tsx";

/** 🧩️ Content host labels for diagrams, docs, steppers, engagement and node graphs (`de`). */
export const contentHostsDe = {
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
    inputDraftConflict: { label: { normal: "Diese Änderung konnte nicht übernommen werden. Dein Entwurf bleibt erhalten. Verwirf ihn oder drücke Escape, um den aktuellen Wert zu verwenden.", beginner: "Diese Änderung konnte nicht übernommen werden. Dein Entwurf bleibt erhalten. Verwirf ihn oder drücke Escape, um den aktuellen Wert zu verwenden." } },
    inputDraftDiscard: { label: { normal: "Entwurf verwerfen", beginner: "Entwurf verwerfen" } },
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
} satisfies Pick<UiTranslationSchema["ui"], "diagram" | "host" | "blockList" | "tableStepper" | "docs" | "ring" | "iconSelector" | "stepper" | "engagement" | "windowSearch" | "flowSpotlight" | "nodeGraph">;

/** 🧩️ Content host labels for diagrams, docs, steppers, engagement and node graphs (`en`). */
export const contentHostsEn = {
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
    inputDraftConflict: { label: { normal: "This edit could not be applied. Your draft is preserved. Discard it or press Escape to use the current value.", beginner: "This edit could not be applied. Your draft is preserved. Discard it or press Escape to use the current value." } },
    inputDraftDiscard: { label: { normal: "Discard draft", beginner: "Discard draft" } },
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
} satisfies Pick<UiTranslationSchema["ui"], "diagram" | "host" | "blockList" | "tableStepper" | "docs" | "ring" | "iconSelector" | "stepper" | "engagement" | "windowSearch" | "flowSpotlight" | "nodeGraph">;
