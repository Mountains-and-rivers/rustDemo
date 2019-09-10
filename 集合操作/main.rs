use std::collections::hash_map::{Entry, HashMap};

struct Item {
  id: u32,
  value: String,
}

struct Obj {
  map: HashMap<String, Item>,
}

impl Obj {
  pub fn check(&mut self, key: &str, check_id: u32) {
    // retrieve current item for key (if it exists)
    let item = self.map.entry(String::from(key));
    // do some complex matching on the item to determine
    // what to do next
    match item {
      Entry::Occupied(ref entry) if entry.get().id == check_id => {
        // item exists, and has same id, nothing more to do
        println!("found checked id");
      }
      Entry::Occupied(mut entry) => {
          println!("found checked id11");
        // item exists, but with an other id, so replace it with a new one
        *entry.get_mut() = Item {
          id: check_id,
          value: String::new(),
        };
      }
      Entry::Vacant(entry) => {
          println!("found checked id22");
        // else, we need to mutate the map, e.g. like this
        entry.insert(Item {
          id: check_id,
          value: String::new(),
        });
      }
    }
  }
}

fn main() {
  let mut obj = Obj {
    map: HashMap::new(),
  };
    let mut obj = Obj {
    map: HashMap::new(),
  };
  // fill like...
  //obj.map.insert(String::from("a"), Item { id: 0, value: String::from("xxx") });
  obj.check("a", 0);
    obj.check("a", 0);
      obj.check("a", 0);
}
