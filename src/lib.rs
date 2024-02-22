pub struct Any(Box<dyn std::any::Any>);

impl Any {
    pub fn new<T: 'static>(value: T) -> Self {
        Any(Box::new(value))
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut()
    }
}
