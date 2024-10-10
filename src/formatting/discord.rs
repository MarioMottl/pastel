#[allow(dead_code)]
pub fn bold(input: &str) -> String {
    format!("**{}**", input)
}

#[allow(dead_code)]
pub fn italics(input: &str) -> String {
    format!("*{}*", input)
}

#[allow(dead_code)]
pub fn underline(input: &str) -> String {
    format!("__{}__", input)
}

#[allow(dead_code)]
pub fn strikethrough(input: &str) -> String {
    format!("~~{}~~", input)
}

#[allow(dead_code)]
pub fn inline_code(input: &str) -> String {
    format!("`{}`", input)
}

#[allow(dead_code)]
pub fn code_block(input: &str) -> String {
    format!("```\n{}\n```", input)
}
