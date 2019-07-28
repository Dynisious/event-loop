//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-07-28

/// An `Entity` has an update function which optionally produces an output.
pub trait Entity {
  /// The output type produced by this `Entity`.
  type Output;
  /// The context of this `Entity`s update.
  type Context;

  /// Update this `Entity` and optionally produce an output.
  /// 
  /// # Params
  /// 
  /// context --- The context of this `Entity`s update.  
  fn update(&mut self, context: Self::Context,) -> Self::Output;
}

/// An `Event` is a pending computation within the context of an event loop.
pub trait Event {
  /// The output type returned by this `Event`.
  type Output;
  /// The context of this `Event`s execution.
  type Context;

  /// Execute this `Event`.
  /// 
  /// # Params
  /// 
  /// context --- The context of this `Event`s execution.  
  fn execute(self, context: Self::Context,) -> Self::Output;
}
