// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, representations and images.

// #endregion 🧲️Header

//#region 🗃️Exports
// Builtin UI icons only — the Metabolism kit fixture (and its derived MetabolismKit* exports) moved
// to `@semio-tech/compose-fixture` (REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT): it had zero runtime
// consumers outside `.storybook/stories/compose/**`, yet this barrel is imported by every document
// through `ui-react`, so the 7.3MB JSON was being parsed+flattened for nothing on every boot.

import { createIconComponent } from "@semio-tech/ui-react";

//#region 🔖️Icons
export const AddIcon = createIconComponent("plus");
export const AlertCircleIcon = createIconComponent("alert-circle");
export const ArrowLeftIcon = createIconComponent("arrow-left");
export const AwardIcon = createIconComponent("award");
export const BookIcon = createIconComponent("book-open");
export const CameraIcon = createIconComponent("camera");
export const ChatIcon = createIconComponent("message-circle");
export const CheckIcon = createIconComponent("check");
export const CheckIconAlt = CheckIcon;
export const ChevronDownIcon = createIconComponent("chevron-down");
export const ChevronDownIconAlt = ChevronDownIcon;
export const ChevronLeftIcon = createIconComponent("chevron-left");
export const ChevronRightIcon = createIconComponent("chevron-right");
export const ChevronsUpDownIcon = createIconComponent("chevrons-up-down");
export const CloseIcon = createIconComponent("x");
export const CloseIconAlt = CloseIcon;
export const CodeIcon = createIconComponent("code");
export const ConnectionIcon = createIconComponent("network");
export const ConnectorIcon = createIconComponent("crosshair");
export const CopyIcon = createIconComponent("copy");
export const DetailsIcon = createIconComponent("info");
export const DiagramIcon = createIconComponent("grid-3x3");
export const DisconnectIcon = createIconComponent("link-2-off");
export const DocumentIcon = createIconComponent("file-text");
export const ExternalLinkIcon = createIconComponent("external-link");
export const FileArchiveIcon = createIconComponent("file-archive");
export const FileCodeIcon = createIconComponent("file-code");
export const FileImageIcon = createIconComponent("file-image");
export const FileJsonIcon = createIconComponent("file-json");
export const FileSpreadsheetIcon = createIconComponent("file-spreadsheet");
export const FileTypeIcon = createIconComponent("file-type");
export const FileVideoIcon = createIconComponent("file-video");
export const FocusIcon = createIconComponent("focus");
export const FolderIcon = createIconComponent("folder");
export const GlobeIcon = createIconComponent("globe");
export const GripVerticalIcon = createIconComponent("grip-vertical");
export const HandIcon = createIconComponent("hand");
export const HashIcon = createIconComponent("hash");
export const HomeIcon = createIconComponent("home");
export const HudIcon = createIconComponent("hud-overlay");
export const HudPanelIcon = createIconComponent("panel-top");
export const InfoIcon = createIconComponent("info");
export const IntersectIcon = createIconComponent("combine");
export const LayoutIcon = createIconComponent("layout");
export const LeftSidePanelIcon = createIconComponent("panel-left");
export const LightbulbIcon = createIconComponent("lightbulb");
export const LoaderIcon = createIconComponent("loader-2");
export const LocalKitIcon = createIconComponent("hard-drive");
export const Maximize2Icon = createIconComponent("maximize-2");
export const MessageCircle = createIconComponent("message-circle");
export const Minimize2Icon = createIconComponent("minimize-2");
export const MonitorIcon = createIconComponent("monitor");
export const MoonIcon = createIconComponent("moon");
export const MoreHorizontalIcon = createIconComponent("more-horizontal");
export const MousePointerIcon = createIconComponent("mouse-pointer");
export const NavigateBackIcon = createIconComponent("arrow-left");
export const NavigateForwardIcon = createIconComponent("arrow-right");
export const NavigateUpIcon = createIconComponent("arrow-up");
export const PauseIcon = createIconComponent("pause");
export const PieceIcon = createIconComponent("puzzle");
export const PlayIcon = createIconComponent("play");
export const PortIcon = createIconComponent("plug");
export const RecordIcon = createIconComponent("circle");
export const RemoteKitIcon = createIconComponent("cloud");
export const RemoveIcon = createIconComponent("minus");
export const ResetIcon = createIconComponent("rotate-ccw");
export const RightSidePanelIcon = createIconComponent("panel-right");
export const SceneIcon = createIconComponent("scene-3d");
export const SearchIcon = createIconComponent("search");
export const SelectUtilityIcon = createIconComponent("mouse-pointer-2");
export const SettingsIcon = createIconComponent("settings");
export const SkipBackIcon = createIconComponent("skip-back");
export const SkipForwardIcon = createIconComponent("skip-forward");
export const SmartphoneIcon = createIconComponent("smartphone");
export const SortAscendingIcon = createIconComponent("arrow-up");
export const SortDescendingIcon = createIconComponent("arrow-down");
export const StatsIcon = createIconComponent("bar-chart-3");
export const StopIcon = createIconComponent("square");
export const SunIcon = createIconComponent("sun");
export const TabletIcon = createIconComponent("tablet");
export const TableViewIcon = createIconComponent("table-2");
export const TemporaryKitIcon = createIconComponent("clock");
export const TriangleAlertIcon = createIconComponent("triangle-alert");
export const TutorialIcon = createIconComponent("graduation-cap");
export const TypeIcon = createIconComponent("typography");
export const UserIcon = createIconComponent("user");
export const UtilitiesIcon = createIconComponent("wrench");
export const UtilityBarIcon = createIconComponent("hammer");
export const WorkbenchIcon = createIconComponent("workbench");
export type { IconName } from "./🔣️icons/🤖️generated/🟦️icons.ts";
export { ICONS, ICON_NAMES, isIconName } from "./🔣️icons/🤖️generated/🟦️icons.ts";
export { assertUniqueIconConceptAssignments, ICON_CONCEPT_ASSIGNMENTS, type IconConceptId } from "./🟦️icon_concepts.ts";
export { resolveCatalogIconNameFromTheme, resolveCatalogIconSvgFromTheme } from "./🟦️icon_resolver.ts";
export {
  SHORTCODE_CATALOG,
  SHORTCODE_EMOJI,
  shortcodeCatalogKey,
  shortcodeEmoji,
  type ShortcodeCatalogName,
  type ShortcodeEmojiName,
} from "./🔣️icons/🤖️generated/🟦️shortcodes.ts";
export { isMetabolismIconName, METABOLISM_ICONS, METABOLISM_ICON_NAMES, type MetabolismIconName } from "./🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts";
export { resolveMetabolismIconNameFromTheme, resolveMetabolismIconSvgFromTheme } from "./🟦️icon_resolver.ts";
//#endregion 🔖️Icons

//#endregion 🗃️Exports
