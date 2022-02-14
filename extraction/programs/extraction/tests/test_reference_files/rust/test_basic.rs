use regex::Regex;

let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, 4), (5, 8)]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);