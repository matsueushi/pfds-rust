use std::collections::LinkedList;

/// Returns a list of suffixes sorted in descending order of length
fn suffixes<T: Clone>(lst: &LinkedList<T>) -> LinkedList<LinkedList<T>> {
    let mut suf_lst = LinkedList::new();
    let mut rest_lst = lst.clone();
    loop {
        suf_lst.push_back(rest_lst.clone());
        if rest_lst.pop_front().is_none() {
            break;
        }
    }
    suf_lst
}

#[test]
fn suffixes_test() {
    let mut lst = LinkedList::new();
    lst.push_back(1);
    lst.push_back(2);
    lst.push_back(3);
    lst.push_back(4);
    println!("list: {:?}", lst);
    println!("suffixes(&lst): {:?}", suffixes(&lst));
}
