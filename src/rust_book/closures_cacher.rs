struct Cacher<T: Fn(u32) -> u32> {
    computation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    pub fn new(computation: T) -> Cacher<T> {
        Cacher {
            computation,
            value: None,
        }
    }

    pub fn value(&mut self, number: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.computation)(number);
                self.value = Some(v);
                return v;
            },
            Some(v) => v,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Cacher;
    use std::time::Instant;

    #[test]
    fn test_main() {
        let closure = |x| (x + 1)*(x + 2)/x;
        let mut cacher = Cacher::new(closure);

        let start = Instant::now();
        println!("Cacher value 1 {}", cacher.value(1));
        let duration1 = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration1);

        let start = Instant::now();
        println!("Cacher value 2 {}", cacher.value(2));
        let duration2 = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration2);
        
        let start = Instant::now();
        println!("Cacher value 3 {}", cacher.value(3));
        let duration3 = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration3);
    
    }
}