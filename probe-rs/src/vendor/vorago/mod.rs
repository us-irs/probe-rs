//! Vorago vendor support.

use probe_rs_target::Chip;
use sequences::va108xx::Va108xx;
use sequences::va416xx::Va416xx;

use crate::{config::DebugSequence, vendor::Vendor};

pub mod sequences;

/// Vorago Technologies
#[derive(docsplay::Display)]
pub struct Vorago;

impl Vendor for Vorago {
    fn try_create_debug_sequence(&self, chip: &Chip) -> Option<DebugSequence> {
        let sequence = if chip.name.starts_with("VA416xx") {
            DebugSequence::Arm(Va416xx::create())
        } else if chip.name.starts_with("VA108xx") {
            DebugSequence::Arm(Va108xx::create())
        } else {
            return None;
        };

        Some(sequence)
    }
}
