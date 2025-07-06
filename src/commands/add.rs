pub fn run(description: String, priority: Option<u8>, due: Option<String>, tags: Option<Vec<String>>) {
    println!("Adding item");
    println!("  Description: {}", description);

    if let Some(p) = priority {
        println!("  Priority: {}", p);
    }

    if let Some(d) = due {
        println!("  Due: {}", d);
    }

    if let Some(t) = tags {
        println!("  Tags: {:?}", t);
    }
}
