/** 📆️ The PDF date value shared by the document metadata and the COS objects. */

export interface PdfDate {
  year: number;
  month?: number;
  day?: number;
  hour?: number;
  minute?: number;
  second?: number;
  offsetMinutes?: number | null;
}
