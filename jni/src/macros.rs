#[macro_export]
macro_rules! unchecked_jnic {
    ($fr:expr,$fna:tt$(, $a:expr)*) => {
        unsafe {(*(*$fr)).$fna.unwrap_unchecked()($fr$(, $a)*)}
    };
}