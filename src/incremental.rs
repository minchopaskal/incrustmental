use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;

type ProductMaterialId = usize;
type ProductId = usize;

pub const MATERIALS_CNT: usize = 2;
pub const PRODUCTS_CNT: usize = 1;

pub const SHOPS: ProductMaterialId = 0;
pub const LEMONS: ProductMaterialId = 1;

pub const LEMONADE: ProductId = 0;

#[derive(Debug)]
pub enum MaterialKind {
    Lemonstand,
    Lemon,
}

pub struct ProductMaterial {
    pub bought: u64,

    pub kind: MaterialKind,
    pub count: u64,
    pub base_price: f64,
    pub growth: f64,

    pub active: bool, // wether or not the product unlocked for the player
}

impl ProductMaterial {
    pub fn price(&self) -> f64 {
        self.base_price * self.growth.powf((self.bought as f64) / 10.0)
    }

    fn try_buy(&mut self, money: &mut f64) {
        let price = self.price();
        if *money < price {
            return;
        }

        *money -= price;
        self.count += 1;
        self.bought += 1;
    }
}


pub struct Product {
    pub name: String,
    pub count: u64,
    pub price: f64,
    pub sold: u64,
    pub timer: Timer,
    pub dependencies: Vec<ProductMaterialId>,
    pub active: bool,
    pub build_fn: fn(&Product, &mut [ProductMaterial; MATERIALS_CNT], &Vec<ProductId>)->u64,

    pub unlocks: Vec<(ProductId, u64)>,
}

impl Product {
    pub fn interest(&self) -> f64 {
        0.5 / self.price + ((self.sold as f64).powf(1.07) / 100.0)
    }

    pub fn construct(
        &mut self,
        mats: &mut [ProductMaterial; MATERIALS_CNT]
    ) -> Vec<ProductId> {
        let count = (self.build_fn)(&self, mats, &self.dependencies);
        self.count += count;

        let mut unlocked = Vec::new();
        for (id, cnt) in self.unlocks.iter() {
            if self.count > *cnt {
                unlocked.push(*id);
            }
        }
        unlocked
    }
}

pub enum Stage {
    Normal,
    Buy(ProductMaterialId),
}

// One lemonade is made from 10 lemons
// Each shop can produce 1 lemonade for lemonade.timer.time() time.
pub fn build_lemonade(product: &Product, mats: &mut [ProductMaterial; MATERIALS_CNT], deps: &Vec<ProductId>) -> u64 {
    let mut shop_cnt = 0;
    let mut lemon_id = 0;
    let mut lemon_cnt = 0;

    for dep in deps.iter() {
        match mats[*dep].kind {
            MaterialKind::Lemonstand => shop_cnt = mats[*dep].count,
            MaterialKind::Lemon => {
                lemon_id = *dep;
                lemon_cnt = mats[*dep].count;
            },
            _ => (),
        }
    }

    let mut count = 0;
    if product.timer.finished() && shop_cnt >= 1 && mats[lemon_id].count >= 10 {
        let batches = (lemon_cnt / 10).min(shop_cnt);
        count = shop_cnt * batches;
        mats[lemon_id].count -= 10 * batches;
    }

    count
}

#[derive(Resource)]
pub struct State {
    pub money: f64,
    pub materials: [ProductMaterial; MATERIALS_CNT],
    pub products: [Product; PRODUCTS_CNT],

    pub stage: Stage,
    pub exit: bool,
}

impl State {
    pub fn activate_product(&mut self, id: ProductId) {
        self.products[id].active = true;

        for dep in self.products[id].dependencies.iter() {
            self.materials[*dep].active = true;
        }
    }

    fn update_product(&mut self, id: ProductId, delta: Duration) {
        if !self.products[id].active {
            return;
        }

        let mut interest = self.products[id].interest();

        let has_product = self.products[id].count > 0;
        let mut sold = 0;
        if has_product {
            if interest > 1. {
                sold += interest.floor() as u64;
                interest = interest.fract();
            }
            let mut rng = rand::thread_rng();

            let sell = rng.gen_bool(interest / 100.0);
            sold = (sold + if sell { 1 } else { 0 }).min(self.products[id].count);
        }

        self.products[id].timer.tick(delta);
        
        let unlocked = self.products[id].construct(&mut self.materials);
        if !unlocked.is_empty() {
            for id in unlocked {
                self.activate_product(id);
            }
        }

        if sold > 0 {
            self.products[id].count -= 1;
            self.products[id].sold += sold;
            self.money += self.products[id].price * sold as f64;
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for id in 0..self.products.len() {
            self.update_product(id, delta);
        }
    }

    pub fn buy_material(&mut self, id: ProductMaterialId, cnt: u32) {
        for _ in 0..cnt {
            self.materials[id].try_buy(&mut self.money);
        }
    }

    pub fn inc_price(&mut self, id: ProductId) {
        self.products[id].price += 0.01;
    }
    
    pub fn dec_price(&mut self, id: ProductId) {
        self.products[id].price = (self.products[id].price - 0.01).max(0.01);
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
