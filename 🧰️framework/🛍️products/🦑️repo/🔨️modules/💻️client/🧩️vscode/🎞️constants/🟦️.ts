/** 🎞️ Extension constants
 *
 * Static configuration for diagnostics and UI strings.
 */

// #region 🎞️Constants
// Constants MUST define static configuration for diagnostics and UI strings.

/**
 * COMPOSE_KIT_LANGUAGE holds the data fields for a COMPOSE_KIT_LANGUAGE record.
 **/
export const COMPOSE_KIT_LANGUAGE = "json";

/**
 * UI_STRINGS holds the data fields for a UI_STRINGS record.
 **/
export const UI_STRINGS = {
  en: {
    sectionsEmpty: "No sections found",
    sectionsNoActiveFile: "No active file",
    sectionsRenamePrompt: "Enter new section name",
    sectionsCreateChildPrompt: "Enter child section name",
    sectionsDeleteConfirm: "Enter section path to delete",
  },
  de: {
    sectionsRenamePrompt: "Neuen Abschnittsnamen eingeben",
    sectionsCreateChildPrompt: "Name des Unterabschnitts eingeben",
    sectionsDeleteConfirm: "Abschnittspfad zum Löschen eingeben",
  },
};
// #endregion 🎞️Constants
