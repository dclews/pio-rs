#![feature(no_std, asm, const_fn)]
#![no_std]

pub struct Port {
    port: u16
}

impl Port {
    pub const fn new(port: u16) -> Port {
        Port{port:port}
    }
    pub unsafe fn read_u8(&self) -> u8 {
        let value: u8;
        asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile"); 
        value
    }
    pub unsafe fn read_u16(&self) -> u16 {
        let value: u16;
        asm!("in $0, $1" : "={ax}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile"); 
        value
    }
    pub unsafe fn read_u32(&self) -> u32 {
        let value: u32;
        asm!("in $0, $1" : "={eax}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        value
    }
    pub unsafe fn write_u8(&mut self, data: u8) {
        asm!("out $1, $0" : : "{al}"(data), "{dx}"(self.port) : "memory" : "intel", "volatile");
    }
    pub unsafe fn write_u16(&mut self, data: u16) {
        asm!("out $1, $0" : : "{ax}"(data), "{dx}"(self.port) : "memory" : "intel", "volatile");
    }
    pub unsafe fn write_u32(&mut self, data: u32) {
        asm!("out $1, $0" : : "{eax}"(data), "{dx}"(self.port) : "memory" : "intel", "volatile");
    }
}
