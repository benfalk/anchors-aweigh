use super::Linkage;

#[derive(Debug)]
pub enum Node {
    Text(String),
    Link(Linkage),
}

impl Node {
    pub fn estimated_string_size(&self) -> usize {
        self.string_contents().len()
    }

    pub fn string_contents(&self) -> &str {
        match self {
            Self::Text(data) => data,
            Self::Link(link) => link.contents.as_deref().unwrap_or(""),
        }
    }

    pub fn compile(&self, buf: &mut String) {
        match self {
            Self::Text(data) => buf.push_str(data),
            Self::Link(link) => link.compile(buf),
        }
    }
}

#[derive(Debug)]
pub struct Assembly {
    pub nodes: Vec<Node>,
}

impl Assembly {
    pub fn compile(&self) -> String {
        self.nodes.iter().fold(
            String::with_capacity(self.estimated_string_size()),
            |mut string, node| {
                node.compile(&mut string);
                string
            },
        )
    }

    pub fn estimated_string_size(&self) -> usize {
        self.nodes
            .iter()
            .fold(0, |total, node| total + node.estimated_string_size())
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<Linkage> for Node {
    fn from(value: Linkage) -> Self {
        Self::Link(value)
    }
}
