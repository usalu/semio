//! 🧮️ The source-authorized leaf compiler emits immutable input declarations, exact fingerprints and explicit refusal facts.
use std::path::Path;
use quote::quote;
use semio_framework_value::ToValue;

pub(super) fn compile(payload: &Path, documents: &[String], contract: &syn::Path) -> Result<proc_macro2::TokenStream, String> {
    let payload = crate::compiler_resources::read(payload).map_err(|error| error.to_string())?;
    let payload = std::str::from_utf8(&payload).map_err(|error| error.to_string())?;
    let documents = documents.iter().map(|path| crate::compiler_resources::read(Path::new(path)).map_err(|error| error.to_string())).collect::<Result<Vec<_>, _>>()?;
    let documents = documents.iter().map(|bytes| std::str::from_utf8(bytes).map_err(|error| error.to_string())).collect::<Result<Vec<_>, _>>()?;
    let compiled = semio_framework_manifest_input_compiler::compile_input_definitions(payload, &documents);
    let schema = compiled.schema_sha256.iter();
    let documents = compiled.document_sha256.iter().map(|digest| { let bytes = digest.iter(); quote!([#(#bytes),*]) }).collect::<Vec<_>>();
    let framework = compiled.framework_document_sha256.iter().map(|digest| { let bytes = digest.iter(); quote!([#(#bytes),*]) }).collect::<Vec<_>>();
    let output = |result: Result<String, semio_framework_manifest_input_compiler::InputSchemaError>| -> Result<proc_macro2::TokenStream, String> {
        Ok(match result {
            Ok(json) => quote!(::core::result::Result::Ok(#json)),
            Err(error) => {
                let code = error.code.to_value();
                let code = code.as_str().ok_or_else(|| "compiled input refusal has no canonical code".to_string())?;
                let pointer = &error.pointer; let detail = &error.detail;
                quote!(::core::result::Result::Err(#contract::CompiledMutationInputError { code: #code, pointer: #pointer, detail: #detail }))
            },
        })
    };
    let definitions = output(compiled.definitions_json)?;
    let seeds = output(compiled.seed_definitions_json)?;
    Ok(quote!(::core::option::Option::Some(#contract::CompiledMutationInputDefinitions {
        provenance: <Self as #contract::MutationLeaf>::PROVENANCE,
        semantic_kind: <Self as #contract::MutationLeaf>::DESCRIPTOR.semantic_kind,
        schema_sha256: [#(#schema),*],
        document_sha256: &[#(#documents),*],
        framework_document_sha256: &[#(#framework),*],
        definitions_json: #definitions,
        seed_definitions_json: #seeds,
    })))
}
