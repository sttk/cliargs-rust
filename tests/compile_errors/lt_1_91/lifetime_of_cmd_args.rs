use cliargs::Cmd;

fn main() {
    fn returns_one_of_cmd_args() -> &'static str {
        let mut cmd = cliargs::Cmd::with_strings(["/path/to/app".to_string(), "foo".to_string()]);
        cmd.parse().unwrap();

        let args = cmd.args();
        let arg1 = args[0];
        println!("command args (within the scope = {arg1:?}");

        arg1
    }

    let arg1 = returns_one_of_cmd_args();
    println!("command args (out of the scope) = {arg1:?}");
}

