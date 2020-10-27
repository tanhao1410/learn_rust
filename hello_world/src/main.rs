fn main() {
    println!("Hello, world!");

    let arr = [1,2,3,4,5];
    let ref1 = &arr[1..];
    let lenth = ref1.len();
    assert_eq!(&arr[1..],[2,3,4,5]);

    let vec1 = vec![1,2,3,4,5];
    //let vec2:Vec = std::collections::
    assert_eq!(&vec1,&arr);
    assert_eq!(vec1,arr);
}
