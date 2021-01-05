#[derive(Debug)]
pub struct Deck { pub length: usize }

#[derive(Debug, PartialEq, Clone)]
pub struct Plank { pub length:usize}

#[derive(Default)]
pub struct PlankHeap { planks: Vec<Plank> , total_length: usize }

impl PlankHeap {
    pub fn add(self, count: usize, length: usize) -> Self {
        let mut planks : Vec<Plank>= (0..count).map(|id| Plank { length: length }).collect();
        planks.extend_from_slice(&self.planks);
        PlankHeap { planks, total_length: self.total_length + count * length }
    }

    pub fn new() -> Self {
        PlankHeap { planks: vec![], total_length: 0 }
    }
}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Vec<Vec<Plank>> {

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
    vec![result.planks]
}
