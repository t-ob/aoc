use std::str::FromStr;

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

impl FromStr for InodeTable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inode_table = Self::new();
        
        let mut dir_inode_numbers = vec![];

        for line in s.lines() {
            let s_bytes = line.as_bytes();

            match (s_bytes[0], s_bytes[2]) {
                (b'$', b'c') => match &s_bytes[5..] {
                    [b'/'] => {
                        let dir_inode_number = inode_table.add_dir_inode(DirInode {
                            parent_inode_number: None,
                            name: String::from("/"),
                            subdir_inodes: vec![],
                            file_inodes: vec![],
                            size: 0,
                        });

                        dir_inode_numbers.push(dir_inode_number);
                    },
                    [b'.', b'.'] => {
                        dir_inode_numbers.pop();
                    },
                    bs => {
                        let parent_inode_number = dir_inode_numbers.last().unwrap();
                        let dir_inode_number = inode_table.add_dir_inode(DirInode {
                            parent_inode_number: Some(*parent_inode_number),
                            name: String::from_utf8(Vec::from(bs)).unwrap(),
                            subdir_inodes: vec![],
                            file_inodes: vec![],
                            size: 0,
                        });

                        dir_inode_numbers.push(dir_inode_number);
                    },
                },
                (b'$', _)
                | (b'd', _) => {},
                _ => {
                    let mut split_line = line.split(' ');
    
                    let size = split_line.next().unwrap().parse().unwrap();
                    let name = String::from(split_line.next().unwrap());
    
                    inode_table.add_file_inode(FileInode {
                        dir_inode_number: *dir_inode_numbers.last().unwrap(),
                        name: name,
                        size: size,
                    });
                }
            }
        }

        Ok(inode_table)
    }
}