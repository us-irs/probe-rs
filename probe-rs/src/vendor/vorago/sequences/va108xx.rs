//! Support for the Vorago VA108xx device family.
use std::{sync::Arc, time::Duration};

use crate::{
    architecture::arm::{
        core::{
            cortex_m::Vtor,
            registers::cortex_m::{PC, SP},
        },
        sequences::ArmDebugSequence,
    }, CoreInterface, MemoryInterface, MemoryMappedRegister, Session
};

const SYSCONFIG_PERIPH_OFFSET: u32 = 0x4000_0000;
const RST_CTRL_ROM_OFFSET: u32 = 4;

const RST_CTRL_ROM_NO_SYSRSTREQ: u32 = 0b111011;

/// Marker structure for the VA108xx device
#[derive(Debug)]
pub struct Va108xx;

impl Va108xx {
    /// Create the sequencer
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl ArmDebugSequence for Va108xx {
    /// This ARM sequence is called if an image was flashed to RAM directly.
    /// It will perform the necessary preparation to run that image.
    fn prepare_running_on_ram(
        &self,
        vector_table_offset: u64,
        session: &mut Session,
    ) -> Result<(), crate::Error> {
        tracing::info!("Performing RAM flash start");
        const SP_MAIN_OFFSET: usize = 0;
        const RESET_VECTOR_OFFSET: usize = 1;

        tracing::debug!("RAM flash start for Cortex-M single core target");
        let mut core = session.core(0)?;
        // Ensure the core is halted in any case.
        core.halt(Duration::from_millis(100))?;

        // Disable the loading of memory from ROM into RAM.
        core.write_32(
            SYSCONFIG_PERIPH_OFFSET as u64 + RST_CTRL_ROM_OFFSET as u64,
            &[RST_CTRL_ROM_NO_SYSRSTREQ],
        )?;

        // See ARMv7-M Architecture Reference Manual Chapter B1.5 for more details. The
        // process appears to be the same for the other Cortex-M architectures as well.
        let vtor = Vtor(vector_table_offset as u32);
        let mut first_table_entries: [u32; 2] = [0; 2];
        core.read_32(vector_table_offset, &mut first_table_entries)?;
        // The first entry in the vector table is the SP_main reset value of the main stack pointer,
        // so we set the stack pointer register accordingly.
        core.write_core_reg(SP.id, first_table_entries[SP_MAIN_OFFSET])?;
        // The second entry in the vector table is the reset vector. It needs to be loaded
        // as the initial PC value on a reset, see chapter A2.3.1 of the reference manual.
        core.write_core_reg(PC.id, first_table_entries[RESET_VECTOR_OFFSET])?;
        core.write_word_32(Vtor::get_mmio_address(), vtor.0)?;

        core.reset_and_halt(Duration::from_millis(200))?;
        Ok(())
    }
}
