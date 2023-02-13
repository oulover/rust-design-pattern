use std::sync::Arc;
use std::sync::Mutex;

static mut SINGLETON_INSTANCE: Option<Singleton> = None;

#[derive(Debug)]
struct Singleton {
    v: usize,
}

impl Singleton {
    fn new() -> &'static mut Singleton {
        unsafe {
            match SINGLETON_INSTANCE {
                Some(ref mut obj) => obj,
                None => {
                    SINGLETON_INSTANCE = Some(Singleton { v: 100 });
                    Singleton::new()
                }
            }
        }
    }
}

#[derive(Debug)]
struct SingletonExample {
    v: usize,
}

impl SingletonExample {
    pub fn get_instance() -> Arc<Mutex<SingletonExample>> {
        static mut POINT: Option<Arc<Mutex<SingletonExample>>> = None;

        unsafe {// Rust中使用可变静态变量都是unsafe的
            POINT.get_or_insert_with(|| {
                // 初始化单例对象的代码
                Arc::new(Mutex::new(SingletonExample { v: 0 }))
            }).clone()
        }
    }


    pub fn get_instance2() -> Arc<SingletonExample> {
        static mut POINT: Option<Arc<SingletonExample>> = None;

        unsafe {// Rust中使用可变静态变量都是unsafe的
            POINT.get_or_insert_with(|| {
                // 初始化单例对象的代码
                Arc::new(SingletonExample { v: 0 })
            }).clone()
        }
    }

    // 返回&'static Point
    pub fn get_instance3 () -> & 'static SingletonExample {
        static mut POINT: Option<Arc<SingletonExample>> = None;

        unsafe {// Rust中使用可变静态变量都是unsafe的
            POINT.get_or_insert_with(|| {
                // 初始化单例对象的代码
                Arc::new(SingletonExample { v: 0 })
            });
            POINT.as_ref().unwrap()
        }
    }
}


fn main() {
    let mut  s1 = SingletonExample::get_instance();
    let  mut s2 = SingletonExample::get_instance();
    println!("{:?}", s1);
    println!("{:?}", s2);

    println!("{:?}", s1);
    println!("{:?}", s2);
}


