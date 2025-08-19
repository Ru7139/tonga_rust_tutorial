fn main() {
    let basic_type: (bool, char, isize, usize, f64) = (true, 'A', 50isize, 99usize, 3.14);
    let fixed_array: [u8; 4] = [0, 1, 2, 3];
    let mut mutable_array: [u8; 4] = [4, 5, 6, 7];
    let u8_array_add_method = |x: usize| -> u8 { mutable_array[x] + fixed_array[x] };
    mutable_array = std::array::from_fn(u8_array_add_method);

    fn consume_basic_to_vector<const N: usize>(
        that_tuple: (bool, char, isize, usize, f64),
        that_array: [u8; N],
    ) -> Vec<u8> {
        std::iter::once(that_tuple.0 as u8)
            .chain((that_tuple.1 as u8).to_le_bytes())
            .chain((that_tuple.2 as u8).to_le_bytes())
            .chain((that_tuple.3 as u8).to_le_bytes())
            .chain((that_tuple.4 as u8).to_le_bytes())
            .chain(that_array.into_iter())
            .collect()
    }
    let _that_basic_vector = consume_basic_to_vector(basic_type, mutable_array);

    let text_1: &'static str = "How did this work?";
    let text_1_string: String = text_1.to_string() + " Just work!";

    if text_1.chars().next().unwrap() as u8 == text_1_string.as_bytes()[0] {
        // println!("{:?}", text_1_string);
        dbg!(text_1_string);
    }
}
