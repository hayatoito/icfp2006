use crate::prelude::Result;
use crate::um::Um;
use crate::um::UmStatus;
use lazy_static::*;
use log::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::default::Default;
use std::io::Read;
use std::rc::Rc;

use crate::adventure::*;

/// I didn't notice that we can "examine item" nor "switch XXX" while solving part1.
/// Therefore, I solved part1 by blute-force:
/// - parse um's output (english) manually
/// - use um's output to confirm whether "combine A with B" is successful or not;  I didn't know that we
///   can know a dependency graph by "examine itemname"

fn parse_inventory_output(output: &str) -> BTreeSet<Item> {
    let re = Regex::new(r"^an? (.+)([,.]| and)$").unwrap();
    output
        .trim()
        .split("\n")
        .flat_map(|line| re.captures(line).map(|matched| Item::from(&matched[1])))
        .collect()
}

// >: examine
// examine
// Junk Room

// You are in a room with a pile of junk. A hallway leads south.
// There is a bolt here.
// Underneath the bolt, there is a spring.
// Underneath the spring, there is a button.
// Underneath the button, there is a (broken) processor.
// Underneath the processor, there is a red pill.
// Underneath the pill, there is a (broken) radio.
// Underneath the radio, there is a cache.
// Underneath the cache, there is a blue transistor.
// Underneath the transistor, there is an antenna.
// Underneath the antenna, there is a screw.
// Underneath the screw, there is a (broken) motherboard.
// Underneath the motherboard, there is a (broken) A-1920-IXB.
// Underneath the A-1920-IXB, there is a red transistor.
// Underneath the transistor, there is a (broken) keypad.
// Underneath the keypad, there is some trash.

fn parse_examine_output(output: &str) -> Option<Item> {
    // Returns the top item
    let re = Regex::new(r"^There is an? (.+) here\.$").unwrap();
    output.trim().split("\n").find_map(|line| {
        // debug!("line: {}", line);
        re.captures(line.trim())
            .map(|matched| Item::from(&matched[1]))
    })
}

fn parse_examine_output_all(output: &str) -> Vec<Item> {
    let re = Regex::new(r"[tT]here is an? ([^.]+?)( here)?\.").unwrap();
    re.captures_iter(&output.replace("\n", " "))
        .map(|cap| Item::from(&cap[1]))
        .collect()
}

// >: w
// Salon E

// You are in Salon E of the Oregon Ballroom. A door leads east.
// There is a bullet-point here.
// Underneath the bullet-point, there is a (broken) slides.ppt.

// >: take bullet-point
// You are now carrying the bullet-point.

// >: take (broken) slides.ppt
// There is no (broken) slides.ppt here.

// >: take slides.ppt
// You are now carrying the slides.ppt.

fn parse_take_output(output: &str) -> Option<String> {
    let re = Regex::new(r"^You are now carrying the (.+)\.$").unwrap();
    output.trim().split("\n").find_map(|line| {
        re.captures(line.trim())
            .map(|matched| matched[1].to_string())
    })
}

// >: incinerate bolt
// incinerate bolt
// The bolt has been destroyed.

// >:
fn parse_incinerate_output(output: &str) -> Option<String> {
    let re = Regex::new(r"^The (.+) has been destroyed\.$").unwrap();
    output.trim().split("\n").find_map(|line| {
        re.captures(line.trim())
            .map(|matched| matched[1].to_string())
    })
}

