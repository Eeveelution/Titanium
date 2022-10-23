#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::serial_println;

    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}