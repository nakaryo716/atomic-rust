use std::{
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time,
};

fn main() {
    // thread_scope();
    // rc_test();
    // arc_shadow();
    println!("{}", lock_time_mutex1());
    println!("{}", lock_time_mutex2());
}

// thread having scope
pub fn thread_scope() {
    let mut buf = vec![32, 64, 128];
    println!("{:?}", buf);

    thread::scope(|s| {
        s.spawn(|| {
            buf.push(256);
        });

        // error due to have mutable borrow and imutable borrow
        // s.spawn(move || {
        //     println!("{:?}", buf)
        // })
    });
    println!("{:?}", buf);
}

pub fn rc_test() {
    let origin = Rc::new(vec![32, 64, 128]);
    let cp = origin.clone();

    // out put example
    // origin: 0x5595c9ac5ba0
    // cp: 0x5595c9ac5ba0
    println!("origin: {:?}\n cp: {:?}", origin.as_ptr(), cp.as_ptr());
}

// when using arc, the code is offten complex,
// using shadow when spawing thread help us.
pub fn arc_shadow() {
    let a = Arc::new("Hello");

    thread::spawn({
        // first arc::clone before write closure
        let a = a.clone();
        move || {
            drop(a);
        }
    });

    println!("{}", a);
    drop(a)
}

// If you want not impl Send for structur, try "PhantomData"
#[allow(dead_code)]
pub fn phantom_data() {
    #[derive(Debug)]
    struct X {
        handle: i32,
        _not_sync: PhantomData<Rc<()>>,
    }

    let x = X {
        handle: 32,
        _not_sync: PhantomData,
    };
    println!("{:#?}", x);

    // error due to trait bound
    // struct X is not Send, so I cant send to another thread

    // thread::spawn(move || {
    //     println!("{:?}", x);
    // });
}

// take about 10 seconds
// MutexGard is droped when after sleep (the time of unlock is MutexGard droped )
pub fn lock_time_mutex1() -> i32 {
    let origin = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut gard = origin.lock().unwrap();
                for _ in 0..100 {
                    *gard += 1;
                }
                sleep(time::Duration::from_secs(1));
            });
        }
    });

    origin.into_inner().unwrap()
}

// take about 1 second
// when the task that have to .lock() (MutexGard) complete, I should drop MutexGard
// Or Take a lot time
pub fn lock_time_mutex2() -> i32 {
    let origin = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut gard = origin.lock().unwrap();
                for _ in 0..100 {
                    *gard += 1;
                }
                drop(gard);
                sleep(time::Duration::from_secs(1));
            });
        }
    });

    origin.into_inner().unwrap()
}
