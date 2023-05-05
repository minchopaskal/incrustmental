#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod incremental {
    use std::time::Duration;
    use rand::Rng;
    use crate::prelude::AutomationId;
    use crate::types::{PerkId, ProductId, ProductMaterialId, Count, Price};
    use crate::timer::Timer;
    pub enum Quantity {
        Money(Price),
        Material(ProductMaterialId, Count),
        Product(ProductId, Count),
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Quantity {}
    #[automatically_derived]
    impl ::core::clone::Clone for Quantity {
        #[inline]
        fn clone(&self) -> Quantity {
            let _: ::core::clone::AssertParamIsClone<Price>;
            let _: ::core::clone::AssertParamIsClone<ProductMaterialId>;
            let _: ::core::clone::AssertParamIsClone<Count>;
            let _: ::core::clone::AssertParamIsClone<ProductId>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Quantity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Quantity::Money(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Money",
                        &__self_0,
                    )
                }
                Quantity::Material(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Material",
                        __self_0,
                        &__self_1,
                    )
                }
                Quantity::Product(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Product",
                        __self_0,
                        &__self_1,
                    )
                }
            }
        }
    }
    impl Quantity {
        pub fn similar(&self, other: &Quantity) -> bool {
            std::mem::discriminant(self) == std::mem::discriminant(other)
        }
        pub fn op(&self, other: &Quantity, op: fn(f64, f64) -> f64) -> Quantity {
            match (self, other) {
                (Quantity::Money(x), Quantity::Money(y)) => Quantity::Money(op(*x, *y)),
                (Quantity::Material(x, y), Quantity::Material(z, w)) => {
                    if x == z {
                        Quantity::Material(*x, op(*y as f64, *w as f64) as Count)
                    } else {
                        *self
                    }
                }
                (Quantity::Product(x, y), Quantity::Product(z, w)) => {
                    if x == z {
                        Quantity::Product(*x, op(*y as f64, *w as f64) as Count)
                    } else {
                        *self
                    }
                }
                _ => *self,
            }
        }
        pub fn quantity(&self) -> f64 {
            match &self {
                Quantity::Money(x) => *x,
                Quantity::Material(_, x) => *x as f64,
                Quantity::Product(_, x) => *x as f64,
            }
        }
    }
    pub enum RelationKind {
        Consumes,
        ManifacturedBy,
        SoldBy,
        Needs,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RelationKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RelationKind {
        #[inline]
        fn eq(&self, other: &RelationKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for RelationKind {}
    #[automatically_derived]
    impl ::core::clone::Clone for RelationKind {
        #[inline]
        fn clone(&self) -> RelationKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RelationKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    RelationKind::Consumes => "Consumes",
                    RelationKind::ManifacturedBy => "ManifacturedBy",
                    RelationKind::SoldBy => "SoldBy",
                    RelationKind::Needs => "Needs",
                },
            )
        }
    }
    pub struct Relation {
        kind: RelationKind,
        quantity: Quantity,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Relation {}
    #[automatically_derived]
    impl ::core::clone::Clone for Relation {
        #[inline]
        fn clone(&self) -> Relation {
            let _: ::core::clone::AssertParamIsClone<RelationKind>;
            let _: ::core::clone::AssertParamIsClone<Quantity>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Relation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Relation",
                "kind",
                &self.kind,
                "quantity",
                &&self.quantity,
            )
        }
    }
    impl Relation {
        pub fn new(kind: RelationKind, quantity: Quantity) -> Self {
            Self { kind, quantity }
        }
        pub fn needs(quantity: Quantity) -> Self {
            Self {
                kind: RelationKind::Needs,
                quantity,
            }
        }
        pub fn consumes(quantity: Quantity) -> Self {
            Self {
                kind: RelationKind::Consumes,
                quantity,
            }
        }
        pub fn kind(&self) -> RelationKind {
            self.kind
        }
        pub fn quantity(&self) -> &Quantity {
            &self.quantity
        }
        pub fn similar_quantity(&self, other: &Quantity) -> bool {
            self.quantity.similar(other)
        }
        fn add(&self, perk: Quantity) -> Relation {
            let mut new_cond = *self;
            new_cond.quantity = self.quantity.op(&perk, |x, y| x + y);
            new_cond
        }
        fn sub(&self, perk: Quantity) -> Relation {
            let mut new_cond = *self;
            new_cond.quantity = self.quantity.op(&perk, |x, y| x - y);
            new_cond
        }
        fn multiply(&self, perk: Quantity) -> Relation {
            let mut new_cond = *self;
            new_cond.quantity = self.quantity.op(&perk, |x, y| x * y);
            new_cond
        }
        fn divide(&self, perk: Quantity) -> Relation {
            let mut new_cond = *self;
            new_cond.quantity = self.quantity.op(&perk, |x, y| x / y);
            new_cond
        }
    }
    pub struct ProductMaterial {
        pub bought: Count,
        pub kind: String,
        pub count: Count,
        pub base_price: Price,
        pub growth: f64,
        pub active: bool,
    }
    impl ProductMaterial {
        pub fn new(
            kind: String,
            base_price: Price,
            growth: f64,
            unlocked: bool,
        ) -> Self {
            Self {
                bought: 0,
                kind,
                count: 0,
                base_price,
                growth,
                active: unlocked,
            }
        }
        pub fn price(&self) -> Price {
            self.base_price * self.growth.powf((self.bought as f64) / 10.0)
        }
        pub(crate) fn buy(&mut self) {
            self.bought += 1;
            self.count += 1;
        }
    }
    pub enum PerkKind {
        Add,
        Substract,
        Multiply,
        Divide,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PerkKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PerkKind {
        #[inline]
        fn eq(&self, other: &PerkKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    pub struct Perk {
        pub name: String,
        pub description: String,
        pub condition: Vec<Quantity>,
        pub buy_price: Vec<Quantity>,
        pub perk: (Quantity, PerkKind),
        pub unlocked: bool,
        pub active: bool,
    }
    impl Perk {
        pub fn new(
            name: String,
            description: String,
            condition: Vec<Quantity>,
            buy_price: Vec<Quantity>,
            perk: (Quantity, PerkKind),
        ) -> Self {
            Self {
                name,
                description,
                condition: condition,
                buy_price,
                perk,
                unlocked: false,
                active: false,
            }
        }
    }
    pub struct Product {
        pub name: String,
        pub count: Count,
        pub price: Option<Price>,
        pub sold: Count,
        pub dependencies: Vec<Relation>,
        pub perks: Vec<PerkId>,
        pub unlocks: Vec<(ProductId, Count)>,
        pub active: bool,
    }
    impl Product {
        pub fn interest(&self) -> f64 {
            match self.price {
                None => 0.0,
                Some(price) => {
                    if !(price > 0.) {
                        ::core::panicking::panic("assertion failed: price > 0.")
                    }
                    let init = if price < 1.0 { 0.5 } else { 0.0 };
                    let price = if price < 1.0 { 1.0 / (1.0 - price) } else { price };
                    init + 0.5 / price + ((self.sold as f64).powf(1.07) / 100.0)
                }
            }
        }
        pub fn price(&self) -> Option<Price> {
            self.price
        }
        pub fn recipe(&self) -> String {
            ::core::panicking::panic("not yet implemented");
        }
    }
    pub struct Automation {
        pub build_prod_id: ProductId,
        pub build_timer: Option<Timer>,
        pub condition: Vec<Quantity>,
        pub buy_price: Vec<Quantity>,
        pub unlocked: bool,
        pub active: bool,
    }
    impl Automation {
        pub fn new(
            automates: ProductId,
            build_timer: Option<Timer>,
            condition: Vec<Quantity>,
            buy_price: Vec<Quantity>,
        ) -> Self {
            Self {
                build_prod_id: automates,
                build_timer,
                condition,
                buy_price,
                unlocked: false,
                active: false,
            }
        }
    }
    pub struct Badge {
        pub name: String,
        pub recipe: String,
        pub condition: Vec<Quantity>,
        pub unlocked: bool,
    }
    pub struct Objective(Vec<Quantity>);
    #[automatically_derived]
    impl ::core::default::Default for Objective {
        #[inline]
        fn default() -> Objective {
            Objective(::core::default::Default::default())
        }
    }
    impl Objective {
        pub fn new(conds: Vec<Quantity>) -> Self {
            Self(conds)
        }
        pub fn win_condition(&self) -> &[Quantity] {
            &self.0
        }
    }
    pub struct State {
        pub money: f64,
        pub objective: Objective,
        pub materials: Vec<ProductMaterial>,
        pub products: Vec<Product>,
        pub badges: Vec<Badge>,
        pub perks: Vec<Perk>,
        pub automations: Vec<Automation>,
        pub win: bool,
    }
    #[automatically_derived]
    impl ::core::default::Default for State {
        #[inline]
        fn default() -> State {
            State {
                money: ::core::default::Default::default(),
                objective: ::core::default::Default::default(),
                materials: ::core::default::Default::default(),
                products: ::core::default::Default::default(),
                badges: ::core::default::Default::default(),
                perks: ::core::default::Default::default(),
                automations: ::core::default::Default::default(),
                win: ::core::default::Default::default(),
            }
        }
    }
    impl State {
        fn quantity_present_count(&self, q: &Quantity) -> Count {
            match q {
                Quantity::Money(money) => (self.money / *money).floor() as Count,
                Quantity::Material(id, cnt) => self.materials[*id].count / *cnt,
                Quantity::Product(id, cnt) => self.products[*id].sold / *cnt,
            }
        }
        fn check_conditions(&self, conds: &[Quantity]) -> bool {
            for cond in conds.iter() {
                if self.quantity_present_count(cond) == 0 {
                    return false;
                }
            }
            true
        }
        fn activate_product(&mut self, id: ProductId) {
            self.products[id].active = true;
            let mut activate_recursive = Vec::new();
            for dep in self.products[id].dependencies.iter() {
                match dep.quantity {
                    Quantity::Money(_) => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    Quantity::Material(id, _) => self.materials[id].active = true,
                    Quantity::Product(id, _) => activate_recursive.push(id),
                }
            }
            for id in activate_recursive {
                self.activate_product(id);
            }
        }
        fn apply_perk(&self, id: ProductId, cond: Relation) -> Relation {
            let mut new_cond = cond;
            for perk_id in &self.products[id].perks {
                let perk = &self.perks[*perk_id];
                if !perk.active {
                    continue;
                }
                let perk = &perk.perk;
                match &perk.1 {
                    PerkKind::Add => new_cond = new_cond.add(perk.0),
                    PerkKind::Substract => new_cond = new_cond.sub(perk.0),
                    PerkKind::Multiply => new_cond = new_cond.multiply(perk.0),
                    PerkKind::Divide => new_cond = new_cond.divide(perk.0),
                }
            }
            new_cond
        }
        fn apply_product_perks(&self, base_build_count: Count, id: ProductId) -> Count {
            let cond = Relation::needs(Quantity::Product(id, base_build_count));
            let cond = self.apply_perk(id, cond);
            if let Quantity::Product(product_id, cnt) = cond.quantity {
                if !(product_id == id) {
                    ::core::panicking::panic("assertion failed: product_id == id")
                }
                cnt
            } else {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        fn build_product_count(&mut self, id: ProductId) -> u64 {
            let mut prices = Vec::new();
            let mut max_buy_count = u64::MAX;
            let mut max_build_count = u64::MAX;
            for cond in &self.products[id].dependencies {
                let cond = self.apply_perk(id, *cond);
                let cnt = self.quantity_present_count(cond.quantity());
                if cnt == 0 && cond.quantity().quantity() > 0.0 {
                    return 0;
                }
                match cond.kind() {
                    RelationKind::Consumes => {
                        max_buy_count = max_buy_count.min(cnt);
                        prices.push(*cond.quantity());
                    }
                    RelationKind::ManifacturedBy => {
                        max_build_count = max_build_count.min(cnt);
                    }
                    _ => {}
                }
            }
            let build_count = max_buy_count.min(max_build_count);
            for price in prices {
                match price {
                    Quantity::Money(money) => {
                        if !(self.money >= build_count as f64 * money) {
                            ::core::panicking::panic(
                                "assertion failed: self.money >= build_count as f64 * money",
                            )
                        }
                        self.money -= build_count as f64 * money;
                    }
                    Quantity::Material(id, cnt) => {
                        if !(self.materials[id].count >= build_count * cnt) {
                            ::core::panicking::panic(
                                "assertion failed: self.materials[id].count >= build_count * cnt",
                            )
                        }
                        self.materials[id].count -= build_count * cnt;
                    }
                    Quantity::Product(id, cnt) => {
                        if !(self.products[id].count >= build_count * cnt) {
                            ::core::panicking::panic(
                                "assertion failed: self.products[id].count >= build_count * cnt",
                            )
                        }
                        self.products[id].count -= build_count * cnt;
                    }
                }
            }
            let built_cnt = self.apply_product_perks(build_count, id);
            built_cnt
        }
        fn construct_product(&mut self, id: ProductId) {
            let count = self.build_product_count(id);
            if count == 0 {
                return;
            }
            self.products[id].count += count;
            let mut products_to_activate = Vec::new();
            for (unlock_id, cnt) in &self.products[id].unlocks {
                if self.products[id].count > *cnt {
                    products_to_activate.push(*unlock_id);
                }
            }
            for id in products_to_activate {
                self.activate_product(id);
            }
        }
        fn sell_product(&mut self, id: ProductId) {
            let product = &self.products[id];
            if product.price.is_none() || product.interest() == 0.0 || product.count == 0
            {
                return;
            }
            let interest = product.interest().min(1.);
            let mut rng = rand::thread_rng();
            let sold = if rng.gen_bool(interest / 100.0) { 1 } else { 0 };
            if sold == 0 {
                return;
            }
            let mut sell_multiplier: Option<Count> = None;
            for dep in product.dependencies.iter() {
                if dep.kind() != RelationKind::SoldBy {
                    continue;
                }
                let cnt = self.quantity_present_count(dep.quantity());
                if cnt == 0 {
                    continue;
                }
                sell_multiplier = match sell_multiplier {
                    Some(mult) => Some(mult.min(cnt)),
                    None => Some(cnt),
                };
            }
            let sold = (sold * sell_multiplier.unwrap_or(1)).min(product.count);
            let product = &mut self.products[id];
            product.count -= sold;
            product.sold += sold;
            self.money += product.price.unwrap() * sold as f64;
        }
        pub fn update(&mut self, delta: Duration) {
            for id in 0..self.products.len() {
                if !self.products[id].active {
                    continue;
                }
                self.sell_product(id);
            }
            let mut products_to_builds = Vec::new();
            for automation in self.automations.iter_mut() {
                if !automation.active {
                    continue;
                }
                match &mut automation.build_timer {
                    Some(timer) => {
                        if timer.tick(delta) {
                            products_to_builds.push(automation.build_prod_id);
                        }
                    }
                    None => {
                        products_to_builds.push(automation.build_prod_id);
                    }
                }
            }
            for id in products_to_builds {
                self.construct_product(id);
            }
            self.win = self.check_conditions(&self.objective.0);
            if self.win {
                return;
            }
            let mut badges_to_win = Vec::new();
            for (i, badge) in self.badges.iter().enumerate() {
                if self.check_conditions(&badge.condition) {
                    badges_to_win.push(i);
                }
            }
            for id in badges_to_win {
                self.badges[id].unlocked = true;
            }
            let mut perks_to_unlock = Vec::new();
            for (id, perk) in self.perks.iter().enumerate() {
                if self.check_conditions(&perk.condition) {
                    perks_to_unlock.push(id);
                }
            }
            for id in perks_to_unlock {
                self.perks[id].unlocked = true;
            }
            let mut autos_to_unlock = Vec::new();
            for (id, auto) in self.automations.iter().enumerate() {
                if self.check_conditions(&auto.condition) {
                    autos_to_unlock.push(id);
                }
            }
            for id in autos_to_unlock {
                self.automations[id].unlocked = true;
            }
            ();
        }
        pub fn buy_material(&mut self, id: ProductMaterialId, cnt: u32) {
            for _ in 0..cnt {
                let price = self.materials[id].price();
                if self.money >= price {
                    self.materials[id].buy();
                    self.money -= price;
                }
            }
        }
        pub fn buy_perk(&mut self, id: PerkId) {
            if !self.perks[id].unlocked {
                ::core::panicking::panic("assertion failed: self.perks[id].unlocked")
            }
            if !!self.perks[id].active {
                ::core::panicking::panic("assertion failed: !self.perks[id].active")
            }
            if !self.check_conditions(&self.perks[id].buy_price) {
                return;
            }
            for price in &self.perks[id].buy_price {
                match price {
                    Quantity::Money(money) => {
                        self.money -= money;
                    }
                    Quantity::Material(id, cnt) => {
                        self.materials[*id].count -= cnt;
                    }
                    Quantity::Product(id, cnt) => {
                        self.products[*id].count -= cnt;
                    }
                }
            }
            self.perks[id].active = true;
        }
        pub fn buy_automation(&mut self, id: AutomationId) {
            if !self.automations[id].unlocked {
                ::core::panicking::panic(
                    "assertion failed: self.automations[id].unlocked",
                )
            }
            if !!self.automations[id].active {
                ::core::panicking::panic(
                    "assertion failed: !self.automations[id].active",
                )
            }
            if !self.check_conditions(&self.automations[id].buy_price) {
                return;
            }
            for price in &self.automations[id].buy_price {
                match price {
                    Quantity::Money(money) => {
                        self.money -= money;
                    }
                    Quantity::Material(id, cnt) => {
                        self.materials[*id].count -= cnt;
                    }
                    Quantity::Product(id, cnt) => {
                        self.products[*id].count -= cnt;
                    }
                }
            }
            self.automations[id].active = true;
        }
        pub fn inc_price(&mut self, id: ProductId) {
            if let Some(price) = self.products[id].price {
                self.products[id].price = Some(price + 0.01);
            }
        }
        pub fn dec_price(&mut self, id: ProductId) {
            if let Some(price) = self.products[id].price {
                self.products[id].price = Some((price - 0.01).max(0.01));
            }
        }
        pub fn win(&self) -> bool {
            self.win
        }
    }
}
pub mod loader {
    use crate::incremental::State;
    pub fn load(file: &str) -> State {
        ::core::panicking::panic("not yet implemented");
    }
}
pub mod timer {
    use std::time::Duration;
    pub struct Timer {
        duration: Duration,
        elapsed: Duration,
    }
    impl Timer {
        pub fn new(duration: Duration) -> Self {
            Self {
                duration,
                elapsed: Duration::from_secs(0),
            }
        }
        pub fn tick(&mut self, delta: Duration) -> bool {
            self.elapsed += delta;
            if self.elapsed >= self.duration {
                let times = self.elapsed.as_nanos() / self.duration.as_nanos();
                self.elapsed -= times as u32 * self.duration;
                true
            } else {
                false
            }
        }
    }
}
pub mod types {
    pub type ProductMaterialId = usize;
    pub type ProductId = usize;
    pub type BadgeId = usize;
    pub type PerkId = usize;
    pub type AutomationId = usize;
    pub type Count = u64;
    pub type Price = f64;
}
pub mod prelude {
    pub use crate::incremental::*;
    pub use crate::loader::*;
    pub use crate::timer::Timer;
    pub use crate::types::*;
}
