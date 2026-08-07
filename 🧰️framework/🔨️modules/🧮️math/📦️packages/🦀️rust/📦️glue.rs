//! 🧮️ The semio math framework: one crate for every mathematical domain the OS kernel, the s-modules and the plugins compute with.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_os_kernel as dsl_schema;
extern crate semio_framework_os_kernel as dsl;
pub use dsl_core::os_dsl;
#[path = "../../➕️algebra/🦀️component.rs"]
pub mod algebra;

#[path = "../../🧮️cas/🦀️component.rs"]
pub mod cas;

#[path = "../../🔗️causal/🦀️component.rs"]
pub mod causal;

#[path = "../../🎲️entropy/🦀️component.rs"]
pub mod entropy;

#[path = "../../🌫️fuzzy/🦀️component.rs"]
pub mod fuzzy;

#[path = "../../📐️geometry/🦀️component.rs"]
pub mod geometry;

#[path = "../../🔷️lie/🦀️component.rs"]
pub mod lie;

#[path = "../../🔢️number/🦀️component.rs"]
pub mod number;

#[path = "../../🎯️optimize/🦀️component.rs"]
pub mod optimize;

#[path = "../../📈️polynomial/🦀️component.rs"]
pub mod polynomial;

#[path = "../../🎲️probability/🦀️component.rs"]
pub mod probability;

#[path = "../../🎲️random/🦀️component.rs"]
pub mod random;

#[path = "../../🎯️sampling/🦀️component.rs"]
pub mod sampling;

#[path = "../../📶️signal/🦀️component.rs"]
pub mod signal;

#[path = "../../🗺️spatial/🦀️component.rs"]
pub mod spatial;

#[path = "../../📊️statistics/🦀️component.rs"]
pub mod statistics;

#[path = "../../📋️tabular/🦀️component.rs"]
pub mod tabular;

#[path = "."]
pub mod graph {
    #[path = "../../🕸️graph/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🕸️graph/🛂️manifest/🦀️component.rs"]
    pub mod manifest;

    #[path = "../../🕸️graph/🚶️traversal/🦀️component.rs"]
    pub mod traversal;

    #[path = "../../🕸️graph/🔧️operators/🦀️component.rs"]
    pub mod operators;

    #[path = "../../🕸️graph/🖊️drawing/🦀️component.rs"]
    pub mod drawing;

    #[path = "../../🕸️graph/🗣️dsl/🦀️component.rs"]
    pub mod dsl;

    #[path = "."]
    pub mod normal {
        #[path = "../../🕸️graph/➕️normal/↔️undirected/🦀️component.rs"]
        pub mod undirected;

        #[path = "../../🕸️graph/➕️normal/➡️directed/🦀️component.rs"]
        pub mod directed;
    }

    #[path = "."]
    pub mod ports {
        #[path = "../../🕸️graph/🔌️ports/↔️undirected/🦀️component.rs"]
        pub mod undirected;

        #[path = "."]
        pub mod directed {
            #[path = "../../🕸️graph/🔌️ports/➡️directed/➕️normal/🦀️component.rs"]
            pub mod normal;
        }
    }
}

