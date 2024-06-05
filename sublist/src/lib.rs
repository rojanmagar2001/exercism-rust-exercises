#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    } else if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }
    if first_list.len() > second_list.len() {
        return false;
    }
    second_list
        .windows(first_list.len())
        .any(|window| window == first_list)
}
