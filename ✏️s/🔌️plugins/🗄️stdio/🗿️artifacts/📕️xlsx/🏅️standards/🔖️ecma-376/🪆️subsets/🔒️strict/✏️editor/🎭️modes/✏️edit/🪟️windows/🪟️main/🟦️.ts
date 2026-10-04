/** 📊️ One worksheet grid projected from canonical SpreadsheetML. */
export interface XlsxWorksheetGrid {
  readonly sheetName: string;
  readonly rowCount: number;
  readonly columnCount: number;
}

/** ✏️ The main window exposes independently windowed A1 grids, including blank coordinates. */
export interface XlsxMainViewModel {
  readonly windowKindId: 'framework.window.table';
  readonly bodyKey: 'framework.window.table';
  readonly sheets: readonly XlsxWorksheetGrid[];
}

/** ✏️ Revision-bound occupied-cell update or vacant-cell insertion. */
export interface XlsxSetCell {
  readonly sheetName: string;
  readonly row: number;
  readonly column: number;
  readonly revision: string;
  readonly value: string;
}

export const XLSX_MAIN_WINDOW_KIND_ID = 'framework.window.table' as const;
export const XLSX_MAIN_BODY_KEY = 'framework.window.table' as const;
