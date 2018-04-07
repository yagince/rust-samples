#[derive(Debug)]
struct Item {
    value: String,
}
fn main() {
    let ary = [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "10",
        "11",
        "12",
        "13",
        "14",
        "15",
        "16",
    ];

    let items = ary.into_iter().map(|x| Item{value: x.to_string()}).collect::<Vec<_>>();
    let items_ref = items.iter().collect();

    println!("{:?}", items);

    let ret = filter_less_than_2char(&items_ref);
    println!("{:?}", ret);
    let ret = filter_less_than_2char(&items_ref);
    println!("{:?}", ret);
}

fn filter_less_than_2char<'a>(items: &Vec<&'a Item>) -> Vec<&'a Item> {
    items.into_iter().filter(|x| x.value.len() < 2).map(|x| *x).collect()
}
