use embedded_hal_mock::{
    i2c::{Mock as I2cMock, Transaction as I2cTrans},
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
    spi::{Mock as SpiMock, Transaction as SpiTrans},
};
use lsm303agr::{interface, mode, Lsm303agr};

#[allow(unused)]
pub const ACCEL_ADDR: u8 = 0b0011001;
#[allow(unused)]
pub const MAG_ADDR: u8 = 0b0011110;

#[allow(unused)]
pub const DEFAULT_CTRL_REG1_A: u8 = 0x7;

#[allow(unused)]
pub const DEFAULT_CFG_REG_A_M: u8 = 0x3;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const STATUS_REG_AUX_A: u8 = 0x07;
    pub const OUT_TEMP_L_A: u8 = 0x0C;
    pub const WHO_AM_I_A: u8 = 0x0F;
    pub const TEMP_CFG_REG_A: u8 = 0x1F;
    pub const CTRL_REG1_A: u8 = 0x20;
    pub const CTRL_REG3_A: u8 = 0x22;
    pub const CTRL_REG4_A: u8 = 0x23;
    pub const CTRL_REG5_A: u8 = 0x24;
    pub const FIFO_CTRL_REG_A: u8 = 0x2E;
    pub const STATUS_REG_A: u8 = 0x27;
    pub const OUT_X_L_A: u8 = 0x28;
    pub const WHO_AM_I_M: u8 = 0x4F;
    pub const CFG_REG_A_M: u8 = 0x60;
    pub const CFG_REG_B_M: u8 = 0x61;
    pub const CFG_REG_C_M: u8 = 0x62;
    pub const STATUS_REG_M: u8 = 0x67;
    pub const OUTX_L_REG_M: u8 = 0x68;
}

#[allow(unused)]
pub const HZ50: u8 = 4 << 4;

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const SPI_RW: u8 = 1 << 7;
    pub const SPI_MS: u8 = 1 << 6;

    pub const LP_EN: u8 = 1 << 3;

    pub const ACCEL_BDU: u8 = 1 << 7;
    pub const HR: u8 = 1 << 3;

    pub const MAG_BDU: u8 = 1 << 4;

    pub const MAG_OFF_CANC: u8 = 1 << 1;
    pub const MAG_OFF_CANC_ONE_SHOT: u8 = 1 << 4;

    pub const XDR: u8 = 1;
    pub const YDR: u8 = 1 << 1;
    pub const ZDR: u8 = 1 << 2;
    pub const XYZDR: u8 = 1 << 3;
    pub const XOR: u8 = 1 << 4;
    pub const YOR: u8 = 1 << 5;
    pub const ZOR: u8 = 1 << 6;
    pub const XYZOR: u8 = 1 << 7;

    pub const TDA: u8 = 1 << 2;
    pub const TOR: u8 = 1 << 6;

    pub const TEMP_EN0: u8 = 1 << 6;
    pub const TEMP_EN1: u8 = 1 << 7;
}

#[allow(unused)]
pub fn default_cs() -> PinMock {
    default_cs_n(1)
}

#[allow(unused)]
pub fn default_cs_n(n: usize) -> PinMock {
    PinMock::new(
        &[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]
            .iter()
            .cycle()
            .take(n * 2)
            .cloned()
            .collect::<Vec<_>>(),
    )
}

#[allow(unused)]
pub fn new_spi_accel(
    transactions: &[SpiTrans],
    accel_cs: PinMock,
) -> Lsm303agr<interface::SpiInterface<SpiMock, PinMock, PinMock>, mode::MagOneShot> {
    Lsm303agr::new_with_spi(SpiMock::new(transactions), accel_cs, PinMock::new(&[]))
}

#[allow(unused)]
pub fn new_spi_mag(
    transactions: &[SpiTrans],
    mag_cs: PinMock,
) -> Lsm303agr<interface::SpiInterface<SpiMock, PinMock, PinMock>, mode::MagOneShot> {
    Lsm303agr::new_with_spi(SpiMock::new(transactions), PinMock::new(&[]), mag_cs)
}

#[allow(unused)]
pub fn new_spi(
    transactions: &[SpiTrans],
    accel_cs: PinMock,
    mag_cs: PinMock,
) -> Lsm303agr<interface::SpiInterface<SpiMock, PinMock, PinMock>, mode::MagOneShot> {
    Lsm303agr::new_with_spi(SpiMock::new(transactions), accel_cs, mag_cs)
}

#[allow(unused)]
pub fn destroy_spi<MODE>(
    sensor: Lsm303agr<interface::SpiInterface<SpiMock, PinMock, PinMock>, MODE>,
) {
    let (mut spi, mut accel_cs, mut mag_cs) = sensor.destroy();
    spi.done();
    accel_cs.done();
    mag_cs.done();
}

#[allow(unused)]
pub fn new_i2c(
    transactions: &[I2cTrans],
) -> Lsm303agr<interface::I2cInterface<I2cMock>, mode::MagOneShot> {
    Lsm303agr::new_with_i2c(I2cMock::new(transactions))
}

#[allow(unused)]
pub fn destroy_i2c<MODE>(sensor: Lsm303agr<interface::I2cInterface<I2cMock>, MODE>) {
    sensor.destroy().done();
}

#[macro_export]
macro_rules! assert_eq_xyz {
    ($data:expr, $x_unit:ident, $y_unit:ident, $z_unit:ident, $xyz_unit:ident) => {{
        let (x_raw, y_raw, z_raw) = $data.xyz_raw();
        assert_eq!($data.x_raw(), x_raw);
        assert_eq!($data.y_raw(), y_raw);
        assert_eq!($data.z_raw(), z_raw);

        let (x_unscaled, y_unscaled, z_unscaled) = $data.xyz_unscaled();
        assert_eq!($data.x_unscaled(), x_unscaled);
        assert_eq!($data.y_unscaled(), y_unscaled);
        assert_eq!($data.z_unscaled(), z_unscaled);

        let (x_unit, y_unit, z_unit) = $data.$xyz_unit();
        assert_eq!($data.$x_unit(), x_unit);
        assert_eq!($data.$y_unit(), y_unit);
        assert_eq!($data.$z_unit(), z_unit);
    }};
}
