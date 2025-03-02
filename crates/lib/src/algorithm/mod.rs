pub mod history;
use crate::People;
use crate::Person;
use genetic_algorithm::{chromosome::GenesOwner, strategy::evolve::prelude::*};
use history::History;
use serde::Deserialize;
use serde::Serialize;
use tracing::{debug, trace};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pairs(Vec<(Person, Person)>);

impl Pairs {
    pub fn inner(self) -> Vec<(Person, Person)> {
        self.0
    }
}

/// This function does the pairing magic. Given some [`People`] and a [`History`] of past pairings,
/// it'll output a new set of [`Pairs`].
pub fn pair(people: People, last: &History) -> Pairs {
    let ids = people.as_ids();

    let genotype = UniqueGenotype::builder()
        .with_allele_list(ids)
        .build()
        .unwrap();

    debug!("{genotype}");

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(50)
        .with_max_stale_generations(1000)
        .with_fitness(PairFitness::new(last.clone()))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        //.with_par_fitness(true)
        .with_replace_on_equal_fitness(true)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new())
        .with_select(SelectElite::new(0.9))
        //.with_reporter(EvolveReporterSimple::new(1000))
        .build()
        .unwrap();

    evolve.call();
    let genes = evolve
        .best_genes()
        .expect("Something went wrong getting best genes");

    let pairs: Vec<(usize, usize)> = genes.chunks(2).map(|c| (c[0], c[1])).collect();
    let pairs: Vec<(Person, Person)> = pairs
        .iter()
        .map(|(id1, id2)| {
            (
                Person::new(*id1, people.name_from_id(*id1).unwrap()),
                Person::new(*id2, people.name_from_id(*id2).unwrap()),
            )
        })
        .collect();
    Pairs(pairs)
}

#[derive(Clone, Debug)]
struct PairFitness {
    last: History,
}

impl PairFitness {
    fn new(last: History) -> PairFitness {
        Self { last }
    }
}
impl Fitness for PairFitness {
    type Genotype = UniqueGenotype<usize>;
    #[allow(clippy::cast_possible_wrap)]
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        chromosome.genes().chunks(2).for_each(|chunk| {
            let (i, j) = (chunk[0], chunk[1]);

            let last = match self.last.get((i, j)) {
                Some(x) => {
                    trace!("Found score {x} for pair ({i}, {j}).");
                    x
                }
                None => {
                    if let Some(x) = self.last.get((j, i)) {
                        trace!("Found score {x} for pair ({j}, {i}).");
                        x
                    } else {
                        trace!("Found no score for pair ({j}, {i}), using 0");
                        0
                    }
                }
            };
            // high score should be bad
            score += last as isize;
        });
        trace!("Score for chromosome {:?}: {score}", chromosome.genes());
        Some(score)
    }
}
