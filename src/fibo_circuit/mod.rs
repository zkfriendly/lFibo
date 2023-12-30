mod fibo_chip;

pub mod fibo_circuit {
    use crate::fibo_circuit::fibo_chip::{FiboChip, FiboConfig};
    use halo2_proofs::arithmetic::FieldExt;
    use halo2_proofs::circuit::{AssignedCell, Layouter, SimpleFloorPlanner};
    use halo2_proofs::pasta::Fp;
    use halo2_proofs::plonk::{Circuit, ConstraintSystem, Error};

    #[derive(Default)]
    struct _Circuit<F: FieldExt> {
        pub a: Option<F>,
        pub b: Option<F>,
    }

    impl<F: FieldExt> Circuit<F> for _Circuit<F> {
        type Config = FiboConfig;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let col_a = meta.advice_column();
            let col_b = meta.advice_column();
            let col_c = meta.advice_column();
            FiboChip::configure(meta, [col_a, col_b, col_c])
        }

        fn synthesize(
            &self,
            config: Self::Config,
            layouter: impl Layouter<F>,
        ) -> Result<(), Error> {
            let chip: FiboChip<F> = FiboChip::construct(config);

            let (a, b, c) = chip.assign_first_row(layouter, self.a, self.b).unwrap();

            Ok(())
        }
    }
}
