#[cfg(test)]
mod calepinage_test {
    use rust::calepinage::*;

    #[test]
    fn should_return_planche_when_terrasse_is_really_small() {
        let terrasse = Terrasse {length : 1};
        let tas_de_planches = TasDePlanches::new(1);

        let actual = calepine(tas_de_planches, terrasse);

        let expected = vec![vec![Planche { id: 0 }]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_list_of_2_planches() {
        let terrasse = Terrasse {length : 2};
        let tas_de_planches = TasDePlanches::new(2);

        let actual = calepine(tas_de_planches, terrasse);

        let expected = vec![vec![Planche{id:0}, Planche{id:1}]];
        assert_eq!(expected, actual);
    }
}