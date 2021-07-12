#[cfg(test)]
mod calepinage_test {
    use rust::calepinage::*;
    use rust::plank_line;
    use spectral::prelude::*;
    use quickcheck_macros::*;
    use quickcheck::{Testable, Gen, TestResult, Arbitrary, QuickCheck};
    use std::collections::HashSet;
    use std::slice::Windows;

    #[test]
    fn should_return_planks_when_deck_is_really_small() {
        let deck = Deck::new( 1, 1).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_return_list_of_2_planks() {
        let deck = Deck::new( 2, 1).unwrap();
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }, Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_use_only_one_plank_if_does_not_need_more() {
        let deck = Deck::new( 1, 1).unwrap();
        let plank_heap = PlankHeap::default().add(2, 1);

        let actual = calepine(plank_heap, deck);

        let expected = Calepinage(vec![Line(vec![Plank { length: 1 }])]);
        assert_that(&actual).is_ok().is_equal_to(expected);
    }

    #[test]
    fn should_use_different_size() {
        let deck = Deck::new( 3, 1).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1).add(1, 2);

        let Calepinage(actual) = calepine(plank_heap, deck).unwrap();
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 2 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
    }

    #[test]
    fn should_use_only_required_planks() {
        let deck = Deck::new( 4, 1).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1).add(2, 3);

        let Calepinage(actual) = calepine(plank_heap, deck).unwrap();
        let flattened: Vec<Plank> = actual.into_iter().flat_map(|Line(line)| line).collect();

