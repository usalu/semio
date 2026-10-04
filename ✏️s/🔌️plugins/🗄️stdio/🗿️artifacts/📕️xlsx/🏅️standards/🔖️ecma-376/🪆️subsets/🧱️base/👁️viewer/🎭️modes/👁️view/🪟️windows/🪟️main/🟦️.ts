/** 📊️ Xlsx viewer (ecma-376/🧱️base) — sparse worksheet-grid view model. */
export interface XlsxWorksheetGrid {
  sheetName: string;
  rowCount: number;
  columnCount: number;
}

export interface XlsxMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  sheets: XlsxWorksheetGrid[];
}

export const XLSX_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const XLSX_MAIN_BODY_KEY = "framework.window.table" as const;
