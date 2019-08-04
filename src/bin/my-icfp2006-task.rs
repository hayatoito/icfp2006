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

mod task_03 {

    use super::*;

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

        fn name_for_take(&self) -> String {
            if let Some(color) = &self.color {
                format!("{} {}", color, self.name)
            } else {
                self.name.clone()
            }
        }

        fn is_unique(&self) -> bool {
            is_unique_item_name(&self.name)
        }
    }

    fn is_unique_item_name(s: &str) -> bool {
        // non-unique item: "X-9247-GWE"
        lazy_static! {
            static ref NON_UNIQUE_ITEM_NAME_RE: Regex =
                Regex::new(r"[A-Z]-\d{4}-[A-Z]{3}").unwrap();
        }
        !NON_UNIQUE_ITEM_NAME_RE.is_match(s)
    }

    #[test]
    fn is_unique_item_name_test() {
        assert!(!is_unique_item_name("J-5065-IQW"));
        assert!(!is_unique_item_name("R-0010-FLH"));
        assert!(is_unique_item_name("USB cable"));
    }

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

    #[derive(Clone, Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    impl std::fmt::Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Direction::North => "north",
                    Direction::South => "south",
                    Direction::East => "east",
                    Direction::West => "west",
                }
            )
        }
    }

    #[derive(Clone, Debug)]
    enum Command {
        Take(String),
        Incinerate(String),
        Combine(String, String),
        // For part 2
        // Go(Direction),  // Use this later
        TakeAt(Location, String),
    }

    // For part2
    type Location = (usize, usize);

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

        fn append(this: &Rc<CommandList>, other: &Rc<CommandList>) -> Rc<CommandList> {
            let mut current = Rc::clone(this);
            for command in other.to_list() {
                current = Rc::new(CommandList {
                    command,
                    prev: Some(Rc::clone(&current)),
                })
            }
            current
        }
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

        // For part1
        fn take(&self, stack_pos: usize) -> Inventory {
            assert!(stack_pos > 0);
            let mut inventory = self.inventory.clone();
            inventory.insert(Inventory::item_stack()[stack_pos - 1].clone());
            Inventory::new(inventory)
        }

        // For part2
        fn take_item(&self, item: Item) -> Inventory {
            let mut inventory = self.inventory.clone();
            inventory.insert(item);
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

    pub mod part_1 {

        use super::*;

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
                        Inventory::item_stack()[stack_pos - 1].name_for_take(),
                    ))),
                }
            }

            fn combine(&self, a: &Item, b: &Item, new_item: Item) -> Pos {
                Pos {
                    state: State {
                        inventory: self.state.inventory.combine(a, b, new_item),
                        stack_pos: self.state.stack_pos,
                    },
                    command: Some(
                        self.add_command(Command::Combine(a.name.clone(), b.name.clone())),
                    ),
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
                            Command::TakeAt(_, _) => {
                                // We don't need these in Part 1
                                unreachable!();
                            }
                        }
                    }
                }
                let current_inventory = um.inventory();
                // TODO: assert_eq!(current_inventory, expected_inventory);

                // Apply combine
                if um.combine(&a.name_for_take(), &b.name_for_take()) {
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
                            Command::TakeAt(_, _) => unreachable!(),
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
                            if let Some(new_item) =
                                self.try_combine(a.clone(), b.clone(), &pos.command)
                            {
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

        pub fn solve_adventure(code: String) -> Result<()> {
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

    }

    pub mod part_2 {

        use super::*;

        fn go(from: Location, to: Location) -> Vec<Direction> {
            if from == to {
                return vec![];
            }
            if from == (0, 1) {
                let mut d = go((1, 1), to);
                d.insert(0, Direction::East);
                return d;
            }
            if to == (0, 1) {
                let mut d = go(from, (1, 1));
                d.push(Direction::West);
                return d;
            }
            let mut d = if from.0 < to.0 {
                vec![Direction::East; to.0 - from.0]
            } else if from.0 > to.0 {
                vec![Direction::West; from.0 - to.0]
            } else {
                vec![]
            };

            let mut dy = if from.1 < to.1 {
                vec![Direction::North; to.1 - from.1]
            } else if from.1 > to.1 {
                vec![Direction::South; from.1 - to.1]
            } else {
                vec![]
            };

            d.append(&mut dy);
            d
        }

        type ItemStackMap = BTreeMap<Location, Vec<Item>>;

        fn init_item_stack_map(mut um: Um) -> ItemStackMap {
            let mut prev_location = (0, 1);

            let mut item_map = ItemStackMap::new();

            for location in &[
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
                (1, 0),
                (2, 0),
                (3, 0),
                (1, 2),
                (2, 2),
                (3, 2),
                (1, 3),
                (2, 3),
                (3, 3),
            ] {
                for direction in go(prev_location, *location) {
                    um.enter_command(&format!("go {}\n", direction));
                }
                item_map.insert(*location, collect_unique_items(&mut um));
                prev_location = *location;
            }
            item_map
        }

        fn collect_unique_items(um: &mut Um) -> Vec<Item> {
            let mut items = vec![];
            while let Some(item) = um.examine() {
                if item.is_unique() {
                    println!("unique item: {}", item);
                    items.push(item.clone())
                }
                um.take(&item.name_for_take());
                // Try to use item, and confirm nothing happens
                assert!(
                    !um.use_item(&item.name_for_take()),
                    format!("use item: {}", item)
                );
                um.incinerate(&item.name_for_take());
            }
            items.reverse();
            items
        }

        type ItemStackPos = BTreeMap<Location, usize>;

        #[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
        struct State {
            inventory: Inventory,
            // TODO: Make it Rc
            stack_pos: ItemStackPos,
        }

        struct Pos {
            state: State,
            command: Option<Rc<CommandList>>,
            location: Location,
            pending_command: Option<Rc<CommandList>>,
            um: Rc<Um>,
        }

        impl Pos {
            // fn add_command(&self, command: Command) -> Rc<CommandList> {
            //     Rc::new(CommandList {
            //         command,
            //         prev: self.command.clone(),
            //     })
            // }

            fn add_pending_command(&self, command: Command) -> Rc<CommandList> {
                Rc::new(CommandList {
                    command,
                    prev: self.pending_command.clone(),
                })
            }

            fn incinerate(&self, item: &Item) -> Pos {
                Pos {
                    state: State {
                        inventory: self.state.inventory.incinerate(item),
                        stack_pos: self.state.stack_pos.clone(),
                    },
                    command: self.command.clone(),
                    location: self.location,
                    pending_command: Some(
                        self.add_pending_command(Command::Incinerate(item.name_for_take())),
                    ),
                    um: Rc::clone(&self.um),
                }
            }

            fn take(
                &self,
                location: Location,
                stack_pos: usize,
                item_stack_map: &ItemStackMap,
            ) -> Pos {
                let item = item_stack_map[&location][stack_pos - 1].clone();
                Pos {
                    state: State {
                        inventory: self.state.inventory.take_item(item.clone()),
                        stack_pos: {
                            let mut stack_pos = self.state.stack_pos.clone();
                            *stack_pos.get_mut(&location).unwrap() -= 1;
                            stack_pos
                        },
                    },
                    command: self.command.clone(),
                    pending_command: Some(
                        self.add_pending_command(Command::TakeAt(location, item.name_for_take())),
                    ),
                    location,
                    um: Rc::clone(&self.um),
                }
            }

            fn combine(&self, a: &Item, b: &Item, new_item: Item) -> Pos {
                Pos {
                    state: State {
                        inventory: self.state.inventory.combine(a, b, new_item),
                        stack_pos: self.state.stack_pos.clone(),
                    },
                    command: self.command.clone(),
                    location: self.location,
                    pending_command: Some(self.add_pending_command(Command::Combine(
                        a.name_for_take(),
                        b.name_for_take(),
                    ))),
                    um: Rc::clone(&self.um),
                }
            }

            fn combine_with_new_um(
                &self,
                a: &Item,
                b: &Item,
                new_item: Item,
                location: Location,
                um: Rc<Um>,
            ) -> Pos {
                Pos {
                    state: State {
                        inventory: self.state.inventory.combine(a, b, new_item),
                        stack_pos: self.state.stack_pos.clone(),
                    },
                    command: Some(Rc::new(CommandList {
                        command: Command::Combine(a.name_for_take(), b.name_for_take()),
                        prev: match (&self.command, &self.pending_command) {
                            (Some(a), None) => Some(Rc::clone(&a)),
                            (None, Some(b)) => Some(Rc::clone(&b)),
                            (None, None) => None,
                            (Some(a), Some(b)) => Some(CommandList::append(&a, &b)),
                        },
                    })),
                    location,
                    pending_command: None,
                    um,
                }
            }
        }

        struct AdventureSolver {
            visited: BTreeSet<State>,
            combine_rule: CombineRule,
            item_stack_map: ItemStackMap,
        }

        enum TryCombineResult {
            CacheHit(Option<Item>),
            Success(Item, Location),
            Fail,
        }

        impl AdventureSolver {
            fn new(item_stack_map: ItemStackMap) -> AdventureSolver {
                AdventureSolver {
                    visited: Default::default(),
                    combine_rule: Default::default(),
                    item_stack_map,
                }
            }

            fn try_combine(
                &mut self,
                a: Item,
                b: Item,
                // for debug
                pos_inventory: &Inventory,
                pos_command: &Option<Rc<CommandList>>,
                pending_command_list: &Option<Rc<CommandList>>,
                mut current_location: Location,
                um: &mut Rc<Um>,
            ) -> TryCombineResult {
                if let Some(result) = self.combine_rule.get(&(a.clone(), b.clone())) {
                    return TryCombineResult::CacheHit(result.clone());
                }
                info!("try to combine: {:?} and {:?}", a, b);
                // Need to ask um. Execute commands in um.
                assert!(Rc::strong_count(um) > 1);
                let um: &mut Um = Rc::make_mut(um);

                if let Some(command_list) = pending_command_list.as_ref() {
                    for command in command_list.to_list() {
                        debug!("command: {:?}", command);
                        match command {
                            Command::TakeAt(location, item_name) => {
                                // Go to location
                                for direction in go(current_location, location) {
                                    // info!(
                                    //     "output: go: {}",
                                    um.enter_command(&format!("go {}\n", direction));
                                    // );
                                }
                                debug!("examine: {}", um.enter_command("examine\n"));

                                // Find an item
                                loop {
                                    if um.examine().is_none() {
                                        error!("um examine failed");
                                        error!("debug pos_inventory: {:#?}", pos_inventory);
                                        error!("debug pos_command: {:#?}", pos_command);
                                        error!("pending command_list: {:#?}", command_list);
                                        error!("item_name: {}", item_name);
                                        error!("examine: {}", um.enter_command("examine\n"));
                                    }
                                    let item = um.examine().expect("um.examine failed");
                                    um.take(&item.name_for_take());
                                    if item.name_for_take() == item_name {
                                        break;
                                    }
                                    um.incinerate(&item.name_for_take());
                                }
                                current_location = location;
                            }
                            Command::Incinerate(item_name) => {
                                um.incinerate(&item_name);
                            }
                            Command::Combine(a, b) => {
                                assert!(
                                    um.combine(&a, &b),
                                    format!("combine {:?} with {:?}", a, b)
                                );
                            }
                            Command::Take(_) => {
                                unreachable!();
                            }
                        }
                    }
                }
                let current_inventory = um.inventory();
                // TODO: assert_eq!(current_inventory, expected_inventory);

                // Apply combine
                if um.combine(&a.name_for_take(), &b.name_for_take()) {
                    let new_inventory = um.inventory();
                    let lost_item = current_inventory
                        .difference(&new_inventory)
                        .next()
                        .expect("lost_item failed");
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
                    // info!("combine_rule:");
                    // dump_combine_rule(&self.combine_rule);
                    info!("combine_rule: {:#?}", self.combine_rule);
                    TryCombineResult::Success(new_item, current_location)
                } else {
                    self.combine_rule.insert((a.clone(), b.clone()), None);
                    TryCombineResult::Fail
                }
            }

            fn solve(&mut self, pos: Pos) -> bool {
                // debug!("pos: {:?}", pos);
                if self.visited.contains(&pos.state) {
                    return false;
                }
                self.visited.insert(pos.state.clone());

                // Try all posibble actions

                // Combine items
                for a in &pos.state.inventory.inventory {
                    if a.broken.is_some() {
                        for b in &pos.state.inventory.inventory {
                            if a == b {
                                continue;
                            }
                            let mut um = Rc::clone(&pos.um);
                            match self.try_combine(
                                a.clone(),
                                b.clone(),
                                &pos.state.inventory,
                                &pos.command,
                                &pos.pending_command,
                                pos.location,
                                &mut um,
                            ) {
                                TryCombineResult::CacheHit(Some(item)) => {
                                    let pos = pos.combine(&a, &b, item);
                                    if self.solve(pos) {
                                        return true;
                                    }
                                }
                                TryCombineResult::Success(item, location) => {
                                    let pos = pos.combine_with_new_um(&a, &b, item, location, um);
                                    if self.solve(pos) {
                                        return true;
                                    }
                                }
                                TryCombineResult::CacheHit(None) | TryCombineResult::Fail => {}
                            }
                        }
                    }
                }

                // take an item
                if pos.state.inventory.inventory.len() < 6 {
                    for (location, stack_pos) in &pos.state.stack_pos {
                        if *stack_pos == 0 {
                            continue;
                        }
                        if self.solve(pos.take(*location, *stack_pos, &self.item_stack_map)) {
                            return true;
                        }
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

        pub fn solve_adventure(code: String) -> Result<()> {
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

            let item_stack_map = init_item_stack_map(um.clone());
            println!("item_stack_map: {:#?}", item_stack_map);

            let inventory = um.inventory();

            let stack_pos = item_stack_map
                .iter()
                .map(|(&location, stack)| (location, stack.len()))
                .collect();

            let mut solver = AdventureSolver::new(item_stack_map);
            solver.solve(Pos {
                state: State {
                    inventory: Inventory { inventory },
                    stack_pos,
                },
                location: (0, 1),
                command: None,
                pending_command: None,
                um: Rc::new(um),
            });
            Ok(())
        }

        fn _unused() {
            let _output = r#"

/*
>: use keypad
ADVTR.KEY=20@999999|36995486a5be3bd747d778916846d2d
You unlock and open the door. Passing through, you find yourself
on the streets of Chicago. Seeing no reason you should ever go
back, you allow the door to close behind you.

>: examine
examine
54th Street and Ridgewood Court

You are standing at the corner of 54th Street and Ridgewood
Court. From here, you can go east.
There is a /etc/passwd here.
Underneath the /etc/passwd, there is a self-addressed note.
Underneath the note, there is a (broken) downloader.
Underneath the downloader, there is a (broken) uploader.


>: go east
go east
54th Street and Dorchester Avenue

You are standing at the corner of 54th Street and Dorchester
Avenue. From here, you can go north, east, south, or west.
There is an orange-red X-9247-GWE here.
Underneath the X-9247-GWE, there is a (broken) magenta
V-0010-XBD.
Underneath the V-0010-XBD, there is a pumpkin F-1403-QDS.
Underneath the F-1403-QDS, there is a (broken) heavy P-5065-WQO.

Underneath the P-5065-WQO, there is a taupe B-4832-LAL.
Underneath the B-4832-LAL, there is a (broken) gray40
L-6458-RNH.
Underneath the L-6458-RNH, there is a (broken) eggplant
T-9887-OFC.
Underneath the T-9887-OFC, there is a (broken) indigo
Z-1623-CEK.
Underneath the Z-1623-CEK, there is a yellow-green H-9887-MKY.
Underneath the H-9887-MKY, there is a (broken) shiny F-6678-DOX.

Underneath the F-6678-DOX, there is a pale-green R-1403-SXU.
Underneath the R-1403-SXU, there is a (broken) USB cable.
Underneath the USB cable, there is a sienna N-4832-NUN.
Underneath the N-4832-NUN, there is a slate-gray J-9247-IRG.
Underneath the J-9247-IRG, there is a dim-gray B-5065-YLQ.

>:

//   -
// - X -
//   -

: go east
go east
54th Street and Blackstone Avenue

You are standing at the corner of 54th Street and Blackstone
Avenue. From here, you can go north, east, south, or west.
There is a textbook here.

//   - -
// - - X -
//   - -

>: go east
go east
54th Street and Harper Avenue

You are standing at the corner of 54th Street and Harper Avenue.
A sign reads, "No access east of Lakeshore Blvd (incl. Museum of
Science and Industry) due to construction." From here, you can
go north, south, or west.
There is a (broken) gray60 Z-4292-PRV here.
Underneath the Z-4292-PRV, there is a (broken) pale-carmine
D-9887-UUE.
Underneath the D-9887-UUE, there is a pale-cornflower-blue
D-9887-UUE.
Underneath the D-9887-UUE, there is an imported D-9887-UUE.
Underneath the D-9887-UUE, there is a (broken) gray90
Z-4292-PRV.
Underneath the Z-4292-PRV, there is a sepia Z-4292-PRV.
Underneath the Z-4292-PRV, there is a (broken) gray30
D-9887-UUE.
Underneath the D-9887-UUE, there is a robin-egg-blue Z-4292-PRV.

Underneath the Z-4292-PRV, there is a (broken) slate-blue
D-9887-UUE.
Underneath the D-9887-UUE, there is a (broken) rosy-brown
D-9887-UUE.
Underneath the D-9887-UUE, there is a gray50 Z-4292-PRV.
Underneath the Z-4292-PRV, there is a (broken) misty-rose
Z-4292-PRV.
Underneath the Z-4292-PRV, there is a cinnamon Z-9887-CUG.
Underneath the Z-9887-CUG, there is a (broken) saffron
N-9247-AWO.
Underneath the N-9247-AWO, there is a (broken) indigo
V-4292-XRX.
Underneath the V-4292-XRX, there is a pale-sandy-brown
R-4832-FAV.
Underneath the R-4832-FAV, there is a (broken)
momemtum-preserving Z-0010-PGL.
Underneath the Z-0010-PGL, there is a swamp-green V-4292-XRX.
Underneath the V-4292-XRX, there is a pale-pink R-1623-SOQ.
Underneath the R-1623-SOQ, there is a cream R-1623-SOQ.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.


//   - - -
// - - - X
//   - - -


>: go south
go south
54th Place and Harper Avenue

You are standing at the corner of 54th Place and Harper Avenue.
A sign reads, "No access east of Lakeshore Blvd (incl. Museum of
Science and Industry) due to construction." From here, you can
go north or west.
There is a tea-green T-9887-OKW here.
Underneath the T-9887-OKW, there is a (broken) chartreuse
N-5065-ALO.
Underneath the N-5065-ALO, there is a (broken) dim-gray
X-6458-TNF.
Underneath the X-6458-TNF, there is a (broken) scarlet
J-9247-IWC.
Underneath the J-9247-IWC, there is a (broken) mustard
D-4292-HMN.
Underneath the D-4292-HMN, there is a (broken) turquoise
N-4832-NAJ.
Underneath the N-4832-NAJ, there is a turquoise F-6678-DTT.
Underneath the F-6678-DTT, there is a (broken) celadon
T-9887-OKW.
Underneath the T-9887-OKW, there is a burnt-orange T-9887-OKW.
Underneath the T-9887-OKW, there is a cadet-blue T-9887-OKW.
Underneath the T-9887-OKW, there is a pastel-green F-6678-DTT.
Underneath the F-6678-DTT, there is a (broken) prussian-blue
X-6458-TNF.
Underneath the X-6458-TNF, there is a (broken) magenta
X-6458-TNF.
Underneath the X-6458-TNF, there is a (broken) safety-orange
F-6678-DTT.
Underneath the F-6678-DTT, there is an imported B-5065-YQM.
Underneath the B-5065-YQM, there is a (broken) puce X-6458-TNF.
Underneath the X-6458-TNF, there is a (broken) water-logged
F-6678-DTT.
Underneath the F-6678-DTT, there is a (broken) thistle-colored
X-6458-TNF.
Underneath the X-6458-TNF, there is a gamboge T-9887-OKW.
Underneath the T-9887-OKW, there is a sky-blue T-9887-OKW.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.


//   - - -
// - - - -
//   - - X

>: go west
go west
54th Place and Blackstone Avenue

You are standing at the corner of 54th Place and Blackstone
Avenue. From here, you can go north, east, or west.
There is a (broken) blaze-orange V-1623-KTK here.
Underneath the V-1623-KTK, there is a (broken) dodger-blue
V-1623-KTK.
Underneath the V-1623-KTK, there is a taupe X-6458-TSZ.
Underneath the X-6458-TSZ, there is a (broken) pale-carmine
V-1623-KTK.
Underneath the V-1623-KTK, there is a (broken) carrot
T-9887-OPS.
Underneath the T-9887-OPS, there is a pale-violet-red
T-9887-OPS.
Underneath the T-9887-OPS, there is a mint-green R-9887-SFW.
Underneath the R-9887-SFW, there is a taupe V-9247-KWY.
Underneath the V-9247-KWY, there is a (broken) dim-gray
Z-4832-PAH.
Underneath the Z-4832-PAH, there is a (broken) old-lace
P-1403-WXQ.
Underneath the P-1403-WXQ, there is a (broken) peach-yellow
R-6678-FTR.
Underneath the R-6678-FTR, there is a (broken) selective-yellow
T-0010-BBX.
Underneath the T-0010-BBX, there is a (broken) vegan D-6678-HOT.

Underneath the D-6678-HOT, there is a (broken) hot-pink
H-9247-MRC.
Underneath the H-9247-MRC, there is a (broken) mysterious
L-6678-ROF.
Underneath the L-6678-ROF, there is a peach R-9887-SFW.
Underneath the R-9887-SFW, there is a (broken) old-gold
V-6458-XIF.
Underneath the V-6458-XIF, there is a (broken) ivory N-4292-NCP.

Underneath the N-4292-NCP, there is a rust R-9887-SFW.
Underneath the R-9887-SFW, there is a (broken) slate-gray
P-6678-JJV.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.

//   - - -
// - - - -
//   - X -

>: go west
go west
54th Place and Dorchester Avenue

You are standing at the corner of 54th Place and Dorchester
Avenue. From here, you can go north or east.
There is a (broken) terra-cotta T-9887-OAU here.
Underneath the T-9887-OAU, there is a carrot L-1623-ETG.
Underneath the L-1623-ETG, there is a (broken) heliotrope
B-1403-YDU.
Underneath the B-1403-YDU, there is a (broken) imported
H-9247-MCE.
Underneath the H-9247-MCE, there is a pear R-0010-FBF.
Underneath the R-0010-FBF, there is a (broken) pale-turquoise
Z-4832-PKJ.
Underneath the Z-4832-PKJ, there is a (broken) sandy-brown
T-0010-BLZ.
Underneath the T-0010-BLZ, there is a (broken) miniature
R-6678-FET.
Underneath the R-6678-FET, there is a (broken) linen-colored
H-9247-MCE.
Underneath the H-9247-MCE, there is a (broken) snow L-4832-RFL.
Underneath the L-4832-RFL, there is a (broken) lilac H-0010-ZQX.

Underneath the H-0010-ZQX, there is a (broken) gray30
X-1623-GOI.
Underneath the X-1623-GOI, there is a (broken) silver
L-4832-RFL.
Underneath the L-4832-RFL, there is a (broken) bisque
V-1623-KEM.
Underneath the V-1623-KEM, there is a (broken) gray40
N-4013-DJW.
Underneath the N-4013-DJW, there is a (broken) pale-red-violet
T-9247-OWG.
Underneath the T-9247-OWG, there is a (broken) peach F-0010-DGD.

Underneath the F-0010-DGD, there is a pine-green H-6458-ZNJ.
Underneath the H-6458-ZNJ, there is a persian-blue X-4832-TAN.
Underneath the X-4832-TAN, there is a (broken) pink P-4292-JWN.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.

//   - - -
// - - - -
//   X - -

>: go north
go north
54th Street and Dorchester Avenue

You are standing at the corner of 54th Street and Dorchester
Avenue. From here, you can go north, east, south, or west.
There is an orange-red X-9247-GWE here.
Underneath the X-9247-GWE, there is a (broken) magenta
V-0010-XBD.
Underneath the V-0010-XBD, there is a pumpkin F-1403-QDS.
Underneath the F-1403-QDS, there is a (broken) heavy P-5065-WQO.

Underneath the P-5065-WQO, there is a taupe B-4832-LAL.
Underneath the B-4832-LAL, there is a (broken) gray40
L-6458-RNH.
Underneath the L-6458-RNH, there is a (broken) eggplant
T-9887-OFC.
Underneath the T-9887-OFC, there is a (broken) indigo
Z-1623-CEK.
Underneath the Z-1623-CEK, there is a yellow-green H-9887-MKY.
Underneath the H-9887-MKY, there is a (broken) shiny F-6678-DOX.

Underneath the F-6678-DOX, there is a pale-green R-1403-SXU.
Underneath the R-1403-SXU, there is a (broken) USB cable.
Underneath the USB cable, there is a sienna N-4832-NUN.
Underneath the N-4832-NUN, there is a slate-gray J-9247-IRG.
Underneath the J-9247-IRG, there is a dim-gray B-5065-YLQ.


//   - - -
// - X - -
//   - - -

>: go north
go north
53th Street and Dorchester Avenue

You are standing at the corner of 53th Street and Dorchester
Avenue. From here, you can go north, east, or south.
There is a fern-green N-1623-AOE here.
Underneath the N-1623-AOE, there is a (broken) burgundy
R-4292-FRL.
Underneath the R-4292-FRL, there is a (broken) pale-magenta
F-6458-DDN.
Underneath the F-6458-DDN, there is a (broken) peach-yellow
H-1623-MYO.
Underneath the H-1623-MYO, there is a (broken) rotating
H-4292-ZHF.
Underneath the H-4292-ZHF, there is a (broken) low-carb
R-6458-FXP.
Underneath the R-6458-FXP, there is a (broken) mysterious
T-6458-BIL.
Underneath the T-6458-BIL, there is a (broken) brass R-9247-SMK.

Underneath the R-9247-SMK, there is a (broken) puce Z-1403-CSY.
Underneath the Z-1403-CSY, there is a (broken) pink N-6678-NJD.
Underneath the N-6678-NJD, there is a (broken) jade X-4292-TWX.
Underneath the X-4292-TWX, there is a (broken) flax Z-6678-PEF.
Underneath the Z-6678-PEF, there is a (broken) pale-blue
H-4832-ZKT.
Underneath the H-4832-ZKT, there is a (broken) gray60
P-0010-JQJ.
Underneath the P-0010-JQJ, there is a (broken) olive-green
J-1403-IDG.
Underneath the J-1403-IDG, there is a swamp-green D-9247-UHM.
Underneath the D-9247-UHM, there is a (broken) khaki N-6458-NDX.

Underneath the N-6458-NDX, there is a (broken) red-violet
V-9887-KUS.
Underneath the V-9887-KUS, there is a (broken) tea-green
N-0010-NGN.
Underneath the N-0010-NGN, there is a (broken) cinnamon
X-1403-GIE.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.

     -
//   X - -
// - - - -
//   - - -


>: go east
go east
53th Street and Blackstone Avenue

You are standing at the corner of 53th Street and Blackstone
Avenue. From here, you can go north, east, south, or west.
There is a (broken) battery here.
Underneath the battery, there is a buff F-6678-DYP.
Underneath the F-6678-DYP, there is a (broken) olive-green
R-4292-FWH.
Underneath the R-4292-FWH, there is a (broken) green D-4832-HUX.

Underneath the D-4832-HUX, there is a (broken) prussian-blue
P-1623-WEU.
Underneath the P-1623-WEU, there is a (broken) black X-9887-GKK.

Underneath the X-9887-GKK, there is a reciprocating H-9887-MUQ.
Underneath the H-9887-MUQ, there is a bright-violet T-4292-BHD.
Underneath the T-4292-BHD, there is a (broken) flax D-4292-HRJ.
Underneath the D-4292-HRJ, there is a (broken) yellow
Z-1623-COC.
Underneath the Z-1623-COC, there is a (broken) goldenrod
X-0010-TVP.
Underneath the X-0010-TVP, there is a forest-green F-6678-DYP.
Underneath the F-6678-DYP, there is a sky-blue N-1623-ATY.
Underneath the N-1623-ATY, there is a (broken) rotating
H-6678-ZJL.
Underneath the H-6678-ZJL, there is a (broken) sienna
T-6678-BEN.
Underneath the T-6678-BEN, there is a heliotrope D-5065-UGE.
Underneath the D-5065-UGE, there is a (broken) sea-green
N-1623-ATY.
Underneath the N-1623-ATY, there is a (broken) ochre D-4292-HRJ.

Underneath the D-4292-HRJ, there is a (broken) cream Z-1623-COC.

Underneath the Z-1623-COC, there is a (broken) powder-blue
T-6678-BEN.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.

     - -
//   - X -
// - - - -
//   - - -

>: go east
go east
53th Street and Harper Avenue

You are standing at the corner of 53th Street and Harper Avenue.
A sign reads, "No access east of Lakeshore Blvd (incl. Museum of
Science and Industry) due to construction." From here, you can
go north, south, or west.
There is a (broken) old X-1623-GTO here.
Underneath the X-1623-GTO, there is a (broken) corn V-0010-XBD.
Underneath the V-0010-XBD, there is a (broken) blue-violet
P-4832-JFJ.
Underneath the P-4832-JFJ, there is a (broken) gamboge
X-0010-TLX.
Underneath the X-0010-TLX, there is a (broken) denim D-4292-HHR.

Underneath the D-4292-HHR, there is a (broken) low-carb
D-5065-UVM.
Underneath the D-5065-UVM, there is a (broken) pear V-9247-KMI.
Underneath the V-9247-KMI, there is a (broken) ultramarine
J-9247-IRG.
Underneath the J-9247-IRG, there is a (broken) pink D-4292-HHR.
Underneath the D-4292-HHR, there is a (broken) discounted
L-6458-RNH.
Underneath the L-6458-RNH, there is a (broken) flax D-4292-HHR.
Underneath the D-4292-HHR, there is a (broken) cinnamon
J-9247-IRG.
Underneath the J-9247-IRG, there is a lemon F-9887-QAE.
Underneath the F-9887-QAE, there is a (broken) low-carb
X-1623-GTO.
Underneath the X-1623-GTO, there is a (broken) amethyst
J-9247-IRG.
Underneath the J-9247-IRG, there is a (broken) reverse-chirality
J-9247-IRG.
Underneath the J-9247-IRG, there is a (broken) lemon T-9887-OFC.

Underneath the T-9887-OFC, there is a (broken) honeydew
B-4292-LWV.
Underneath the B-4292-LWV, there is a (broken) white F-9887-QAE.

Underneath the F-9887-QAE, there is a (broken) black B-4292-LWV.

There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.


     - - -
//   - - X
// - - - -
//   - - -


>: go north
go north
52nd Street and Harper Avenue

You are standing at the corner of 52nd Street and Harper Avenue.
A sign reads, "No access east of Lakeshore Blvd (incl. Museum of
Science and Industry) due to construction." From here, you can
go south or west.
There is a (broken) moccasin N-4292-NWT here.
Underneath the N-4292-NWT, there is a gray-tea-green L-4832-RPN.

Underneath the L-4832-RPN, there is a (broken) pale-chestnut
T-9247-OHI.
Underneath the T-9247-OHI, there is a pale-cornflower-blue
V-6458-XDJ.
Underneath the V-6458-XDJ, there is a (broken) misty-rose
T-9247-OHI.
Underneath the T-9247-OHI, there is a (broken) sangria
N-4292-NWT.
Underneath the N-4292-NWT, there is a (broken) jade P-1403-WSU.
Underneath the P-1403-WSU, there is a (broken) yellow-green
P-6678-JEZ.
Underneath the P-6678-JEZ, there is a (broken) crimson
P-6678-JEZ.
Underneath the P-6678-JEZ, there is a (broken) scarlet
Z-5065-CGQ.
Underneath the Z-5065-CGQ, there is a (broken) organic
T-9247-OHI.
Underneath the T-9247-OHI, there is a bright-turquoise
J-1623-ITM.
Underneath the J-1623-ITM, there is a (broken) mint-green
X-4832-TKP.
Underneath the X-4832-TKP, there is a (broken) denim T-9247-OHI.

Underneath the T-9247-OHI, there is a (broken) gray30
X-4832-TKP.
Underneath the X-4832-TKP, there is a gray60 T-9247-OHI.
Underneath the T-9247-OHI, there is a (broken) azure H-6458-ZXL.

Underneath the H-6458-ZXL, there is a carmine D-6678-HJX.
Underneath the D-6678-HJX, there is a cornflower-blue
D-6678-HJX.
Underneath the D-6678-HJX, there is a (broken) tan D-6678-HJX.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.


     - - X
//   - - -
// - - - -
//   - - -


>: go west
go west
52nd Street and Blackstone Avenue

You are standing at the corner of 52nd Street and Blackstone
Avenue. From here, you can go east, south, or west.
There is a manual here.

     - X -
//   - - -
// - - - -
//   - - -

>: go west
go west
52nd Street and Dorchester Avenue

You are standing at the corner of 52nd Street and Dorchester
Avenue. From here, you can go east or south.
There is a (broken) cerise V-5065-KLY here.
Underneath the V-5065-KLY, there is a foreign J-5065-IQW.
Underneath the J-5065-IQW, there is a floating X-5065-GVU.
Underneath the X-5065-GVU, there is a chartreuse B-6678-LYD.
Underneath the B-6678-LYD, there is a lavender N-1403-AIY.
Underneath the N-1403-AIY, there is a bondi-blue R-0010-FLH.
Underneath the R-0010-FLH, there is an old V-1623-KOO.
Underneath the V-1623-KOO, there is a (broken) deep-sky-blue
Z-1403-CDC.
Underneath the Z-1403-CDC, there is a hot-pink B-9887-YKI.
Underneath the B-9887-YKI, there is a (broken) ultramarine
P-9887-WPG.
Underneath the P-9887-WPG, there is a (broken) fern-green
F-9247-QCK.
Underneath the F-9247-QCK, there is a (broken) lime-green
B-9247-YHS.
Underneath the B-9247-YHS, there is a (broken) reverse-chirality
J-1403-INI.
Underneath the J-1403-INI, there is a school-bus-yellow
L-4292-RMX.
Underneath the L-4292-RMX, there is a (broken) yellow-green
R-6458-FIR.
Underneath the R-6458-FIR, there is a (broken) puce R-9247-SWM.
Underneath the R-9247-SWM, there is a carrot V-4832-XAT.
Underneath the V-4832-XAT, there is a (broken) discounted
H-4832-ZUV.
Underneath the H-4832-ZUV, there is a hot-pink N-6678-NTF.
Underneath the N-6678-NTF, there is a (broken) exceptional
L-6678-RJJ.
There are more items beneath it, but the pile is so tall you
feel a little dizzy.
Try switching your goggles.

     X - -
//   - - -
// - - - -
//   - - -


>:
"#;
        }

    }
}

#[derive(StructOpt)]
struct Cli {
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
    #[structopt(name = "solve-adventure-2")]
    SolveAdventure2 { code: String },
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    loggerv::init_with_verbosity(args.verbose).unwrap();

    match args.cmd {
        Cmd::Task03 { cmd } => match cmd {
            Task03::SolveAdventure { code } => task_03::part_1::solve_adventure(code)?,
            Task03::SolveAdventure2 { code } => task_03::part_2::solve_adventure(code)?,
        },
    }
    Ok(())
}
