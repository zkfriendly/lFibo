use halo2_proofs::circuit::{AssignedCell, Layouter};
use halo2_proofs::plonk::{Advice, Column, Error};
use halo2_proofs::poly::Rotation;
use halo2_proofs::{arithmetic::FieldExt, plonk::*};
use std::marker::PhantomData;
use std::result;

#[derive(Clone, Debug)]
pub struct FiboConfig {
    pub advice: Column<Advice>,
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
        advice: Column<Advice>,
        instance: Column<Instance>,
    ) -> FiboConfig {
        let selector = meta.selector();

        meta.enable_equality(advice);
        meta.enable_equality(instance);

        // col_a  | col_b  | col_c | selector
        //   a       b         c       s
        meta.create_gate("add", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(advice, Rotation::cur());
            let b = meta.query_advice(advice, Rotation::next());
            let c = meta.query_advice(advice, Rotation(2));
            vec![s * (a + b - c)]
        });

        FiboConfig {
            advice,
            selector,
            instance,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        nrows: usize,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "first row",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;
                let mut a_cell = region.assign_advice_from_instance(
                    || "f(0)",
                    self.config.instance,
                    0,
                    self.config.advice,
                    0,
                )?;

                let mut b_cell = region.assign_advice_from_instance(
                    || "f(1)",
                    self.config.instance,
                    0,
                    self.config.advice,
                    1,
                )?;

                for i in 2..nrows {
                    if i < nrows - 2 {
                        self.config.selector.enable(&mut region, i)?;
                    }

                    let r = a_cell
                        .value()
                        .copied()
                        .and_then(|a| b_cell.value().copied().map(|b| a + b));

                    let c_cell = region.assign_advice(
                        || "f(n)",
                        self.config.advice,
                        i,
                        || r.ok_or(Error::Synthesis),
                    )?;

                    a_cell = b_cell;
                    b_cell = c_cell;
                }

                region.assign_advice_from_instance(
                    || "out",
                    self.config.instance,
                    2,
                    self.config.advice,
                    nrows - 1,
                )?;

                Ok(b_cell)
            },
        )
    }
}
