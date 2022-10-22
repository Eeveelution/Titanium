use crate::titanium;

pub trait TtyOutput {
    fn print(&mut self, data: &[u8]);
}

const REQUIRED_TTY_OUTPUTS: usize = 1;


pub fn print(data: &[u8]) {
    let mut tty_outputs: [ Option<&mut dyn TtyOutput> ; REQUIRED_TTY_OUTPUTS] = [None; REQUIRED_TTY_OUTPUTS];

    let mut vga_tty = titanium::drivers::vga_tty::create_tty_output();

    tty_outputs[0] = Some(&mut vga_tty);

    for(i, output) in tty_outputs.iter_mut().enumerate() {
        match output {
            Some(tty_output) => tty_output.print(data), 
            None => (),
        };
    }
}