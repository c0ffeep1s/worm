
extern crate spinner;


pub struct Spinner {
    handle: spinner::SpinnerHandle,
}


impl Spinner {
    
    pub fn new(message: String) -> Self {
        let v = vec!["/", "|", "\\", "-", "/", "|", "\\", "-"];
        let handle = spinner::SpinnerBuilder::new(message.into()).spinner(v).start();

        // return Spinner
        Spinner {handle}
    }

    // stop Spinner
    pub fn stop(self) {
        self.handle.close();
    }
}
