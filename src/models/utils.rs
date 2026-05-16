use chrono::Local;

pub struct Logger<'a>{
    module_name:&'a str,
}

impl Logger<'_> {
    pub const fn new(module_name:&'_ str)->Logger<'_>{
        Logger{
            module_name
        }
    }

    fn get_formated_time()->String{
        let current_time = Local::now();
        current_time.format("%m-%d-%Y %H:%M:%S").to_string()
    }

    pub fn info(&self, data: &str){
        println!("[{}] [{}] : {}",Self::get_formated_time(), &self.module_name, data)
    }
}