use std::cell::RefCell;

fn main() {
    println!("Hello, world!");
}

thread_local! {
    static SOME_REF_CELL: RefCell<Vec<String>> = RefCell::new(vec![]);
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
