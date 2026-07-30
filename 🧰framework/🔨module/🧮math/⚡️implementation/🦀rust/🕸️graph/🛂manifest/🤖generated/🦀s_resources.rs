// Generated from s-resources.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const SRESOURCES_DESCRIPTOR_R2D_NOTE: &str = "2d.note";
pub const SRESOURCES_DESCRIPTOR_R2D_DRAWING: &str = "2d.drawing";
pub const SRESOURCES_DESCRIPTOR_R2D_RASTER: &str = "2d.raster";
pub const SRESOURCES_DESCRIPTOR_R2D_MAP: &str = "2d.map";
pub const SRESOURCES_DESCRIPTOR_R2D_PROCEDURAL: &str = "2d.procedural";
pub const SRESOURCES_DESCRIPTOR_R2D_SHOOTING: &str = "2d.shooting";
pub const SRESOURCES_DESCRIPTOR_R2D_PUZZLE: &str = "2d.puzzle";
pub const SRESOURCES_DESCRIPTOR_R3D_PUZZLE: &str = "3d.puzzle";
pub const SRESOURCES_DESCRIPTOR_R5D_PUZZLE: &str = "5d.puzzle";
pub const SRESOURCES_DESCRIPTOR_R3D_PROCEDURAL: &str = "3d.procedural";
pub const SRESOURCES_DESCRIPTOR_R3D_PROCESS: &str = "3d.process";
pub const SRESOURCES_DESCRIPTOR_R3D_CAD: &str = "3d.cad";
pub const SRESOURCES_DESCRIPTOR_COMPUTATION_FLOW: &str = "computation.flow";
pub const SRESOURCES_DESCRIPTOR_GRAPH_TRINITY: &str = "graph.trinity";
pub const SRESOURCES_DESCRIPTOR_GRAPH_DAG: &str = "graph.dag";
pub const SRESOURCES_DESCRIPTOR_TEXT_DOCUMENT: &str = "text.document";
pub const SRESOURCES_DESCRIPTOR_FORM_DICTIONARY: &str = "form.dictionary";
pub const SRESOURCES_DESCRIPTOR_KIT_COMPOSE: &str = "kit.compose";
pub const SRESOURCES_DESCRIPTOR_ANIMATE_PRESENT_DECK: &str = "animate.present.deck";
pub const SRESOURCES_DESCRIPTOR_R3D_MESH: &str = "3d.mesh";
pub const SRESOURCES_DESCRIPTOR_CATALOGUE_KINDS: &str = "catalogue.kinds";
pub const SRESOURCES_DESCRIPTOR_R3D_LOWPOLY: &str = "3d.lowpoly";
pub const SRESOURCES_DESCRIPTOR_COMPUTATION_SEQUENCE: &str = "computation.sequence";
pub const SRESOURCES_DESCRIPTOR_R2D_LAYOUT: &str = "2d.layout";
pub const SRESOURCES_DESCRIPTOR_COMPUTATION_IMPERATIVE: &str = "computation.imperative";
pub const SRESOURCES_DESCRIPTOR_VCS_DOCUMENT: &str = "vcs.document";
pub const SRESOURCES_DESCRIPTOR_PARAMETER_VALUE: &str = "parameter.value";
pub const SRESOURCES_DESCRIPTOR_CATALOGUE_SOURCING: &str = "catalogue.sourcing";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SResourcesDescriptorKind {
    #[serde(rename = "2d.note")]
    R2dNote,
    #[serde(rename = "2d.drawing")]
    R2dDrawing,
    #[serde(rename = "2d.raster")]
    R2dRaster,
    #[serde(rename = "2d.map")]
    R2dMap,
    #[serde(rename = "2d.procedural")]
    R2dProcedural,
    #[serde(rename = "2d.shooting")]
    R2dShooting,
    #[serde(rename = "2d.puzzle")]
    R2dPuzzle,
    #[serde(rename = "3d.puzzle")]
    R3dPuzzle,
    #[serde(rename = "5d.puzzle")]
    R5dPuzzle,
    #[serde(rename = "3d.procedural")]
    R3dProcedural,
    #[serde(rename = "3d.process")]
    R3dProcess,
    #[serde(rename = "3d.cad")]
    R3dCad,
    #[serde(rename = "computation.flow")]
    ComputationFlow,
    #[serde(rename = "graph.trinity")]
    GraphTrinity,
    #[serde(rename = "graph.dag")]
    GraphDag,
    #[serde(rename = "text.document")]
    TextDocument,
    #[serde(rename = "form.dictionary")]
    FormDictionary,
    #[serde(rename = "kit.compose")]
    KitCompose,
    #[serde(rename = "animate.present.deck")]
    AnimatePresentDeck,
    #[serde(rename = "3d.mesh")]
    R3dMesh,
    #[serde(rename = "catalogue.kinds")]
    CatalogueKinds,
    #[serde(rename = "3d.lowpoly")]
    R3dLowpoly,
    #[serde(rename = "computation.sequence")]
    ComputationSequence,
    #[serde(rename = "2d.layout")]
    R2dLayout,
    #[serde(rename = "computation.imperative")]
    ComputationImperative,
    #[serde(rename = "vcs.document")]
    VcsDocument,
    #[serde(rename = "parameter.value")]
    ParameterValue,
    #[serde(rename = "catalogue.sourcing")]
    CatalogueSourcing,
}

