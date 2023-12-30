use halo2_proofs::circuit::{AssignedCell, Layouter};
use halo2_proofs::plonk::{Advice, Column, Error};
use halo2_proofs::poly::Rotation;
use halo2_proofs::{arithmetic::FieldExt, plonk::*};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct FiboConfig {
    pub advice: [Column<Advice>; 3],
    pub selector: Selector,
    pub instance: Column<Instance>,
}

pub struct FiboChip<F: FieldExt> {
    config: FiboConfig,
    _marker: std::marker::PhantomData<F>,
}

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(AssignedCell<F, F>);

impl<F: FieldExt> FiboChip<F> {
    pub fn construct(config: FiboConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 3],
        instance: Column<Instance>,
    ) -> FiboConfig {
        let col_a = advice[0];
        let col_b = advice[1];
        let col_c = advice[2];
        let selector = meta.selector();

        meta.enable_equality(col_a);
        meta.enable_equality(col_b);
        meta.enable_equality(col_c);
        meta.enable_equality(instance);

        // col_a  | col_b  | col_c | selector
        //   a       b         c       s
        meta.create_gate("add", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(col_a, Rotation::cur());
            let b = meta.query_advice(col_b, Rotation::cur());
            let c = meta.query_advice(col_c, Rotation::cur());
            vec![s * (a + b - c)]
        });

        FiboConfig {
            advice: [col_a, col_b, col_c],
            selector,
            instance,
        }
    }

    pub fn assign_first_row(
        &self,
        mut layouter: impl Layouter<F>,
        a: Option<F>,
        b: Option<F>,
    ) -> Result<(ACell<F>, ACell<F>, ACell<F>), Error> {
        layouter.assign_region(
            || "first row",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;

                let cell_a = region
                    .assign_advice(
                        || "cell_a",
                        self.config.advice[0],
                        0,
                        || a.ok_or(Error::Synthesis),
                    )
                    .map(ACell)?;

                let cell_b = region
                    .assign_advice(
                        || "cell_b",
                        self.config.advice[1],
                        0,
                        || b.ok_or(Error::Synthesis),
                    )
                    .map(ACell)?;
                let c = a.and_then(|a| b.map(|b| a + b));
                let cell_c = region
                    .assign_advice(
                        || "cell_c",
                        self.config.advice[2],
                        0,
                        || c.ok_or(Error::Synthesis),
                    )
                    .map(ACell)?;

                Ok((cell_a, cell_b, cell_c))
            },
        )
    }

    pub fn assign_row(
        &self,
        mut layouter: impl Layouter<F>,
        prev_b: &ACell<F>,
        prev_c: &ACell<F>,
    ) -> Result<ACell<F>, Error> {
        layouter.assign_region(
            || "other rows",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;

                region
                    .assign_advice(
                        || "cell_n_a",
                        self.config.advice[0],
                        0,
                        || prev_b.0.value().copied().ok_or(Error::Synthesis),
                    )
                    .map(ACell)?;

                region
                    .assign_advice(
                        || "cell_n_b",
                        self.config.advice[1],
                        0,
                        || prev_c.0.value().copied().ok_or(Error::Synthesis),
                    )
                    .map(ACell)?;

                let c = prev_b
                    .0
                    .value()
                    .and_then(|a| prev_c.0.value().map(|b| *a + *b));

                let cell_c = region
                    .assign_advice(
                        || "cell_n_c",
                        self.config.advice[2],
                        0,
                        || c.ok_or(Error::Synthesis),
                    )
                    .map(ACell)?;

                Ok(cell_c)
            },
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: &ACell<F>,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(cell.0.cell(), self.config.instance, row)
    }
}
