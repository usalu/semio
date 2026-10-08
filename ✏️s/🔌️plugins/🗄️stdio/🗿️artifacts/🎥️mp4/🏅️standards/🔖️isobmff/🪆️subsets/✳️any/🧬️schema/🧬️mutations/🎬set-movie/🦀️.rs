//! 🎬 `set-movie` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-movie")]
pub struct SetMovie {
    #[dsl(block)]
    pub movie: Mp4Movie,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for SetMovie {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "movie", kind: "set-movie", record: "SetMovie" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { movie } = self;
        protocol::MutationOutcome::new(Mp4Diff { ftyp: None, movie: Some(movie.clone()), tracks: None })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        Ok(vec![Mp4Mutation::SetMovie(set_movie::SetMovie { movie: base.movie.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set movie header", "Film-Header setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
