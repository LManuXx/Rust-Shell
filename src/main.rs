mod modules {
    pub mod input;
    pub mod command;
}


fn main() {
    loop {
        let input = modules::input::read_input();

        if input == "exit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let command = tokens[0];
        let args = &tokens[1..];

        modules::command::execute_command(command, args);
    }
}