#[test]
fn parse_inventory_output_test() {
    let output = "
You are carrying:
a spring,
a bolt,
a (broken) slides.ppt,
a bullet-point,
a manifesto and
a pamphlet.
";
    let inventory = parse_inventory_output(output);
    assert_eq!(
        inventory,
        vec![
            Item::new("spring".to_string(), None, None),
            Item::new("bolt".to_string(), None, None),
            Item::new_broken("slides.ppt".to_string(), None),
            Item::new("bullet-point".to_string(), None, None),
            Item::new("manifesto".to_string(), None, None),
            Item::new("pamphlet".to_string(), None, None),
        ]
        .into_iter()
        .collect()
    );

    assert_eq!(
        parse_inventory_output(
            "
You are carrying:
a red pill,
a blue transistor,
an antenna,
a red transistor and
a pamphlet.
"
        ),
        vec![
            Item::new("pill".to_string(), Some("red".to_string()), None),
            Item::new("transistor".to_string(), Some("blue".to_string()), None),
            Item::new("antenna".to_string(), None, None),
            Item::new("transistor".to_string(), Some("red".to_string()), None),
            Item::new("pamphlet".to_string(), None, None),
        ]
        .into_iter()
        .collect()
    );
}

#[test]
fn parse_examine_output_test() {
    let output = "
Junk Room

You are in a room with a pile of junk. A hallway leads south.
There is a bolt here.
Underneath the bolt, there is a spring.
Underneath the spring, there is a button.
Underneath the button, there is a (broken) processor.
Underneath the processor, there is a red pill.
Underneath the pill, there is a (broken) radio.
Underneath the radio, there is a cache.
Underneath the cache, there is a blue transistor.
Underneath the transistor, there is an antenna.
Underneath the antenna, there is a screw.
Underneath the screw, there is a (broken) motherboard.
Underneath the motherboard, there is a (broken) A-1920-IXB.
Underneath the A-1920-IXB, there is a red transistor.
Underneath the transistor, there is a (broken) keypad.
Underneath the keypad, there is some trash.
";
    assert_eq!(
        parse_examine_output(output),
        Some(Item::new("bolt".to_string(), None, None))
    );
}

#[test]
fn parse_examine_output_all_test() {
    let output = "
54th Street and Dorchester Avenue

You are standing at the corner of 54th Street and Dorchester
Avenue. From here, you can go north, east, south, or west.
There is an orange-red X-9247-GWE here.
Underneath the X-9247-GWE, there is a (broken) magenta
V-0010-XBD.
Underneath the V-0010-XBD, there is a pumpkin F-1403-QDS.
Underneath the F-1403-QDS, there is a (broken) heavy P-5065-WQO.

Underneath the P-5065-WQO, there is a taupe B-4832-LAL.
";
    assert_eq!(
        parse_examine_output_all(output),
        vec![
            Item::new(
                "X-9247-GWE".to_string(),
                Some("orange-red".to_string()),
                None
            ),
            Item::new_broken("V-0010-XBD".to_string(), Some("magenta".to_string())),
            Item::new("F-1403-QDS".to_string(), Some("pumpkin".to_string()), None),
            Item::new_broken("P-5065-WQO".to_string(), Some("heavy".to_string())),
            Item::new("B-4832-LAL".to_string(), Some("taupe".to_string()), None),
        ]
    );
}

#[test]
fn parse_take_output_test() {
    // >: take bolt
    // take bolt
    // You are now carrying the bolt.
    let output = "You are now carrying the bolt.\n";
    assert_eq!(parse_take_output(output), Some("bolt".to_string()));
}

