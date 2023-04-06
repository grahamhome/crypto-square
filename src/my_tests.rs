#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn factors_work() {
        assert_eq!(factor(20), Some(Factor(4, 5)));
        assert_eq!(factor(15), None);
        assert_eq!(factor(25), Some(Factor(5, 5)));
        assert_eq!(factor(132), Some(Factor(11, 12)));
    }

    #[test]
    fn factors() {
        assert_eq!(factor(0), None);
        assert_eq!(factor(1), Some(Factor(1, 1)));
        assert_eq!(factor(2), Some(Factor(2, 2)));
        assert_eq!(factor(3), None);
        assert_eq!(factor(4), Some(Factor(2, 2)));
        assert_eq!(factor(5), None);
        assert_eq!(factor(6), Some(Factor(2,3)));
        assert_eq!(factor(7), None);
        assert_eq!(factor(8), None);
        assert_eq!(factor(9), Some(Factor(3,3)));
        assert_eq!(factor(10), None);
    }


    #[test]
    fn normalize_works() {
        assert_eq!(normalize("Hello, it's me: Lord Birmingham-Smith.\t\n\t\t\t:)\n\t\t\tSincerely, Lord Birmingham-Smith"),"helloitsmelordbirminghamsmithsincerelylordbirminghamsmith")
    }
}
