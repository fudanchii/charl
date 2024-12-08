use core::marker::PhantomData;

use core::fmt::Write;

pub struct WrappingOverflow;
pub struct WrappingWrap;
pub struct WrappingClip;

pub struct BehaviorStatic;
pub struct BehaviorMarqueeJumpBack;
pub struct BehaviorMarqueeContinuous;
pub struct BehaviorMarqueeScrollBack;

pub struct Line<W, B> {
    _w: PhantomData<W>,
    _b: PhantomData<B>,
}

impl Write for Line<WrappingWrap, BehaviorStatic> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}

impl Write for Line<WrappingClip, BehaviorStatic> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}

impl Write for Line<WrappingOverflow, BehaviorMarqueeJumpBack> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}

impl Write for Line<WrappingOverflow, BehaviorMarqueeContinuous> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}

impl Write for Line<WrappingOverflow, BehaviorMarqueeScrollBack> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}

pub struct Buffer<W1, B1, W2, B2, W3, B3, W4, B4> {
    line1: Line<W1, B1>,
    line2: Line<W2, B2>,
    line3: Line<W3, B3>,
    line4: Line<W4, B4>,
}

pub struct Cursor {}

impl<W1, B1, W2, B2, W3, B3, W4, B4> Buffer<W1, B1, W2, B2, W3, B3, W4, B4> {
    pub fn line1(&self) -> &Line<W1, B1> {
        &self.line1
    }

    pub fn line2(&self) -> &Line<W2, B2> {
        &self.line2
    }

    pub fn line3(&self) -> &Line<W3, B3> {
        &self.line3
    }

    pub fn line4(&self) -> &Line<W4, B4> {
        &self.line4
    }
}
