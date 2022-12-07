use crate::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Input<'a> {
    CdRoot,
    CdUp,
    CdDown { name: &'a str },
    Ls,
    FileLine { size: usize, name: &'a str },
    DirectoryLine { name: &'a str },
}

impl<'a> Input<'a> {
    pub fn from_str(line: &'a str) -> Result<Self, &'static str> {
        if line == "$ cd /" {
            Ok(Input::CdRoot)
        } else if line == "$ cd .." {
            Ok(Input::CdUp)
        } else if let Some(name) = line.strip_prefix("$ cd ") {
            Ok(Input::CdDown { name })
        } else if line == "$ ls" {
            Ok(Input::Ls)
        } else if let Some(name) = line.strip_prefix("dir ") {
            Ok(Input::DirectoryLine { name })
        } else if let Some((size, name)) = line.split_once(' ') {
            let size = size.parse::<usize>().map_err(|_| "invalid size for file")?;
            Ok(Input::FileLine { size, name })
        } else {
            Err("Invalid format")
        }
    }
}

pub fn build_fs(input: Vec<Input<'_>>) -> Result<fs::Entry, &'static str> {
    let mut iter = input.into_iter();
    let first = iter.next().ok_or("Expected the first line of input")?;
    if first != Input::CdRoot {
        return Err("Expected the first input to be `cd /`");
    }

    let root = fs::Directory::new(String::from("/"));
    let mut stack: Vec<fs::Directory> = vec![root];

    for item in iter {
        match item {
            Input::CdRoot => return Err("Unexpected second `cd /`"),
            Input::CdUp => {
                let directory = stack.pop().expect("expected that we were in a directory");
                let parent = stack.last_mut().ok_or("Can't cd .. when in `/`")?;
                parent.push_child(directory.into());
            }
            Input::CdDown { name } => {
                let directory = fs::Directory::new(name.to_string());
                stack.push(directory);
            }
            Input::Ls => {
                // Don't actually need to do anything here.
            }
            Input::FileLine { size, name } => {
                let file = fs::File::new(name.to_string(), size);
                let cwd = stack.last_mut().ok_or("Couldn't get cwd")?;
                cwd.push_child(file.into());
            }
            Input::DirectoryLine { .. } => {
                // Ignore this. We *assume* that every directory we see in the `ls` output we
                // will also `cd <name>` into

                // let directory = fs::Directory::new(name.to_string());
                // let cwd = stack.last_mut().ok_or("Couldn't get cwd")?;
                // cwd.push_child(directory.into());
            }
        }
    }

    while let Some(directory) = stack.pop() {
        if let Some(parent) = stack.last_mut() {
            parent.push_child(directory.into());
        } else {
            // It's the root!
            return Ok(directory.into());
        }
    }

    unreachable!()
}

pub fn parse_input(input: &str) -> Result<fs::Entry, &'static str> {
    let input = input
        .trim()
        .lines()
        .map(|line| Input::from_str(line))
        .collect::<Result<Vec<Input<'_>>, &'static str>>()?;
    build_fs(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = parse_input(include_str!("example.txt")).expect("successful parse");
        let pointer = "/a/e/i".parse().unwrap();
        let entry = input.pointer(pointer).expect("Expected to find file");
        assert_eq!(entry.name(), "i");
        assert_eq!(entry.as_file().unwrap().size(), 584);
    }
}
