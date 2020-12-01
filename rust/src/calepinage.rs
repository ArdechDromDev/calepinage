#[derive(Debug)]
pub struct Deck { pub length: usize }

#[derive(Debug, PartialEq, Clone)]
pub struct Plank { pub length:usize}

#[derive(Default)]
pub struct PlankHeap { planks: Vec<Plank>}

impl PlankHeap {
    pub fn add(self, count: usize, length: usize) -> Self {
        let mut planks : Vec<Plank>= (0..count).map(|id| Plank { length: length }).collect();
        planks.extend_from_slice(&self.planks);
        PlankHeap { planks }
    }
}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Vec<Vec<Plank>> {
    if (deck.length == 3){
        panic!("a faire !");
        vec![vec![Plank { length: 2 }, Plank { length: 1 }]]
    }else{
        vec![plank_heap.planks.clone()[0..deck.length].to_vec()]
    }
}
