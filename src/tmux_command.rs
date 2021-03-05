use crate::tmux_output::TmuxOutput;
use std::borrow::Cow;
use std::process::{Command, Stdio};

// 2. String for hooks and mutability
// 1. bin and cmd must be in same struct?
//      [x] one struct, understanding~+
//      [ ] two structs, complexity~+, usability~-
//      call wrapping impossible cmd+args(tmux+args) != tmux args cmd args
// - Check tmux order options flags matters?
//
// - exec vs run
//      [x] exec - execute
//      [ ] run
//
//  - String or str in struct
//      [ ] &str - chep runtime
//      [ ] String - modification
//      [ ] Cow - both
//
// - no need to sset Option<bool>
#[derive(Default, Debug, Clone)]
pub struct TmuxCommand<'a> {
    // XXX: rename tmux?
    pub bin: Option<Cow<'a, str>>,
    pub bin_args: Option<Vec<Cow<'a, str>>>,
    pub cmd: Option<Cow<'a, str>>,
    pub cmd_args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> TmuxCommand<'a> {
    const TMUX: &'static str = "tmux";

    //pub fn create(
    //bin: Option<Cow<'a, str>>,
    //bin_args: Option<Vec<Cow<'a, str>>>,
    //cmd: Option<Cow<'a, str>>,
    //cmd_args: Option<Vec<Cow<'a, str>>>,
    //) -> Self {
    //TmuxCommand {
    //bin: bin.clone(),
    //bin_args: bin_args.clone(),
    //cmd: cmd.clone(),
    //cmd_args: cmd_args.clone(),
    //}
    //}

    pub fn bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
        //self.tmux.bin = bin;
        self.bin = Some(bin.into());
        self
    }

    pub fn cmd<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.cmd = Some(cmd.into());
        self
    }

    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //cmd.stdin(Stdio::inherit());
    /// run command
    pub fn exec(&mut self) -> TmuxOutput {
        let tmux_bin = &**self
            .bin
            .as_ref()
            .unwrap_or(&Cow::Borrowed(TmuxCommand::TMUX));
        let mut command = Command::new(tmux_bin);

        // XXX: ugly?
        if let Some(s) = &self.bin_args {
            for a in s {
                command.arg(&**a);
            }
        }

        if let Some(s) = &self.cmd {
            command.arg(&**s);
        }

        // XXX: ugly?
        if let Some(s) = &self.cmd_args {
            for a in s {
                command.arg(&**a);
            }
        }

        println!("{:?}", &self);
        command.stdin(Stdio::inherit());
        let output = command.output().unwrap();
        println!("{:?}", output);
        TmuxOutput(output)
    }

    /// insert a single flag (`-f, --flag`)
    pub fn insert_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) -> &mut Self {
        self.insert_param(flag.into())
    }

    /// insert an option (`-f, --flag <option>`)
    pub fn insert_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.cmd_args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    /// insert a single parameter (`[param]`)
    pub fn insert_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.cmd_args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    pub fn new() -> Self {
        TmuxCommand {
            bin: None,
            bin_args: None,
            cmd: None,
            cmd_args: None,
        }
    }
}
