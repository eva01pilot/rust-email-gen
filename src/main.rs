use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Template<'a> {
    paragraph: &'a str,
    heading: &'a str,
    btn_link: &'a str,
    directory: &'a str,
    btn_href: &'a str,
}
fn main() {
    let json_path = std::path::Path::new("templates.json");
    let json_string = std::fs::read_to_string(json_path).unwrap();
    let variants: Vec<Template<'_>> = serde_json::from_str(&json_string).unwrap();
    let path = std::path::Path::new("template.html");
    let template = std::fs::read_to_string(path).unwrap();
    for tem in variants {
        let mut new_template = String::new();
        template.lines().for_each(|line| {
            let mut changed_string = line.to_owned();
            if line.contains("{{paragraph}}") {
                changed_string = line.replace("{{paragraph}}", tem.paragraph);
            }
            if line.contains("{{heading}}") {
                changed_string = line.replace("{{heading}}", tem.heading);
            }
            if line.contains("{{btnLink}}") {
                changed_string = line.replace("{{btnLink}}", tem.btn_link);
            }
            if line.contains("{{btnHref}}") {
                changed_string = line.replace("{{btnHref}}", tem.btn_href)
            }
            changed_string.push_str("\n");
            new_template.push_str(changed_string.as_str());
        });
        let output_path = std::path::Path::new(tem.directory);
        let prefix = output_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        std::fs::write(output_path, new_template).unwrap();
    }
}
