#[derive(Debug)]
pub struct Deck { pub length: usize }

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

impl Calepinage{
    pub fn with_line(self, new_line_to_add:Line) -> Self{
        let Calepinage(old_lines) = self;
        let mut lines = vec![new_line_to_add];
        lines.extend(old_lines);
        Calepinage(lines)
    }
}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Calepinage {
    let mut sorted_planks: Vec<Plank> = plank_heap.planks.clone();
    sorted_planks.sort_by(|a, b| b.length.cmp(&a.length));

    let select_planks_fitting_length_goal = |selected_planks: PlankHeap, plank: &Plank| {
        if selected_planks.total_length + plank.length <= deck.length {
            selected_planks.add(1, plank.length)
        } else {
            selected_planks
        }
    };

    let result = sorted_planks.iter().fold(PlankHeap::new(), select_planks_fitting_length_goal);
    Calepinage::default().with_line(Line(result.planks))
}
