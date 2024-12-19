use std::collections::HashMap;
use wordbreaker::Dictionary;

fn main() {
    static INPUT: &str = include_str!("../../../day19.txt");

    let dictionary = INPUT
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .collect::<Dictionary<_>>();

    let successes = INPUT
        .lines()
        .skip(2)
        .filter(|&target| dictionary.concatenations_for(target).next().is_some())
        .count();

    println!("{successes}");

    let mut words = INPUT
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .collect::<Vec<_>>();
    words.sort_unstable();

    let successes = INPUT
        .lines()
        .skip(2)
        .inspect(|line| println!("{line}"))
        .map(|target| f(&mut HashMap::new(), 0, target, &words))
        .sum::<usize>();

    println!("{successes}");
}

fn f(memoizer: &mut HashMap<usize, usize>, i: usize, word: &str, dictionary: &[&str]) -> usize {
    if i == word.len() {
        1
    } else if memoizer.contains_key(&i) {
        *memoizer.get(&i).unwrap()
    } else {
        let count = dictionary
            .into_iter()
            .filter(|&dict_word| word[i..].starts_with(dict_word))
            .map(|dict_word| f(memoizer, i + dict_word.len(), word, dictionary))
            .sum::<usize>();
        memoizer.insert(i, count);
        count
    }
}
