#[cfg(test)]
mod calepinage_test {
    use rust::calepinage::*;

    #[test]
    fn should_return_planche_when_terrasse_is_really_small() {
        let terrasse = Terrasse {};
        let tas_de_planches = TasDePlanches::new(1);

        let actual : Vec<Vec<Planche>> = calepine(tas_de_planches, terrasse);

        let expected = vec![vec![Planche{id:0}]];
        assert_eq!(expected, actual);
    }
}