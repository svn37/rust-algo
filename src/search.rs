pub fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    let (text, pattern) = (text.as_bytes(), pattern.as_bytes());
    if text.is_empty() || pattern.is_empty() {
        return None;
    }
    let lps = lps_array(pattern);

    let (mut i, mut j) = (0, 0);
    while i < text.len() {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }

        if j == pattern.len() {
            // first index of the occurrence
            return Some(i - j);
        }
    }
    None
}

// Compute longest proper prefix, which is also suffix, array
fn lps_array(pattern: &[u8]) -> Vec<usize> {
    // first index is 0
    let mut lps = vec![0usize; pattern.len()];

    // l -- len of the previous longest prefix suffix
    let (mut i, mut l) = (1, 0);
    while i < pattern.len() {
        if pattern[i] == pattern[l] {
            l += 1;
            lps[i] = l;
            i += 1;
        } else if l != 0 {
            // tricky case - AAACAAAA and i = 7. s[7] = 3, because s[3-1] = 3
            l = lps[l - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

pub fn rabin_karp_search(text: &str, pattern: &str) -> Option<usize> {
    const BASE: usize = 31;
    const PRIME: usize = 997;

    let (text, pattern) = (text.as_bytes(), pattern.as_bytes());
    let (n, m) = (text.len(), pattern.len());
    if text.is_empty() || pattern.is_empty() || m > n {
        return None;
    }

    fn find_hash(bytes: &[u8]) -> usize {
        bytes.iter().fold(0, |hash, &byte| {
            ((BASE * hash) % PRIME + (byte as usize)) % PRIME
        })
    }

    let mut mult = 1;
    for _ in 1..m {
        mult = (mult * BASE) % PRIME;
    }

    let pattern_hash = find_hash(pattern);
    let mut hash = find_hash(&text[..m]);

    for i in m..=n {
        if hash == pattern_hash && &text[i - m..i] == pattern {
            return Some(i - m);
        }
        if i < n {
            hash = (hash + PRIME - (mult * (text[i - m] as usize) % PRIME)) % PRIME;
            hash = (hash * BASE + (text[i] as usize)) % PRIME;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        for &(string, substring, result) in &[
            (
                "testwafwafawfawfawfawfawfawfawfa",
                "fawfawfawfawfa",
                Some(9),
            ),
            (
                "tesseatesgawatewtesaffawgfawtteafawtesftawfawfawfwfawftest",
                "test",
                Some(54),
            ),
            ("aaabaabacdedfaabaabaaa", "aaabaabacdedfaabaabaaa", Some(0)),
            ("abxabcabcaby", "abcaby", Some(6)),
            ("decadaafcdf", "daf", None),
            ("aefoaefcdaefcdaed", "aefcdaed", Some(9)),
            (
                "testwherethefullstringmatches",
                "testwherethefullstringmatchees",
                None,
            ),
            (
                "asfd oaisdufkhasdgfisdfghaskfyaeiurnfkfabcdefaksjfhksjdfhk",
                "abcd",
                Some(39),
            ),
            (
                "GATACCCATtesseatesgawatewtesaffawgfawtteafawtesftawfawfawfwfawftestCGAGTCGGATCGAGT aaabaabacdedfaabaabaaaoaisdufkhasdgfisdfghaskfyaeiurnfkfabcdefaksjfhksjdfhk",
                "oaisdufkhasdgfisdfghaskfyaeiurnfkfabcdefaksjfhksjdfhk",
                Some(105),
            ),
        ] {
            assert_eq!(kmp_search(string, substring), result);
            assert_eq!(rabin_karp_search(string, substring), result);
        }
    }
}
