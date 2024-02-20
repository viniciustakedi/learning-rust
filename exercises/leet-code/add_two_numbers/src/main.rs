#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers<ListNode>(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!()
}

fn main() {
    println!("{:?}", add_two_numbers([2, 4, 3], [5, 6, 4]));
}