#[test]
fn parse_incinerate_output_test() {
    let output = "The bolt has been destroyed.\n";
    assert_eq!(parse_incinerate_output(output), Some("bolt".to_string()));
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Item {
    name: String,
    color: Option<String>,
    broken: Option<BrokenStatus>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct BrokenStatus {
    combined: BTreeSet<Item>,
}

impl Item {
    fn new(name: String, color: Option<String>, broken: Option<BrokenStatus>) -> Item {
        Item {
            name,
            color,
            broken,
        }
    }

    #[cfg(test)]
    fn new_broken(name: String, color: Option<String>) -> Item {
        Item::new(
            name,
            color,
            Some(BrokenStatus {
                combined: Default::default(),
            }),
        )
    }

    fn from(s: &str) -> Item {
        // e.g.
        // (broken) heavy P-5065-WQO
        // pumpkin F-1403-QDS
        lazy_static! {
            static ref ITEM_RE: Regex = Regex::new(
                r"(?x)
^(?P<broken>\(broken\)\s)?  # broken
((?P<color>\S+)\s)?  # color
(?P<name>\S+)$   # name
"
            )
            .unwrap();
        }
        let cap = ITEM_RE.captures(s).unwrap();
        Item::new(
            cap.name("name").unwrap().as_str().to_string(),
            cap.name("color").map(|s| s.as_str().to_string()),
            cap.name("broken").and(Some(BrokenStatus {
                combined: Default::default(),
            })),
        )
    }

    fn full_name(&self) -> String {
        // TODO: cache full_name()
        if let Some(color) = &self.color {
            format!("{} {}", color, self.name)
        } else {
            self.name.clone()
        }
    }

    // fn is_unique(&self) -> bool {
    //     is_unique_item_name(&self.name)
    // }
}

// fn is_unique_item_name(s: &str) -> bool {
//     // non-unique item: "X-9247-GWE"
//     lazy_static! {
//         static ref NON_UNIQUE_ITEM_NAME_RE: Regex = Regex::new(r"[A-Z]-\d{4}-[A-Z]{3}").unwrap();
//     }
//     !NON_UNIQUE_ITEM_NAME_RE.is_match(s)
// }

// #[test]
// fn is_unique_item_name_test() {
//     assert!(!is_unique_item_name("J-5065-IQW"));
//     assert!(!is_unique_item_name("R-0010-FLH"));
//     assert!(is_unique_item_name("USB cable"));
// }

#[test]
fn parse_item_str_test() {
    assert_eq!(
        Item::from("orange-red X-9247-GWE"),
        Item::new(
            "X-9247-GWE".to_string(),
            Some("orange-red".to_string()),
            None
        )
    );
    assert_eq!(
        Item::from("(broken) heavy P-5065-WQO"),
        Item::new_broken("P-5065-WQO".to_string(), Some("heavy".to_string()),)
    );
    assert_eq!(
        Item::from("button"),
        Item::new("button".to_string(), None, None)
    );
    assert_eq!(
        Item::from("(broken) motherboard"),
        Item::new_broken("motherboard".to_string(), None)
    );
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(broken_status) = &self.broken {
            write!(f, "(broken) {:?} ", broken_status)?
        }
        if let Some(color) = &self.color {
            write!(f, "{} ", color)?
        }
        write!(f, "{}", self.name)
    }
}

trait UmAdventureExt: UmContinueExt {
    // fn state(&self) -> State;
    fn inventory(&mut self) -> BTreeSet<Item>;
    fn combine(&mut self, a: &str, b: &str) -> bool;
    fn examine(&mut self) -> Option<Item>;
    fn take(&mut self, a: &str);
    // fn drop(&mut self, a: &Item);
    fn use_item(&mut self, a: &str) -> bool;
    fn incinerate(&mut self, a: &str);
}

impl UmAdventureExt for Um {
    fn inventory(&mut self) -> BTreeSet<Item> {
        let input = "inventory\n";
        let output = self.enter_command(input);
        debug!("output: {}", output);
        parse_inventory_output(&output)
    }

    fn combine(&mut self, a: &str, b: &str) -> bool {
        let input = format!("combine {} with {}\n", a, b);
        let output = self.enter_command(&input);
        assert!(
            !output.contains("You aren't carrying"),
            format!("a: {} + b: {} => output: {}", a, b, output)
        );
        // output:
        // You have successfully combined xxx with
        output.contains("You have successfully combined")
    }

    fn examine(&mut self) -> Option<Item> {
        let input = "examine\n";
        let output = self.enter_command(input);
        parse_examine_output(&output)
    }

    fn take(&mut self, a: &str) {
        let input = format!("take {}\n", a);
        let output = self.enter_command(&input);
        let taken_item = parse_take_output(&output);
        assert!(
            taken_item.is_some(),
            format!("input: {}, output: {}", input, output)
        );
        // > take red pill
        // You are now carrying the pill.
        assert!(a.ends_with(&taken_item.unwrap()));
    }
    fn incinerate(&mut self, a: &str) {
        let input = format!("incinerate {}\n", a);
        let output = self.enter_command(&input);
        let destroyed_item = parse_incinerate_output(&output);
        assert!(destroyed_item.is_some());
        assert!(a.ends_with(&destroyed_item.unwrap()));
    }
    fn use_item(&mut self, a: &str) -> bool {
        let input = format!("use {}\n", a);
        let output = self.enter_command(&input);
        assert!(!output.contains("You aren't carrying"));
        if output.contains("Your efforts are to no avail:") {
            false
        } else {
            println!("use item: {}: output: {}", a, output);
            true
        }
    }
}

type CombineRule = BTreeMap<(Item, Item), Option<Item>>;

fn dump_combine_rule(rule: &CombineRule) {
    for ((a, b), item) in rule {
        if item.is_none() {
            continue;
        }
        info!("combine {} with {} => {}", a, b, item.as_ref().unwrap());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Take(String),
    Incinerate(String),
    Combine(String, String),
}

#[derive(Debug, Clone)]
struct CommandList {
    command: Command,
    prev: Option<Rc<CommandList>>,
}

impl CommandList {
    fn to_list(&self) -> Vec<Command> {
        let mut current = self;
        let mut commands = vec![self.command.clone()];
        while let Some(prev) = current.prev.as_ref() {
            commands.push(prev.command.clone());
            current = prev.as_ref();
        }
        commands.reverse();
        commands
    }

    // fn append(this: &Rc<CommandList>, other: &Rc<CommandList>) -> Rc<CommandList> {
    //     let mut current = Rc::clone(this);
    //     for command in other.to_list() {
    //         current = Rc::new(CommandList {
    //             command,
    //             prev: Some(Rc::clone(&current)),
    //         })
    //     }
    //     current
    // }
}

#[test]
fn command_list_test() {
    let this = Rc::new(CommandList {
        command: Command::Take("foo2".to_string()),
        prev: Some(Rc::new(CommandList {
            command: Command::Take("foo1".to_string()),
            prev: None,
        })),
    });

    assert_eq!(
        this.to_list(),
        vec![
            Command::Take("foo1".to_string()),
            Command::Take("foo2".to_string())
        ]
    );

    // let other = Rc::new(CommandList {
    //     command: Command::Take("foo4".to_string()),
    //     prev: Some(Rc::new(CommandList {
    //         command: Command::Take("foo3".to_string()),
    //         prev: None,
    //     })),
    // });

    // assert_eq!(
    //     CommandList::append(&this, &other).to_list(),
    //     vec![
    //         Command::Take("foo1".to_string()),
    //         Command::Take("foo2".to_string()),
    //         Command::Take("foo3".to_string()),
    //         Command::Take("foo4".to_string())
    //     ]
    // );
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct Inventory {
    inventory: BTreeSet<Item>,
}

impl Inventory {
    fn item_stack() -> &'static [Item] {
        lazy_static! {
            static ref OUTPUT: &'static str = "
Junk Room

You are in a room with a pile of junk. A hallway leads south.
There is a bolt here.
Underneath the bolt, there is a spring.
Underneath the spring, there is a button.
Underneath the button, there is a (broken) processor.
Underneath the processor, there is a red pill.
Underneath the pill, there is a (broken) radio.
Underneath the radio, there is a cache.
Underneath the cache, there is a blue transistor.
Underneath the transistor, there is an antenna.
Underneath the antenna, there is a screw.
Underneath the screw, there is a (broken) motherboard.
Underneath the motherboard, there is a (broken) A-1920-IXB.
Underneath the A-1920-IXB, there is a red transistor.
Underneath the transistor, there is a (broken) keypad.
Underneath the keypad, there is some trash.
";
            static ref INVENTORY_STACK: Vec<Item> = {
                let mut items = parse_examine_output_all(&OUTPUT);
                items.reverse();
                items
            };
        }
        &INVENTORY_STACK
    }

    fn new(inventory: BTreeSet<Item>) -> Inventory {
        Inventory { inventory }
    }

    fn incinerate(&self, item: &Item) -> Inventory {
        let mut inventory = self.inventory.clone();
        inventory.remove(item);
        Inventory::new(inventory)
    }

    fn take(&self, stack_pos: usize) -> Inventory {
        assert!(stack_pos > 0);
        let mut inventory = self.inventory.clone();
        inventory.insert(Inventory::item_stack()[stack_pos - 1].clone());
        Inventory::new(inventory)
    }

    fn combine(&self, a: &Item, b: &Item, new_item: Item) -> Inventory {
        let mut inventory = self.inventory.clone();
        assert!(inventory.remove(a));
        assert!(inventory.remove(b));
        inventory.insert(new_item);
        Inventory::new(inventory)
    }

    // For part1
    fn is_goal(&self) -> bool {
        self.inventory
            .get(&Item::new("keypad".to_string(), None, None))
            .is_some()
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct State {
    inventory: Inventory,
    stack_pos: usize,
}

#[derive(Debug)]
struct Pos {
    state: State,
    command: Option<Rc<CommandList>>,
}

impl Pos {
    fn add_command(&self, command: Command) -> Rc<CommandList> {
        Rc::new(CommandList {
            command,
            prev: self.command.clone(),
        })
    }

    fn incinerate(&self, item: &Item) -> Pos {
        Pos {
            state: State {
                inventory: self.state.inventory.incinerate(item),
                stack_pos: self.state.stack_pos,
            },
            command: Some(self.add_command(Command::Incinerate(item.full_name().clone()))),
        }
    }

    fn take(&self, stack_pos: usize) -> Pos {
        Pos {
            state: State {
                inventory: self.state.inventory.take(stack_pos),
                stack_pos: self.state.stack_pos - 1,
            },
            command: Some(self.add_command(Command::Take(
                Inventory::item_stack()[stack_pos - 1].full_name(),
            ))),
        }
    }

    fn combine(&self, a: &Item, b: &Item, new_item: Item) -> Pos {
        Pos {
            state: State {
                inventory: self.state.inventory.combine(a, b, new_item),
                stack_pos: self.state.stack_pos,
            },
            command: Some(self.add_command(Command::Combine(
                a.full_name().clone(),
                b.full_name().clone(),
            ))),
        }
    }
}

struct AdventureSolver {
    um: Um,
    visited: BTreeSet<State>,
    combine_rule: CombineRule,
}

impl AdventureSolver {
    fn new(um: Um) -> AdventureSolver {
        AdventureSolver {
            um,
            visited: Default::default(),
            combine_rule: Default::default(),
        }
    }

    fn try_combine(
        &mut self,
        a: Item,
        b: Item,
        command_list: &Option<Rc<CommandList>>,
    ) -> Option<Item> {
        if let Some(result) = self.combine_rule.get(&(a.clone(), b.clone())) {
            return result.clone();
        }
        info!("try to combine: {:?} and {:?}", a, b);
        // Need to ask um. Execute commands in um.
        let mut um = self.um.clone();
        if let Some(command_list) = command_list.as_ref() {
            for command in command_list.to_list() {
                match command {
                    Command::Take(item_name) => {
                        um.take(&item_name);
                    }
                    Command::Incinerate(item_name) => {
                        um.incinerate(&item_name);
                    }
                    Command::Combine(a, b) => {
                        assert!(um.combine(&a, &b));
                    }
                }
            }
        }
        let current_inventory = um.inventory();
        // TODO: assert_eq!(current_inventory, expected_inventory);

        // Apply combine
        if um.combine(&a.full_name(), &b.full_name()) {
            let new_inventory = um.inventory();
            let lost_item = current_inventory.difference(&new_inventory).next().unwrap();
            let new_item = {
                let (a, b) = {
                    if lost_item.full_name() == a.full_name() {
                        (&b, &a)
                    } else {
                        assert_eq!(lost_item.full_name(), b.full_name());
                        (&a, &b)
                    }
                };
                // TODO: Check new item's status
                let new_item = new_inventory.difference(&current_inventory).next();
                if new_item.is_some() {
                    // Case: A + B => C
                    new_item.unwrap().clone()
                } else {
                    // Case: (broken) A + B => (broken) A
                    let mut combined = if let Some(BrokenStatus { combined }) = &a.broken {
                        combined.clone()
                    } else {
                        unreachable!()
                    };
                    combined.insert(b.clone());
                    Item::new(
                        a.name.clone(),
                        a.color.clone(),
                        Some(BrokenStatus { combined }),
                    )
                }
            };
            self.combine_rule
                .insert((a.clone(), b.clone()), Some(new_item.clone()));
            info!("  => new_item: {:?}", new_item);
            if log_enabled!(log::Level::Info) {
                info!("combine_rule:");
                dump_combine_rule(&self.combine_rule);
            }
            Some(new_item)
        } else {
            self.combine_rule.insert((a.clone(), b.clone()), None);
            None
        }
    }

    fn solve(&mut self, pos: Pos) -> bool {
        // debug!("pos: {:?}", pos);
        if self.visited.contains(&pos.state) {
            return false;
        }
        self.visited.insert(pos.state.clone());

        // goal: Got (unbroken) keypad
        if pos.state.inventory.is_goal() {
            info!("goal reached: pos: {:?}", pos);
            println!("commands start");
            for command in pos.command.unwrap().to_list() {
                match command {
                    Command::Take(item_name) => {
                        println!("take {}", item_name);
                    }
                    Command::Incinerate(item_name) => {
                        println!("incinerate {}", item_name);
                    }
                    Command::Combine(a, b) => {
                        println!("combine {} with {}", a, b);
                    }
                }
            }
            println!("commands end");
            return true;
        }

        // Try all posibble actions

        // Combine items
        for a in &pos.state.inventory.inventory {
            if a.broken.is_some() {
                for b in &pos.state.inventory.inventory {
                    if let Some(new_item) = self.try_combine(a.clone(), b.clone(), &pos.command) {
                        let pos = pos.combine(&a, &b, new_item);
                        if self.solve(pos) {
                            return true;
                        }
                    }
                }
            }
        }

        // take an item
        if pos.state.inventory.inventory.len() < 6 && pos.state.stack_pos > 0 {
            if self.solve(pos.take(pos.state.stack_pos)) {
                return true;
            }
        }

        // Destroy items
        if pos.state.inventory.inventory.len() == 6 {
            for item in &pos.state.inventory.inventory {
                let pos = pos.incinerate(&item);
                if self.solve(pos) {
                    return true;
                }
            }
        }
        false
    }
}

pub fn solve(code: String) -> Result<()> {
    // Solve the first part of "./adventure" game
    let mut f = std::fs::File::open(code)?;
    let mut code = Vec::new();
    f.read_to_end(&mut code)?;
    let input: Vec<u8> = b"howie
xyzzy
./adventure
take pamphlet
take manifesto
go west
take bullet-point
take slides.ppt
go east
go north
"
    .iter()
    .cloned()
    .collect();

    let mut um = Um::new(code);
    um.set_print_stdin(true);
    let result = um.run(&mut (input.as_ref() as &[u8]), &mut std::io::stdout());
    assert_eq!(result, UmStatus::NoInput);

    um.set_print_stdin(false);
    let inventory = um.inventory();
    println!("inventory: {:?}", inventory);

    let mut solver = AdventureSolver::new(um);
    solver.solve(Pos {
        state: State {
            inventory: Inventory { inventory },
            stack_pos: Inventory::item_stack().len(),
        },
        command: None,
    });
    Ok(())
}
