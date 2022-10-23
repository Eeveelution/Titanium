pub trait TtyOutput {
    fn print(&mut self, data: &[u8]);
}

//const REQUIRED_TTY_OUTPUTS: usize = 1;

pub fn print(tty_output: &mut impl TtyOutput, data: &[u8]) {
    tty_output.print(data)
}

pub fn print_string(tty_output: &mut impl TtyOutput, string: &str) {
    print(tty_output, string.as_bytes())
}