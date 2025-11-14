use crate::lib::shared::UserPrompt;

pub fn handle_arguments(argv: Vec<String>, argc: usize) -> UserPrompt{
    if argc <= 2 {
        return UserPrompt::new();
    }
    let mut up: UserPrompt = UserPrompt::new();
    for i in 1..argc {
        let arg: &String = &argv[i];
        match arg.as_str() {
            "-n" => up.number_all = true,
            "-b" => up.avoid_empty_number = true,
            "-E" => up.ds_eol = true,
            "-s" => up.combine_empty_lines = true,
            "-T" => up.tabs = true,
            "-v" => up.npc = true,
            _ => {}
        }
    }
    return up;
}

