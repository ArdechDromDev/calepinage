#[derive(Debug)]
pub struct Terrasse { pub length: usize }

#[derive(Debug, PartialEq, Clone)]
pub struct Planche { pub id:usize}

pub struct TasDePlanches {planches : Vec<Planche>}
impl TasDePlanches {

    pub fn new(size:usize) -> Self {
        let planches = (0..size).map(|id| Planche { id }).collect();
        TasDePlanches{planches }
    }
}

pub fn calepine(tas_de_planches: TasDePlanches, terrasse: Terrasse) -> Vec<Vec<Planche>> {
    vec![tas_de_planches.planches.clone()]
}
