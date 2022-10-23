use crate::titanium::drivers::serial::serial::SERIAL_1;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    SERIAL_1.lock().write_fmt(args).expect("Printing to SERIAL1 failed!");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::titanium::drivers::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::titanium::drivers::serial::_print("\n"));
    ($fmt:expr) => ($crate::titanium::drivers::serial::_print(   format_args!( concat!($fmt, "\n")) )    );
    ($fmt:expr, $($arg:tt)*) => ($crate::titanium::drivers::serial::_print(  format_args!(  concat!($fmt, "\n"), $($arg)*) ) );
}


//#[macro_export]
//macro_rules! serial_println {
//    () => ($crate::titanium::drivers::serial::serial_print!("\n"));
//    ($fmt:expr) => ($crate::titanium::drivers::serial::serial_print!());
//    ($fmt:expr, $($arg:tt)*) => ($crate::titanium::drivers::serial::serial_print!(
//        ));
//}