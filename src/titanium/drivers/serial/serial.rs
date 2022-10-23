use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

const SERIAL1_IOBASE: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL_1 : Mutex<SerialPort> = {
        let mut serial_port = unsafe {
            SerialPort::new(SERIAL1_IOBASE)
        };

        serial_port.init();

        Mutex::new(serial_port)
    };
}