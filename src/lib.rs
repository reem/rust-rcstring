#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # RcString
//!
//! Reference counted strings and slices.

use std::rc::Rc;
use std::sync::Arc;

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

}

impl RcString for Arc<String> {
    type RcSlice = ArcStr;

}

impl RcStrExt for RcStr {
    type RcOwned = Rc<String>;

}

impl RcStrExt for ArcStr {
    type RcOwned = Arc<String>;

}

impl IntoRc for String {
    type Rc = Rc<String>;

    fn rc(self) -> Rc<String> { Rc::new(self) }
    fn arc(self) -> Arc<String> { Arc::new(self) }
}

#[derive(Clone)]
pub struct RcStr {
    data: &'static str,
    rc: Rc<String>
}

#[derive(Clone)]
pub struct ArcStr {
    data: &'static str,
    arc: Arc<String>
}

