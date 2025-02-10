use genetic_algorithm::{chromosome::GenesOwner, strategy::evolve::prelude::*};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
pub struct Person {
    id: usize,
    name: String,
}

pub fn pair(people: Vec<Person>) -> Vec<(Person, Person)> {
    let mut h = HashMap::new();
    for p in people {
        h.insert(p.id, p);
    }

    let ids = h.keys().copied().collect();

    let genotype = UniqueGenotype::builder()
        .with_allele_list(ids)
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut last = HashMap::new();

    let current: usize = *last.values().max().unwrap_or(&1usize) + 1;

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(200)
        .with_max_stale_generations(20000)
        .with_fitness(PairFitness::new(last))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_replace_on_equal_fitness(true)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new())
        .with_select(SelectElite::new(0.9))
        .with_reporter(EvolveReporterSimple::new(100))
        .build()
        .unwrap();

    evolve.call();
    let genes = evolve
        .best_genes()
        .expect("Something went wrong getting best genes");

    let pairs = genes
        .chunks(2)
        .map(|c| (h.get(&c[0]).unwrap().clone(), h.get(&c[1]).unwrap().clone()))
        .collect();
    pairs
}

#[derive(Clone, Debug)]
struct PairFitness {
    last: HashMap<(usize, usize), usize>,
}

impl PairFitness {
    fn new(last: HashMap<(usize, usize), usize>) -> PairFitness {
        Self { last }
    }
}
impl Fitness for PairFitness {
    type Genotype = UniqueGenotype<usize>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        chromosome.genes().chunks(2).for_each(|chunk| {
            let (i, j) = (chunk[0], chunk[1]);
            let last = match self.last.get(&(i, j)) {
                Some(x) => x,
                None => self.last.get(&(j, i)).unwrap_or(&0),
            };
            // high score should be bad
            score += *last as isize;
        });
        Some(score)
    }
}
