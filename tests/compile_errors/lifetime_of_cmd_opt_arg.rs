use cliargs::Cmd;

fn main() {
    fn returns_opt_arg() -> &'static str {
        let mut cmd =
            cliargs::Cmd::with_strings(["/path/to/app".to_string(), "--foo=bar".to_string()]);
        cmd.parse().unwrap();

        let opt_arg = cmd.opt_arg("foo").unwrap();
        println!("option arg (within the scope = {opt_arg:?}");

        opt_arg
    }

    let opt_arg = returns_opt_arg();
    println!("option arg (out of the scope) = {opt_arg:?}");
}
