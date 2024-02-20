// struct s_average_system {
//         name: String,
//         class: String,
//         tests: Vec<f64>,
//         average: Option<f64>,
// }

// impl average_system{
//     pub fn new(name: &str, class: &str, tests: Vec<f64>, average: f64) -> Self {
//         Self {
//             name: name.to_string(),
//             class: class.to_string(),
//             tests,
//             average
//         }
//     }

//     pub fn get_average(&self) -> s_average_system {
//         let mut average: f64 = self.tests.iter().sum() / self.tests.len();

//         { 
//             name: self.name,
//             class: self.class,
//             average: average
//         }
//     }
// }