use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;

pub trait Reduc {
    fn connect_client(&mut self); 
}

#[derive(Debug, Default)]
pub struct Client {
    pub screen_num: usize,
    pub window_id: u32,
}

impl Reduc for Client {

    fn connect_client(&mut self) {
        
        let (con, num) = x11rb::connect(None).unwrap();
        self.screen_num = num;
        self.window_id = con.generate_id().unwrap();

        let screen = &con.setup().roots[self.screen_num];
        con.create_window(
            COPY_DEPTH_FROM_PARENT,
            self.window_id,
            screen.root,
            0,
            0,
            600,
            600,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().background_pixel(screen.white_pixel),
        ).unwrap();
        con.map_window(self.window_id).unwrap();
        con.flush().unwrap();
        loop {
            println!("Event: {:?}", con.wait_for_event().unwrap());
        }
    }
}
