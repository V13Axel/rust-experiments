use std::cell::RefCell;

struct Minion<Type: MinionType> {
    minion_type: Type,
    task: String,
}

trait MinionType {
    fn run_actual(&self);
}

impl<T: MinionType> Minion<T> {
    fn run(&self) {
        println!("Performing task {}", self.task);
        self.minion_type.run_actual();
    }
}

struct Harvester;

impl MinionType for Harvester {
    fn run_actual(&self) {
        println!("Harvesting");
    }
}

struct Builder;

impl MinionType for Builder {
    fn run_actual(&self) {
        println!("Building");
    }
}

fn main() {
    let harvester = Minion {
        minion_type: Harvester,
        task: "Does harvesting".to_string()
    };

    let builder = Minion {
        minion_type: Builder,
        task: "Does building".to_string()
    };

    harvester.run();
    builder.run();
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
