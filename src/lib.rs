/// Boyer-Boore majority vote algorithm

pub fn majority_candidate(votes: &[u8]) -> u8 {
    let mut m: u8 = 0;
    let mut c: usize = 0;
    let mut i: usize = 0;

    while i < votes.len() {
        let v = votes[i];

        if c == 0 {
            m = v;
            c = 1;
        } else if m == v {
            c += 1;
        } else {
            c -= 1;
        }

        i += 1;
    }

    m
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_majority() {
        let votes = [2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_candidate(&votes), 2);
    }

    #[test]
    fn test_majority_single() {
        assert_eq!(majority_candidate(&[5]), 5);
    }


    #[test]
    fn test_empty() {
        assert_eq!(majority_candidate(&[]), 0);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn majority_is_returned(
            a in any::<u8>(),
            others in proptest::collection::vec(any::<u8>(), 0..64)
        ) {
            let mut votes = others.clone(); 

            votes.extend(vec![a; others.len() + 1]);

            prop_assert_eq!(majority_candidate(&votes), a);
        }
        
    }
}
