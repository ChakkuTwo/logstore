mod error;
mod models;
mod storage;

use error::StoreError;
use models::{Entry, Priority};
use storage::LogStore;

fn run() -> Result<(), StoreError> {
    let db_path = "entries.db";
    let mut store = LogStore::new(db_path);

    // 1. Construct entries with validation
    let e1 = Entry::new(
        1,
        "Configure native Rust on Monterey",
        vec!["rust".into(), "macos".into()],
        Priority::Medium,
    )?;
    let e2 = Entry::new(
        2,
        "Practice ownership and borrow checker",
        vec!["rust".into(), "learning".into()],
        Priority::Low,
    )?;

    // 2. Ownership transfer (move): `e1` and `e2` move into `store`
    store.add(e1);
    store.add(e2);
    // Uncommenting the next line triggers a compile error (`borrow of moved value: e1`):
    // println!("{e1}");

    // 3. Mutable borrow: escalate priority in place
    let updated = store.escalate_entry(2)?;
    println!("Escalated: {updated}");

    // 4. Save to disk and reload into a separate LogStore instance
    store.save()?;
    let reloaded = LogStore::load(db_path)?;

    // 5. Shared borrow with lifetime: zero-copy filtered view
    println!("\nEntries tagged 'rust':");
    for entry in reloaded.find_by_tag("rust") {
        println!("  {entry}");
    }

    // 6. Handle recoverable domain errors explicitly
    if let Err(err) = Entry::new(3, "   ", vec![], Priority::Low) {
        println!("\nHandled validation error -> {err}");
    }
    if let Err(err) = store.escalate_entry(99) {
        println!("Handled lookup error     -> {err}");
    }

    // 7. Consume `reloaded` by value (`self` is moved out)
    let all_entries = reloaded.into_entries();
    println!("\nTotal owned entries extracted: {}", all_entries.len());

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Application failed: {err}");
        std::process::exit(1);
    }
}