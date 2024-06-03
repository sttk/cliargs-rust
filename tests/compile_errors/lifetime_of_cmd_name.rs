use cliargs::Cmd;

fn main() {
    fn returns_cmd_name() -> &'static str {
        let mut cmd = Cmd::with_strings(["/path/to/app".to_string()]);
        cmd.parse().unwrap();

        let name = cmd.name();
        println!("command name (within the scope = {name}");

        name
    }

    let name = returns_cmd_name();
    println!("command name (out of the scope) = {name}");
}
