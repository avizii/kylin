use crate::config::*;
use crate::mm::page_table::PageTableEntry;
use core::fmt::{Debug, Formatter};
use core::slice::from_raw_parts_mut;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicAddress(pub usize);

impl PhysicAddress {
    pub fn page_offset(&self) -> usize {
        let mask = PAGE_SIZE - 1;
        self.0 & mask
    }

    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }

    pub fn floor(&self) -> PhysicPageNum {
        PhysicPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysicPageNum {
        PhysicPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddress(pub usize);

impl VirtualAddress {
    pub fn floor(&self) -> VirtualPageNum {
        VirtualPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtualPageNum {
        VirtualPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicPageNum(pub usize);

impl PhysicPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysicAddress = (*self).into();
        unsafe { from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512) }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysicAddress = (*self).into();
        unsafe { from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysicAddress = (*self).into();
        unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualPageNum(pub usize);

impl VirtualPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0_usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}

impl Debug for PhysicAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl Debug for VirtualAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl Debug for PhysicPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

impl Debug for VirtualPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}

impl From<usize> for PhysicAddress {
    fn from(v: usize) -> Self {
        let mask = (1 << PA_WIDTH_SV39) - 1; // 0x00ff_ffff_ffff_ffff
        Self(v & mask)
    }
}

impl From<usize> for PhysicPageNum {
    fn from(v: usize) -> Self {
        let mask = (1 << PPN_WIDTH_SV39) - 1;
        Self(v & mask)
    }
}

impl From<usize> for VirtualAddress {
    fn from(v: usize) -> Self {
        let mask = (1 << VA_WIDTH_SV39) - 1;
        Self(v & mask)
    }
}

impl From<usize> for VirtualPageNum {
    fn from(v: usize) -> Self {
        let mask = (1 << VPN_WIDTH_SV39) - 1;
        Self(v & mask)
    }
}

impl From<PhysicAddress> for usize {
    fn from(v: PhysicAddress) -> Self {
        v.0
    }
}

impl From<PhysicPageNum> for usize {
    fn from(v: PhysicPageNum) -> Self {
        v.0
    }
}

impl From<VirtualAddress> for usize {
    fn from(v: VirtualAddress) -> Self {
        v.0
    }
}

impl From<VirtualPageNum> for usize {
    fn from(v: VirtualPageNum) -> Self {
        v.0
    }
}

impl From<PhysicAddress> for PhysicPageNum {
    fn from(v: PhysicAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysicPageNum> for PhysicAddress {
    fn from(v: PhysicPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtualAddress> for VirtualPageNum {
    fn from(v: VirtualAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtualPageNum> for VirtualAddress {
    fn from(v: VirtualPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

pub trait StepByOne {
    fn step(&mut self);
}

impl StepByOne for VirtualPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

pub struct SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    l: T,
    r: T,
}

impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}", start, end);
        Self { l: start, r: end }
    }

    pub fn get_start(&self) -> T {
        self.l
    }

    pub fn get_end(&self) -> T {
        self.r
    }
}

pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T,
}

impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}

impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}

pub type VPNRange = SimpleRange<VirtualPageNum>;
