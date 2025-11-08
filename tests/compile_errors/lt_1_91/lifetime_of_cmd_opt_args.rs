use cliargs::Cmd;

fn main() {
    fn returns_one_of_opt_args() -> &'static str {
        let mut cmd =
            cliargs::Cmd::with_strings(["/path/to/app".to_string(), "--foo=bar".to_string()]);
        cmd.parse().unwrap();

        let opt_args = cmd.opt_args("foo").unwrap();
        println!("option arg (within the scope = {:?}", opt_args[0]);

        opt_args[0]
    }

    let opt_arg = returns_one_of_opt_args();
    println!("option arg (out of the scope) = {opt_arg:?}");
}
