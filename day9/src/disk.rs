use std::fs;
use std::collections::VecDeque;

struct File {
    id: usize,
    block: Block,
}

struct Block {
    start_index: usize,
    num_blocks: usize,
}

pub struct DiskManager {
    disk_string: String,
    disk: Vec<usize>,
    free_indexes: VecDeque<usize>,
    free_blocks: VecDeque<Block>,
    files_on_disk: VecDeque<File>,
}

enum DISK_STRING_POSITION {
    FILE_SIZE,
    FREE_SPACE,
}

fn char_to_usize(c: char) -> usize {
    // assuming this is ASCII and it is a numeric character subtract 48 from the decimal value of the character
    (c as usize) - 48usize
}

impl DiskManager {

    pub fn new() -> DiskManager {
        DiskManager { disk_string: "".to_string(), 
                    disk: vec![], 
                    free_indexes: VecDeque::new(),
                    free_blocks: VecDeque::new(),
                    files_on_disk: VecDeque::new(),
                    }
    }

    pub fn parse_disk_string(&mut self) {
        let disk_string_options: Vec<DISK_STRING_POSITION> = vec![DISK_STRING_POSITION::FILE_SIZE, DISK_STRING_POSITION::FREE_SPACE];

        let disk_characters: Vec<char> = self.disk_string.chars().collect();
        let mut file_id: usize = 0;

        for i in 0..disk_characters.len() {
            match disk_string_options[i%disk_string_options.len()] {
                DISK_STRING_POSITION::FILE_SIZE => {
                    let file_size = char_to_usize(disk_characters[i]);
                    println!("file_size         {} {} {}", self.disk.len(), file_id, file_size);

                    self.files_on_disk.push_back(File{id: file_id, 
                                                    block: Block{start_index: self.disk.len(), num_blocks: file_size}});

                    for _ in 0..file_size {
                        self.disk.push(file_id);
                    }

                    file_id += 1;
                },
                DISK_STRING_POSITION::FREE_SPACE => {
                    let free_space_blocks = char_to_usize(disk_characters[i]);

                    if free_space_blocks == 0 {
                        continue;
                    }
                    //println!("free_space_blocks {} {}", disk_characters[i], free_space_blocks);

                    // empty space here
                    println!("free_space_blocks {} {}", self.disk.len(), free_space_blocks);


                    self.free_blocks.push_back(Block{start_index: self.disk.len(), num_blocks: free_space_blocks});


                    for _ in 0..free_space_blocks {
                        self.free_indexes.push_back(self.disk.len());
                        self.disk.push(usize::MAX);
                    }

                }
            }
        }

    }

    fn coalesce_free_block(&mut self, block: Block) {
        // before adding a block try to add it to an existing free block

        for existing_block in &mut self.free_blocks {
            if block.start_index + block.num_blocks == existing_block.start_index {
                // this block precedes the existing block, adjust the existing block
                existing_block.start_index = block.start_index;
                existing_block.num_blocks += block.num_blocks;
                return;
            } else if existing_block.start_index + existing_block.num_blocks == block.start_index {
                // this block is after the existing block, adjust the existing block
                existing_block.num_blocks += block.num_blocks;
                return;
            }
        }

        // no free bock was found adjacent to this block
        self.free_blocks.push_back(block);

    }

    pub fn compact_files(&mut self) {
        while self.files_on_disk.len() > 0 {
            let file = self.files_on_disk.pop_back().unwrap();

            for i in 0..self.free_blocks.len() {
                let block = self.free_blocks.get_mut(i).unwrap();
                if file.block.start_index < block.start_index {
                    // do not try to move the block backwards
                    continue;
                }

                if file.block.num_blocks <= block.num_blocks {
                    // found free blocks this file can fit in

                    // move file here
                    for j in block.start_index..block.start_index+file.block.num_blocks {
                        self.disk[j] = file.id;
                    }

                    // free up file indexes
                    for j in file.block.start_index..file.block.start_index+file.block.num_blocks {
                        self.disk[j] = usize::MAX;
                    }

                    // update block that was free
                    block.start_index += file.block.num_blocks;
                    block.num_blocks -= file.block.num_blocks;
                    if block.num_blocks == 0 {
                        // no more free room here
                        self.free_blocks.remove(i);
                    }

                    self.coalesce_free_block(Block{start_index: file.block.start_index, num_blocks: file.block.num_blocks});

                    break;
                }
            }
        }
    }

    pub fn compact_blocks(&mut self) {
        // start at the end of the disk and start shifting blocks over

        let mut new_free_space = VecDeque::new();

        for i in (0..self.disk.len()).rev() {
            let block_id = self.disk[i];
            if block_id == usize::MAX {
                // no need to move free blocks
                continue;
            }

            let free_block_index: usize;

            if self.free_indexes.len() == 0 {
                free_block_index = new_free_space.pop_front().unwrap();
            } else {
                free_block_index = self.free_indexes.pop_front().unwrap();
            }
            
            if free_block_index > i {
                // no more free spaces below our current index
                return;
            }
            // move the block to the free space
            self.disk[free_block_index] = block_id;

            // the old block index is now free
            self.disk[i] = usize::MAX;
            // keep pushing new free spaces to the front as they come available since they will be the new lowest index
            new_free_space.push_front(i); 

            println!("move {} from {} to {}", block_id, i, free_block_index);
        }
    }

    pub fn get_from_file(&mut self, file_name: &str) {
        self.disk_string = fs::read_to_string(file_name).expect("Error reading file");
    }

    fn checksum(&self) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..self.disk.len() {
            if self.disk[i] == usize::MAX {
                // do not sum free blocks
                continue;
            }
            sum += (self.disk[i] as u64) * (i as u64);
        }

        return sum;
    }

    pub fn print(&self) {
        println!("{}", self.disk_string);

        println!("{:?}", self.disk);

        println!("checksum {}", self.checksum());

    }
}



