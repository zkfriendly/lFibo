mod fibo_chip;

use crate::fibo_circuit2::fibo_chip::{FiboChip, FiboConfig};
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{Layouter, SimpleFloorPlanner};
use halo2_proofs::plonk::{Circuit, ConstraintSystem, Error};

#[derive(Default)]
pub struct FiboCircuit<F: FieldExt> {
    pub a: Option<F>,
    pub b: Option<F>,
}

impl<F: FieldExt> Circuit<F> for FiboCircuit<F> {
    type Config = FiboConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advices = meta.advice_column();
        let instance = meta.instance_column();
        FiboChip::configure(meta, advices, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip: FiboChip<F> = FiboChip::construct(config);

        chip.assign(layouter.namespace(|| "First Row Assignment"), 10)
            .unwrap();

        Ok(())
    }
}
