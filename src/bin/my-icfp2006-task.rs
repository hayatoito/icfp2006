// use icfp2006::um;
// use std::io::prelude::*;
use icfp2006::um::Um;
use icfp2006::um::UmStatus;
use lazy_static::*;
use log::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::default::Default;
use std::io::Read;
use std::rc::Rc;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, failure::Error>;

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

fn parse_inventory_output(output: &str) -> BTreeSet<Item> {
    let re = Regex::new(r"^an? (.+)([,.]| and)$").unwrap();
    output
        .trim()
        .split("\n")
        .flat_map(|line| re.captures(line).map(|matched| Item::from(&matched[1])))
        .collect()
}

fn parse_examine_output(output: &str) -> Option<Item> {
    // Returns the top item
    let re = Regex::new(r"^There is an? (.+) here\.$").unwrap();
    output.trim().split("\n").find_map(|line| {
        // debug!("line: {}", line);
        re.captures(line.trim())
            .map(|matched| Item::from(&matched[1]))
    })
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

// TODO: (broken) should be ignored...
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
            Item::new("spring".to_string(), ItemStatus::Normal),
            Item::new("bolt".to_string(), ItemStatus::Normal),
            Item::new(
                "slides.ppt".to_string(),
                ItemStatus::Broken {
                    combined: Default::default()
                }
            ),
            Item::new("bullet-point".to_string(), ItemStatus::Normal),
            Item::new("manifesto".to_string(), ItemStatus::Normal),
            Item::new("pamphlet".to_string(), ItemStatus::Normal),
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
            Item::new("pill".to_string(), ItemStatus::Red),
            Item::new("transistor".to_string(), ItemStatus::Blue),
            Item::new("antenna".to_string(), ItemStatus::Normal),
            Item::new("transistor".to_string(), ItemStatus::Red),
            Item::new("pamphlet".to_string(), ItemStatus::Normal),
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
        Some(Item::new("bolt".to_string(), ItemStatus::Normal))
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
    status: ItemStatus,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
enum ItemStatus {
    Normal,
    Broken { combined: BTreeSet<Item> },
    Red,
    Blue,
}

impl Item {
    fn new(name: String, status: ItemStatus) -> Item {
        Item { name, status }
    }

    fn new_broken(name: String) -> Item {
        Item::new(
            name,
            ItemStatus::Broken {
                combined: Default::default(),
            },
        )
    }

    fn from(s: &str) -> Item {
        if s.starts_with("(broken) ") {
            Item::new_broken(s["(broken) ".len()..].to_string())
        } else if s.starts_with("red ") {
            Item::new(s["red ".len()..].to_string(), ItemStatus::Red)
        } else if s.starts_with("blue ") {
            Item::new(s["blue ".len()..].to_string(), ItemStatus::Blue)
        } else {
            Item::new(s.to_string(), ItemStatus::Normal)
        }
    }

    fn name_for_take(&self) -> String {
        match &self.status {
            ItemStatus::Normal | ItemStatus::Broken { .. } => self.name.clone(),
            ItemStatus::Red => format!("red {}", self.name),
            ItemStatus::Blue => format!("blue {}", self.name),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.status {
            ItemStatus::Normal => write!(f, "{}", self.name),
            ItemStatus::Broken { combined } => write!(f, "(broken) {} {:?}", self.name, combined),
            ItemStatus::Red => write!(f, "red {}", self.name),
            ItemStatus::Blue => write!(f, "blue {}", self.name),
        }
    }
}

trait UmContinueExt {
    fn enter_command(&mut self, input: &str) -> String;
}

impl UmContinueExt for Um {
    fn enter_command(&mut self, input: &str) -> String {
        let mut output = Vec::new();
        let status = self.continue_with(&mut input.as_bytes(), &mut output);
        assert_eq!(status, UmStatus::NoInput);
        let output = String::from_utf8(output).unwrap();
        // debug!("input: {}", input);
        // debug!("output: {}", output);
        output
    }
}

trait UmAdventureExt: UmContinueExt {
    // fn state(&self) -> State;
    fn inventory(&mut self) -> BTreeSet<Item>;
    fn combine(&mut self, a: &str, b: &str) -> bool;
    fn examine(&mut self) -> Option<Item>;
    fn take(&mut self, a: &str);
    // fn drop(&mut self, a: &Item); // TODO: How to drop an item?
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

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct State {
    inventory: Inventory,
    stack_pos: usize,
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct Inventory {
    inventory: BTreeSet<Item>,
}

impl Inventory {
    fn item_stack() -> &'static [Item] {
        lazy_static! {
            static ref INVENTORY_STACK: Vec<Item> = {
                let mut items: Vec<Item> = vec![
                    // ("bolt", ItemStatus::Normal),
                    // ("spring", ItemStatus::Normal),
                    ("button", ItemStatus::Normal),
                    ("processor", ItemStatus::Broken {
                            combined: Default::default(),
                        },
                    ),
                    ("pill", ItemStatus::Red),
                    (
                        "radio",
                        ItemStatus::Broken {
                            combined: Default::default(),
                        },
                    ),
                    ("cache", ItemStatus::Normal),
                    ("transistor", ItemStatus::Blue),
                    ("antenna", ItemStatus::Normal),
                    ("screw", ItemStatus::Normal),
                    (
                        "motherboard",
                        ItemStatus::Broken {
                            combined: Default::default(),
                        },
                    ),
                    (
                        "A-1920-IXB",
                        ItemStatus::Broken {
                            combined: Default::default(),
                        },
                    ),
                    ("transistor", ItemStatus::Red),
                    (
                        "keypad",
                        ItemStatus::Broken {
                            combined: Default::default(),
                        },
                    ),
                    ("junk", ItemStatus::Normal),
                ]
                .into_iter()
                .map(|(name, status)| Item::new(name.to_string(), status))
                .collect();
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
        inventory.insert(Inventory::item_stack()[stack_pos].clone());
        Inventory::new(inventory)
    }

    fn combine(&self, a: &Item, b: &Item, new_item: Item) -> Inventory {
        let mut inventory = self.inventory.clone();
        assert!(inventory.remove(a));
        assert!(inventory.remove(b));
        inventory.insert(new_item);
        Inventory::new(inventory)
    }

    fn is_goal(&self) -> bool {
        self.inventory
            .get(&Item::new("keypad".to_string(), ItemStatus::Normal))
            .is_some()
    }
}

#[derive(Clone, Debug)]
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
            command: Some(self.add_command(Command::Incinerate(item.name.clone()))),
        }
    }

    fn take(&self, stack_pos: usize) -> Pos {
        Pos {
            state: State {
                inventory: self.state.inventory.take(stack_pos),
                stack_pos: self.state.stack_pos - 1,
            },
            command: Some(self.add_command(Command::Take(
                Inventory::item_stack()[stack_pos].name_for_take(),
            ))),
        }
    }

    fn combine(&self, a: &Item, b: &Item, new_item: Item) -> Pos {
        Pos {
            state: State {
                inventory: self.state.inventory.combine(a, b, new_item),
                stack_pos: self.state.stack_pos,
            },
            command: Some(self.add_command(Command::Combine(a.name.clone(), b.name.clone()))),
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
        if um.combine(&a.name, &b.name) {
            let new_inventory = um.inventory();
            let lost_item = current_inventory.difference(&new_inventory).next().unwrap();
            let new_item = {
                let (a, b) = {
                    if lost_item.name == a.name {
                        (&b, &a)
                    } else {
                        assert_eq!(lost_item.name, b.name);
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
                    let mut combined = if let ItemStatus::Broken { combined } = &a.status {
                        combined.clone()
                    } else {
                        unreachable!()
                    };
                    combined.insert(b.clone());
                    Item::new(a.name.clone(), ItemStatus::Broken { combined })
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
            if let ItemStatus::Broken { .. } = &a.status {
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

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt)]
enum Cmd {
    #[structopt(name = "task03")]
    Task03 {
        #[structopt(subcommand)]
        cmd: Task03,
    },
}

#[derive(StructOpt)]
enum Task03 {
    #[structopt(name = "solve-adventure")]
    SolveAdventure { code: String },
    #[structopt(name = "solve-adventure=2")]
    SolveAdventure2 { code: String },
}

fn solve_adventure(code: String) -> Result<()> {
    // Solve the fist part of "./adventure" game
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
take bolt
take spring
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
            stack_pos: Inventory::item_stack().len() - 1,
        },
        command: None,
    });
    Ok(())
}

fn solve_adventure_2(code: String) -> Result<()> {
    // Solve the 2nd part of "./adventure" game
    let mut f = std::fs::File::open(code)?;
    let mut code = Vec::new();
    f.read_to_end(&mut code)?;
    let mut um = Um::new(code);
    um.set_print_stdin(true);

    // input_02.txt
    let mut input = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input.push("task/task_03/input_02.txt");
    let mut f = std::fs::File::open(input)?;
    let mut input = Vec::new();
    f.read_to_end(&mut input)?;

    let result = um.run(&mut (input.as_ref() as &[u8]), &mut std::io::stdout());
    assert_eq!(result, UmStatus::NoInput);

    // X(fixed) X (fixed) X (text book) X (tall...)
    um.enter_command("go east\n");
    um.enter_command("go east\n");
    um.enter_command("go east\n");

    // Try take forever.

    // TODO: Exten Item to support arbinary adjactive.

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose).unwrap();

    match opt.cmd {
        Cmd::Task03 { cmd } => match cmd {
            Task03::SolveAdventure { code } => solve_adventure(code)?,
            Task03::SolveAdventure2 { code } => solve_adventure_2(code)?,
        },
    }
    Ok(())
}
