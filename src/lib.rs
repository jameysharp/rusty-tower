#![feature(associated_consts)]

pub mod component;

use component::{Monitor,Emitter};

struct Buffer<T> { contents : Option<T> }

impl<T> Buffer<T> {
	fn new() -> Buffer<T>
	{
		Buffer { contents : None }
	}
}

impl<T> Emitter<T> for Buffer<T> {
	fn emit(&mut self, event: T)
	{
		assert!(self.contents.is_none());
		self.contents = Some(event);
	}
}

#[test]
fn it_works() {
	let mut ctr = component::Counter::MONITOR_INIT;
	let mut prn = component::Printer::MONITOR_INIT;
	let mut buf = Buffer::new();

	ctr.set_enabled(true);
	ctr.event((), &mut buf);
	for event in buf.contents {
		prn.count_changed(event);
	}
}
