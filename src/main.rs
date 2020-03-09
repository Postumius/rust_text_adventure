use std::collections::HashMap;
use std::io;


struct Thing {
    desc: String,
    contents: HashMap<String, Thing>,
    adj: HashMap<String, String>
}
impl Thing {
    fn new(desc: &str) -> Thing {
        Thing {
            desc: String::from(desc),
            contents: HashMap::new(),
            adj: HashMap::new()
        }
    }
    fn with_contents(mut self, v: Vec<(&str, Thing)>) -> Thing {
        self.contents =
            v.into_iter().
            map(|(dir, thing)| (String::from(dir), thing))
            .collect();
        self
    }
    fn with_adj(mut self, v: Vec<(&str, &str)>) -> Thing {
        self.adj =
            v.into_iter().
            map(|(dir, name)| (String::from(dir), String::from(name)))
            .collect();
        self
    }
    fn insert(&mut self, name:&str, there:&str) {
        let thing = self.contents.remove(name)
            .expect("insert: couldn't find thing");
        let container = self.contents.get_mut(there)
            .expect("insert: couldn't find container");
        container.contents.insert(String::from(name), thing);
    }
    fn remove(&mut self, name:&str, here:&str) {
        let thing = self.contents.get_mut(here)
            .expect("remove: couldn't find container")
            .contents.remove(name)
            .expect("remove: couldn't find thing");
        self.contents.insert(String::from(name), thing);
    }
}

struct World(HashMap<String, Thing>);
impl World {
    fn check(&self) -> bool {
        for (here, room) in self.0.iter() {
            for (_, there) in room.adj.iter() {
                if self.0.get(there).is_none() {
                    panic!("Tried to connect {} to {}, but {} doesn't exist.",
                           here, there, there);
                }
            }
        }
        true
    }
    fn move_thing(&mut self, name:&str, here:&str, there:&str) {
        let thing = self.0.get_mut(here)
            .expect("move_thing: origin does not exist")
            .contents.remove(name)
            .expect("move_thing: thing does not exist");
        let dest_room = self.0.get_mut(there)
            .expect("move_thing: destination does not exist");
        
        dest_room.contents.insert(String::from(name), thing);
       
    }
}

fn repl(mut world: World, addr: String) {
    
    let mut room = world.0.get_mut(&addr).unwrap();
    println!("You are here: {}", addr);
    println!("You see:");
    for (name, _) in room.contents.iter() {
        println!("  a {}", name);
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("read_line failed");
    let args: Vec<&str> = input.split(' ').map(str::trim).collect();

    let mut new_addr = addr.clone();
    match args[0] {
        "exit" => {return ()},
        "look" => {println!("{}", room.desc)},
        "insert" => {try_inserting(&mut room, "player", args[1], args[2])},
        "remove" => {try_removing(&mut room, "player", args[1], args[2])},
        _ => match room.adj.get(args[0]) {
            Some(string) => {
                new_addr = string.clone();
                world.move_thing("player", &addr, &new_addr);
            },                    
            None => ()
        }
    };
    repl(world, new_addr)
}

fn try_inserting(room:&mut Thing, actor:&str, item:&str, container:&str) {
    if room.contents.get(item).is_none() {
        println!("{} couldn't find the {}.", actor, item);
    } else if room.contents.get(container).is_none() {
        println!("{} couldn't find the {}.", actor, container);
    } else if container == item {
        println!("{} can't insert {} into itself.", actor, item);
    } else {
        room.insert(item, container);
        println!("{} put the {} into the {}.", actor, item, container);
    }
}

fn try_removing(room:&mut Thing, actor:&str, item:&str, container:&str) {
    match room.contents.get(container) {
        None => {
            println!("{} couldn't find the {}.", actor, container);},
        Some(bag) => match bag.contents.get(item) {
            None => {
                println!("{} couldn't find a {} in the {}.", actor, item, container);},
            Some(_) => {
                room.remove(item, container);
                println!("{} took the {} out of the {}.", actor, item, container);}
        }
    }
}


fn main() {    
    let mut world = World(HashMap::new());
    world.0.insert
        (String::from("a"),
         Thing::new("This is a")
         .with_adj(vec![("north", "b")])
         .with_contents(
             vec![("cube",
                   Thing::new("It's a freaking cube.")),
                  ("player",
                   Thing::new("It's you!"))]));
    world.0.insert
         (String::from("b"),
          Thing::new("This is b")
          .with_adj(vec![("south", "a")]));

    if world.check() {
        repl(world, String::from("a"));
    }
}
