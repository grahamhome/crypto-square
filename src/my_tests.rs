#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn flat_chunks_works() {
        assert_eq!(
            flat_chunks(String::from("thedogishappy"), 4),
            "thed\n\
             ogis\n\
             happ\n\
             y   "
        )
    }

    #[test]
    fn encoded_flat_works() {
        assert_eq!(
            encoded_flat(String::from("thedogishappy"), 4),
            "tohyhgaeipdsp"
        )
    }

    #[test]
    fn closest_factor_works() {
        assert_eq!(find_nearest_factor(3), Some(Square { cols: 2, rows: 2 }));
        assert_eq!(find_nearest_factor(5), Some(Square { cols: 3, rows: 2 }));
        assert_eq!(find_nearest_factor(7), Some(Square { cols: 3, rows: 3 }));
        assert_eq!(find_nearest_factor(8), Some(Square { cols: 3, rows: 3 }));
        assert_eq!(find_nearest_factor(10), Some(Square { cols: 4, rows: 3 }));
        assert_eq!(find_nearest_factor(11), Some(Square { cols: 4, rows: 3 }));
        assert_eq!(find_nearest_factor(13), Some(Square { cols: 4, rows: 4 }));
        assert_eq!(find_nearest_factor(15), Some(Square { cols: 4, rows: 4 }));
    }

    #[test]
    fn factors_work() {
        assert_eq!(factor(0), None);
        assert_eq!(factor(1), Some(Square { cols: 1, rows: 1 }));
        assert_eq!(factor(2), Some(Square { cols: 2, rows: 2 }));
        assert_eq!(factor(3), None);
        assert_eq!(factor(4), Some(Square { cols: 2, rows: 2 }));
        assert_eq!(factor(5), None);
        assert_eq!(factor(6), Some(Square { cols: 3, rows: 2 }));
        assert_eq!(factor(7), None);
        assert_eq!(factor(8), None);
        assert_eq!(factor(9), Some(Square { cols: 3, rows: 3 }));
        assert_eq!(factor(10), None);
        assert_eq!(factor(11), None);
        assert_eq!(factor(12), Some(Square { cols: 4, rows: 3 }));
        assert_eq!(factor(15), None);
        assert_eq!(factor(20), Some(Square { cols: 5, rows: 4 }));
        assert_eq!(factor(25), Some(Square { cols: 5, rows: 5 }));
        assert_eq!(factor(132), Some(Square { cols: 12, rows: 11 }));
    }

    #[test]
    fn normalize_works() {
        assert_eq!(normalize("Hello, it's me: Lord Birmingham-Smith.\t\n\t\t\t:)\n\t\t\tSincerely, Lord Birmingham-Smith"),"helloitsmelordbirminghamsmithsincerelylordbirminghamsmith")
    }
}
