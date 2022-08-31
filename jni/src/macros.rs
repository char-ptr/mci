#[macro_export]
macro_rules! unchecked_jnic {
    ($fr:expr,$fna:tt$(, $a:expr)*) => {
        unsafe {(*(*$fr)).$fna.unwrap_unchecked()($fr$(, $a)*)}
    };
}
#[macro_export]
macro_rules! unchecked_jnice {
    ($fr:expr,$fna:tt$(, $a:expr)*) => {
        (|| unsafe {
            let q = (*(*$fr)).$fna.unwrap_unchecked()($fr$(, $a)*);
            if (*(*$fr)).ExceptionCheck.unwrap_unchecked()($fr) == jdk_sys::JNI_TRUE as u8 {
                (*(*$fr)).ExceptionDescribe.unwrap_unchecked()($fr);
                println!("{}:{} caused exception",std::file!(),std::line!());
                return Err(());
            }
            Ok(q)
        })()
    };
}


#[macro_export]
macro_rules! catch {
    ( move $b:block ) => {
        (move || $b)()
    };
    ( $b:block ) => {
        (|| $b)()
    };
}