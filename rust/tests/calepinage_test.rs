#[cfg(test)]
mod calepinage_test {
    use rust::calepinage::*;

    #[test]
    fn should_return_planks_when_deck_is_really_small() {
        let deck = Deck {length : 1};
        let plank_heap = PlankHeap::default().add(1,1);

        let actual = calepine(plank_heap, deck);

        let expected = vec![vec![Plank { length: 1 }]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_list_of_2_planks() {
        let deck = Deck {length : 2};
        let plank_heap = PlankHeap::default().add(2,1);

        let actual = calepine(plank_heap, deck);

        let expected = vec![vec![Plank { length:1}, Plank { length:1}]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_use_only_one_plank_if_does_not_need_more() {
        let deck = Deck {length : 1};
        let plank_heap = PlankHeap::default().add(2,1);

        let actual = calepine(plank_heap, deck);

        let expected = vec![vec![Plank { length: 1 }]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_use_different_size() {
        let deck = Deck {length : 3};
        let plank_heap = PlankHeap::default()
                                            .add(1,1)
                                            .add(1,2);

        let actual = calepine(plank_heap, deck);

        let expected = vec![vec![Plank { length: 2 }, Plank { length: 1 }]];
        assert_eq!(expected, actual);
    }
}