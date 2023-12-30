mod fibo_chip;

use crate::fibo_circuit::fibo_chip::{FiboChip, FiboConfig};
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
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_c = meta.advice_column();
        let instance = meta.instance_column();
        FiboChip::configure(meta, [col_a, col_b, col_c], instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip: FiboChip<F> = FiboChip::construct(config);

        let (a, mut prev_b, mut prev_c) = chip
            .assign_first_row(
                layouter.namespace(|| "First Row Assignment"),
                self.a,
                self.b,
            )
            .unwrap();

        chip.expose_public(layouter.namespace(|| "private a"), &a, 0)?;
        chip.expose_public(layouter.namespace(|| "private b"), &prev_b, 1)?;

        for i in 3..10 {
            let c = chip
                .assign_row(
                    layouter.namespace(|| format!("Assining {i}")),
                    &prev_b,
                    &prev_c,
                )
                .unwrap();

            prev_b = prev_c;
            prev_c = c;
        }

        chip.expose_public(layouter.namespace(|| "out"), &prev_c, 2)?;

        Ok(())
    }
}
