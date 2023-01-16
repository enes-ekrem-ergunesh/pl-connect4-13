pub mod practice {

    /**
     * How to use references to change a variable of the other class
     */
    pub mod references {
        /**
         * modify the reference
         */
        pub fn modify(num: &mut u32) {
            *num = 4;
            println!("num in bool_class: {}", num);
        }

        use lazy_static::lazy_static;
        use std::sync::atomic::{AtomicI32, Ordering};

        lazy_static! {
            static ref X: AtomicI32 = AtomicI32::new(5);
        }
        pub fn read_x() -> i32 {
            X.load(Ordering::Relaxed)
        }
        
        pub fn update_x(new_x:i32) {
            X.store(new_x, Ordering::Relaxed);
        }
    }

    /**
     * How to print an element from bool array?
     */
    pub fn get_bool() {
        let mut a: [[bool; 6]; 7];
        a = [
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
        ];
        println!("{}", a[3][5]);
    }

    /**
     * How to print an array?
     */
    pub fn bool_array() {
        let mut a: [[bool; 6]; 7];
        a = [
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
            [false, true, false, true, false, true],
        ];
        println!("{:?}", a);
    }

    /**
     * How to print a bool?
     */
    pub fn bool() {
        let a = true;
        println!("{}", a);
    }

    /**
     * How to initialize a matrix with reference variables?
     */
    pub fn init_matrix() -> [[&'static u8; 7]; 6] {
        let mut matrix: [[&u8; 7]; 6] = [
            [&0, &0, &0, &0, &0, &0, &0],
            [&0, &0, &0, &0, &0, &0, &0],
            [&0, &0, &0, &0, &0, &0, &0],
            [&0, &0, &0, &0, &0, &0, &0],
            [&0, &0, &0, &0, &0, &0, &0],
            [&0, &0, &0, &0, &0, &0, &0],
        ];
        matrix
    }

    /**
     * How to sleep?
     */
    use std::{thread, time};
    pub fn sleep(time_in_millis: u64) {
        let ten_millis = time::Duration::from_millis(time_in_millis);
        let now = time::Instant::now();

        thread::sleep(ten_millis);

        assert!(now.elapsed() >= ten_millis);
    }
}
