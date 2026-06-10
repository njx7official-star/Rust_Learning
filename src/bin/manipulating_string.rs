fn longest_word(s: &str) {
    let words = s.split_whitespace().collect::<Vec<&str>>();
    let mut prev_num = 0;
    let mut long = "";

    for c in words {
        let num = c.len();
        if num > prev_num {
            prev_num = num;
            long = c;
        }
    }
    println!("Longest word : {:?}", long);
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog";
    longest_word(sentence);
    // let mut a = 0;
    // let mut e = 0;
    // let mut i = 0;
    // let mut o = 0;
    // let mut u = 0;
    // // Use slicing to get the first three characters of the sentence
    // println!("{}", &sentence[0..=4]);

    // // concatenate using format!
    // let description = format!("Title: Quick story\n{}", sentence);
    // println!("{}", description);

    // // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => {
    //             println!("Got a vowel!");
    //             if c == 'a' {
    //                 a += 1;
    //             } else if c == 'e' {
    //                 e += 1;
    //             } else if c == 'i' {
    //                 i += 1;
    //             } else if c == 'o' {
    //                 o += 1;
    //             } else if c == 'u' {
    //                 u += 1;
    //             }
    //         }
    //         _ => continue,
    //     }
    // }

    // // Split and collect into a vector
    // //let words: Vec<&str> = sentence.split_whitespace().collect();
    // let words = sentence.split(' ').collect::<Vec<&str>>();
    // println!("{:?}", words);

    // let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);

    // println!("{}", a);
    // println!("{}", e);
    // println!("{}", i);
    // println!("{}", o);
    // println!("{}", u);
}
