/** 🆔️ Chrome panel ids: the stable ids of the display, settings, chat and task manager panels and of their tabs. */
//#region 🔖️os-chrome-panels
//#region DisplayPanel
export const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
export const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";
export const FRAMEWORK_SETTINGS_PANEL_ID = "framework.settings";
export const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";
export const FRAMEWORK_SETTINGS_THEME_TAB_ID = "framework.settings.theme";
export const FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID = "framework.settings.keybindings";
/** 👁️✏️ Mirrors Rust `PanelTabKind::SettingsDefaultApps.id_str()` (`🛂️manifest/🦀️.rs:2607`,
 * contract freeze §1). */
export const FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID = "framework.settings.default-apps";
/** ⚖️ Conflicts panel tab (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
 * CLASS-CONFLICTS` §C9) — open first-class conflicts with Accept/Discard, mirrors
 * `FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID`'s "omitted entirely when no host is wired" idiom. */
export const FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID = "framework.settings.conflicts";
/** 💬 MCP agent chat — flat leaf on the `top-right` (Details) dock anchor beside app inspection tabs. */
export const FRAMEWORK_CHAT_PANEL_ID = "framework.chat";
/** 🧵️ The actor task manager (`🧵️TaskManager/🟦️.tsx`) — the one central progress + cancel surface for
 * actor-level work. Its id is the `os.task-manager` controller id that element's row actions already
 * address, so the window a human opens and the window an action names are the same thing. Ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1, audit ranked item 1. */
export const FRAMEWORK_TASK_MANAGER_PANEL_ID = "os.task-manager";
//#endregion DisplayPanel
//#endregion 🔖️os-chrome-panels
