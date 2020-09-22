// This is rusts style of OOP programming. Limit the means to mutate data through methods implemented on the structs.
// create struct AverageColelction and make public, but keep data within private
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

// implement methods on the struct to manage the functionality open to it
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self ) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}