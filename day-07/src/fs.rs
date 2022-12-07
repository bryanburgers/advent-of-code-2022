use std::{collections::VecDeque, fmt::Display, str::FromStr};

pub enum Entry {
    Directory(Directory),
    File(File),
}

impl Entry {
    pub fn as_directory_mut(&mut self) -> Option<&mut Directory> {
        match self {
            Self::Directory(directory) => Some(directory),
            _ => None,
        }
    }
    pub fn as_directory(&self) -> Option<&Directory> {
        match self {
            Self::Directory(directory) => Some(directory),
            _ => None,
        }
    }
    pub fn as_file_mut(&mut self) -> Option<&mut File> {
        match self {
            Self::File(file) => Some(file),
            _ => None,
        }
    }
    pub fn as_file(&self) -> Option<&File> {
        match self {
            Self::File(file) => Some(file),
            _ => None,
        }
    }
    pub fn is_directory(&self) -> bool {
        self.as_directory().is_some()
    }
    pub fn is_file(&self) -> bool {
        self.as_file().is_some()
    }
    pub fn name(&self) -> &str {
        match self {
            Self::Directory(directory) => directory.name(),
            Self::File(file) => file.name(),
        }
    }

    pub fn pointer(&self, pointer: Pointer) -> Option<&Entry> {
        if pointer.is_empty() {
            Some(self)
        } else if let Some(directory) = self.as_directory() {
            directory.pointer(pointer)
        } else {
            None
        }
    }

    pub fn pointer_mut(&mut self, pointer: Pointer) -> Option<&mut Entry> {
        if pointer.is_empty() {
            Some(self)
        } else if let Some(directory) = self.as_directory_mut() {
            directory.pointer_mut(pointer)
        } else {
            None
        }
    }

    pub fn visit(&self, visitor: &mut impl Visitor) {
        let pointer = Pointer::new();
        self.visit_internal(pointer, visitor);
    }

    fn visit_internal(&self, mut pointer: Pointer, visitor: &mut impl Visitor) {
        match self {
            Self::Directory(directory) => {
                if directory.name() != "/" {
                    pointer.push_back(directory.name().to_string());
                }
                visitor.visit_directory_before(&pointer, directory);
                for child in directory.children() {
                    child.visit_internal(pointer.clone(), visitor);
                }
                visitor.visit_directory_after(&pointer, directory);
            }
            Self::File(ref file) => {
                pointer.push_back(file.name().to_string());
                visitor.visit_file(&pointer, file);
            }
        }
    }
}

impl From<Directory> for Entry {
    fn from(inner: Directory) -> Self {
        Entry::Directory(inner)
    }
}

impl From<File> for Entry {
    fn from(inner: File) -> Self {
        Entry::File(inner)
    }
}

pub struct Directory {
    name: String,
    children: Vec<Entry>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: Vec::new(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn children(&self) -> &[Entry] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [Entry] {
        &mut self.children
    }

    pub fn find(&self, name: &str) -> Option<&Entry> {
        self.children().iter().find(|entry| entry.name() == name)
    }

    pub fn find_mut(&mut self, name: &str) -> Option<&mut Entry> {
        self.children_mut()
            .iter_mut()
            .find(|entry| entry.name() == name)
    }

    pub fn push_child(&mut self, entry: Entry) {
        self.children.push(entry);
    }

    pub fn pointer(&self, mut pointer: Pointer) -> Option<&Entry> {
        let front = pointer.pop_front()?;
        let child = self.find(&front)?;
        child.pointer(pointer)
    }

    pub fn pointer_mut(&mut self, mut pointer: Pointer) -> Option<&mut Entry> {
        let front = pointer.pop_front()?;
        let child = self.find_mut(&front)?;
        child.pointer_mut(pointer)
    }
}

pub struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug)]
pub struct Pointer(VecDeque<String>);

impl Pointer {
    pub fn pop_back(&mut self) -> Option<String> {
        self.0.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<String> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push_back(&mut self, string: String) {
        self.0.push_back(string)
    }
}

impl FromStr for Pointer {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix('/').ok_or("Must begin with a '/'")?;
        let queue = rest.split('/').map(|v| v.to_string()).collect();
        Ok(Self(queue))
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            f.write_str("/")
        } else {
            for item in &self.0 {
                f.write_str("/")?;
                f.write_str(item)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // - / (dir)
        //   - a (dir)
        //     - e (dir)
        //       - i (file, size=584)
        //     - f (file, size=29116)
        //     - g (file, size=2557)
        //     - h.lst (file, size=62596)
        //   - b.txt (file, size=14848514)
        //   - c.dat (file, size=8504156)
        //   - d (dir)
        //     - j (file, size=4060174)
        //     - d.log (file, size=8033020)
        //     - d.ext (file, size=5626152)
        //     - k (file, size=7214296)

        let root = {
            let mut root = Directory::new(String::from("/"));

            let a = {
                let mut a = Directory::new(String::from("a"));

                let e = {
                    let mut e = Directory::new(String::from("e"));
                    let i = File::new(String::from("i"), 584);
                    e.push_child(i.into());
                    e
                };
                a.push_child(e.into());
                let f = File::new(String::from("f"), 29116);
                a.push_child(f.into());
                let g = File::new(String::from("g"), 2557);
                a.push_child(g.into());
                let h_lst = File::new(String::from("h.lst"), 62596);
                a.push_child(h_lst.into());
                a
            };
            root.push_child(a.into());

            let c_dat = File::new(String::from("c.dat"), 8504156);
            root.push_child(c_dat.into());
            let b_txt = File::new(String::from("b.txt"), 14848514);
            root.push_child(b_txt.into());

            let d = {
                let mut d = Directory::new(String::from("d"));
                let j = File::new(String::from("j"), 4060174);
                d.push_child(j.into());
                let d_log = File::new(String::from("d.log"), 8033020);
                d.push_child(d_log.into());
                let d_ext = File::new(String::from("d.ext"), 5626152);
                d.push_child(d_ext.into());
                let k = File::new(String::from("k"), 7214296);
                d.push_child(k.into());
                d
            };
            root.push_child(d.into());

            root
        };

        let pointer = "/a/e/i".parse().unwrap();
        let entry = root.pointer(pointer).expect("Expected to find file");
        assert_eq!(entry.name(), "i");
        assert_eq!(entry.as_file().unwrap().size(), 584);
    }
}

pub trait Visitor {
    fn visit_directory_before(&mut self, pointer: &Pointer, directory: &Directory) {}
    fn visit_directory_after(&mut self, pointer: &Pointer, directory: &Directory) {}
    fn visit_file(&mut self, pointer: &Pointer, file: &File) {}
}
