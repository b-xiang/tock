use core::cell::Cell;
use kernel::hil::gpio::Pin;
use kernel::hil::spi;
use virtual_spi::VirtualSpiMasterDevice;
use kernel::returncode::ReturnCode;
use rf233_const::{RF233Register, RF233BusCommand};

#[allow(unused_variables,dead_code,non_camel_case_types)]
enum InternalState {
    START,
    START_PART_READ,
    START_IRQ_READ,
    START_TURNING_OFF,
    START_STATUS_OFF,
    START_CTRL1_SET,
    START_CCA_SET,
    START_PWR_SET,
    START_CTRL2_SET,
    START_IRQMASK_SET,
    START_XAH_SET,
    START_FRAMERETRY_SET,
    START_CSMARETRY_SET,
    START_PANID1_SET,
    START_PANID2_SET,
    START_IEEE1_SET,
    START_IEEE2_SET,
    START_IEEE3_SET,
    START_IEEE4_SET,
    START_IEEE5_SET,
    START_IEEE6_SET,
    START_IEEE7_SET,
    START_IEEE8_SET,
    START_SHORT1_SET,
    START_SHORT2_SET,
    START_RPC_SET,

    ON_STATUS_READ,
    ON_PLL_SET,

    READY,

    UNKNOWN,
}


pub struct RF233 <'a, S: spi::SpiMasterDevice + 'a> {
    spi: &'a S,
    radio_on: Cell<bool>,
    transmitting: Cell<bool>,
    spi_busy: Cell<bool>,
    reset_pin: &'a Pin,
    sleep_pin: &'a Pin,
    state: InternalState,
}

static mut read_buf: [u8; 2] =  [0x0; 2];
static mut write_buf: [u8; 2] = [0x0; 2];

impl <'a, S: spi::SpiMasterDevice + 'a> spi::SpiMasterClient for RF233 <'a, S> {
    fn read_write_done(&self,
                       write: &'static mut [u8],
                       read: Option<&'static mut [u8]>,
                       len: usize) {
        match self.state {
            InternalState::START => {}
            InternalState::START_PART_READ => {}
            InternalState::START_IRQ_READ => {}
            InternalState::START_TURNING_OFF => {}
            InternalState::START_STATUS_OFF => {}
            InternalState::START_CTRL1_SET => {}
            InternalState::START_CCA_SET => {}
            InternalState::START_PWR_SET => {}
            InternalState::START_CTRL2_SET => {}
            InternalState::START_IRQMASK_SET => {}
            InternalState::START_XAH_SET => {}
            InternalState::START_FRAMERETRY_SET => {}
            InternalState::START_CSMARETRY_SET => {}
            InternalState::START_PANID1_SET => {}
            InternalState::START_PANID2_SET => {}
            InternalState::START_IEEE1_SET => {}
            InternalState::START_IEEE2_SET => {}
            InternalState::START_IEEE3_SET => {}
            InternalState::START_IEEE4_SET => {}
            InternalState::START_IEEE5_SET => {}
            InternalState::START_IEEE6_SET => {}
            InternalState::START_IEEE7_SET => {}
            InternalState::START_IEEE8_SET => {}
            InternalState::START_SHORT1_SET => {}
            InternalState::START_SHORT2_SET => {}
            InternalState::START_RPC_SET => {}
            InternalState::ON_STATUS_READ => {}
            InternalState::ON_PLL_SET => {}
            InternalState::READY => {}
            InternalState::UNKNOWN => {}
        }
    }
}

impl<'a, S: spi::SpiMasterDevice + 'a> RF233 <'a, S> {
    pub fn new(spi: &'a S,
               reset: &'a Pin,
               sleep: &'a Pin) -> RF233<'a, S> {
        RF233 {
            spi: spi,
            reset_pin: reset,
            sleep_pin: sleep,
            radio_on: Cell::new(false),
            transmitting: Cell::new(false),
            spi_busy: Cell::new(false),
            state: InternalState::START,
        }
    }

    pub fn initialize(&self) -> ReturnCode {
        //self.spi.spi.set_client(&self.spi);
        self.spi.configure(spi::ClockPolarity::IdleLow,
                           spi::ClockPhase::SampleLeading,
                           100000);
        self.reset()
    }

    pub fn reset(&self) -> ReturnCode {
        self.reset_pin.make_output();
        self.sleep_pin.make_output();
        self.reset_pin.clear();
        // delay 1 ms
        self.reset_pin.set();
        self.sleep_pin.clear();
        self.transmitting.set(false);
        self.radio_on.set(true);
        ReturnCode::SUCCESS
    }

    pub fn start(&self) -> ReturnCode {
        self.register_read(RF233Register::PART_NUM);
        ReturnCode::SUCCESS
    }

    fn register_write(&self,
                      reg: RF233Register,
                      val: u8) -> ReturnCode {

        if self.spi_busy.get() {return ReturnCode::EBUSY;}
        unsafe {
            write_buf[0] = (reg as u8) | RF233BusCommand::REGISTER_WRITE as u8;
            write_buf[1] = val;
            self.spi.read_write_bytes(&mut write_buf, None, 2);
            self.spi_busy.set(true);
        }
        ReturnCode::SUCCESS
    }

    fn register_read(&self,
                     reg: RF233Register) -> ReturnCode {

        if self.spi_busy.get() {return ReturnCode::EBUSY;}
        unsafe {
            write_buf[0] = (reg as u8) | RF233BusCommand::REGISTER_READ as u8;
            write_buf[1] = 0;
            self.spi.read_write_bytes(&mut write_buf, Some(&mut read_buf), 2);
            self.spi_busy.set(true);
        }
        ReturnCode::SUCCESS
    }


}