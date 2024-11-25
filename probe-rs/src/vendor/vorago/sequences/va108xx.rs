//! Support for the Vorago VA108xx device family.
use std::sync::Arc;

use crate::architecture::arm::sequences::ArmDebugSequence;

/// Marker structure for the VA108xx device
#[derive(Debug)]
pub struct Va108xx;

impl Va108xx {
    /// Create the sequencer
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl ArmDebugSequence for Va108xx {}
