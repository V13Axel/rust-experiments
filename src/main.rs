use std::{cell::RefCell, io::{self, stdin}, fmt::Display};
use cli_table::{TableDisplay, Table};

#[derive(Debug)]
struct PlacementGrid {
    spaces: Vec<usize>,
    width: usize,
    height: usize,
}

impl Display for PlacementGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: Vec<Vec<usize>> = vec![];
        for row in 0..(self.height) {
            output.push(self.spaces[(row * self.width)..((row*self.width) + self.width)].to_vec());
        }
        write!(f, "{}", output.table().display().unwrap())
    }
}

impl PlacementGrid {
    pub fn of_size(width: usize, height: usize) -> Self {
        Self {
            spaces: vec![0; width * height],
            width,
            height
        }
    }
}

impl Default for PlacementGrid {
    fn default() -> Self {
        Self {
            spaces: vec![0; 8 * 8],
            width: 8,
            height: 8,
        }
    }
}

fn main() {
    let mut spaces = PlacementGrid::of_size(8, 8);
    // let table = spaces.spaces.table().display();

    let mut count = 0;

    for i in spaces.spaces.iter_mut() {
        *i = count;
        count += 1;
    }

    dbg!(&spaces.spaces.len());

    println!("{}", spaces);
}

#[test]
fn test_range_looping() {
    let loop_limit_1 = 5;
    let loop_limit_2 = 0;

    let mut loop_results_1 = vec![];
    let mut loop_results_2 = vec![];

    for looped in 1..loop_limit_1 {
        loop_results_1.push(looped);
    }

    for looped in 1..loop_limit_2 {
        loop_results_2.push(looped);
    }

    assert_eq!(loop_results_1, vec![1, 2, 3, 4]);
    assert_eq!(loop_results_2, vec![]);
}

#[test]
fn test_enum_comparison() {
    #[derive(PartialEq)]
    #[allow(dead_code)]
    enum Task {
        Harvest,
        Upgrade,
        Build
    }

    let option1 = Task::Harvest;
    let option2 = Task::Upgrade;

    assert!(option1 == Task::Harvest);
    assert!(option2 == Task::Upgrade);
}

#[test]
fn test_enum_impl() {
    #[derive(Debug)]
    enum Task {
        Harvest(u32),
        Upgrade(u32),
        Build(u32)
    }

    impl Task {
        fn run(&self) -> String {
            format!("{:?}", self)
        }
    }

    let harvest_one = Task::Harvest(1);
    let harvest_two = Task::Harvest(35);
    let upgrade_one = Task::Upgrade(4);
    let upgrade_two = Task::Upgrade(50);
    let build_one = Task::Build(120);

    assert_eq!(harvest_one.run(), "Harvest(1)");
    assert_eq!(harvest_two.run(), "Harvest(35)");
    assert_eq!(upgrade_one.run(), "Upgrade(4)");
    assert_eq!(upgrade_two.run(), "Upgrade(50)");
    assert_eq!(build_one.run(), "Build(120)");
}

#[test]
fn test_pseudo_inheritance() {
    // Step 1: Declare a struct that accepts a trait as a type
    struct Minion<Type: MinionType> {
        minion_type: Type,
        task: String,
    }

    // Step 2: Declare your trait
    trait MinionType {
        fn run_actual(&self) -> String;
    }

    // Step 3: Declare an implementation of that trait, for the parent struct that does the actual
    // job of running the "actual" method from your trait
    impl<T: MinionType> Minion<T> {
        fn run(&self) -> String {
            println!("Performing task {}", self.task);

            self.minion_type.run_actual()
        }
    }

    // Step 4: Declare your "child" structs
    struct Harvester;
    struct Builder;

    // Step 5: Implement the underlying method for each of the child structs
    impl MinionType for Harvester {
        fn run_actual(&self) -> String {
            String::from("Running harvester")
        }
    }

    impl MinionType for Builder {
        fn run_actual(&self) -> String {
            String::from("Running builder")
        }
    }

    // If the above worked ... You're golden!
    let harvester = Minion {
        minion_type: Harvester,
        task: "Does harvesting".to_string()
    };

    let builder = Minion {
        minion_type: Builder,
        task: "Does building".to_string()
    };

    assert_eq!(harvester.run(), "Running harvester".to_string());
    assert_eq!(builder.run(), "Running builder".to_string());
}

thread_local! {
    static SOME_REF_CELL: RefCell<Vec<String>> = RefCell::new(vec![]);
}


// The experiment/assertion:
// Ranges go _up to_ the higher value, not including it: 
// https://doc.rust-lang.org/rust-by-example/flow_control/for.html#:~:text=This%20yields%20values%20from%20a%20(inclusive)%20to%20b%20(exclusive)%20in%20steps%20of%20one.
#[test]
fn test_range_relative_from_variables() {
    let x = 3;
    let y = 7;
    let mut xvalues = [0; 3];
    let mut yvalues = [0; 9];
    let mut xindex = 0;
    let mut yindex = 0;

    for xpos in (x-1)..(x+2) {
        xvalues[xindex] = xpos;
        for ypos in (y-1)..(y+2) {
            yvalues[yindex] = ypos;

            yindex += 1;
        }
        xindex += 1;
    }

    assert_eq!(xvalues, [2,3,4]);
    assert_eq!(yvalues, [6,7,8,6,7,8,6,7,8]);
}


#[test]
fn test_refcell_threadlocal_replacement_thoughts() {
    SOME_REF_CELL.with(|some_ref_cell| {
        let mut borrowed_cell = some_ref_cell.borrow_mut();

        borrowed_cell.push("Testing".into());

        let mut testing_similar: Vec<String> = Vec::new();
        testing_similar.push(String::from("Testing"));

        assert_eq!(borrowed_cell.len(), testing_similar.len());

        borrowed_cell.iter().enumerate().for_each(|(i, item)| {
            assert_eq!(item.to_owned(), testing_similar[i]);
        });

        drop(borrowed_cell);

        some_ref_cell.replace(testing_similar);
    });
}

#[test]
fn test_refcell_accesses() {
    SOME_REF_CELL.with(|some_ref_cell| {
        let owned = some_ref_cell.to_owned();
        dbg!(owned);
    });
}
