use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn main() {
    let bank = fs::read_to_string("./input/11_cabel_connections.txt").unwrap();
    let connections = Connection::news(&bank);
    let you = connections
        .iter()
        .find(|x| x.borrow().name == "you")
        .unwrap();
    let n_paths = you.borrow_mut().get_paths_to("out");
    println!("Day 11: Paths {}", n_paths);
    // 603

    // let svr = connections.iter().find(|x| x.borrow().name == "dac").unwrap();
    // let valid_paths = svr.borrow_mut().get_paths_to("out");

    // println!("Day 11: Valid Paths from dac to out {}", valid_paths);
    //5921

    let connections_dac_to_out = Connection::news(&bank);
    let connections_fft_to_dac = Connection::news(&bank);
    let connections_svr_to_fft = Connection::news(&bank);
    let svr = connections_svr_to_fft
        .iter()
        .find(|x| x.borrow().name == "svr")
        .unwrap();
    let n_paths_to_fft = svr.borrow_mut().get_paths_to("fft");

    let fft = connections_fft_to_dac
        .iter()
        .find(|x| x.borrow().name == "fft")
        .unwrap();
    let n_paths_to_dac = fft.borrow_mut().get_paths_to("dac");

    let dac = connections_dac_to_out
        .iter()
        .find(|x| x.borrow().name == "dac")
        .unwrap();
    let n_paths_to_out = dac.borrow_mut().get_paths_to("out");
    println!("Day 11:");
    println!("svr->fft : {}", n_paths_to_fft);
    println!("fft->dac : {}", n_paths_to_dac);
    println!("dac->out : {}", n_paths_to_out);

    println!(
        "Total : {}",
        n_paths_to_fft * n_paths_to_dac * n_paths_to_out
    );
    // 380961604031372

    //let visited:usize = connections.iter().filter(|x| x.borrow().visited.is_some()).map(|_| 1).sum();
    //println!("Day 11: Visited {} of {} connections", visited, connections.len());

    //let svr = connections.iter().find(|x| x.borrow().name == "fft").unwrap();
    //let valid_paths = svr.borrow_mut().get_paths_to("dac");

    //println!("Day 11: Valid Paths from fft to dac {}", valid_paths);

    //let svr = connections.iter().find(|x| x.borrow().name == "svr").unwrap();
    //let valid_paths = svr.borrow_mut().get_paths_to("out");
    //println!("Day 11: All paths from svr to out {}", valid_paths);
}

#[derive(Debug, Clone)]
struct Connection {
    name: String,
    nexts: Vec<Rc<RefCell<Self>>>,
    visited: Option<(usize, bool, bool)>,
}

impl Connection {
    pub fn news(bank: &str) -> Vec<Rc<RefCell<Connection>>> {
        let bank = bank.trim();
        let splits = bank.split('\n');
        let mut named_connections = Vec::new();
        let mut machines = Vec::new();
        for machine_text in splits {
            let mut nexts = machine_text.split(' ');
            let parent = &nexts.next().unwrap()[..3];
            let mut children = Vec::new();
            for child in nexts {
                children.push(child);
            }
            named_connections.push((parent, children));
            let machine = Rc::new(RefCell::new(Connection {
                name: parent.to_string(),
                nexts: Vec::new(),
                visited: None,
            }));
            machines.push(machine);
        }
        let out = Rc::new(RefCell::new(Connection {
            name: "out".to_string(),
            nexts: Vec::new(),
            visited: None,
        }));
        for (parent, children) in named_connections.iter() {
            for child_name in children {
                let parent = machines
                    .iter()
                    .find(|x| x.borrow().name == *parent)
                    .unwrap();
                if child_name == &"out" {
                    parent.borrow_mut().nexts.push(out.clone());
                } else {
                    let child = machines
                        .iter()
                        .find(|x| x.borrow().name == *child_name)
                        .unwrap();
                    parent.borrow_mut().nexts.push(child.clone());
                }
            }
        }

        machines
    }

    pub fn get_paths_to(&mut self, target: &str) -> usize {
        let mut finish = 0;
        for child_cell in self.nexts.iter() {
            let mut child = child_cell.borrow_mut();
            if child.name == target {
                finish += 1;
            } else if let Some((number, _, _)) = child.visited {
                finish += number;
            } else {
                finish += child.get_paths_to(target);
            }
        }
        self.visited = Some((finish, false, false));

        finish
    }

    pub fn get_valid_paths(&mut self, fft: bool, dac: bool) -> usize {
        let mut finish = 0;
        for child_cell in self.nexts.iter() {
            let mut child = child_cell.borrow_mut();
            if fft && dac && child.name == "out" {
                finish += 1
            } else if self.name == "fft" {
                finish += child.get_valid_paths(true, dac)
            } else if self.name == "dac" {
                finish += child.get_valid_paths(fft, true)
            } else if let Some((number, c_fft, c_dac)) = child.visited
                && c_fft
                && c_dac
            {
                finish += number;
            } else {
                finish += child.get_valid_paths(fft, dac)
            }
            println!(
                "{} to {} with fft:{} && dac:{} = {}",
                self.name, child.name, fft, dac, finish
            );
        }
        self.visited = Some((finish, fft, dac));

        finish
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let bank = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out";
        let connections = Connection::news(bank);
        let you = connections
            .iter()
            .find(|x| x.borrow().name == "you")
            .unwrap();
        let n_paths = you.borrow_mut().get_paths_to("out");

        assert_eq!(n_paths, 5)
    }

    #[test]
    fn test_example_2() {
        let bank = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out\n";
        // maybe we have to do a has_been shared between all connections. So if a path rejoins the path, then we do just add that number to the total.

        let connections_dac_to_out = Connection::news(bank);
        let connections_fft_to_dac = Connection::news(bank);
        let connections_svr_to_fft = Connection::news(bank);
        let svr = connections_svr_to_fft
            .iter()
            .find(|x| x.borrow().name == "svr")
            .unwrap();
        let n_paths_to_fft = svr.borrow_mut().get_paths_to("fft");

        let fft = connections_fft_to_dac
            .iter()
            .find(|x| x.borrow().name == "fft")
            .unwrap();
        let n_paths_to_dac = fft.borrow_mut().get_paths_to("dac");

        let dac = connections_dac_to_out
            .iter()
            .find(|x| x.borrow().name == "dac")
            .unwrap();
        let n_paths_to_out = dac.borrow_mut().get_paths_to("out");

        assert_eq!(n_paths_to_fft * n_paths_to_dac * n_paths_to_out, 2);
        //assert!(false);
        // There are 5921 paths from dac to out. This one might work.
    }
}
