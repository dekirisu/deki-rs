pub use deki_core as core;
pub use deki_macros as macros;

#[cfg(feature = "proc")]
pub use deki_proc as proc;

// Re-export proc macros so they're available at `deki::*`
pub use deki_macros::{
    Cycle, ForceDefault, xoxo, quimp, imp, match_fns, foname,
};
