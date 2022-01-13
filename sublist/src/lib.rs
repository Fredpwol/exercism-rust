#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut count = 0;
    if _first_list.len() == 0{
        if _second_list.len()
    }
    while count < _second_list.len() {
        if _first_list[0] == _second_list[count] {
            let mut inner_count = 1;
            while inner_count < _first_list.len() {
                if _first_list[inner_count] != _second_list[inner_count + count] {
                    break;
                }
                inner_count += 1;
            }
            if inner_count == _first_list.len() {
                if _first_list.len() == _second_list.len() {
                    return Comparison::Equal;
                }
                return Comparison::Sublist;
            }
            count += inner_count;
        }
        count += 1;
    }
    let mut second_count = 0;
    while second_count < _first_list.len() {
        if _second_list[0] == _first_list[second_count] {
            let mut inner_count = 1;
            while inner_count < _second_list.len() {
                if _second_list[inner_count] != _first_list[inner_count + second_count] {
                    break;
                }
                inner_count += 1;
            }
            if inner_count == _first_list.len() {
                return Comparison::Superlist;
            }
            second_count += inner_count;
        }
        second_count += 1;
    }

    return Comparison::Unequal;
}
