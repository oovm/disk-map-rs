use nyar_collection::NyarTuple;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_range1() {
    let all = NyarTuple::<i32>::from_iter(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("0..9[1: 1] = {:?}", all.get_range(1, 1, 1).collect::<Vec<_>>());
    println!("0..9[1:-1] = {:?}", all.get_range(1, -1, 1).collect::<Vec<_>>());
    println!("0..9[1: 2] = {:?}", all.get_range(1, 2, 1).collect::<Vec<_>>());
    println!("0..9[1:-2] = {:?}", all.get_range(1, -2, 1).collect::<Vec<_>>());
    println!("0..9[1: 3] = {:?}", all.get_range(1, 3, 1).collect::<Vec<_>>());
    println!("0..9[1:-3] = {:?}", all.get_range(1, -3, 1).collect::<Vec<_>>());
}

#[test]
fn test_range3() {
    let all = NyarTuple::<i32>::from_iter(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("0..9[:: 1] = {:?}", all.get_range(1, -1, 1).collect::<Vec<_>>());
    println!("0..9[::-1] = {:?}", all.get_range(1, -1, -1).collect::<Vec<_>>());
    println!("0..9[:: 2] = {:?}", all.get_range(1, -1, 2).collect::<Vec<_>>());
    println!("0..9[::-2] = {:?}", all.get_range(1, -1, -2).collect::<Vec<_>>());
    println!("0..9[:: 3] = {:?}", all.get_range(1, -1, 3).collect::<Vec<_>>());
    println!("0..9[::-3] = {:?}", all.get_range(1, -1, -3).collect::<Vec<_>>());
}
