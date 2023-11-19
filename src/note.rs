use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use markdown::mdast;

#[derive(Debug)]
pub struct Note {
    pub path: PathBuf,
    pub title: String,
    links: Vec<Link>,
}

#[derive(Debug, PartialEq)]
struct Link {
    title: Option<String>,
    url: String,
}

impl Note {
    pub fn link_to(&self, path: &Path) -> bool {
        for link in &self.links {
            if Path::new(link.url.as_str()) == path {
                return true;
            }
        }

        return false;
    }

    pub fn build(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut note = Self {
            path: PathBuf::from(path),
            title: String::from(
                path.file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ),
            links: Vec::new(),
        };
        note.parse(std::fs::read_to_string(path)?.as_str())?;

        Ok(note)
    }

    fn parse(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        let node = markdown::to_mdast(content, &markdown::ParseOptions::default())?;
        self.parse_node(&node);
        Ok(())
    }

    fn parse_node(&mut self, node: &mdast::Node) {
        match node {
            mdast::Node::Link(link) => self.parse_link(link),
            mdast::Node::Heading(heading) => self.parse_heading(heading),
            _ => {}
        }

        if let Some(children) = node.children() {
            children.iter().for_each(|node| self.parse_node(&node));
        }
    }

    fn parse_link(&mut self, link: &mdast::Link) {
        let title = link.children.first().and_then(|node| match &node {
            mdast::Node::Text(text) => Some(text.value.clone()),
            _ => None,
        });

        let url = if !Self::is_filesystem_url(link.url.as_str()) {
            link.url.clone()
        } else {
            let dirpath = self.path.parent().unwrap();
            String::from(dirpath.join(link.url.as_str()).to_str().unwrap())
        };

        self.links.push(Link {
            title: title,
            url: url,
        });
    }

    // TODO: Parse front matter and other as title.
    fn parse_heading(&mut self, heading: &mdast::Heading) {
        if heading.depth == 1 {
            let title = heading.children.first().and_then(|node| match &node {
                mdast::Node::Text(text) => Some(text.value.clone()),
                _ => None,
            });
            if title.is_some() {
                self.title = title.unwrap();
            }
        }
    }

    fn is_filesystem_url(url: &str) -> bool {
        // TODO: maybe use regex to filter any $.*://
        !url.starts_with("https://") && !url.starts_with("http://")
    }
}

#[cfg(test)]
mod note_tests {
    use std::io::Write;

    use super::*;

    fn temp_mdfile() -> Result<tempfile::NamedTempFile, Box<dyn Error>> {
        let mut file = tempfile::NamedTempFile::new()?;
        file.write_all(
            "# Title
This is a [link title](link_url).
"
            .as_bytes(),
        )?;
        Ok(file)
    }

    #[test]
    fn test_note_parse() -> Result<(), Box<dyn Error>> {
        let mdfile = temp_mdfile()?;
        let note = Note::build(mdfile.path())?;

        assert_eq!(
            note.links,
            vec![Link {
                title: Some(String::from("link title")),
                url: String::from("/tmp/link_url"),
            }]
        );

        assert_eq!(note.title, "Title");

        Ok(())
    }
}
