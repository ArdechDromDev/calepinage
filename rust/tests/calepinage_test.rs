#[cfg(test)]
mod calepinage_test {
    use rust::calepinage::*;

    #[test]
    fn should_return_planks_when_deck_is_really_small() {
        let deck = Deck {length : 1};
        let plank_heap = PlankHeap::new(1);

        let actual = calepine(plank_heap, deck);

        let expected = vec![vec![Plank { id: 0 }]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_list_of_2_planks() {
        let deck = Deck {length : 2};
        let plank_heap = PlankHeap::new(2);

        let actual = calepine(plank_heap, deck);

        let expected = vec![vec![Plank {id:0}, Plank {id:1}]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_use_only_one_plank_if_does_not_need_more() {
        let deck = Deck {length : 1};
        let plank_heap = PlankHeap::new(2);

        let actual = calepine(plank_heap, deck);

        let planks_count = actual.iter().flatten().count();
        assert_eq!(1, planks_count);
    }
}