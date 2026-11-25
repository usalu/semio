//! 📌️ Prepares a revision-pinned PDF JPEG edit and its exact retained resource together.
use super::{
    baseline_resource::AdmittedPdfJpeg, check_image_placement, fault, image_names, paint_image,
    Fault, ImagePlacement, PdfColorSpace, PdfImage, PdfImageBody, PdfMutation, PdfSnapshot, SetImage,
};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::dct_view::{
    PdfDctImageColor, PdfDctImageView,
};

/// 🧩️ The private native image row and the two typed changes await one graph/history publication.
pub struct PendingDctPlacement {
    revision: [u8; 32],
    resource: AdmittedPdfJpeg,
    mutations: [PdfMutation; 2],
}

impl PendingDctPlacement {
    /// 🛬️ Stages the exact native bytes with the same image and paint references in a private transaction.
    pub fn prepare(
        revision: [u8; 32],
        snapshot: &PdfSnapshot,
        placement: ImagePlacement,
        resource: AdmittedPdfJpeg,
    ) -> Result<Self, (Fault, AdmittedPdfJpeg)> {
        let Some(view) = resource.stream_view() else {
            return Err((fault("PDF JPEG has no admitted native stream"), resource));
        };
        let mutations = match place(snapshot, placement, view) {
            Ok(mutations) => mutations,
            Err(error) => return Err((error, resource)),
        };
        Ok(Self { revision, resource, mutations })
    }

    /// 🧾️ Lends the complete native and logical transaction under the original revision.
    pub fn borrow(&self, current: [u8; 32]) -> Result<(PdfDctImageView<'_>, &[PdfMutation; 2]), Fault> {
        if current != self.revision { return Err(fault("PDF changed while the JPEG was prepared")); }
        let resource = self.resource.stream_view().ok_or_else(|| fault("PDF JPEG native stream disappeared"))?;
        Ok((resource, &self.mutations))
    }

    /// 🎟️ Transfers the original and both mutations only to the revision-checked publication boundary.
    pub fn take_for_publication(self, current: [u8; 32]) -> Result<(AdmittedPdfJpeg, [PdfMutation; 2]), Self> {
        if current != self.revision { return Err(self); }
        Ok((self.resource, self.mutations))
    }

    /// ♻️ Returns the still-owned resource for paid retirement if publication is abandoned.
    pub fn abort(self) -> AdmittedPdfJpeg { self.resource }
}

fn place(
    snapshot: &PdfSnapshot,
    placement: ImagePlacement,
    resource: PdfDctImageView<'_>,
) -> Result<[PdfMutation; 2], Fault> {
    let page = check_image_placement(snapshot, placement)?;
    let (width, height) = resource.dimensions();
    let color = match resource.color() {
        PdfDctImageColor::Gray { .. } => PdfColorSpace::DeviceGray,
        PdfDctImageColor::Rgb { .. } => PdfColorSpace::DeviceRgb,
    };
    let id = image_names(snapshot).next().expect("available image XObject id");
    let mut image = PdfImage::packed(&id, u32::from(width), u32::from(height), color, 8, Vec::new());
    image.body = PdfImageBody::Artifact { reference: resource.reference() };
    Ok([PdfMutation::SetImage(SetImage { image, index: None }), paint_image(page, placement, id)])
}