        let expected: Vec<Plank> = vec![Plank { length: 3 }, Plank { length: 1 }];
        assert_that(&flattened).contains_all_of(&expected.iter());
        assert_that(&flattened).has_length(2)
    }

    #[test]
    fn should_use_longest_planks_first() {
        let deck = Deck::new( 4, 1).unwrap();
        let plank_heap = PlankHeap::default().add(10, 1).add(2, 3);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage =
            a_calepinage().with_line(plank_line![Plank { length: 3 }, Plank { length: 1 }]);
        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    #[test]
    fn should_calepine_2_lines_deck() {
        let deck = Deck::new( 2, 2).unwrap();
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
        let deck = Deck::new( 2, 2).unwrap();
        let plank_heap = PlankHeap::default().add(2, 1).add(1, 2);

        let actual = calepine(plank_heap, deck);

        let expected: Calepinage = a_calepinage()
            .with_line(plank_line![Plank { length: 2 }])
            .with_line(plank_line![Plank { length: 1 }, Plank { length: 1 }]);

        assert_that(&actual).is_ok().is_equal_to(&expected);
    }

    #[test]
    fn should_return_an_error_if_not_enough_planks() {
        let deck = Deck::new( 2, 2).unwrap();
        let plank_heap = PlankHeap::default().add(1, 1);

        let result = calepine(plank_heap, deck);

        assert_that(&result)
            .is_err()
            .is_equal_to(CalepinageError::NotEnoughPlanks);
    }

    #[test]
    fn should_return_an_error_if_not_enough_planks_but_some_planks_too_big() {
        let deck = Deck::new( 2, 2).unwrap();
        let plank_heap = PlankHeap::default().add(1, 100);

        let result = calepine(plank_heap, deck);

        assert_that(&result)
            .is_err()
            .is_equal_to(CalepinageError::OnlyUnusablePlanksRemaining);
    }

    #[test]
    #[ignore] //waiting fix for with_line
    fn should_invert_longest_plank_in_each_line() {
        let deck = Deck::new( 3, 2).unwrap();
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
            Deck::new(self.length, self.width).unwrap()
        }
    }

     impl Arbitrary for DeckForTest {
        fn arbitrary(g: &mut Gen) -> Self {
            DeckForTest {
                length: usize::arbitrary(g) % Deck::MAX_LENGTH + 1,
                width: usize::arbitrary(g) % Deck::MAX_LENGTH + 1
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
                length: (usize::arbitrary(g)) % Plank::MAX_LENGTH + 1
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

    fn assert_no_adjacent_junction(lhs: &Line, rhs: &Line) -> bool {
        let junctions_lhs: HashSet<Junction> = lhs.compute_junction().into_iter().collect();
        let junctions_rhs: HashSet<Junction> = rhs.compute_junction().into_iter().collect();
        junctions_lhs.is_disjoint(&junctions_rhs)
    }

    #[test]
    fn check() {
        QuickCheck::new().tests(10000).max_tests(10000).quickcheck(two_junctions_should_not_be_adjacent as fn(DeckForTest, PlankHeapForTest) -> bool);
    }

    fn two_junctions_should_not_be_adjacent(deck: DeckForTest, plank_heap: PlankHeapForTest) -> bool {
        //println!("deck {:?} heap : {:?} ", deck, plank_heap);
        match calepine(plank_heap.to_plank_heap(), deck.to_deck()) {
            Ok(Calepinage(lines)) => {
                let lines_with_next: Vec<(&Line, &Line)> = lines
                    .windows(2)
                    .map(|v| (&v[0], &v[1]))
                    .collect()
                    ;
                lines_with_next.iter().all(|tuple| assert_no_adjacent_junction(tuple.0, tuple.1))
            },
            Err(_) => true
        }
    }

    /*
    thread 'calepinage_test::check' panicked at '[quickcheck] TEST FAILED.
    Arguments: (
    DeckForTest { length: 87857, width: 2 },
    PlankHeapForTest { planks: [
    PlankForTest { length: 9007 },
    PlankForTest { length: 6335 },
    PlankForTest { length: 6864 },
    PlankForTest { length: 7858 },
    PlankForTest { length: 3337 },
    PlankForTest { length: 3044 },
    PlankForTest { length: 7720 },
    PlankForTest { length: 6743 },
    PlankForTest { length: 6462 },
    PlankForTest { length: 2905 },
    PlankForTest { length: 8423 },
    PlankForTest { length: 5890 },
    PlankForTest { length: 4738 },
    PlankForTest { length: 2 },
    PlankForTest { length: 6337 },
    PlankForTest { length: 7751 },
    PlankForTest { length: 9397 },
    PlankForTest { length: 4576 },
    PlankForTest { length: 8102 },
    PlankForTest { length: 17 },
    PlankForTest { length: 8843 },
    PlankForTest { length: 1396 },
    PlankForTest { length: 1 },
    PlankForTest { length: 4376 },
    PlankForTest { length: 2974 },
    PlankForTest { length: 4675 },
    PlankForTest { length: 9480 },
    PlankForTest { length: 6278 },
    PlankForTest { length: 1 },
    PlankForTest { length: 1 },
    PlankForTest { length: 3737 },
    PlankForTest { length: 1999 },
    PlankForTest { length: 6473 },
    PlankForTest { length: 5271 },
    PlankForTest { length: 2 },
    PlankForTest { length: 3496 },
    PlankForTest { length: 2 },
    PlankForTest { length: 6164 },
    PlankForTest { length: 9563 },
    PlankForTest { length: 53 },
    PlankForTest { length: 2091 },
    PlankForTest { length: 826 },
    PlankForTest { length: 2581 },
    PlankForTest { length: 4737 },
    PlankForTest { length: 1 },
    PlankForTest { length: 2570 },
    PlankForTest { length: 1 },
    PlankForTest { length: 7774 },
    PlankForTest { length: 6734 },
    PlankForTest { length: 2146 },
    PlankForTest { length: 4527 },
    PlankForTest { length: 9922 },
    PlankForTest { length: 7801 },
    PlankForTest { length: 1973 },
    PlankForTest { length: 2409 },
    PlankForTest { length: 8874 },
    PlankForTest { length: 7301 },
    PlankForTest { length: 5893 },
    PlankForTest { length: 8010 },
    PlankForTest { length: 5883 },
    PlankForTest { length: 7323 },
    PlankForTest { length: 2626 },
    PlankForTest { length: 6077 },
    PlankForTest { length: 1404 },
    PlankForTest { length: 7177 },
    PlankForTest { length: 8439 },
    PlankForTest { length: 2507 },
    PlankForTest { length: 1912 },
    PlankForTest { length: 8471 },
    PlankForTest { length: 6691 },
    PlankForTest { length: 2857 },
    PlankForTest { length: 4781 },
    PlankForTest { length: 4605 },
    PlankForTest { length: 7474 },
    PlankForTest { length: 149 },
    PlankForTest { length: 3036 },
    PlankForTest { length: 577 },
    PlankForTest { length: 1530 },
    PlankForTest { length: 6891 },
    PlankForTest { length: 1 },
    PlankForTest { length: 6863 },
    PlankForTest { length: 6694 },
    PlankForTest { length: 938 },
    PlankForTest { length: 5857 },
    PlankForTest { length: 7220 },
    PlankForTest { length: 545 },
    PlankForTest { length: 6799 },
    PlankForTest { length: 4687 },
    PlankForTest { length: 7278 },
    PlankForTest { length: 5254 }] })', /home/matthieu/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:165:28

     */

}
