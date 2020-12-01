#[derive(Debug)]
pub struct Deck { pub length: usize }

#[derive(Debug, PartialEq, Clone)]
pub struct Plank { pub id:usize}

pub struct PlankHeap { planks: Vec<Plank>}
impl PlankHeap {

    pub fn new(size:usize) -> Self {
        let planks = (0..size).map(|id| Plank { id }).collect();
        PlankHeap { planks }
    }
}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Vec<Vec<Plank>> {
    vec![plank_heap.planks.clone()[0..deck.length].to_vec()]
}