impl SResourcesDescriptorKind {
    pub const ALL: &'static [Self] = &[SResourcesDescriptorKind::R2dNote, SResourcesDescriptorKind::R2dDrawing, SResourcesDescriptorKind::R2dRaster, SResourcesDescriptorKind::R2dMap, SResourcesDescriptorKind::R2dProcedural, SResourcesDescriptorKind::R2dShooting, SResourcesDescriptorKind::R2dPuzzle, SResourcesDescriptorKind::R3dPuzzle, SResourcesDescriptorKind::R5dPuzzle, SResourcesDescriptorKind::R3dProcedural, SResourcesDescriptorKind::R3dProcess, SResourcesDescriptorKind::R3dCad, SResourcesDescriptorKind::ComputationFlow, SResourcesDescriptorKind::GraphTrinity, SResourcesDescriptorKind::GraphDag, SResourcesDescriptorKind::TextDocument, SResourcesDescriptorKind::FormDictionary, SResourcesDescriptorKind::KitCompose, SResourcesDescriptorKind::AnimatePresentDeck, SResourcesDescriptorKind::R3dMesh, SResourcesDescriptorKind::CatalogueKinds, SResourcesDescriptorKind::R3dLowpoly, SResourcesDescriptorKind::ComputationSequence, SResourcesDescriptorKind::R2dLayout, SResourcesDescriptorKind::ComputationImperative, SResourcesDescriptorKind::VcsDocument, SResourcesDescriptorKind::ParameterValue, SResourcesDescriptorKind::CatalogueSourcing];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::R2dNote => "2d.note",
            Self::R2dDrawing => "2d.drawing",
            Self::R2dRaster => "2d.raster",
            Self::R2dMap => "2d.map",
            Self::R2dProcedural => "2d.procedural",
            Self::R2dShooting => "2d.shooting",
            Self::R2dPuzzle => "2d.puzzle",
            Self::R3dPuzzle => "3d.puzzle",
            Self::R5dPuzzle => "5d.puzzle",
            Self::R3dProcedural => "3d.procedural",
            Self::R3dProcess => "3d.process",
            Self::R3dCad => "3d.cad",
            Self::ComputationFlow => "computation.flow",
            Self::GraphTrinity => "graph.trinity",
            Self::GraphDag => "graph.dag",
            Self::TextDocument => "text.document",
            Self::FormDictionary => "form.dictionary",
            Self::KitCompose => "kit.compose",
            Self::AnimatePresentDeck => "animate.present.deck",
            Self::R3dMesh => "3d.mesh",
            Self::CatalogueKinds => "catalogue.kinds",
            Self::R3dLowpoly => "3d.lowpoly",
            Self::ComputationSequence => "computation.sequence",
            Self::R2dLayout => "2d.layout",
            Self::ComputationImperative => "computation.imperative",
            Self::VcsDocument => "vcs.document",
            Self::ParameterValue => "parameter.value",
            Self::CatalogueSourcing => "catalogue.sourcing",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "2d.note" => Ok(Self::R2dNote),
            "2d.drawing" => Ok(Self::R2dDrawing),
            "2d.raster" => Ok(Self::R2dRaster),
            "2d.map" => Ok(Self::R2dMap),
            "2d.procedural" => Ok(Self::R2dProcedural),
            "2d.shooting" => Ok(Self::R2dShooting),
            "2d.puzzle" => Ok(Self::R2dPuzzle),
            "3d.puzzle" => Ok(Self::R3dPuzzle),
            "5d.puzzle" => Ok(Self::R5dPuzzle),
            "3d.procedural" => Ok(Self::R3dProcedural),
            "3d.process" => Ok(Self::R3dProcess),
            "3d.cad" => Ok(Self::R3dCad),
            "computation.flow" => Ok(Self::ComputationFlow),
            "graph.trinity" => Ok(Self::GraphTrinity),
            "graph.dag" => Ok(Self::GraphDag),
            "text.document" => Ok(Self::TextDocument),
            "form.dictionary" => Ok(Self::FormDictionary),
            "kit.compose" => Ok(Self::KitCompose),
            "animate.present.deck" => Ok(Self::AnimatePresentDeck),
            "3d.mesh" => Ok(Self::R3dMesh),
            "catalogue.kinds" => Ok(Self::CatalogueKinds),
            "3d.lowpoly" => Ok(Self::R3dLowpoly),
            "computation.sequence" => Ok(Self::ComputationSequence),
            "2d.layout" => Ok(Self::R2dLayout),
            "computation.imperative" => Ok(Self::ComputationImperative),
            "vcs.document" => Ok(Self::VcsDocument),
            "parameter.value" => Ok(Self::ParameterValue),
            "catalogue.sourcing" => Ok(Self::CatalogueSourcing),
            other => Err(format!("unknown descriptor kind {other:?} for SResources")),
        }
    }
}

