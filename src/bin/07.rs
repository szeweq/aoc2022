/*
 * Day 7: No Space Left On Device
 * See [https://adventofcode.com/2022/day/7]
 */

#[derive(PartialEq)]
struct FTree {
    parent: Option<usize>,
    name: String,
    dir: bool,
    children: Vec<usize>,
    size: u32
}

struct FS {
    nodes: Vec<FTree>
}

impl FS {
    fn new() -> Self {
        Self { nodes: vec![
            FTree { parent: None, name: "".to_string(), dir: true, children: Vec::new(), size: 0 }
        ] }
    }

    fn find_dir(&self, top: usize, name: &str) -> Option<&usize> {
        let children = &self.nodes[top].children;
        children.iter().find(|&&id| name == self.nodes[id].name)
    }

    fn add(&mut self, top: usize, name: &str, dir: bool, size: u32) {
        let ft = FTree {
            parent: Some(top),
            name: name.to_string(),
            dir,
            children: vec![],
            size
        };
        let l = self.nodes.len();
        self.nodes.push(ft);
        let mut tft = &mut self.nodes[top];
        tft.children.push(l);
        tft.size += size;
        while let Some(pp) = tft.parent {
            tft = &mut self.nodes[pp];
            tft.size += size;
        }
    }
}

fn parse_tree(input: &str) -> FS {
    let mut fs = FS::new();
    let mut cd = 0;
    for l in input.lines() {
        let bl = l.as_bytes();
        match bl[0] {
            b'$' => {
                let mut sp = l.split_ascii_whitespace().skip(1);
                match sp.next().expect("No command given") {
                    "cd" => {
                        cd = match sp.next().expect("Unknown dir type") {
                            ".." => { fs.nodes[cd].parent.unwrap() }
                            "/" => { 0 }
                            x => {
                                if let Some(nd) = fs.find_dir(cd, x) {
                                    *nd
                                } else {
                                    panic!("Cannot find dir {}", x);
                                }
                            }
                        };
                    }
                    "ls" => {}
                    x => { println!("Unknown command: {}", x); }
                }
            },
            x if (b'0'..=b'9').contains(&x) => {
                let mut sp = l.split_ascii_whitespace();
                let n = sp.next().and_then(|d| d.parse::<u32>().ok()).expect("Unknown size");
                let d = sp.next().unwrap();
                fs.add(cd, d.clone(), false, n);
            },
            b'd' => {
                let mut sp = l.split_ascii_whitespace();
                sp.next().unwrap();
                let d = sp.next().unwrap();
                fs.add(cd, d.clone(), true, 0);
            }
            _ => {
                println!("Invalid line: {}", l);
            }
        }
    }

    fs
}

pub fn part_1(input: &str) -> Option<u32> {
    let fs = parse_tree(input);
    let vs = fs.nodes.iter()
        .filter_map(|ft| {
            if !ft.dir { return None; }
            let sz = ft.size;
            if sz > 0 && sz <= 100000 { Some(sz) } else { None }
        })
        .sum();
    Some(vs)
}

pub fn part_2(input: &str) -> Option<u32> {
    let fs = parse_tree(input);
    let nn = 30000000 - (70000000 - fs.nodes[0].size);
    let mut vs: Vec<u32> = fs.nodes.iter()
        .filter_map(|ft| {
            if !ft.dir { return None; }
            let sz = ft.size;
            if sz > 0 { Some(sz) } else { None }
        })
        .collect();
    vs.sort_by(|a, b| b.cmp(a));
    let pos = vs.iter().position(|&x| x <= nn).unwrap();
    Some(vs[pos - 1])
}

aoc2022::solve!(part_1, part_2);

#[cfg(test)]
mod tests {
    use aoc2022::assert_ex;
    use super::*;

    #[test]
    fn test_part_1() { assert_ex!(part_1, 95437); }

    #[test]
    fn test_part_2() { assert_ex!(part_2, 24933642); }
}