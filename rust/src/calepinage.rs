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
#[derive(Debug, Clone)]
pub struct Deck {
    pub length: usize,
    pub width: usize,
}

impl Deck {
    pub const MAX_LENGTH: usize = 1_000_000;

    pub fn new(length: usize, width: usize) -> Result<Self, String> {
        if length == 0 || width == 0 {
            Err("a deck can't have any zero dimension".to_string())
        } else if length > Self::MAX_LENGTH {
            Err(format!("max length of deck is {}", Self::MAX_LENGTH))
        } else {
            Ok(Deck { length, width })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Plank {
    pub length: usize,
}

impl Plank {
    pub const MAX_LENGTH: usize = 10000;

    pub fn new(length: usize) -> Result<Self, String> {
        if length > Self::MAX_LENGTH {
            Err(format!("max length of plank is {}", Self::MAX_LENGTH))
        } else {
            Ok(Plank { length })
        }
    }
}

#[derive(Default, Debug)]
pub struct PlankHeap {
    planks: Vec<Plank>,
    total_length: usize,
}

impl PlankHeap {
    pub fn add(self, count: usize, length: usize) -> Self {
        let planks_to_be_added: Vec<Plank> =
            (0..count).map(|_| Plank::new(length).unwrap()).collect();
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

    pub fn compute_junction(&self) -> Vec<Junction> {
        if self.0.len() > 1 {
            self.0
                .iter()
                .scan(0, |acc, plank| {
                    *acc = *acc + plank.length;
                    Some(*acc)
                })
                .map(|j| Junction(j))
                .take(self.0.len() - 1)
                .collect()
        } else {
            Vec::<Junction>::new()
        }
    }
}

/// A Junction is a coordinate in a 1 dimension plan corresponding to two plank edges
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Junction(usize);

#[test]
fn empty_line_should_have_no_junction() {
    assert_eq!(Vec::<Junction>::new(), plank_line!().compute_junction());
}

#[test]
fn single_plank_line_should_have_no_junction() {
    assert_eq!(
        Vec::<Junction>::new(),
        plank_line!(Plank::new(1).unwrap()).compute_junction()
    );
}

#[test]
fn two_planks_line_should_have_one_junction() {
    assert_eq!(
        vec![Junction(3)],
        plank_line!(Plank::new(3).unwrap(), Plank::new(1).unwrap()).compute_junction()
    );
}

#[test]
fn should_build_line() {
    let actual = plank_line![]
        .with_plank(Plank::new(2).unwrap())
        .with_plank(Plank::new(1).unwrap());

    let expected = Line(vec![Plank::new(2).unwrap(), Plank::new(1).unwrap()]);
    assert_eq!(expected, actual);
}

#[test]
fn should_use_macro() {
    let actual = plank_line![Plank::new(2).unwrap()];

    let expected = Line(vec![Plank::new(2).unwrap()]);
    assert_eq!(expected, actual);
}

#[test]
fn should_use_macro_with_2_planks() {
    let actual = plank_line![Plank::new(2).unwrap(), Plank::new(1).unwrap()];

    let expected = Line(vec![Plank::new(2).unwrap(), Plank::new(1).unwrap()]);
    assert_eq!(expected, actual);
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Calepinage(pub Vec<Line>);

impl Calepinage {
    pub fn with_line(self, new_line_to_add: Line) -> Self {
        let Calepinage(mut lines) = self;

        lines.push(new_line_to_add);
        Calepinage(lines)
    }
}

#[test]
fn with_line_should_append_lines_in_order() {
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank::new(1).unwrap()])
        .with_line(plank_line![Plank::new(2).unwrap()]);

    let Calepinage(lines) = calepinage;
    assert_eq!(&lines[0], &plank_line![Plank::new(1).unwrap()]);
    assert_eq!(&lines[1], &plank_line![Plank::new(2).unwrap()]);
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

    let step = the_plank_heap
        .planks
        .iter()
        .fold(CalepineStep::default(), select_planks_fitting_length_goal);

    assert_length_goal_fulfilled(step, deck_length)
}

fn assert_length_goal_fulfilled(
    step: CalepineStep,
    deck_length: usize,
) -> Result<CalepineStep, CalepinageError> {
    if step.selected.total_length < deck_length {
        if step.remaining.total_length == 0 {
            Err(CalepinageError::NotEnoughPlanks)
        } else {
            Err(CalepinageError::OnlyUnusablePlanksRemaining)
        }
    } else {
        Ok(step)
    }
}
