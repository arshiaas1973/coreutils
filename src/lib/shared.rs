use std::{fs, io::Error};

pub struct UserPrompt {
    pub files: Vec<String>,
    pub number_all: bool,          // Start each row with numbers | -n
    pub avoid_empty_number: bool,  // Avoid all empty line from numbering | -b
    pub ds_eol: bool,              // Puts "$" at the end of each line | -E
    pub combine_empty_lines: bool, // Combines empty lines into one empty line | -s
    pub tabs: bool,                // Show tabs as "^I" | -T
    pub npc: bool,                 // Shows non-printable characters (NPC) | -v
}

impl UserPrompt {
    pub fn new() -> UserPrompt {
        UserPrompt {
            files: vec![],
            number_all: false,
            avoid_empty_number: false,
            ds_eol: false,
            combine_empty_lines: false,
            tabs: false,
            npc: false,
        }
    }
    pub fn get_files_content(&self) -> Result<Vec<String>, Error> {
        let mut contents: Vec<String> = vec![];
        for file in &self.files {
            let content = fs::read_to_string(file);
            if content.is_err() {
                return Err(content.err().unwrap());
            }
            contents.push(content.unwrap());
        }
        return Ok(contents);
    }
}
