use linked_lists::TransactionLog;

fn main() {
    let mut log = TransactionLog::new_empty();
    log.append("a".to_string());
    log.append("b".to_string());

    assert_eq!(2, log.length);

    println!("The lists length is: {}", log.length);
}
