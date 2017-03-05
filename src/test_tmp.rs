// extern crate rand;

// use rand::Rng;

// struct St {}
//
// struct Ut;
//
// fn find(haystack: &str, needle: char) -> Option<usize> {
//     haystack.find(needle)
// }
//
// fn extension(file_name: &str) -> Option<&str> {
//     find(file_name, '.').map(|i| &file_name[i + 1..])
// }

fn main() {
    //
    // let mut tmp_bottom = Vec::new();
    let mut full_lines = vec![5, 9, 0, 3, 6];
    full_lines.sort();
    println!("{:?}", full_lines);

    // for i in 0..50 {
    //     let i = rand::thread_rng().gen_range(0, 7);
    //     println!("{:?}", i);
    // }

    // let collection: Vec<u32> = (0..30).filter(|p| {
    //     *p > 5
    // }).map(|p| {
    //     if p < 10 {
    //         p + 50
    //     } else {
    //         p
    //     }
    //     // p + 10
    // }).collect();
    // println!("{:?}", collection);
    //
    // let st = St{};
    // let ut = Ut;

    // let mut iter_bottom = self.bottom.clone();
    //
    // for l in full_lines.iter() {
    //     tmp_bottom.clear();
    //     for p in iter_bottom.iter() {
    //         if p.1 < *l {
    //             tmp_bottom.push(*p + Point(0, 1));
    //         } else if p.1 > *l {
    //             tmp_bottom.push(*p);
    //         } else {}
    //     }
    //     iter_bottom = tmp_bottom.clone();
    // }
    //
    // self.bottom = tmp_bottom;


    let v0: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut vec: Vec<String> = vec!["".to_string()];
    let mut vec_tmp: Vec<String> = Vec::new();

    for _ in 0..v0.len() {
        //
        for s0 in &v0 {
            for svec in &vec {
                if svec.find(s0) == None {
                    vec_tmp.push(s0.clone() + svec);
                }
            }
        }

        vec = vec_tmp.clone();
        vec_tmp.clear();
        for j in &vec {
            println!("{:?}", j);
        }
        println!("----------------");
    }

}
