//! 🕸️ Shared mesh-topology summary contract.

//#region 🕸️Topology
#[derive(Clone, Copy)]
pub(crate) struct Topology {
    pub(crate) components: u64,
    pub(crate) boundary_loops: u64,
    pub(crate) chi: i64,
    pub(crate) genus: Option<u64>,
    pub(crate) manifold: bool,
    pub(crate) watertight: bool,
    pub(crate) oriented: bool,
}
//#endregion 🕸️Topology
