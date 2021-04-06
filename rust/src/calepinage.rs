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
pub struct Deck { pub length: usize, pub width: usize }

impl Deck {
    pub(crate) fn area(&self) -> usize {
            self.length * self.width
    }
}

impl Deck {}

#[derive(Debug, PartialEq, Clone)]
pub struct Plank { pub length: usize }

#[derive(Default)]
pub struct PlankHeap { planks: Vec<Plank>, total_length: usize }

impl PlankHeap {
    pub fn add(self, count: usize, length: usize) -> Self {
        let planks_to_be_added: Vec<Plank> = (0..count).map(|_| Plank { length }).collect();
        let mut planks = self.planks.clone();
        planks.extend_from_slice(&planks_to_be_added);
        PlankHeap { planks, total_length: self.total_length + count * length }
    }

    pub fn new() -> Self {
        PlankHeap { planks: vec![], total_length: 0 }
    }

    pub fn from_planks(planks: Vec<Plank>) -> Self {
        planks.iter().fold(PlankHeap::new(), |heap, plank| heap.add(1, plank.length))
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
    pub(crate) fn area(&self) -> usize {
        self.0.iter().fold(0, |sum, plank| { sum + plank.length })
    }

    pub fn with_plank(self, new_plank_to_add: Plank) -> Self {
        let Line(old_planks) = self;
        let mut planks = old_planks.clone();
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
    pub(crate) fn area(&self) -> usize {
        self.0.iter().fold(0, |sum, line| { sum + line.area()})
    }

    pub fn with_line(self, new_line_to_add: Line) -> Self {
        let Calepinage(old_lines) = self;
        let mut lines = vec![new_line_to_add];
        lines.extend(old_lines);
        Calepinage(lines)
    }
}

#[test]
fn should_calculate_area_for_calepinage(){
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank { length: 3 }])
        .with_line(plank_line![Plank { length: 2 }, Plank { length: 1 }]);

    assert_eq!(6, calepinage.area());
}

#[derive(Default)]
struct CalepineStep {
    remaining: PlankHeap,
    selected: PlankHeap,
}

impl CalepineStep {}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Result<Calepinage, &'static str> {
    let select_planks_fitting_length_goal = |step: CalepineStep, plank: &Plank| {
        if step.selected.total_length + plank.length <= deck.length {
            let selected = step.selected.add(1, plank.length);
            CalepineStep { selected, ..step }
        } else {
            let remaining = step.remaining.add(1, plank.length);
            CalepineStep { remaining, ..step }
        }
    };

    let mut initial_plank_heap: PlankHeap = PlankHeap::from_planks(plank_heap.planks.clone());
    initial_plank_heap.planks.sort_by(|a, b| b.length.cmp(&a.length));
    
    let result_calpinage = (0..deck.width).into_iter()
        .try_fold((Calepinage::default(), initial_plank_heap), |(calepinage, remaining), _| {

            if remaining.total_length == 0 {
                return Err("Olalala ! ")
            }

            let CalepineStep { selected: result, remaining: next_remaining } = remaining.planks.iter().fold(CalepineStep::default(), select_planks_fitting_length_goal);
   
            Ok((calepinage.with_line(Line(result.planks)), next_remaining))
        },
        );

    let (calpinage, _remaining) = result_calpinage?;

    Ok(calpinage)
}
