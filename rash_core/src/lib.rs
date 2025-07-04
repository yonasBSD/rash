#![allow(clippy::derive_partial_eq_without_eq)]

pub mod context;
pub mod docopt;
pub mod error;
pub mod jinja;
pub mod logger;
pub mod modules;
pub mod task;
pub mod utils;
pub mod vars;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    use context::{Context, GlobalParams};
    use task::parse_file;
    use vars::env;

    #[test]
    fn test_command_ls() {
        let file = r#"
            #!/bin/rash
            - name: test ls
              command: ls

            - command:
                cmd: ls /
            "#;

        let global_params = GlobalParams::default();
        let context = Context::new(
            parse_file(file, &global_params).unwrap(),
            env::load(vec![]),
            None,
        );
        let last_context = context.exec().unwrap();

        assert!(last_context.tasks.is_empty());
    }
}
