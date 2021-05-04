// This is a deck with length = 6 and width = 4
// It's made with 8 planks.
// p1 has length = 2
// p3 has length = 4
//
// /===========\
// |p1|  |p5|p7|
// |  |p3|  |--|
// |--|  |  |p8|
// |p2|  |--|  |
// |  |--|p6|  |
// |  |p4|  |  |
// \===========/
#[derive(Debug)]
pub struct Deck {
    pub length: usize,
    pub width: usize,
}

impl Deck {}

#[derive(Debug, PartialEq, Clone)]
pub struct Plank {
    pub length: usize,
}

#[derive(Default)]
pub struct PlankHeap {
    planks: Vec<Plank>,
    total_length: usize,
}

impl PlankHeap {
    pub fn add(self, count: usize, length: usize) -> Self {
        let planks_to_be_added: Vec<Plank> = (0..count).map(|_| Plank { length }).collect();
        let mut planks = self.planks.clone();
        planks.extend_from_slice(&planks_to_be_added);
        PlankHeap {
            planks,
            total_length: self.total_length + count * length,
        }
    }

    pub fn new() -> Self {
        PlankHeap {
            planks: vec![],
            total_length: 0,
        }
    }

    pub fn from_planks(planks: Vec<Plank>) -> Self {
        planks
            .iter()
            .fold(PlankHeap::new(), |heap, plank| heap.add(1, plank.length))
    }
}

#[macro_export]
macro_rules! plank_line {
    ( $($head: expr), *) => {{  // {{ pcq Bloc d'instructions
        let line = Line::default();
        $(
          let line = line.with_plank($head);
        )*
        line
      }};
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Line(pub Vec<Plank>);

impl Line {
    pub fn with_plank(self, new_plank_to_add: Plank) -> Self {
        let Line(old_planks) = self;
        let mut planks = old_planks;
        planks.push(new_plank_to_add);
        Line(planks)
    }
}

#[test]
fn should_build_line() {
    let actual = plank_line![]
        .with_plank(Plank { length: 2 })
        .with_plank(Plank { length: 1 });

    let expected = Line(vec![Plank { length: 2 }, Plank { length: 1 }]);
    assert_eq!(expected, actual);
}

#[test]
fn should_use_macro() {
    let actual = plank_line![Plank { length: 2 }];

    let expected = Line(vec![Plank { length: 2 }]);
    assert_eq!(expected, actual);
}

#[test]
fn should_use_macro_with_2_planks() {
    let actual = plank_line![Plank { length: 2 }, Plank { length: 1 }];

    let expected = Line(vec![Plank { length: 2 }, Plank { length: 1 }]);
    assert_eq!(expected, actual);
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Calepinage(pub Vec<Line>);

impl Calepinage {
    pub fn with_line(self, new_line_to_add: Line) -> Self {
        let Calepinage(old_lines) = self;
        let mut lines = vec![new_line_to_add];
        lines.extend(old_lines);
        Calepinage(lines)
    }
}

#[derive(Default)]
struct CalepineStep {
    remaining: PlankHeap,
    selected: PlankHeap,
}

impl CalepineStep {}

#[derive(Debug, PartialEq)]
pub enum CalepinageError {
    NotEnoughPlanks,
    OnlyUnusablePlanksRemaining,
}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Result<Calepinage, CalepinageError> {
    let mut the_plank_heap: PlankHeap = PlankHeap::from_planks(plank_heap.planks);
    let decreasing_length = |a: &Plank, b: &Plank| b.length.cmp(&a.length);
    the_plank_heap.planks.sort_by(decreasing_length);

    let mut calepinage = Calepinage::default();
    for _ in 0..deck.width {
        let CalepineStep {
            selected: result,
            remaining: next_remaining,
        } = select_planks_for_line(&mut the_plank_heap, deck.length)?;
        the_plank_heap = next_remaining;
        calepinage = calepinage.with_line(Line(result.planks));
    }

    Ok(calepinage)
}

fn select_planks_for_line(
    the_plank_heap: &mut PlankHeap,
    deck_length: usize,
) -> Result<CalepineStep, CalepinageError> {
    let select_planks_fitting_length_goal = |step: CalepineStep, plank: &Plank| {
        if step.selected.total_length + plank.length <= deck_length {
            let selected = step.selected.add(1, plank.length);
            CalepineStep { selected, ..step }
        } else {
            let remaining = step.remaining.add(1, plank.length);
            CalepineStep { remaining, ..step }
        }
    };

    let result = the_plank_heap
        .planks
        .iter()
        .fold(CalepineStep::default(), select_planks_fitting_length_goal);
    if result.selected.total_length < deck_length {
        if result.remaining.total_length == 0 {
            Err(CalepinageError::NotEnoughPlanks)
        } else {
            Err(CalepinageError::OnlyUnusablePlanksRemaining)
        }
    } else {
        Ok(result)
    }
}
