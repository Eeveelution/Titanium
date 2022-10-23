use x86_64::instructions::interrupts;

pub trait TtyOutput : Send {
    fn print(&mut self, data: &[u8]);
}

#[allow(dead_code)]
const REQUIRED_TTY_OUTPUTS: usize = 1;

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