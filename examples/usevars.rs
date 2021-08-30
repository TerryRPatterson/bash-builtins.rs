//! Bash builtin to access shell variables.

use bash_builtins::variables::{self, Variable};
use bash_builtins::{builtin_metadata, Args, Builtin, Result};
use std::io::{self, BufWriter, Write};

builtin_metadata!(name = "usevars", create = UseVars::default);

#[derive(Default)]
struct UseVars;

impl Builtin for UseVars {
    fn call(&mut self, args: &mut Args) -> Result<()> {
        let stdout_handle = io::stdout();
        let mut output = BufWriter::new(stdout_handle.lock());

        for name in args.string_arguments() {
            let mut name_parts = name?.splitn(2, '=');
            match (name_parts.next(), name_parts.next()) {
                (Some(name), None) => match variables::find(name) {
                    Some(var) => write_var(&mut output, name, var)?,
                    None => (),
                },

                (Some(name), Some("")) => {
                    if variables::unset(&name) {
                        writeln!(&mut output, "unset: {}", name)?;
                    }
                }

                (Some(name), Some(value)) => {
                    variables::set(&name, &value)?;
                }

                _ => (),
            }
        }

        Ok(())
    }
}

fn write_var(mut output: impl Write, name: &str, var: Variable) -> io::Result<()> {
    match var {
        Variable::Str(s) => writeln!(&mut output, "{} = {:?}", name, s)?,

        Variable::Array(a) => {
            for (idx, elem) in a.iter().enumerate() {
                writeln!(&mut output, "{}[{}] = {:?}", name, idx, elem)?;
            }
        }

        Variable::Assoc(a) => {
            for (key, value) in a.iter() {
                writeln!(&mut output, "{}[{:?}] = {:?}", name, key, value)?;
            }
        }
    }

    Ok(())
}