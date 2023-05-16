use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::prelude::{Count, Price, ProductMaterial, Timer};

#[derive(Deserialize, Serialize)]
pub(crate) struct ProductMaterialDef {
    init_bought: Count,
    limit: Option<Count>,
    kind: String,
    base_price: Price,
    growth: f64,
    unlocked: bool,
}

impl From<ProductMaterialDef> for ProductMaterial {
    fn from(product: ProductMaterialDef) -> Self {
        Self::new(
            product.init_bought,
            product.limit,
            product.kind,
            product.base_price,
            product.growth,
            product.unlocked,
        )
    }
}

impl Into<ProductMaterialDef> for ProductMaterial {
    fn into(self) -> ProductMaterialDef {
        ProductMaterialDef {
            init_bought: self.count(), // not entirely correct!
            limit: self.limit(),
            kind: self.name().to_string(),
            base_price: self.base_price,
            growth: self.growth,
            unlocked: self.active,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TimerDef(f64);

impl From<TimerDef> for Timer {
    fn from(timer_def: TimerDef) -> Self {
        Self::new(Duration::from_millis((timer_def.0 * 1000.) as u64))
    }
}

impl Into<TimerDef> for Timer {
    fn into(self) -> TimerDef {
        TimerDef(self.duration().as_millis() as f64 / 1000.0)
    }
}
