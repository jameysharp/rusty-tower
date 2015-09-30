pub trait Emitter<T> {
	fn emit(&mut self, event: T);
}

pub trait Monitor {
	const MONITOR_INIT : Self;
}

pub struct Counter {
	count: usize,
	enabled: bool,
}

impl Monitor for Counter {
	const MONITOR_INIT : Counter = Counter { count: 0, enabled: false };
}

impl Counter {
	pub fn event<T : Emitter<usize>>(&mut self, _event: (), current: &mut T)
	{
		if self.enabled {
			self.count += 1;
			current.emit(self.count);
		}
	}

	pub fn set_enabled(&mut self, enabled: bool)
	{
		self.enabled = enabled;
	}
}

pub struct Printer;

impl Monitor for Printer {
	const MONITOR_INIT : Printer = Printer;
}

impl Printer {
	pub fn count_changed(&mut self, new_count: usize)
	{
		println!("new count: {}", new_count);
	}
}
