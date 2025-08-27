use std::collections::HashMap;

fn main() {
    // 基本类型与tuple
    let basic_type: (bool, char, isize, usize, f64) = (true, 'A', 50isize, 99usize, 3.14);

    // 数组与可变
    let fixed_array: [u8; 4] = [0, 1, 2, 3];
    let mut mutable_array: [u8; 4] = [4, 5, 6, 7];

    // 闭包和把闭包作为参数传入
    let u8_array_add_method = |x: usize| -> u8 { mutable_array[x] + fixed_array[x] };
    mutable_array = std::array::from_fn(u8_array_add_method);

    // 函数与泛型函数，Vector向量数组，闭包
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

    // 使用函数，所有权与内存释放
    let that_basic_vector = consume_basic_to_vector(basic_type, mutable_array);

    // 字符串，生命周期，String
    let text_1: &'static str = "How did this work?";
    let text_1_string: String = text_1.to_string() + " Just work!";

    // if else判断，返回值
    let text_if_same = if text_1.chars().next().unwrap() as u8 == text_1_string.as_bytes()[0] {
        dbg!(text_1_string);
        "debug is useful".to_string()
    } else {
        println!("{:?}", text_1_string);
        "println! is also useful".to_string()
    };

    // match匹配，返回值，下划线关闭提示
    let _match_example = match text_if_same.as_str() {
        "debug is useful" => "is same",
        "println! is also useful" => "not same",
        _ => unreachable!(),
    };

    // for循环与累加
    let mut accumlator = 0u32;
    for i in that_basic_vector.clone() {
        accumlator += i as u32;
    }

    // 迭代器累加， assert判断
    let acc: u32 = that_basic_vector.into_iter().map(|x| x as u32).sum();
    assert_eq!(accumlator, acc);

    // 自定义数据类型 enum枚举与struct结构体
    enum RocketType {
        REDSTART,
        _BLUEFISH,
    }

    struct Rocket {
        rocket_type: RocketType,
        launch_post: String,
        target_post: String,
        code: u16,
    }

    let mut new_york_rocket = Rocket {
        rocket_type: RocketType::REDSTART,
        launch_post: "NewYork".into(),
        target_post: "Tonga".into(),
        code: 0u16,
    };

    // 为类型添加函数与功能 impl
    impl RocketType {
        fn show_rocket_type(&self) {
            match self {
                RocketType::REDSTART => println!("REDSTART"),
                RocketType::_BLUEFISH => println!("BLUEFISH"),
            }
        }
    }

    impl Rocket {
        fn launch_and_change_code(&mut self) {
            let acc = self
                .launch_post
                .as_bytes()
                .iter()
                .map(|&x| x as u16)
                .chain(self.target_post.as_bytes().iter().map(|&x| x as u16))
                .sum::<u16>();
            if self.code == 0u16 {
                self.code = acc;
                println!("Rocket launched")
            } else if self.code == acc {
                println!("Rocket already on air");
            } else {
                println!("something wrong with the rocket code ---> {}", self.code);
            }
        }

        fn show_rocket(&self) {
            self.rocket_type.show_rocket_type();
            println!("{}\n{}\n{}", self.launch_post, self.target_post, self.code);
        }
    }

    new_york_rocket.launch_and_change_code();
    new_york_rocket.show_rocket();

    // mod与use，toml与第三方库

    // 介绍std::collecntions::*
    // Hashmap, VecDeque
    let mut hmap: HashMap<u32, String> = HashMap::new();
    hmap.entry(35).or_default();
    dbg!(hmap);
    // 迭代器
}
