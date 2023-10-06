macro_rules! ptr {
    ($v: expr) => {
        $v.as_ptr().cast()
    };
}