//! 🌊️ Wave Function Collapse as a finite-domain constraint solver: one propagation/search kernel
//! under three solvers (arbitrary graphs, dense 2D grids, dense 3D grids), with compiled
//! tile/pattern models, global constraints, and deterministic replayable search.

// #region 🔖️Modules
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️beam.rs"]
pub(crate) mod beam;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️bitset.rs"]
pub mod bitset;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️chunk.rs"]
pub(crate) mod chunk;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️constraint.rs"]
pub mod constraint;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️constraints_card.rs"]
pub mod constraints_card;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️constraints_conn.rs"]
pub mod constraints_conn;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️diag.rs"]
pub mod diag;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️domain.rs"]
pub mod domain;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️error.rs"]
pub mod error;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️evolve.rs"]
pub mod evolve;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️extract.rs"]
pub mod extract;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️flow.rs"]
pub mod flow;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️grid2d.rs"]
pub mod grid2d;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️grid3d.rs"]
pub mod grid3d;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️heuristics.rs"]
pub mod heuristics;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️hierarchy.rs"]
pub(crate) mod hierarchy;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️ids.rs"]
pub mod ids;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️model.rs"]
pub mod model;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️motif.rs"]
pub(crate) mod motif;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️nogood.rs"]
pub(crate) mod nogood;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️oracle.rs"]
pub mod oracle;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️outcome.rs"]
pub mod outcome;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️parallel.rs"]
pub(crate) mod parallel;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️prop_ac3.rs"]
pub(crate) mod prop_ac3;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️prop_ac4.rs"]
pub(crate) mod prop_ac4;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️propagate.rs"]
pub(crate) mod propagate;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️repair.rs"]
pub(crate) mod repair;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️sample.rs"]
pub mod sample;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️search.rs"]
pub mod search;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️serial.rs"]
pub mod serial;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️soft.rs"]
pub mod soft;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️solver_graph.rs"]
pub mod solver_graph;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️solver_grid2d.rs"]
pub mod solver_grid2d;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️solver_grid3d.rs"]
pub mod solver_grid3d;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️sparse3d.rs"]
pub mod sparse3d;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️symmetry.rs"]
pub mod symmetry;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️tiled.rs"]
pub mod tiled;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️topology.rs"]
pub(crate) mod topology;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️trail.rs"]
pub(crate) mod trail;
#[path = "../../../🧩️wfc/⚡️implementations/🦀️rust/📂️src/🦀️weights.rs"]
pub mod weights;
// #endregion 🔖️Modules

// #region 🔖️Exports
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
// #endregion 🔖️Exports
