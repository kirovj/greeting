use generic::{Summary, Tweet};

// fn find_largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!(
        "1 new tweet: {}\nfrom: {}",
        tweet.summarize(),
        tweet.summarize_author()
    );
}
