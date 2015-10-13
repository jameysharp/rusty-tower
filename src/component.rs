pub trait Emitter<T> {
	fn emit(&mut self, event: T);
}

pub trait Monitor {
	const MONITOR_INIT : Self;
}

macro_rules! monitor_state {
	($monitor:ident {}) => (
		pub struct $monitor;
		impl Monitor for $monitor {
			const MONITOR_INIT : $monitor = $monitor;
		}
	);
	($monitor:ident {
		$( $statename:ident : $statetype:ty = $stateinit:expr, )+
	}) => (
		pub struct $monitor {
			$( $statename : $statetype, )*
		}

		impl Monitor for $monitor {
			const MONITOR_INIT : $monitor = $monitor {
				$( $statename : $stateinit, )*
			};
		}
	);
}

macro_rules! monitor {
	(
		state $monitor:ident $state:tt
		$( handler $name:ident ( $_self:ident , $chan:ident : $chantype:ty ) ($($emitter:ident : $emittype:ty),*) $body:block )*
	) => {
		monitor_state!($monitor $state);
		impl $monitor {
			$( pub fn $name(&mut self, $chan : $chantype $(, $emitter : &mut Emitter<$emittype>)*)
			{
				let $_self = self;
				$body
			})*
		}
	}
}

monitor! {
	state Counter {
		count: usize = 0,
		enabled: bool = false,
	}

	handler event (state, _event: ()) (current: usize)
	{
		if state.enabled {
			state.count += 1;
			current.emit(state.count);
		}
	}

	handler set_enabled (state, enabled: bool) ()
	{
		state.enabled = enabled;
	}
}

monitor! {
	state Printer { }

	handler count_changed (_state, new_count: usize) ()
	{
		println!("new count: {}", new_count);
	}
}
