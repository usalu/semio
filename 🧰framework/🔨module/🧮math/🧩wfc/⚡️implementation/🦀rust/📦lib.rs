//! 🌊 Wave Function Collapse as a finite-domain constraint solver: one propagation/search kernel
//! under three solvers (arbitrary graphs, dense 2D grids, dense 3D grids), with compiled
//! tile/pattern models, global constraints, and deterministic replayable search.

// #region 🔖Modules
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀ids.rs"]
pub mod ids;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀bitset.rs"]
pub mod bitset;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀weights.rs"]
pub mod weights;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀error.rs"]
pub mod error;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀domain.rs"]
pub mod domain;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀model.rs"]
pub mod model;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀oracle.rs"]
pub mod oracle;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀tiled.rs"]
pub mod tiled;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀topology.rs"]
pub(crate) mod topology;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀propagate.rs"]
pub(crate) mod propagate;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀prop_ac3.rs"]
pub(crate) mod prop_ac3;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀prop_ac4.rs"]
pub(crate) mod prop_ac4;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀heuristics.rs"]
pub mod heuristics;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀sample.rs"]
pub mod sample;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀trail.rs"]
pub(crate) mod trail;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀constraint.rs"]
pub mod constraint;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀constraints_card.rs"]
pub mod constraints_card;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀constraints_conn.rs"]
pub mod constraints_conn;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀soft.rs"]
pub mod soft;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀flow.rs"]
pub mod flow;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀sparse3d.rs"]
pub mod sparse3d;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀motif.rs"]
pub(crate) mod motif;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀beam.rs"]
pub(crate) mod beam;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀nogood.rs"]
pub(crate) mod nogood;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀search.rs"]
pub mod search;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀repair.rs"]
pub(crate) mod repair;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀parallel.rs"]
pub(crate) mod parallel;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀chunk.rs"]
pub(crate) mod chunk;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀hierarchy.rs"]
pub(crate) mod hierarchy;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀evolve.rs"]
pub mod evolve;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀outcome.rs"]
pub mod outcome;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀diag.rs"]
pub mod diag;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀solver_graph.rs"]
pub mod solver_graph;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀grid2d.rs"]
pub mod grid2d;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀solver_grid2d.rs"]
pub mod solver_grid2d;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀symmetry.rs"]
pub mod symmetry;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀extract.rs"]
pub mod extract;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀grid3d.rs"]
pub mod grid3d;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀solver_grid3d.rs"]
pub mod solver_grid3d;
#[path = "../../../🧩wfc/⚡️implementation/🦀rust/📂src/🦀serial.rs"]
pub mod serial;
// #endregion 🔖Modules

// #region 🔖Exports
pub use beam::BeamConfig;
pub use bitset::PatternSet;
pub use constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
pub use constraints_card::{CardinalityConstraint, Scope};
pub use constraints_conn::{ConnectivityConstraint, ReachabilityConstraint};
pub use diag::{DiagLevel, Event, EventSink, Metrics, TraceReplay};
pub use domain::{Domain, DomainStore, RestrictResult};
pub use error::{ConstraintError, ModelError, SolveError, TopologyError};
pub use evolve::{EvolveConfig, EvolveResult, evolve};
pub use extract::{Extract2dConfig, ExtractedModel2d, PatternDecoder2d, Sample2d, extract_2d};
pub use flow::FlowConstraint;
pub use grid2d::{Boundary, Grid2dTopology, Stencil2d, declare_stencil_relations, declare_stencil_relations_tiled};
pub use grid3d::{Grid3dTopology, Stencil3d, declare_stencil_relations_3d, declare_stencil_relations_3d_tiled};
pub use heuristics::ObserveHeuristic;
pub use ids::{ConstraintId, DecisionId, NodeId, PatternId, PortId, RegionId, RelationId, TileId};
pub use model::{CompiledModel, LintFinding, ModelBuilder, ModelStats, PatternInfo, RelationInfo};
pub use nogood::NogoodConfig;
pub use oracle::{ArcSpec, OracleResult, Violation, check_assignment, enumerate};
pub use outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
pub use sample::ValueSampler;
pub use search::{Budget, CancelToken, RestartSchedule, SearchConfig, SearchMode};
pub use serial::{CHECKPOINT_VERSION, CheckpointDoc, PairDoc, PatternDoc, RelationDoc, SOURCE_MODEL_VERSION, SourceModelDoc};
pub use soft::{Attempt, BestOfNKeep, ScoreFn, SoftConstraint, WeightField, best_of_n};
pub use solver_grid2d::{Grid2dSolver, Grid2dSolverBuilder};
pub use solver_grid3d::{Grid3dSolver, Grid3dSolverBuilder};
pub use solver_graph::{GraphSolver, GraphSolverBuilder};
pub use sparse3d::{SparseVolume, VoxelCoord};
pub use symmetry::{SymmetryGroup2d, SymmetryGroup3d, Transform2d, Transform3d, cube_rotations_24, cube_symmetries_48};
pub use tiled::TiledModelBuilder;
pub use topology::{GraphTopology, GraphTopologyBuilder, from_graph_view};
pub use trail::Checkpoint;
pub use weights::{WeightMode, WeightTable, ZeroWeightPolicy};
// #endregion 🔖Exports
