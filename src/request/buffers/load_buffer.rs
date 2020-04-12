use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LOAD_BUFFER: &'static str = "load-buffer";

    /// Load the contents of the specified paste buffer from path.
    ///
    /// # Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux load-buffer [-b buffer-name] path
    /// (alias: loadb)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux load-buffer [-b buffer-index] path
    /// (alias: loadb)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux load-buffer [-b buffer-index] [-t target-session] path
    /// (alias: loadb)
    /// ```
    pub fn load_buffer(&mut self, buffer_name: Option<&str>, path: &str) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::LOAD_BUFFER, &args)?;
        Ok(output)
    }
}
