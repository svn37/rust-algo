pub fn kmp_search(string: &str, substring: &str) -> Option<usize> {
    let (string, substring) = (string.as_bytes(), substring.as_bytes());
    if string.is_empty() || substring.is_empty() {
        return None;
    }
    let lps = lps_array(substring);

    let (mut i, mut j) = (0, 0);
    while i < string.len() {
        if string[i] == substring[j] {
            i += 1;
            j += 1;
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }

        if j == substring.len() {
            // first index of the occurrence
            return Some(i - j);
        }
    }
    None
}

// Compute longest proper prefix, which is also suffix, array
fn lps_array(substring: &[u8]) -> Vec<usize> {
    // first index is 0
    let mut lps = vec![0usize; substring.len()];

    // l -- len of the previous longest prefix suffix
    let (mut i, mut l) = (1, 0);
    while i < substring.len() {
        if substring[i] == substring[l] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_search() {
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
        ] {
            assert_eq!(kmp_search(string, substring), result)
        }
    }
}
