#[cfg(test)]
mod tests_of_print_help {
    use cliargs::OptCfgParam::*;

    #[test]
    fn help_of_curl() {
        // The source of the following text is the output of `curl --help` in
        // curl 7.87.0. (https://curl.se/docs/copyright.html)

        let mut help = cliargs::Help::new();
        help.add_text("Usage: curl [options...] <url>".to_string());

        help.add_opts_with_margins(
            &[
                cliargs::OptCfg::with([
                    store_key("data"),
                    names(&["d", "data"]),
                    has_arg(true),
                    desc("HTTP POST data"),
                    arg_in_help("<data>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("fail"),
                    names(&["f", "fail"]),
                    desc("Fail fast with no output on HTTP errors"),
                    arg_in_help("<data>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("help"),
                    names(&["h", "help"]),
                    has_arg(true),
                    desc("Get help for commands"),
                    arg_in_help("<category>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("include"),
                    names(&["i", "include"]),
                    desc("Include protocol response headers in the output"),
                ]),
                cliargs::OptCfg::with([
                    store_key("output"),
                    names(&["o", "output"]),
                    has_arg(true),
                    desc("Write to file instead of stdout"),
                    arg_in_help("<file>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("remove_name"),
                    names(&["O", "remove-name"]),
                    desc("Write output to a file named as the remote file"),
                ]),
                cliargs::OptCfg::with([
                    store_key("silent"),
                    names(&["s", "silent"]),
                    desc("Silent mode"),
                ]),
                cliargs::OptCfg::with([
                    store_key("upload_file"),
                    names(&["T", "upload-file"]),
                    has_arg(true),
                    desc("Transfer local FILE to destination"),
                    arg_in_help("<file>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("user"),
                    names(&["u", "user"]),
                    has_arg(true),
                    desc("Server user and password"),
                    arg_in_help("<user:password>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("user_agent"),
                    names(&["A", "user-agent"]),
                    has_arg(true),
                    desc("Send User-Agent <name> to server"),
                    arg_in_help("<name>"),
                ]),
                cliargs::OptCfg::with([
                    store_key("verbose"),
                    names(&["v", "verbose"]),
                    desc("Make the operation more talkative"),
                ]),
                cliargs::OptCfg::with([
                    store_key("version"),
                    names(&["V", "version"]),
                    desc("Show version number and quit"),
                ]),
            ],
            1,
            0,
        );

        help.add_text(
            "
            This is not the full help, this menu is stripped into categories.
            Use \"--help category\" to get an overview of all categories.
            For all options use the manual or \"--help all\"."
                .to_string(),
        );

        help.print();
    }
}
