// pub struct LsArgs{
//     pub all: bool,
//     pub long: bool,
//     pub path: String,
// }
// 
// impl LsArgs {
//     fn new() -> Self{
//         LsArgs{
//             all: false,
//             long: false,
//             path: ".".to_string(),
//         }
//     }
// 
//     fn set_arg(&mut self, arg: Vec<String>) {
//         self.path = ".".to_string();
//         let mut args = arg.into_iter();
//         while let Some(arg) = args.next(){
//             match arg.as_str() {
//                 "-a" => self.all = true,
//                 "-l" => self.long = true,
//                 _ => self.path = arg,
//             }
//         }
//     }
// }
