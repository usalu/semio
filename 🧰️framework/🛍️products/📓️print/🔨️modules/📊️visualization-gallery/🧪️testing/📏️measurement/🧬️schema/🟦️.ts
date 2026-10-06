/** 📄️ One rendered kind's page and independently extracted text. */
export type GalleryKindPage = { readonly page: number; readonly text: string };

/** 📏️ The measured pages, kind locations and canonical PDF digest of one render. */
export type GalleryVariantMeasurement = { readonly pages: number; readonly kinds: Readonly<Record<string, GalleryKindPage>>; readonly hash: string };

/** 🖼️ A testing renderer's actual report, keyed by section, theme and language. */
export type GalleryRenderEvidence = { readonly schemaVersion: 1; readonly generatedBy: string; readonly variants: Readonly<Record<string, GalleryVariantMeasurement>> };
