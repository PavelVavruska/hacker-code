use std::collections::HashMap;
use core::hash::Hash;

struct Cacher_u32<T: Fn(u32) -> u32> {
    computation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher_u32<T> {
    pub fn new(computation: T) -> Cacher_u32<T> {
        Cacher_u32 {
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

struct Cacher<T: Fn(U) -> U, U: Copy> {
    computation: T,
    value: Option<U>,
}

impl<T: Fn(U) -> U, U: Copy> Cacher<T, U> {
    pub fn new(computation: T) -> Cacher<T, U> {
        Cacher {
            computation,
            value: None,
        }
    }

    pub fn value(&mut self, input_value: U) -> U {
        match self.value {
            None => {
                let v = (self.computation)(input_value);
                self.value = Some(v);
                return v;
            },
            Some(v) => v,
        }
    }
}

/// Computation closure with support of different types as input/output, 
/// e.g. input a string slice and return usize values
struct CacherHash<T: Fn(K) -> V, K: Eq+Hash, V: Copy+Eq+Hash > {
    computation: T,
    map: HashMap<K, V>,
}

impl<T: Fn(K) -> V, K: Clone+Eq+Hash, V: Copy+Eq+Hash> CacherHash<T, K, V> {
    pub fn new(computation: T) -> CacherHash<T, K, V> {
        CacherHash {
            computation,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, input_value: K) -> V {
        match self.map.get(&input_value) {
            None => {
                let v = (self.computation)(input_value.clone());
                self.map.insert(input_value, v);
                return v;
            },
            Some(v) => v.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cacher_u32, Cacher, CacherHash};
    use std::time::Instant;

    #[test]
    fn test_u32() {
        let closure = |x| (x + 1);
        let mut cacher = Cacher_u32::new(closure);

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

    #[test]
    fn test_generic_i32() {
        let closure = |x| (x + 1);
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

    #[test]
    fn test_generic_usize() {
        let closure = |x| (x + 1)  as usize;
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

    #[test]
    fn test_generic_hashmap_usize() {
        let closure = |x| (x*x*x*x/(x-1)/(x-1)/(x-1) + 1) as usize;  // x must be != 1 
        let mut cacher = CacherHash::new(closure);

        for i in 1..10 {
            let start = Instant::now();
            println!("{} Cacher value 1 {}", i, cacher.value(5));
            let duration1 = start.elapsed();
            println!("{} Time elapsed in expensive_function() is: {:?}", i, duration1);
        }

        println!();  

        for i in 1..10 {
            let start = Instant::now();
            println!("{} Cacher value 2 {}", i, cacher.value(10));
            let duration1 = start.elapsed();
            println!("{} Time elapsed in expensive_function() is: {:?}", i, duration1);
        }
    
    }

    #[test]
    fn test_generic_hashmap_usize_string_slice() {
        let closure = |x: &str| {
            for i in 1..1000 {
                print!("."); // simulate slow computation
            }
            
            x.len()
        };
        let mut cacher = CacherHash::new(closure);

        for i in 1..10 {
            let start = Instant::now();
            println!("{} Cacher value 1 {}", i, cacher.value("12345"));
            let duration1 = start.elapsed();
            println!("{} Time elapsed in expensive_function() is: {:?}", i, duration1);
        }

        println!();  

        for i in 1..10 {
            let start = Instant::now();
            println!("{} Cacher value 2 {}", i, cacher.value("1234567890"));
            let duration1 = start.elapsed();
            println!("{} Time elapsed in expensive_function() is: {:?}", i, duration1);
        }
    
    }
}