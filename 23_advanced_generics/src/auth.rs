pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advanced_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature2 for {}", self.name);
    }
}

impl Personal for Customer<Personal> {
    fn advanced_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        );
    }
}

pub struct FreePlan;
pub struct PersonalPlan(i32);
