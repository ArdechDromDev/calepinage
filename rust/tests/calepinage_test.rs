#[cfg(test)]
mod calepinage_test {
    use rust::calepinage::*;
    use rust::plank_line;
    use spectral::prelude::*;
    use quickcheck_macros::*;
    use quickcheck::{Testable, Gen, TestResult, Arbitrary};
    use std::collections::HashSet;

    #[test]
    fn should_return_planks_when_deck_is_really_small() {
        let deck = Deck {
            length: 1,
            width: 1,
        };
        let plank_heap = PlankHeap::default().add(1, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_return_list_of_2_planks() {
        let deck = Deck {
            length: 2,
            width: 1,
        };
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }, Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_use_only_one_plank_if_does_not_need_more() {
        let deck = Deck {
            length: 1,
            width: 1,
        };
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_use_different_size() {
        let deck = Deck {
            length: 3,
            width: 1,
        };
        let plank_heap = PlankHeap::default().add(1, 1).add(1, 2);

        let Calepinage(actual) = calepine(plank_heap, deck).unwrap();
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 2 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
    }

    #[test]
    fn should_use_only_required_planks() {
        let deck = Deck {
            length: 4,
            width: 1,
        };
        let plank_heap = PlankHeap::default().add(1, 1).add(2, 3);

        let Calepinage(actual) = calepine(plank_heap, deck).unwrap();
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 3 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
        assert_that(&flattened).has_length(2)
    }

    #[test]
    fn should_use_longest_planks_first() {
        let deck = Deck {
            length: 4,
            width: 1,
        };
        let plank_heap = PlankHeap::default().add(10, 1).add(2, 3);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage =
            a_calepinage().with_line(plank_line![Plank { length: 3 }, Plank { length: 1 }]);
        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    #[test]
    fn should_calepine_2_lines_deck() {
        let deck = Deck {
            length: 2,
            width: 2,
        };
        let plank_heap = PlankHeap::default().add(4, 1);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage = a_calepinage()
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 1 }])
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 1 }]);

        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    fn a_calepinage() -> Calepinage {
        Calepinage::default()
    }

    #[test]
    fn should_calepine_2_lines_deck_with_different_sizes() {
        let deck = Deck {
            length: 2,
            width: 2,
        };
        let plank_heap = PlankHeap::default().add(2, 1).add(1, 2);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage = a_calepinage()
            .with_line(plank_line![Plank { length: 2 }])
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 1 }]);

        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    #[test]
    fn should_return_an_error_if_not_enough_planks() {
        let deck = Deck {
            length: 2,
            width: 2,
        };
        let plank_heap = PlankHeap::default().add(1, 1);

        let result = calepine(plank_heap, deck);

        assert_that(&result)
            .is_err()
            .is_equal_to(CalepinageError::NotEnoughPlanks);
    }

    #[test]
    fn should_return_an_error_if_not_enough_planks_but_some_planks_too_big() {
        let deck = Deck {
            length: 2,
            width: 2,
        };
        let plank_heap = PlankHeap::default().add(1, 100);

        let result = calepine(plank_heap, deck);

        assert_that(&result)
            .is_err()
            .is_equal_to(CalepinageError::OnlyUnusablePlanksRemaining);
    }

    #[test]
    #[ignore] //waiting fix for with_line
    fn should_invert_longest_plank_in_each_line() {
        let deck = Deck {
            length: 3,
            width: 2,
        };
        let plank_heap = PlankHeap::default().add(2, 1).add(2, 2);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage = a_calepinage()
            .with_line(plank_line![Plank { length: 2 }, Plank { length: 1 }])
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 2 }]);

        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

/*  L1 L2 L3 L4
// /===========\
// |p1|  |p5|p7|
// |  |p3|  |--|
// |--|  |  |p8|
// |p2|  |--|  |
// |  |--|p6|  |
// |  |p4|  |  |
// \===========/

two_junctions_should_not_be_adjacent

 */
    #[derive(Clone, Debug)]
    struct DeckForTest {
        length: usize,
        width: usize
    }

    impl DeckForTest {
        fn to_deck(self) -> Deck {
            Deck {
                length: self.length,
                width: self.width,
            }
        }
    }

     impl Arbitrary for DeckForTest {
        fn arbitrary(g: &mut Gen) -> Self {
            DeckForTest {
                length: usize::arbitrary(g),
                width: usize::arbitrary(g)
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct PlankHeapForTest {
        planks: Vec<PlankForTest>,
        total_length: usize,
    }

    impl Arbitrary for PlankHeapForTest {
        fn arbitrary(g: &mut Gen) -> Self {
            PlankHeapForTest {
                planks: Vec::arbitrary(g),
                total_length: usize::arbitrary(g)
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct PlankForTest {
        length: usize,
    }

    impl Arbitrary for PlankForTest {
        fn arbitrary(g: &mut Gen) -> Self {
            PlankForTest {
                length: usize::arbitrary(g)
            }
        }
    }

    impl PlankHeapForTest {
        fn to_plank_heap(self) -> PlankHeap {
            PlankHeap::from_planks(self.planks.into_iter().map(|plank| plank.to_plank()).collect())
        }
    }

    impl PlankForTest {
        fn to_plank(self) -> Plank {
            Plank{length: self.length}
        }
    }
/*
    fn assert_no_adjacent_junction(lhs: Line, rhs: Line) -> bool {
        let junctions_lhs: HashSet<usize> = compute_junctions(lhs);
        let junctions_rhs: HashSet<usize> = compute_junctions(rhs);
        intersect(junctions_lhs, junctions_rhs).is_empty()
    }

    #[quickcheck]
    fn two_junctions_should_not_be_adjacent(deck: DeckForTest, plank_heap: PlankHeapForTest) -> bool {
        let Calepinage(lines) = calepine(plank_heap.to_plank_heap(), deck.to_deck());

        let lines_with_next: Vec<(Line, Line)> = unimplemented!();
        lines_with_next.all(assert_no_adjacent_junction)
    }

*/
}
