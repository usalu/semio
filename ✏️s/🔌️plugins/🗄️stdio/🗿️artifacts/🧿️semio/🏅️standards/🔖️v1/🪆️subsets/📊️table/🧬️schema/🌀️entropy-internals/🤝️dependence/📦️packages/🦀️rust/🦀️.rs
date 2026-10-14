//! 🤝️ Dependence measures over table columns: divergences, mutual and transfer information, partial information decomposition, matrix entropies and Fisher criteria.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod entropy_internals {
        #[path = "../../↔️divergence/🦀️.rs"]
        pub mod divergence;
        #[path = "../../🤝️mutual/🦀️.rs"]
        pub mod mutual;
        #[path = "../../🧩️pid/🦀️.rs"]
        pub mod pid;
        #[path = "../../🎣️fisher/🦀️.rs"]
        pub mod fisher;
        #[path = "../../🔲️matrix/🦀️.rs"]
        pub mod matrix;
        #[path = "../../➡️transfer/🦀️.rs"]
        pub mod transfer;
    }
}
