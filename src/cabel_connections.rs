use std::rc::Rc;
use std::fs;
use std::cell::RefCell;


pub fn main(){
    let bank = fs::read_to_string("./input/11_cabel_connections.txt").unwrap();
    let connections = Connection::news(&bank);
    let you = connections.iter().find(|x| x.borrow().name == "you").unwrap();
    let n_paths = you.borrow().get_paths();
    println!("Day 11: Paths {}", n_paths);
    // 603

    let svr = connections.iter().find(|x| x.borrow().name == "svr").unwrap();
    let mut has_been = Vec::new();
    let valid_paths = svr.borrow().get_valid_paths(false, false, &mut has_been);

    println!("Day 11: Valid Paths {}", valid_paths);

}


#[derive(Debug)]
struct Connection{
    name : String,
    nexts : Vec<Rc<RefCell<Self>>>,
}

impl Connection {
    pub fn news(bank : &str) -> Vec<Rc<RefCell<Connection>>>{
        let bank = bank.trim();
        let splits = bank.split('\n');
        let mut named_connections = Vec::new();
        let mut machines = Vec::new();
        for machine_text in splits{
            let mut nexts = machine_text.split(' ');
            let parent = &nexts.next().unwrap()[..3];
            let mut children = Vec::new();
            for child in nexts{
                children.push(child);
            }
            named_connections.push((parent, children));
            let machine = Rc::new(RefCell::new(Connection{name:parent.to_string(), nexts:Vec::new()}));
            machines.push(machine);
        }
        let out = Rc::new(RefCell::new(Connection{name:"out".to_string(),nexts:Vec::new() }));
        for (parent, children) in named_connections.iter(){
            for child_name in children{
                let parent = machines.iter().find(|x| x.borrow().name == *parent).unwrap();
                if child_name == &"out"{
                    parent.borrow_mut().nexts.push(out.clone());
                }else{
                    let child = machines.iter().find(|x| x.borrow().name == *child_name).unwrap();
                    parent.borrow_mut().nexts.push(child.clone());
                }
            }
        }

        machines

    }


    pub fn get_paths(&self) -> usize{
        let mut finish = 0;
        for child_cell in self.nexts.iter(){
            let child = child_cell.borrow();
            if child.name == "out"{
                finish += 1;
            } else {
                finish += child.get_paths()
            }

        }

        finish
    }

    pub fn get_valid_paths(&self, fft:bool, dac:bool, has_been : &mut Vec<String>)->usize{
        let mut finish = 0;
        if has_been.contains(&self.name){
            return finish
        }
        has_been.push(self.name.clone());
        for child_cell in self.nexts.iter(){
            let mut new_been = has_been.clone();
            let child = child_cell.borrow();
            //println!("{} to {} with fft:{} && dac:{}", self.name, child.name, fft, dac);
            if fft && dac && child.name == "out"{
                finish += 1
            } else if self.name == "fft" {
                finish += child.get_valid_paths(true, dac, &mut new_been)
            } else if self.name == "dac" {
                finish += child.get_valid_paths(fft, true,  &mut new_been)
            } else {
                finish += child.get_valid_paths(fft, dac, &mut new_been)
            }
        }

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
        let you = connections.iter().find(|x| x.borrow().name == "you").unwrap();
        let n_paths = you.borrow().get_paths();

        assert_eq!(n_paths, 5)
    }

    #[test]
    fn test_example_2() {
        let bank = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out\n";
        let connections = Connection::news(bank);
        let svr = connections.iter().find(|x| x.borrow().name == "svr").unwrap();
        let mut has_been = Vec::new();
        let n_paths = svr.borrow().get_valid_paths(false, false, &mut has_been);

        assert_eq!(n_paths, 2);
        //assert!(false);

    }
}