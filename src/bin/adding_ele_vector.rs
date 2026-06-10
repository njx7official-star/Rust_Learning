fn beg_end_vec(v: &mut Vec<i32>, value: i32) {
    v.insert(0, value);
    v.push(value);
    println!("the updated vector = {:?}", v);
}
fn join_vectors(v: &mut Vec<i32>, w: &mut Vec<i32>) {
    v.append(w);
    println!("New joined vectors = {:?}", v);
}
fn main() {
    let mut v = vec![1, 2, 3];
    // beg_end_vec(&mut v, 3);
    let mut w = vec![9, 0];
    join_vectors(&mut v, &mut w);
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
}