pub const SRESOURCES_DESCRIPTOR_IDS: &[&str] = &["2d.note", "2d.drawing", "2d.raster", "2d.map", "2d.procedural", "2d.shooting", "2d.puzzle", "3d.puzzle", "5d.puzzle", "3d.procedural", "3d.process", "3d.cad", "computation.flow", "graph.trinity", "graph.dag", "text.document", "form.dictionary", "kit.compose", "animate.present.deck", "3d.mesh", "catalogue.kinds", "3d.lowpoly", "computation.sequence", "2d.layout", "computation.imperative", "vcs.document", "parameter.value", "catalogue.sourcing"];
pub const SRESOURCES_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"s-resources\",\"name\":\"S Resource Kinds\",\"descriptorKinds\":[{\"id\":\"2d.note\",\"name\":\"2D Note\",\"presentation\":{\"sourceFormat\":\"note.document\",\"componentKind\":\"note\",\"dimension\":\"2d\"}},{\"id\":\"2d.drawing\",\"name\":\"2D Drawing\",\"presentation\":{\"sourceFormat\":\"draw.document\",\"componentKind\":\"draw\",\"dimension\":\"2d\"}},{\"id\":\"2d.raster\",\"name\":\"2D Raster\",\"presentation\":{\"sourceFormat\":\"raster.document\",\"componentKind\":\"raster\",\"dimension\":\"2d\"}},{\"id\":\"2d.map\",\"name\":\"2D Map\",\"presentation\":{\"sourceFormat\":\"gis.map\",\"componentKind\":\"gismap\",\"dimension\":\"2d\"}},{\"id\":\"2d.procedural\",\"name\":\"2D Procedural\",\"presentation\":{\"sourceFormat\":\"procedural.2d\",\"componentKind\":\"puzzle2d\",\"dimension\":\"2d\"}},{\"id\":\"2d.shooting\",\"name\":\"2D Shooting\",\"presentation\":{\"sourceFormat\":\"shooting.scene\",\"componentKind\":\"shooting\",\"dimension\":\"2d\"}},{\"id\":\"2d.puzzle\",\"name\":\"2D Puzzle\",\"presentation\":{\"sourceFormat\":\"puzzle.2d\",\"componentKind\":\"puzzle2d\",\"dimension\":\"2d\"}},{\"id\":\"3d.puzzle\",\"name\":\"3D Puzzle\",\"presentation\":{\"sourceFormat\":\"puzzle.3d\",\"componentKind\":\"puzzle3d\",\"dimension\":\"3d\"}},{\"id\":\"5d.puzzle\",\"name\":\"5D Puzzle\",\"presentation\":{\"sourceFormat\":\"puzzle.5d\",\"componentKind\":\"puzzle5d\",\"dimension\":\"5d\"}},{\"id\":\"3d.procedural\",\"name\":\"3D Procedural\",\"presentation\":{\"sourceFormat\":\"procedural.3d\",\"componentKind\":\"puzzle3d\",\"dimension\":\"3d\"}},{\"id\":\"3d.process\",\"name\":\"3D Process\",\"presentation\":{\"sourceFormat\":\"process.3d\",\"componentKind\":\"puzzle3d\",\"dimension\":\"3d\"}},{\"id\":\"3d.cad\",\"name\":\"3D CAD\",\"presentation\":{\"sourceFormat\":\"cad.scene\",\"componentKind\":\"cad\",\"dimension\":\"3d\"}},{\"id\":\"computation.flow\",\"name\":\"Flow\",\"presentation\":{\"sourceFormat\":\"flow.document\",\"componentKind\":\"flow\",\"dimension\":\"graph\"}},{\"id\":\"graph.trinity\",\"name\":\"Trinity Graph\",\"presentation\":{\"sourceFormat\":\"trinity.graph\",\"componentKind\":\"trinity\",\"dimension\":\"graph\"}},{\"id\":\"graph.dag\",\"name\":\"DAG\",\"presentation\":{\"sourceFormat\":\"flow.dag\",\"componentKind\":\"dag\",\"dimension\":\"graph\"}},{\"id\":\"text.document\",\"name\":\"Text Document\",\"presentation\":{\"sourceFormat\":\"writer.document\",\"componentKind\":\"writer\",\"dimension\":\"text\"}},{\"id\":\"form.dictionary\",\"name\":\"Form Dictionary\",\"presentation\":{\"sourceFormat\":\"forms.dictionary\",\"componentKind\":\"forms\",\"dimension\":\"data\"}},{\"id\":\"kit.compose\",\"name\":\"Compose Kit\",\"presentation\":{\"sourceFormat\":\"compose.kit\",\"componentKind\":\"virtualFileSystem\",\"dimension\":\"kit\"}},{\"id\":\"animate.present.deck\",\"name\":\"Animate Present Deck\",\"presentation\":{\"sourceFormat\":\"animate.present.deck\",\"componentKind\":\"panel\",\"dimension\":\"2d\"}},{\"id\":\"3d.mesh\",\"name\":\"3D Mesh\",\"presentation\":{\"sourceFormat\":\"mesh.reference\",\"componentKind\":\"mesh\",\"dimension\":\"3d\"}},{\"id\":\"catalogue.kinds\",\"name\":\"Kind Catalogue\",\"presentation\":{\"sourceFormat\":\"catalogue.kinds\",\"componentKind\":\"catalogue\",\"dimension\":\"data\"}},{\"id\":\"3d.lowpoly\",\"name\":\"3D Lowpoly\",\"presentation\":{\"sourceFormat\":\"lowpoly.fixture\",\"componentKind\":\"lowpoly\",\"dimension\":\"3d\"}},{\"id\":\"computation.sequence\",\"name\":\"Sequence\",\"presentation\":{\"sourceFormat\":\"sequence.fixture\",\"componentKind\":\"sequence\",\"dimension\":\"graph\"}},{\"id\":\"2d.layout\",\"name\":\"Layout\",\"presentation\":{\"sourceFormat\":\"layout.fixture\",\"componentKind\":\"layout\",\"dimension\":\"2d\"}},{\"id\":\"computation.imperative\",\"name\":\"Imperative\",\"presentation\":{\"sourceFormat\":\"imperative.document\",\"componentKind\":\"imperative\",\"dimension\":\"graph\"}},{\"id\":\"vcs.document\",\"name\":\"VCS Document\",\"presentation\":{\"sourceFormat\":\"vcs.demo\",\"componentKind\":\"vcs\",\"dimension\":\"data\"}},{\"id\":\"parameter.value\",\"name\":\"Parameter\",\"presentation\":{\"sourceFormat\":\"parameter.value\",\"componentKind\":\"parameter\",\"dimension\":\"data\"}},{\"id\":\"catalogue.sourcing\",\"name\":\"Sourcing Curation\",\"presentation\":{\"sourceFormat\":\"sourcing.curate\",\"componentKind\":\"catalogue\",\"dimension\":\"data\"}}]}";

pub fn s_resources_manifest() -> Manifest {
    serde_json::from_str(SRESOURCES_MANIFEST_JSON).expect("manifest json")
}
