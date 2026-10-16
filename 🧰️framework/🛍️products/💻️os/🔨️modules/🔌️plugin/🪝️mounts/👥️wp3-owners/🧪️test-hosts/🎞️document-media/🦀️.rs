//! 🧪️ Facade test host of `document-media`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod document_media_unit_1 {
    use semio_framework::MediaError;
    use semio_framework_plugin_document_media::media_operation_refusal::MediaOperationRefusal;
    use semio_framework_plugin_document_media::media_operation_refusal::MediaOperationSource;
    use semio_framework::MediaType;
    use semio_framework_job::OriginalDomainProjection;
    use semio_framework_value::RetainedCloneProgress;
    use semio_framework_job::OriginalDomainView as V;
    use semio_framework_plugin_decl_app::media_type;
    use semio_framework_job::OriginalDomainFailure;
    #[cfg(any(test,feature="media-turn-testing"))]
    #[path="../../../../🎞️media/⚠️turn/🧪️tests/🦀️.rs"]
    pub mod tests;
}
