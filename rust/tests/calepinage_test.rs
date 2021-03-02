#[cfg(test)]
mod calepinage_test {
    use rust::plank_line;
    use rust::calepinage::*;
    use spectral::prelude::*;

    #[test]
    fn should_return_planks_when_deck_is_really_small() {
        let deck = Deck { length: 1, width: 1 };
        let plank_heap = PlankHeap::default().add(1, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_list_of_2_planks() {
        let deck = Deck { length: 2, width: 1 };
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }, Plank { length: 1 }])]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_use_only_one_plank_if_does_not_need_more() {
        let deck = Deck { length: 1, width: 1 };
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_use_different_size() {
        let deck = Deck { length: 3, width: 1 };
        let plank_heap = PlankHeap::default()
            .add(1, 1)
            .add(1, 2);

        let Calepinage(actual) = calepine(plank_heap, deck);
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 2 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
    }

    #[test]
    fn should_use_only_required_planks() {
        let deck = Deck { length: 4, width: 1 };
        let plank_heap = PlankHeap::default()
            .add(1, 1)
            .add(2, 3);

        let Calepinage(actual) = calepine(plank_heap, deck);
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 3 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
        assert_that(&flattened).has_length(2)
    }

    #[test]
    fn should_use_longest_planks_first() {
        let deck = Deck { length: 4, width: 1 };
        let plank_heap = PlankHeap::default()
            .add(10, 1)
            .add(2, 3);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage = Calepinage::default().with_line(
            plank_line![Plank { length: 3 }, Plank { length: 1 }]
        );
        assert_that(&actual).is_equal_to(&expected);
    }
}