//! 🌊 Wave Function Collapse as a finite-domain constraint solver: one propagation/search kernel
//! under three solvers (arbitrary graphs, dense 2D grids, dense 3D grids), with compiled
//! tile/pattern models, global constraints, and deterministic replayable search.

// #region 🔖Modules
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀ids.rs"]
pub mod ids;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀bitset.rs"]
pub mod bitset;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀weights.rs"]
pub mod weights;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀error.rs"]
pub mod error;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀domain.rs"]
pub mod domain;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀model.rs"]
pub mod model;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀oracle.rs"]
pub mod oracle;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀tiled.rs"]
pub mod tiled;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀topology.rs"]
pub(crate) mod topology;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀propagate.rs"]
pub(crate) mod propagate;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀prop_ac3.rs"]
pub(crate) mod prop_ac3;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀prop_ac4.rs"]
pub(crate) mod prop_ac4;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀heuristics.rs"]
pub mod heuristics;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀sample.rs"]
pub mod sample;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀trail.rs"]
pub(crate) mod trail;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀constraint.rs"]
pub mod constraint;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀constraints_card.rs"]
pub mod constraints_card;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀constraints_conn.rs"]
pub mod constraints_conn;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀soft.rs"]
pub mod soft;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀flow.rs"]
pub mod flow;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀sparse3d.rs"]
pub mod sparse3d;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀motif.rs"]
pub(crate) mod motif;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀beam.rs"]
pub(crate) mod beam;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀nogood.rs"]
pub(crate) mod nogood;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀search.rs"]
pub mod search;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀repair.rs"]
pub(crate) mod repair;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀parallel.rs"]
pub(crate) mod parallel;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀chunk.rs"]
pub(crate) mod chunk;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀hierarchy.rs"]
pub(crate) mod hierarchy;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀evolve.rs"]
pub mod evolve;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀outcome.rs"]
pub mod outcome;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀diag.rs"]
pub mod diag;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀solver_graph.rs"]
pub mod solver_graph;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀grid2d.rs"]
pub mod grid2d;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀solver_🦀grid2d.rs"]
pub mod solver_grid2d;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀symmetry.rs"]
pub mod symmetry;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀extract.rs"]
pub mod extract;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀grid3d.rs"]
pub mod grid3d;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀solver_🦀grid3d.rs"]
pub mod solver_grid3d;
#[path = "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🦀rust/🧩wfc/📂src/🦀serial.rs"]
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
