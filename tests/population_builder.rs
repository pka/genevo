#[cfg(test)] #[macro_use] extern crate hamcrest;

use hamcrest::prelude::*;

extern crate genevo;
extern crate fixedbitset;

use genevo::prelude::*;
use genevo::population::{BinaryEncodedGenomeBuilder, ValueEncodedGenomeBuilder};
use genevo::random::Rng;
use fixedbitset::FixedBitSet;


#[test]
fn create_population_of_fixedbitset_uniform_at_random() {
    let population: Population<FixedBitSet> = build_population()
        .with_genome_builder(BinaryEncodedGenomeBuilder::new(20))
        .of_size(200)
        .uniform_at_random();

    println!("{:?}", population);
    assert_that!(population.size(), is(equal_to(200)));
}

#[test]
fn create_population_of_vec_of_bool_uniform_at_random() {
    let population: Population<Vec<bool>> = build_population()
        .with_genome_builder(BinaryEncodedGenomeBuilder::new(4))
        .of_size(200)
        .uniform_at_random();

    println!("{:?}", population);
    assert_that!(population.size(), is(equal_to(200)));
}

#[test]
fn create_population_of_vec_of_f64_uniform_at_random() {
    let population: Population<Vec<f64>> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(4, -2., 2.))
        .of_size(200)
        .uniform_at_random();

    println!("{:?}", population);
    assert_that!(population.size(), is(equal_to(200)));
}

#[test]
fn create_population_of_custom_genotype_uniform_at_random() {
    #[derive(Clone,Debug,PartialEq)]
    struct Pos {
        x: usize,
        y: usize,
    }

    struct PositionsBuilder;
    impl GenomeBuilder<Vec<Pos>> for PositionsBuilder {

        fn build_genome<R>(&self, _: usize, rng: &mut R) -> Vec<Pos>
            where R: Rng + Sized
        {
            (0..8).map(|row|
                Pos {
                    x: row,
                    y: rng.gen_range(0, 8)
                }
            ).collect()
        }
    }

    let population: Population<Vec<Pos>> =
        build_population()
            .with_genome_builder(PositionsBuilder)
            .of_size(200)
            .uniform_at_random();

    println!("{:?}", population);
    assert_that!(population.size(), is(equal_to(200)));
}

#[test]
fn create_population_with_custom_number_generator() {
    let population: Population<FixedBitSet> = build_population()
        .with_genome_builder(BinaryEncodedGenomeBuilder::new(8))
        .of_size(200)
        .using_seed([42, 0]);

    println!("{:?}", population);
    assert_that!(population.size(), is(equal_to(200)));
}
