#[derive(Debug)]
struct Entry {
    size: u64,
    children: Vec<Entry>,
}
impl Entry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &Entry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let mut stack = vec![Entry {
        size: 0,
        children: vec![],
    }];

    for line in include_str!("input.txt").lines() {
        if line == "$ ls" || line.starts_with("dir ") {
            // ignoring
        } else if line.starts_with("$ cd") {
            let dir = line.strip_prefix("$ cd").unwrap().trim();

            if dir == "/" {
                // ignoring
            } else if dir == ".." {
                let child = stack.pop();
                stack.last_mut().unwrap().children.push(child.unwrap());
            } else {
                let node = Entry {
                    size: 0,
                    children: vec![],
                };
                stack.push(node);
            }
        } else {
            let split_strs: Vec<&str> = line.split_whitespace().collect();
            let entry_size: u64 = split_strs[0].parse().unwrap();

            let node = Entry {
                size: entry_size,
                children: vec![],
            };
            stack.last_mut().unwrap().children.push(node);
        }
    }

    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }

    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    println!("part 1: {}", sum);

    let total_space = 70000000_u64;
    let used_space = root.total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min()
        .unwrap();
    println!("part 2: {}", size_to_remove);

    Ok(())
}
