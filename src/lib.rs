#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # RcString
//!
//! Reference counted strings and slices.

use std::rc::Rc;
use std::sync::Arc;
use std::mem;

pub trait IntoRc {
    type Rc;
    type Arc;

    fn rc(self) -> Self::Rc;
    fn arc(self) -> Self::Arc;
}

pub trait RcStringExt {
    type RcSlice: RcStrExt;

    fn as_rc_slice(&self) -> Self::RcSlice;
    fn rc_slice_to(&self, usize) -> Self::RcSlice;
    fn rc_slice_from(&self, usize) -> Self::RcSlice;
    fn rc_slice(&self, usize, usize) -> Self::RcSlice;
    fn rc_split_at(&self, usize) -> (Self::RcSlice, Self::RcSlice);
}

pub trait RcStrExt {
    type RcOwned: RcStringExt;

    fn to_rc_string(&self) -> Self::RcOwned;

    fn rc_slice_to(&self, usize) -> Self;
    fn rc_slice_from(&self, usize) -> Self;
    fn rc_slice_at(&self, usize, usize) -> Self;
    fn rc_split_at(&self, usize) -> (Self, Self);
}

impl RcStringExt for Rc<String> {
    type RcSlice = RcStr;

    fn as_rc_slice(&self) -> RcStr {
        RcStr {
            data: &self[] as *const _,
            rc: self.clone()
        }
    }

    fn rc_slice_to(&self, to: usize) -> RcStr {
        RcStr {
            data: &self[..to] as *const _,
            rc: self.clone()
        }
    }

    fn rc_slice_from(&self, from: usize) -> RcStr {
        RcStr {
            data: &self[from..] as *const _,
            rc: self.clone()
        }
    }

    fn rc_slice(&self, from: usize, to: usize) -> RcStr {
        RcStr {
            data: &self[from..to] as *const _,
            rc: self.clone()
        }
    }

    fn rc_split_at(&self, at: usize) -> (RcStr, RcStr) {
        (RcStr {
            data: &self[..to] as *const _,
            rc: self.clone()
        }, RcStr {
            data: &self[to..] as *const _,
            rc: self.clone()
        })
    }
}

impl RcStringExt for Arc<String> {
    type RcSlice = ArcStr;

    fn as_rc_slice(&self) -> ArcStr {
        ArcStr {
            data: &self[] as *const _,
            arc: self.clone()
        }
    }

    fn rc_slice_to(&self, to: usize) -> ArcStr {
        ArcStr {
            data: &self[..to] as *const _,
            arc: self.clone()
        }
    }

    fn rc_slice_from(&self, from: usize) -> ArcStr {
        ArcStr {
            data: &self[from..] as *const _,
            arc: self.clone()
        }
    }

    fn rc_slice(&self, from: usize, to: usize) -> ArcStr {
        ArcStr {
            data: &self[from..to] as *const _,
            arc: self.clone()
        }
    }

    fn rc_split_at(&self, at: usize) -> (ArcStr, ArcStr) {
        (ArcStr {
            data: &self[..to] as *const _,
            arc: self.clone()
        }, ArcStr {
            data: &self[to..] as *const _,
            arc: self.clone()
        })
    }
}

impl RcStrExt for RcStr {
    type RcOwned = Rc<String>;

}

impl RcStrExt for ArcStr {
    type RcOwned = Arc<String>;

}

impl IntoRc for String {
    type Rc = Rc<String>;
    type Arc = Arc<String>;

    fn rc(self) -> Rc<String> { Rc::new(self) }
    fn arc(self) -> Arc<String> { Arc::new(self) }
}

#[derive(Clone)]
pub struct RcStr {
    data: *const str,
    // Unless the internals of Rc are exposed, we can't do better than this.
    //
    // In an ideal world we would store a `*mut Cell<usize>` which points to
    // the strong count of the underlying `Rc<String>`, eliminating an
    // unnecessary layer of indirection.
    rc: Rc<String>
}

impl Deref for RcStr {
    type Target = str;

    fn deref(&self) -> &str { self.data }
}

#[derive(Clone)]
pub struct ArcStr {
    data: *const str,

    // Ditto from the RcStr comment, except we'd store a reference to the
    // AtomicUsize.
    arc: Arc<String>
}

impl Deref for ArcStr {
    type Target = str;

    fn deref(&self) -> &str { self.data }
}

