use std::str::FromStr;

#[derive(Debug)]
pub enum ChangeDirArg {
    In(String),
    Out,
    Root,
}

#[derive(Debug)]
pub enum TerminalOutputLine {
    ChangeDir(ChangeDirArg),
    List,
    File(usize, String),
    Dir(String),
}

impl FromStr for TerminalOutputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_bytes = s.as_bytes();
        match (s_bytes[0], s_bytes[2]) {
            (b'$', b'c') => match &s_bytes[5..] {
                [b'/'] => Ok(Self::ChangeDir(ChangeDirArg::Root)),
                [b'.', b'.'] => Ok(Self::ChangeDir(ChangeDirArg::Out)),
                bs => Ok(Self::ChangeDir(ChangeDirArg::In(
                    String::from_utf8(Vec::from(bs)).unwrap(),
                ))),
            },
            (b'$', _) => Ok(Self::List),
            (b'd', _) => Ok(Self::Dir(
                String::from_utf8(Vec::from(&s_bytes[4..])).unwrap(),
            )),
            _ => {
                let mut split_line = s.split(' ');

                let size = split_line.next().unwrap().parse().unwrap();
                let name = String::from(split_line.next().unwrap());

                Ok(Self::File(size, name))
            }
        }
    }
}

#[derive(Debug)]
struct DirInode {
    parent_inode_number: Option<usize>,
    name: String,
    subdir_inodes: Vec<usize>,
    file_inodes: Vec<usize>,
    size: usize,
}

#[derive(Debug)]
struct FileInode {
    dir_inode_number: usize,
    name: String,
    size: usize,
}

#[derive(Debug)]
pub struct InodeTable {
    file_inodes: Vec<FileInode>,
    dir_inodes: Vec<DirInode>,
}

impl InodeTable {
    fn new() -> Self {
        let file_inodes = vec![];
        let dir_inodes = vec![];
        Self {
            file_inodes,
            dir_inodes,
        }
    }

    pub fn from_terminal_output_lines(terminal_ouput_lines: &[TerminalOutputLine]) -> InodeTable {
        let mut inode_table = InodeTable::new();

        let mut line_number = 0;
        let mut dir_inode_numbers = vec![];
        while line_number < terminal_ouput_lines.len() {
            let line = &terminal_ouput_lines[line_number];
            let next_line = &terminal_ouput_lines[line_number + 1];
            match (line, next_line) {
                (TerminalOutputLine::ChangeDir(ChangeDirArg::Out), _) => {
                    dir_inode_numbers.pop();

                    line_number += 1;
                }
                (TerminalOutputLine::ChangeDir(arg), TerminalOutputLine::List) => {
                    let dir_inode = match arg {
                        ChangeDirArg::Root => inode_table.add_dir_inode(DirInode {
                            parent_inode_number: None,
                            name: String::from("/"),
                            subdir_inodes: vec![],
                            file_inodes: vec![],
                            size: 0,
                        }),
                        ChangeDirArg::In(subdir) => inode_table.add_dir_inode(DirInode {
                            parent_inode_number: Some(*dir_inode_numbers.last().unwrap()),
                            name: String::from(subdir),
                            subdir_inodes: vec![],
                            file_inodes: vec![],
                            size: 0,
                        }),
                        _ => unreachable!(),
                    };

                    dir_inode_numbers.push(dir_inode);

                    let mut j = line_number + 2;

                    while j < terminal_ouput_lines.len() {
                        match &terminal_ouput_lines[j] {
                            TerminalOutputLine::File(file_size, file_name) => {
                                inode_table.add_file_inode(FileInode {
                                    dir_inode_number: dir_inode,
                                    name: String::from(file_name),
                                    size: *file_size,
                                });
                            }
                            TerminalOutputLine::ChangeDir(_) => {
                                break;
                            }
                            _ => {}
                        }
                        j += 1
                    }
                    line_number = j;
                }
                _ => unreachable!(),
            }
        }

        inode_table
    }

    fn add_dir_inode(&mut self, inode: DirInode) -> usize {
        self.dir_inodes.push(inode);

        let parent_inode_number = self.dir_inodes.last().unwrap().parent_inode_number;

        if let Some(parent_inode) = parent_inode_number {
            let last_inode_number = self.dir_inodes.len() - 1;
            self.dir_inodes[parent_inode]
                .subdir_inodes
                .push(last_inode_number)
        }

        self.dir_inodes.len() - 1
    }

    fn add_file_inode(&mut self, inode: FileInode) -> usize {
        let dir_inode_number = inode.dir_inode_number;
        let size = inode.size;
        self.file_inodes.push(inode);

        let file_inode_number = self.file_inodes.len() - 1;

        self.dir_inodes[dir_inode_number]
            .file_inodes
            .push(file_inode_number);

        // Update dir sizes
        let mut next_dir_inode_number = Some(dir_inode_number);
        while let Some(curr_dir_inode_number) = next_dir_inode_number {
            self.dir_inodes[curr_dir_inode_number].size += size;
            next_dir_inode_number = self.dir_inodes[curr_dir_inode_number].parent_inode_number
        }

        file_inode_number
    }

    pub fn dir_size(&self, inode_number: usize) -> usize {
        self.dir_inodes[inode_number].size
    }

    pub fn dir_sizes(&self, from_inode_number: usize) -> Vec<(usize, usize)> {
        let mut path = vec![from_inode_number];
        let mut result = vec![];

        while let Some(next_inode_number) = path.pop() {
            let next_dir_inode = &self.dir_inodes[next_inode_number];
            result.push((next_inode_number, next_dir_inode.size));
            for inode in &next_dir_inode.subdir_inodes {
                path.push(*inode);
            }
        }

        result
    }
}
