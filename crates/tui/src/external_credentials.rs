//! Capability-gated I/O for credentials owned by another CLI.
//!
//! Keeping every external stat/read behind an opaque grant makes the disabled
//! default enforceable at the filesystem boundary instead of relying on UI
//! state or caller discipline.

use std::fs;

use anyhow::{Context, Result};
use codewhale_config::ExternalCredentialReadGrant;

#[cfg(test)]
use std::cell::Cell;

#[cfg(test)]
thread_local! {
    static STAT_CALLS: Cell<usize> = const { Cell::new(0) };
    static READ_CALLS: Cell<usize> = const { Cell::new(0) };
}

#[must_use]
pub(crate) fn exists(grant: &ExternalCredentialReadGrant) -> bool {
    #[cfg(test)]
    STAT_CALLS.with(|count| count.set(count.get() + 1));
    grant.path().exists()
}

pub(crate) fn read_to_string(grant: &ExternalCredentialReadGrant) -> Result<String> {
    #[cfg(test)]
    READ_CALLS.with(|count| count.set(count.get() + 1));
    fs::read_to_string(grant.path()).with_context(|| {
        format!(
            "reading external {} credential file {}",
            grant.source().as_str(),
            grant.path().display()
        )
    })
}

#[cfg(test)]
pub(crate) fn reset_side_effect_trap() {
    STAT_CALLS.with(|count| count.set(0));
    READ_CALLS.with(|count| count.set(0));
}

#[cfg(test)]
#[must_use]
pub(crate) fn side_effect_trap_counts() -> (usize, usize) {
    (STAT_CALLS.with(Cell::get), READ_CALLS.with(Cell::get))
}
