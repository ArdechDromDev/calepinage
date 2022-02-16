#[cfg(test)]
mod calepinage_test {
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
    use rust::calepinage::*;
    use rust::plank_line;
    use spectral::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn should_return_planks_when_deck_is_really_small() {
        let deck = Deck::new(1, 1).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_return_list_of_2_planks() {
        let deck = Deck::new(2, 1).unwrap();
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }, Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_use_only_one_plank_if_does_not_need_more() {
        let deck = Deck::new(1, 1).unwrap();
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_use_different_size() {
        let deck = Deck::new(3, 1).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1).add(1, 2);

        let Calepinage(actual) = calepine(plank_heap, deck).unwrap();
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 2 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
    }

    #[test]
    fn should_use_only_required_planks() {
        let deck = Deck::new(4, 1).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1).add(2, 3);

        let Calepinage(actual) = calepine(plank_heap, deck).unwrap();
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 3 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
        assert_that(&flattened).has_length(2)
    }

    #[test]
    fn should_use_longest_planks_first() {
        let deck = Deck::new(4, 1).unwrap();
        let plank_heap = PlankHeap::default().add(10, 1).add(2, 3);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage =
            a_calepinage().with_line(plank_line![Plank { length: 3 }, Plank { length: 1 }]);
        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    #[test]
    fn should_calepine_2_lines_deck() {
        let deck = Deck::new(2, 2).unwrap();
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
        let deck = Deck::new(2, 2).unwrap();
        let plank_heap = PlankHeap::default().add(2, 1).add(1, 2);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage = a_calepinage()
            .with_line(plank_line![Plank { length: 2 }])
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 1 }]);

        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    #[test]
    fn should_return_an_error_if_not_enough_planks() {
        let deck = Deck::new(2, 2).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1);

        let result = calepine(plank_heap, deck);

        assert_that(&result)
            .is_err()
            .is_equal_to(CalepinageError::NotEnoughPlanks);
    }

    #[test]
    fn should_return_an_error_if_not_enough_planks_but_some_planks_too_big() {
        let deck = Deck::new(2, 2).unwrap();
        let plank_heap = PlankHeap::default().add(1, 100);

        let result = calepine(plank_heap, deck);

        assert_that(&result)
            .is_err()
            .is_equal_to(CalepinageError::OnlyUnusablePlanksRemaining("[Plank { length: 100 }]".to_string()));
    }



    // fn should_reuse remaining plank
    #[test]
    fn should_invert_longest_plank_in_each_line() {
        let deck = Deck::new(3, 2).unwrap();
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
        width: usize,
    }

    impl DeckForTest {
        fn to_deck(self) -> Deck {
            Deck::new(self.length, self.width).unwrap()
        }
    }

    impl Arbitrary for DeckForTest {
        fn arbitrary(g: &mut Gen) -> Self {
            DeckForTest {
                length: usize::arbitrary(g) % Deck::MAX_LENGTH + 1,
                width: usize::arbitrary(g) % Deck::MAX_LENGTH + 1,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct PlankHeapForTest {
        planks: Vec<PlankForTest>,
    }

    impl Arbitrary for PlankHeapForTest {
        fn arbitrary(g: &mut Gen) -> Self {
            PlankHeapForTest {
                planks: Vec::arbitrary(g),
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
                length: (usize::arbitrary(g)) % Plank::MAX_LENGTH + 1,
            }
        }
    }

    impl PlankHeapForTest {
        fn to_plank_heap(self) -> PlankHeap {
            PlankHeap::from_planks(
                self.planks
                    .into_iter()
                    .map(|plank| plank.to_plank())
                    .collect(),
            )
        }
    }

    impl PlankForTest {
        fn to_plank(self) -> Plank {
            Plank {
                length: self.length,
            }
        }
    }

    fn list_adjacent_junctions(lhs: &Line, rhs: &Line) -> Vec<Junction> {
        let junctions_lhs: HashSet<Junction> = lhs.compute_junction().into_iter().collect();
        let junctions_rhs: HashSet<Junction> = rhs.compute_junction().into_iter().collect();
        junctions_lhs
            .intersection(&junctions_rhs)
            .into_iter()
            .cloned()
            .collect::<Vec<Junction>>()
    }

    #[test]
    #[ignore]
    fn check() {
        QuickCheck::new()
            .tests(100000)
            .max_tests(100000)
            .quickcheck(
                two_junctions_should_not_be_adjacent
                    as fn(DeckForTest, PlankHeapForTest) -> TestResult,
            );
    }
    /*
    #[test]
    fn check_adjacent_assertion_detect_specific_case() {
        let input: Calepinage = a_calepinage()
            .with_line(plank_line![Plank { length: 2 }, Plank { length: 1 }])
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 1 }, Plank { length: 1 }]);
        assert_that(&assert_calepinage_has_no_adjacent_junction(&input)).is_false();
    }*/

    fn two_junctions_should_not_be_adjacent(
        deck: DeckForTest,
        plank_heap: PlankHeapForTest,
    ) -> TestResult {
        //println!("deck {:?} heap : {:?} ", deck, plank_heap);
        match calepine(plank_heap.to_plank_heap(), deck.to_deck()) {
            Ok(calepinage) => match find_first_adjacent_junction(&calepinage) {
                Some(_junction) => TestResult::error("found invalid junction"),
                None => TestResult::passed(),
            },
            Err(_) => TestResult::passed(),
        }
    }

    fn find_first_adjacent_junction(calepinage: &Calepinage) -> Option<Junction> {
        let Calepinage(lines) = calepinage;
        let lines_with_next: Vec<(&Line, &Line)> =
            lines.windows(2).map(|v| (&v[0], &v[1])).collect();
        //all -> validate predicates on all entries
        //lines_with_next.iter().all(|tuple| assert_no_adjacent_junction(tuple.0, tuple.1))
        //list violations of the predicate
        let list_junctions = lines_with_next
            .iter()
            .flat_map(|tuple| list_adjacent_junctions(tuple.0, tuple.1))
            .collect::<Vec<Junction>>();
        list_junctions.first().cloned()
    }

    #[test]
    fn failing_pbt_test_case_2() {
        let deck = Deck {
            length: 12,
            width: 2,
        };
        let plank_heap = PlankHeap::from_planks(
            vec![
                Plank { length: 10 },
                Plank { length: 10 },
                Plank { length: 2 },
                Plank { length: 2 },
            ], //
        );
        let result = calepine(plank_heap, deck);
        let calepinage = result.unwrap();
        let line_sizes = calepinage
            .clone()
            .0
            .iter()
            .map(|line| line.0.iter().fold(0, |total, plank| total + plank.length))
            .collect::<Vec<usize>>();
        println!("{:?}", calepinage);
        println!("{:?}", line_sizes);

        assert_that(&find_first_adjacent_junction(&calepinage)).is_none();
    }



    #[test]
    fn make_stash_algo_fail() {
        let deck = Deck {
            length: 12,
            width: 3,
        };
        let plank_heap = PlankHeap::from_planks(
            vec![
                Plank { length: 10 },
                Plank { length: 10 },
                Plank { length: 10 },
                Plank { length: 2 },
                Plank { length: 2 },
                Plank { length: 2 },
            ], //
        );
        let result = calepine(plank_heap, deck);
        let calepinage = result.unwrap();

        assert_that(&find_first_adjacent_junction(&calepinage)).is_none();
    }

}

