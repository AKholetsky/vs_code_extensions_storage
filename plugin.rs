#[derive(Debug)]
pub struct Plugin {
    author: String,
    name: String,
    version: String
}

impl Plugin {
    pub fn new(cmd_line: &str) -> Plugin {
        let splited: Vec<&str> = cmd_line.split("@").collect();
        let name_part: Vec<&str> = splited[0].split(".").collect();
        Plugin {
            author: name_part[0].to_string(),
            name: name_part[1].to_string(),
            version: splited[1].to_string()
        }
    }

    pub fn author(&self) -> &String {
        &self.author
    }
}