// Tests in this module are original
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn normalize_works() {
        assert_eq!(Square::new("Hello, it's me: Lord Birmingham-Smith.\t\n\t\t\t:)\n\t\t\tSincerely, Lord Birmingham-Smith").0, "helloitsmelordbirminghamsmithsincerelylordbirminghamsmith")
    }

    #[test]
    fn factor_works() {
        assert_eq!(Square::factor(0), None);
        assert_eq!(Square::factor(1), Some((1, 1)));
        assert_eq!(Square::factor(2), Some((1, 2)));
        assert_eq!(Square::factor(3), None);
        assert_eq!(Square::factor(4), Some((2, 2)));
        assert_eq!(Square::factor(5), None);
        assert_eq!(Square::factor(6), Some((2, 3)));
        assert_eq!(Square::factor(7), None);
        assert_eq!(Square::factor(8), None);
        assert_eq!(Square::factor(9), Some((3, 3)));
        assert_eq!(Square::factor(10), None);
        assert_eq!(Square::factor(11), None);
        assert_eq!(Square::factor(12), Some((3, 4)));
        assert_eq!(Square::factor(15), None);
        assert_eq!(Square::factor(20), Some((4, 5)));
        assert_eq!(Square::factor(25), Some((5, 5)));
        assert_eq!(Square::factor(132), Some((11, 12)));
    }

    #[test]
    fn closest_factor_works() {
        assert_eq!(Square::nearest_factor(3), Some((2, 2)));
        assert_eq!(Square::nearest_factor(5), Some((2, 3)));
        assert_eq!(Square::nearest_factor(7), Some((3, 3)));
        assert_eq!(Square::nearest_factor(8), Some((3, 3)));
        assert_eq!(Square::nearest_factor(10), Some((3, 4)));
        assert_eq!(Square::nearest_factor(11), Some((3, 4)));
        assert_eq!(Square::nearest_factor(13), Some((4, 4)));
        assert_eq!(Square::nearest_factor(15), Some((4, 4)));
    }

    #[test]
    fn encrypted_input_works() {
        assert_eq!(
            Square::new("thedogishappy").encrypted_input(4),
            "tohyhgaeipdsp"
        );
        assert_eq!(
            Square::new("ifmanwasmeanttostayonthegroundgodwouldhavegivenusroots")
                .encrypted_input(8),
            "imtgdvsfearwermayoogoanouuiontnnlvtwttddesaohghnsseoau"
        )
    }

    #[test]
    fn encrypt_works() {
        assert_eq!(
            Square::new("If man was meant to stay on the ground, god would have given us roots.")
                .encrypt(),
            "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
        )
    }
}