#[path = "."]
pub mod wfc {
    #[path = "../../🧩️wfc/🔦️beam/🦀️component.rs"]
    pub(crate) mod beam;
    #[path = "../../🧩️wfc/🎛️bitset/🦀️component.rs"]
    pub mod bitset;
    #[path = "../../🧩️wfc/🍰️chunk/🦀️component.rs"]
    pub(crate) mod chunk;
    #[path = "../../🧩️wfc/⛓️constraint/🦀️component.rs"]
    pub mod constraint;
    #[path = "../../🧩️wfc/🔢️constraints-card/🦀️component.rs"]
    pub mod constraints_card;
    #[path = "../../🧩️wfc/🔗️constraints-conn/🦀️component.rs"]
    pub mod constraints_conn;
    #[path = "../../🧩️wfc/🩺️diag/🦀️component.rs"]
    pub mod diag;
    #[path = "../../🧩️wfc/🌐️domain/🦀️component.rs"]
    pub mod domain;
    #[path = "../../🧩️wfc/⚠️error/🦀️component.rs"]
    pub mod error;
    #[path = "../../🧩️wfc/🧬️evolve/🦀️component.rs"]
    pub mod evolve;
    #[path = "../../🧩️wfc/⛏️extract/🦀️component.rs"]
    pub mod extract;
    #[path = "../../🧩️wfc/🌊️flow/🦀️component.rs"]
    pub mod flow;
    #[path = "../../🧩️wfc/🔲️grid-2d/🦀️component.rs"]
    pub mod grid2d;
    #[path = "../../🧩️wfc/🧊️grid-3d/🦀️component.rs"]
    pub mod grid3d;
    #[path = "../../🧩️wfc/🧭️heuristics/🦀️component.rs"]
    pub mod heuristics;
    #[path = "../../🧩️wfc/🪜️hierarchy/🦀️component.rs"]
    pub(crate) mod hierarchy;
    #[path = "../../🧩️wfc/🆔️ids/🦀️component.rs"]
    pub mod ids;
    #[path = "../../🧩️wfc/🏗️model/🦀️component.rs"]
    pub mod model;
    #[path = "../../🧩️wfc/🎼️motif/🦀️component.rs"]
    pub(crate) mod motif;
    #[path = "../../🧩️wfc/🚫️nogood/🦀️component.rs"]
    pub(crate) mod nogood;
    #[path = "../../🧩️wfc/🔮️oracle/🦀️component.rs"]
    pub mod oracle;
    #[path = "../../🧩️wfc/🏁️outcome/🦀️component.rs"]
    pub mod outcome;
    #[path = "../../🧩️wfc/🧵️parallel/🦀️component.rs"]
    pub(crate) mod parallel;
    #[path = "../../🧩️wfc/🔁️prop-ac3/🦀️component.rs"]
    pub(crate) mod prop_ac3;
    #[path = "../../🧩️wfc/🔄️prop-ac4/🦀️component.rs"]
    pub(crate) mod prop_ac4;
    #[path = "../../🧩️wfc/📣️propagate/🦀️component.rs"]
    pub(crate) mod propagate;
    #[path = "../../🧩️wfc/🔧️repair/🦀️component.rs"]
    pub(crate) mod repair;
    #[path = "../../🧩️wfc/🎲️sample/🦀️component.rs"]
    pub mod sample;
    #[path = "../../🧩️wfc/🔍️search/🦀️component.rs"]
    pub mod search;
    #[path = "../../🧩️wfc/💾️serial/🦀️component.rs"]
    pub mod serial;
    #[path = "../../🧩️wfc/🪶️soft/🦀️component.rs"]
    pub mod soft;
    #[path = "../../🧩️wfc/🕸️solver-graph/🦀️component.rs"]
    pub mod solver_graph;
    #[path = "../../🧩️wfc/🔳️solver-grid-2d/🦀️component.rs"]
    pub mod solver_grid2d;
    #[path = "../../🧩️wfc/🧱️solver-grid-3d/🦀️component.rs"]
    pub mod solver_grid3d;
    #[path = "../../🧩️wfc/🕳️sparse-3d/🦀️component.rs"]
    pub mod sparse3d;
    #[path = "../../🧩️wfc/🪞️symmetry/🦀️component.rs"]
    pub mod symmetry;
    #[path = "../../🧩️wfc/🀄️tiled/🦀️component.rs"]
    pub mod tiled;
    #[path = "../../🧩️wfc/🗺️topology/🦀️component.rs"]
    pub(crate) mod topology;
    #[path = "../../🧩️wfc/🐾️trail/🦀️component.rs"]
    pub(crate) mod trail;
    #[path = "../../🧩️wfc/⚖️weights/🦀️component.rs"]
    pub mod weights;

    pub use beam::BeamConfig;
    pub use bitset::PatternSet;
    pub use constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
    pub use constraints_card::{CardinalityConstraint, Scope};
    pub use constraints_conn::{ConnectivityConstraint, ReachabilityConstraint};
    pub use diag::{DiagLevel, Event, EventSink, Metrics, TraceReplay};
    pub use domain::{Domain, DomainStore, RestrictResult};
    pub use error::{ConstraintError, ModelError, SolveError, TopologyError};
    pub use evolve::{evolve, EvolveConfig, EvolveResult};
    pub use extract::{extract_2d, Extract2dConfig, ExtractedModel2d, PatternDecoder2d, Sample2d};
    pub use flow::FlowConstraint;
    pub use grid2d::{declare_stencil_relations, declare_stencil_relations_tiled, Boundary, Grid2dTopology, Stencil2d};
    pub use grid3d::{declare_stencil_relations_3d, declare_stencil_relations_3d_tiled, Grid3dTopology, Stencil3d};
    pub use heuristics::ObserveHeuristic;
    pub use ids::{ConstraintId, DecisionId, NodeId, PatternId, PortId, RegionId, RelationId, TileId};
    pub use model::{CompiledModel, LintFinding, ModelBuilder, ModelStats, PatternInfo, RelationInfo};
    pub use nogood::NogoodConfig;
    pub use oracle::{check_assignment, enumerate, ArcSpec, OracleResult, Violation};
    pub use outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
    pub use sample::ValueSampler;
    pub use search::{Budget, CancelToken, RestartSchedule, SearchConfig, SearchMode};
    pub use serial::{CheckpointDoc, PairDoc, PatternDoc, RelationDoc, SourceModelDoc, CHECKPOINT_VERSION, SOURCE_MODEL_VERSION};
    pub use soft::{best_of_n, Attempt, BestOfNKeep, ScoreFn, SoftConstraint, WeightField};
    pub use solver_graph::{GraphSolver, GraphSolverBuilder};
    pub use solver_grid2d::{Grid2dSolver, Grid2dSolverBuilder};
    pub use solver_grid3d::{Grid3dSolver, Grid3dSolverBuilder};
    pub use sparse3d::{SparseVolume, VoxelCoord};
    pub use symmetry::{cube_rotations_24, cube_symmetries_48, SymmetryGroup2d, SymmetryGroup3d, Transform2d, Transform3d};
    pub use tiled::TiledModelBuilder;
    pub use topology::{from_graph_view, GraphTopology, GraphTopologyBuilder};
    pub use trail::Checkpoint;
    pub use weights::{WeightMode, WeightTable, ZeroWeightPolicy};
}
