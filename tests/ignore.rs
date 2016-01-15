#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! ignored_tests {

    ignore "should be ingored" {
        assert!(false);
    }
}
