//! SH1106 display variant

use crate::{display::DisplayVariant, command::Command};
use display_interface::{AsyncWriteOnlyDataCommand, DisplayError};

/// Generic 128x64 with SH1106 controller
#[derive(Debug, Clone, Copy)]
pub struct Sh1106_128_64 {}

impl DisplayVariant for Sh1106_128_64 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 64;
    const COLUMN_OFFSET: u8 = 2;

    async fn init_column_mode<DI>(
        iface: &mut DI,
        //display_rotation: DisplayRotation,
    ) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand,
    {
        super::sh1107::init_column_mode_common(iface, Self::dimensions()).await?;
        Command::DisplayOffset(0).send(iface).await?;
        Command::ComPinConfig(true).send(iface).await?;

        Ok(())
    }
}
