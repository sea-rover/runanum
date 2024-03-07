pub struct Cases<'a> {
    pub nom: &'a str,
    pub gen: &'a str,
    pub plu: &'a str,
}

//crun - correct russian noun
pub fn crun<'a>(num: i32, cases: &Cases<'a>) -> &'a str {
    let num = num.abs();
    
    let word = if num.to_string().contains('.') {
        cases.gen
    } else {
        if num % 10 == 1 && num % 100 != 11 {
            cases.nom
        } else if num % 10 >= 2 && num % 10 <= 4 && (num % 100 < 10 || num % 100 >= 20) {
            cases.gen
        } else {
            cases.plu
        }
    };
    
    word
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn girls() {
        let cases = Cases {
            nom: "девушка",
            gen: "девушки",
            plu: "девушек",
        };
        let mut num = 5;
        let result = crun(num, &cases);
        assert_eq!(result, "девушек");

        num = 17;
        let result = crun(num, &cases);
        assert_eq!(result, "девушек");

        num = 4;
        let result = crun(num, &cases);
        assert_eq!(result, "девушки");

        num = 21;
        let result = crun(num, &cases);
        assert_eq!(result, "девушка");

        num = 2100800;
        let result = crun(num, &cases);
        assert_eq!(result, "девушек");
    }
}
