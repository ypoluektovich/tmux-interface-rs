#[test]
fn set_option() {
    use crate::{Error, SetOption, SetOptionBuilder, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-option [-aFgopqsuw] [-t target-pane] option value
        // (alias: set)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-option", "-a", "-F", "-g", "-o", "-p", "-q", "-s", "-u", "-w", "-t", "1", "2", "3"]"#
        );
        Err(Error::Hook)
    }));

    let set_option = SetOption {
        append: Some(true),
        format: Some(true),
        global: Some(true),
        not_overwrite: Some(true),
        pane: Some(true),
        quiet: Some(true),
        server: Some(true),
        unset: Some(true),
        window: Some(true),
        target: Some(&TargetPane::Raw("1")),
    };
    tmux.set_option(Some(&set_option), "2", "3").unwrap_err();

    let set_option = SetOptionBuilder::new()
        .append()
        .format()
        .global()
        .not_overwrite()
        .pane()
        .quiet()
        .server()
        .unset()
        .window()
        .target(&TargetPane::Raw("1"))
        .build();
    tmux.set_option(Some(&set_option), "2", "3").unwrap_err();
}
