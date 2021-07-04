use crate::um::Um;
use crate::um::UmStatus;

pub mod part1;
// pub mod part2a;
pub mod part2;

trait UmContinueExt {
    fn enter_command(&mut self, input: &str) -> String;
}

impl UmContinueExt for Um {
    fn enter_command(&mut self, input: &str) -> String {
        let mut output = Vec::new();
        let status = self.continue_with(&mut input.as_bytes(), &mut output);
        assert_eq!(status, UmStatus::NoInput);
        let output = String::from_utf8(output).unwrap();
        // debug!("input: {}", input);
        // debug!("output: {}", output);
        output
    }
}
