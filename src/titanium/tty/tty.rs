use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

pub trait TtyOutput : Send {
    fn print(&mut self, data: &[u8]);
}

#[allow(dead_code)]
const REQUIRED_TTY_OUTPUTS: usize = 1;

lazy_static! {
    pub static ref TTY_OUTPUTS: 
    [  
        Mutex< 
            Option< 
                &'static mut dyn TtyOutput 
            > 
        >; REQUIRED_TTY_OUTPUTS
    ] = [ Mutex::new(None); REQUIRED_TTY_OUTPUTS];
}

#[allow(dead_code)]
pub fn print(tty_output: &mut impl TtyOutput, data: &[u8]) {
    interrupts::without_interrupts(|| {
        tty_output.print(data)
    });
}

#[allow(dead_code)]
pub fn print_string(tty_output: &mut impl TtyOutput, string: &str) {
    print(tty_output, string.as_bytes())
}